//! Integration tests using wiremock to mock the Mindat API.

use mindat_rs::{
    CrystalSystem, GeomaterialsQuery, ImaMineralsQuery, LocalitiesQuery, MindatClient,
    PaginatedResponse, Reference, ReferencesQuery,
};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn setup_mock_client() -> (MockServer, MindatClient) {
    let mock_server = MockServer::start().await;
    let client = MindatClient::builder()
        .token("test-token")
        .base_url(mock_server.uri())
        .build()
        .expect("Failed to build client");
    (mock_server, client)
}

#[tokio::test]
async fn test_references_search() {
    let (mock_server, client) = setup_mock_client().await;

    Mock::given(method("GET"))
        .and(path("/references/"))
        .and(query_param("q", "quartz"))
        .and(header("Authorization", "Token test-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "count": 1,
            "next": null,
            "previous": null,
            "results": [
                {"ref_id": 42, "ref_title": "On Quartz", "ref_journal": "Am. Min.", "ref_year": 1990}
            ]
        })))
        .mount(&mock_server)
        .await;

    let result = client
        .references(ReferencesQuery::new().search("quartz"))
        .await
        .expect("Request failed");

    assert_eq!(result.count, Some(1));
    assert_eq!(result.results[0].ref_id, 42);
    assert_eq!(result.results[0].ref_title, Some("On Quartz".to_string()));
}

#[tokio::test]
async fn test_geomaterial_by_id() {
    let (mock_server, client) = setup_mock_client().await;

    Mock::given(method("GET"))
        .and(path("/geomaterials/3337/"))
        .and(header("Authorization", "Token test-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 3337,
            "name": "Quartz",
            "mindat_formula": "SiO2",
            "csystem": "Trigonal",
            "hmin": 7.0,
            "hmax": 0.0,
            "ima_status": ["APPROVED"],
            "entrytype": 0
        })))
        .mount(&mock_server)
        .await;

    let result = client.geomaterial(3337).await.expect("Request failed");

    assert_eq!(result.id, 3337);
    assert_eq!(result.name, Some("Quartz".to_string()));
    assert_eq!(result.mindat_formula, Some("SiO2".to_string()));
    assert_eq!(result.csystem, Some("Trigonal".to_string()));
}

#[tokio::test]
async fn test_geomaterials_with_query() {
    let (mock_server, client) = setup_mock_client().await;

    Mock::given(method("GET"))
        .and(path("/geomaterials/"))
        .and(query_param("name", "quartz"))
        .and(query_param("ima", "true"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "count": 1,
            "next": null,
            "previous": null,
            "results": [
                {"id": 3337, "name": "Quartz", "mindat_formula": "SiO2"}
            ]
        })))
        .mount(&mock_server)
        .await;

    let query = GeomaterialsQuery::new().name("quartz").ima_approved(true);
    let result = client.geomaterials(query).await.expect("Request failed");

    assert_eq!(result.count, Some(1));
    assert_eq!(result.results[0].name, Some("Quartz".to_string()));
}

#[tokio::test]
async fn test_geomaterials_with_elements() {
    let (mock_server, client) = setup_mock_client().await;

    // Regression guard: geomaterials must send `el_inc` (the API renamed the
    // old `elements_inc` parameter). The mock only matches on `el_inc`.
    Mock::given(method("GET"))
        .and(path("/geomaterials/"))
        .and(query_param("el_inc", "Cu,S"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "count": 2,
            "next": null,
            "previous": null,
            "results": [
                {"id": 1, "name": "Chalcopyrite", "mindat_formula": "CuFeS2"},
                {"id": 2, "name": "Covellite", "mindat_formula": "CuS"}
            ]
        })))
        .mount(&mock_server)
        .await;

    let query = GeomaterialsQuery::new().with_elements("Cu,S");
    let result = client.geomaterials(query).await.expect("Request failed");

    assert_eq!(result.count, Some(2));
    assert_eq!(result.results[0].name, Some("Chalcopyrite".to_string()));
}

#[tokio::test]
async fn test_localities_by_country() {
    let (mock_server, client) = setup_mock_client().await;

    Mock::given(method("GET"))
        .and(path("/localities/"))
        .and(query_param("country", "Brazil"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "next": null,
            "previous": null,
            "results": [
                {"id": 100, "txt": "Minas Gerais", "country": "Brazil", "latitude": -19.0, "longitude": -44.0}
            ]
        })))
        .mount(&mock_server)
        .await;

    let query = LocalitiesQuery::new().country("Brazil");
    let result = client.localities(query).await.expect("Request failed");

    assert_eq!(result.results.len(), 1);
    assert_eq!(result.results[0].txt, Some("Minas Gerais".to_string()));
}

#[tokio::test]
async fn test_ima_minerals_anonymous() {
    let mock_server = MockServer::start().await;
    let client = MindatClient::builder()
        .base_url(mock_server.uri())
        .build()
        .expect("Failed to build client");

    Mock::given(method("GET"))
        .and(path("/minerals-ima/"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "count": 6000,
            "next": "http://example.com/minerals-ima/?page=2",
            "previous": null,
            "results": [
                {"id": 1, "name": "Abelsonite", "ima_formula": "NiC31H32N4", "ima_symbol": "Abs"}
            ]
        })))
        .mount(&mock_server)
        .await;

    let query = ImaMineralsQuery::new();
    let result = client.minerals_ima(query).await.expect("Request failed");

    assert_eq!(result.count, Some(6000));
    assert!(result.has_next());
    assert!(!result.has_previous());
    assert_eq!(result.results[0].name, Some("Abelsonite".to_string()));
}

#[tokio::test]
async fn test_auth_required_error() {
    let mock_server = MockServer::start().await;
    let client = MindatClient::builder()
        .token("invalid-token")
        .base_url(mock_server.uri())
        .build()
        .expect("Failed to build client");

    Mock::given(method("GET"))
        .and(path("/geomaterials/"))
        .respond_with(
            ResponseTemplate::new(401)
                .set_body_string("Authentication credentials were not provided."),
        )
        .mount(&mock_server)
        .await;

    let result = client.geomaterials(GeomaterialsQuery::new()).await;
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(matches!(
        err,
        mindat_rs::MindatError::AuthenticationRequired
    ));
}

#[tokio::test]
async fn test_rate_limit_error() {
    let (mock_server, client) = setup_mock_client().await;

    Mock::given(method("GET"))
        .and(path("/geomaterials/"))
        .respond_with(ResponseTemplate::new(429).set_body_string("Rate limit exceeded"))
        .mount(&mock_server)
        .await;

    let result = client.geomaterials(GeomaterialsQuery::new()).await;
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(matches!(err, mindat_rs::MindatError::RateLimited));
}

#[tokio::test]
async fn test_not_found_error() {
    let (mock_server, client) = setup_mock_client().await;

    Mock::given(method("GET"))
        .and(path("/geomaterials/99999999/"))
        .respond_with(ResponseTemplate::new(404).set_body_string("Not found"))
        .mount(&mock_server)
        .await;

    let result = client.geomaterial(99999999).await;
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(matches!(err, mindat_rs::MindatError::NotFound(_)));
}

#[tokio::test]
async fn test_pagination() {
    let (mock_server, client) = setup_mock_client().await;

    // Regression guard: pagination must send `page-size` (hyphen), not the
    // old `page_size`. The mock only matches on `page-size`.
    Mock::given(method("GET"))
        .and(path("/geomaterials/"))
        .and(query_param("page", "1"))
        .and(query_param("page-size", "10"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "count": 25,
            "next": "http://example.com/geomaterials/?page=2&page-size=10",
            "previous": null,
            "results": []
        })))
        .mount(&mock_server)
        .await;

    let query = GeomaterialsQuery::new().page(1).page_size(10);
    let result = client.geomaterials(query).await.expect("Request failed");

    assert_eq!(result.count, Some(25));
    assert!(result.has_next());
    assert!(!result.has_previous());
    assert_eq!(result.total_pages(10), Some(3));
}

#[tokio::test]
async fn test_localities_pagination_uses_hyphen_page_size() {
    let (mock_server, client) = setup_mock_client().await;

    // Localities keep `elements_inc`/`elements_exc` but still use `page-size`.
    Mock::given(method("GET"))
        .and(path("/localities/"))
        .and(query_param("elements_inc", "Au"))
        .and(query_param("page-size", "5"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "count": 1,
            "next": null,
            "previous": null,
            "results": [
                {"id": 100, "txt": "Some Mine", "country": "Brazil"}
            ]
        })))
        .mount(&mock_server)
        .await;

    let query = LocalitiesQuery::new().with_elements("Au").page_size(5);
    let result = client.localities(query).await.expect("Request failed");

    assert_eq!(result.count, Some(1));
    assert_eq!(result.results[0].txt, Some("Some Mine".to_string()));
}

#[tokio::test]
async fn test_geomaterials_multichoice_filters_are_sent() {
    let (mock_server, client) = setup_mock_client().await;

    // Regression guard: enum multi-choice filters (crystal_system, entrytype)
    // must actually be serialized as repeated params — they were previously
    // exposed on the builder but silently dropped.
    Mock::given(method("GET"))
        .and(path("/geomaterials/"))
        .and(query_param("crystal_system", "Hexagonal"))
        .and(query_param("entrytype", "0"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "count": 1, "next": null, "previous": null,
            "results": [ {"id": 3337, "name": "Quartz"} ]
        })))
        .mount(&mock_server)
        .await;

    let query = GeomaterialsQuery::new()
        .crystal_systems(vec![CrystalSystem::Hexagonal])
        .entry_types(vec![0]);
    let result = client.geomaterials(query).await.expect("Request failed");
    assert_eq!(result.results[0].id, 3337);
}

#[tokio::test]
async fn test_reference_tolerates_string_encoded_primary_key() {
    // Mindat sometimes returns integer ids as JSON strings. A string-encoded
    // primary key in one row must not fail the whole page.
    let json = serde_json::json!({
        "count": 2, "next": null, "previous": null,
        "results": [
            {"ref_id": 41, "ref_title": "A"},
            {"ref_id": "42", "ref_title": "B", "ref_year": "1990"}
        ]
    });
    let page: PaginatedResponse<Reference> =
        serde_json::from_value(json).expect("string-encoded ref_id should deserialize");
    assert_eq!(page.results.len(), 2);
    assert_eq!(page.results[1].ref_id, 42);
    assert_eq!(page.results[1].ref_year, Some(1990));
}

#[tokio::test]
async fn test_minerals_ima_uses_el_inc() {
    let mock_server = MockServer::start().await;
    let client = MindatClient::builder()
        .base_url(mock_server.uri())
        .build()
        .expect("Failed to build client");

    // minerals-ima is now a full search endpoint and uses `el_inc` + `page-size`.
    Mock::given(method("GET"))
        .and(path("/minerals-ima/"))
        .and(query_param("el_inc", "Ni"))
        .and(query_param("page-size", "3"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "count": 1,
            "next": null,
            "previous": null,
            "results": [
                {"id": 1, "name": "Abelsonite", "ima_formula": "NiC31H32N4"}
            ]
        })))
        .mount(&mock_server)
        .await;

    let query = ImaMineralsQuery::new().with_elements("Ni").page_size(3);
    let result = client.minerals_ima(query).await.expect("Request failed");

    assert_eq!(result.count, Some(1));
    assert_eq!(result.results[0].name, Some("Abelsonite".to_string()));
}
