use httpmock::{
    Method::{GET, POST},
    MockServer,
};
use somfy_protect_client::client::SomfyProtectClient;

fn somfy_protect_client_for(server: &MockServer) -> SomfyProtectClient {
    SomfyProtectClient::new_with_base_url(
        format!("{}/api", server.base_url()),
        format!("{}/oauth", server.base_url()),
        //"http://localhost:8889".to_string(),
        "client_id!".to_string(),
        "client_Secret!".to_string(),
    )
}

#[tokio::test]
async fn can_list_sites() {
    let server = MockServer::start();
    let list_sites_mock = server.mock(|when, then| {
        when.method(GET).path("/api/site");
        then.status(200)
            .body_from_file("tests/resources/list_sites.json");
    });

    let client = somfy_protect_client_for(&server);

    let sites = client
        .list_sites()
        .await
        .expect("a successful site listing");

    list_sites_mock.assert();
    assert_eq!(sites.len(), 1, "one site is fetched");
    assert_eq!(
        sites.get(0).unwrap().site_id,
        "Szr5IxqYraaPqh2FFGNms2BQUT0R0hNT".to_string()
    );
    assert_eq!(sites.get(0).unwrap().name, Some("Hausalarm".to_string()));
}

#[tokio::test]
async fn can_list_devices() {
    let server = MockServer::start();
    let list_devices_mock = server.mock(|when, then| {
        when.method(GET)
            .path("/api/site/Szr5IxqYraaPqh2FFGNms2BQUT0R0hNT/device");
        then.status(200)
            .body_from_file("tests/resources/list_devices.json");
    });

    let client = somfy_protect_client_for(&server);

    let devices = client
        .list_devices("Szr5IxqYraaPqh2FFGNms2BQUT0R0hNT".to_string())
        .await
        .expect("a successful device listing");

    list_devices_mock.assert();
    assert_eq!(devices.len(), 8, "expect 8 devices");
    for device in devices {
        assert_eq!(
            device.site_id,
            "Szr5IxqYraaPqh2FFGNms2BQUT0R0hNT".to_string(),
            "expect consistent site_id"
        );
    }
}
#[tokio::test]
async fn can_list_compatible_devices() {
    let server = MockServer::start();
    let list_compatible_devices_mock = server.mock(|when, then| {
        when.method(GET)
            .path("/api/site/Szr5IxqYraaPqh2FFGNms2BQUT0R0hNT/device-compatible");
        then.status(200)
            .body_from_file("tests/resources/list_compatible_devices.json");
    });

    let client = somfy_protect_client_for(&server);

    let devices = client
        .list_compatible_devices("Szr5IxqYraaPqh2FFGNms2BQUT0R0hNT".to_string())
        .await
        .expect("a successful compatible device listing");

    list_compatible_devices_mock.assert();
    assert_eq!(devices.len(), 15, "expect 15 device types");
}
