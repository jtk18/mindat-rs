use mindat_rs::{GeomaterialsQuery, ImaMineralsQuery, LocalitiesQuery, MindatClient};

#[tokio::main]
async fn main() -> mindat_rs::Result<()> {
    let token = std::env::var("MINDAT_API_KEY").expect("Set MINDAT_API_KEY env var");
    let client = MindatClient::new(&token);

    println!("=== Testing Mindat API ===\n");

    // Test 1: IMA Minerals (no auth required)
    println!("1. Testing IMA Minerals (anonymous)...");
    let anon_client = MindatClient::anonymous();
    let ima_query = ImaMineralsQuery::new().page_size(3);
    match anon_client.minerals_ima(ima_query).await {
        Ok(result) => {
            println!(
                "   ✓ Found {} IMA minerals (showing first 3):",
                result.count.unwrap_or(0)
            );
            for m in result.results.iter().take(3) {
                println!(
                    "     - {} ({})",
                    m.name.as_deref().unwrap_or("?"),
                    m.ima_formula.as_deref().unwrap_or("?")
                );
            }
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    // Test 2: Countries
    println!("\n2. Testing Countries...");
    match client.countries().await {
        Ok(result) => {
            println!("   ✓ Found {} countries", result.count.unwrap_or(0));
            for c in result.results.iter().take(3) {
                println!("     - {} ({})", c.text, c.iso);
            }
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    // Test 3: Geomaterials - search for quartz
    println!("\n3. Testing Geomaterials (search: quartz)...");
    let geo_query = GeomaterialsQuery::new()
        .name("quartz")
        .ima_approved(true)
        .page_size(5);
    match client.geomaterials(geo_query).await {
        Ok(result) => {
            println!(
                "   ✓ Found {} geomaterials matching 'quartz'",
                result.count.unwrap_or(0)
            );
            for m in result.results.iter().take(5) {
                println!("     - {} (ID: {})", m.name.as_deref().unwrap_or("?"), m.id);
            }
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    // Test 4: Geomaterials - by elements
    println!("\n4. Testing Geomaterials (elements: Cu,S)...");
    let geo_query2 = GeomaterialsQuery::new()
        .with_elements("Cu,S")
        .ima_approved(true)
        .page_size(5);
    match client.geomaterials(geo_query2).await {
        Ok(result) => {
            println!(
                "   ✓ Found {} minerals with Cu and S",
                result.count.unwrap_or(0)
            );
            for m in result.results.iter().take(5) {
                println!(
                    "     - {} ({})",
                    m.name.as_deref().unwrap_or("?"),
                    m.mindat_formula.as_deref().unwrap_or("?")
                );
            }
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    // Test 5: Localities
    println!("\n5. Testing Localities (country: Brazil)...");
    let loc_query = LocalitiesQuery::new().country("Brazil");
    match client.localities(loc_query).await {
        Ok(result) => {
            println!("   ✓ Found localities in Brazil (showing first 3):");
            for loc in result.results.iter().take(3) {
                println!(
                    "     - {} (ID: {})",
                    loc.txt.as_deref().unwrap_or("?"),
                    loc.id
                );
            }
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    // Test 6: Get specific geomaterial (Quartz, ID 3337)
    println!("\n6. Testing specific geomaterial (ID: 3337 - Quartz)...");
    match client.geomaterial(3337).await {
        Ok(m) => {
            println!("   ✓ Retrieved: {}", m.name.as_deref().unwrap_or("?"));
            println!(
                "     Formula: {}",
                m.mindat_formula.as_deref().unwrap_or("?")
            );
            println!("     Crystal System: {:?}", m.csystem);
            println!("     Hardness: {:?} - {:?}", m.hmin, m.hmax);
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    // Test 7: Dana classification
    println!("\n7. Testing Dana-8 groups...");
    match client.dana8_groups().await {
        Ok(result) => {
            println!("   ✓ Retrieved Dana-8 groups");
            if let Some(arr) = result.as_array() {
                println!("     Found {} groups", arr.len());
            }
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    println!("\n=== Tests Complete ===");
    Ok(())
}
