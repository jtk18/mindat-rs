//! Smoke test / demo for the endpoints added in 0.2.0.
//!
//! Most of these require authentication. Run with:
//!
//! ```bash
//! MINDAT_API_KEY=your_key cargo run --example new_endpoints
//! ```

use mindat_rs::*;

async fn report<T>(name: &str, r: Result<T>, f: impl Fn(&T) -> String) {
    match r {
        Ok(v) => println!("  OK   {:<26} {}", name, f(&v)),
        Err(e) => println!("  FAIL {:<26} {}", name, e),
    }
}

#[tokio::main]
async fn main() {
    let key = std::env::var("MINDAT_API_KEY").expect("Set MINDAT_API_KEY");
    let c = MindatClient::new(key);

    // A well-known mineral id (Quartz) used as a filter anchor.
    let quartz = 3337;

    println!("--- References ---");
    report(
        "references",
        c.references(ReferencesQuery::new().search("quartz").page_size(3))
            .await,
        |p| format!("first_id={:?}", p.results.first().map(|r| r.ref_id)),
    )
    .await;
    report("reference_types", c.reference_types(None).await, |p| {
        format!("{} rows", p.results.len())
    })
    .await;
    report(
        "reference_citations",
        c.reference_citations(ReferenceCitationsQuery::new().mineral(quartz).page_size(3))
            .await,
        |p| format!("{} rows", p.results.len()),
    )
    .await;

    println!("--- Occurrences ---");
    report(
        "occurrences",
        c.occurrences(OccurrencesQuery::new().mineral(quartz).page_size(3))
            .await,
        |p| format!("{} rows", p.results.len()),
    )
    .await;
    report(
        "occurrence_statistics",
        c.occurrence_statistics(
            OccurrenceStatisticsQuery::new()
                .mineral(quartz)
                .page_size(3),
        )
        .await,
        |p| format!("{} rows", p.results.len()),
    )
    .await;
    // `inc` is required and the upstream endpoint 500s without a non-empty `exc`.
    report("loc_by_min", c.loc_by_min("52", Some("1")).await, |v| {
        format!("{} locality ids", v.len())
    })
    .await;

    println!("--- Crystallography / Relations / Translations ---");
    report(
        "crystal_classes",
        c.crystal_classes(CrystalClassesQuery::new()).await,
        |p| format!("{} rows", p.results.len()),
    )
    .await;
    report(
        "space_groups",
        c.space_groups(SpaceGroupsQuery::new()).await,
        |p| format!("{} rows", p.results.len()),
    )
    .await;
    report(
        "relations",
        c.relations(RelationsQuery::new().mineral(quartz)).await,
        |p| format!("{} rows", p.results.len()),
    )
    .await;
    report(
        "locality_translations",
        c.locality_translations(LocalityTranslationsQuery::new().page_size(3))
            .await,
        |p| format!("{} rows", p.results.len()),
    )
    .await;

    println!("--- Newly-wired multi-choice filter (should return only Hexagonal) ---");
    match c
        .geomaterials(
            GeomaterialsQuery::new()
                .crystal_systems(vec![CrystalSystem::Hexagonal])
                .ima_approved(true)
                .page_size(5),
        )
        .await
    {
        Ok(p) => {
            let systems: Vec<String> = p.results.iter().filter_map(|g| g.csystem.clone()).collect();
            println!(
                "  {} results, crystal systems: {:?}",
                p.results.len(),
                systems
            );
        }
        Err(e) => println!("  FAIL geomaterials filter: {}", e),
    }
}
