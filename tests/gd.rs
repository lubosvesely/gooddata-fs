extern crate gooddata_fs;

use gooddata_fs::*;

fn connect() -> gd::GoodDataClient {
    // Create instance of GoodData HTTP Connector
    let connector = gooddata_fs::gd::Connector::new(gooddata_fs::rest::url::SERVER.to_string());
    // Create instance of GoodData REST API Client
    let mut gd = gooddata_fs::gd::GoodDataClient::new(connector, None);
    gd.connect("tomas.korcak+gem_tester@gooddata.com", "jindrisska");
    gd
}

#[test]
fn it_creates_connector() {
    // Create instance of GoodData HTTP Connector
    let connector = gooddata_fs::gd::Connector::new(gooddata_fs::rest::url::SERVER.to_string());
    assert_eq!(connector.token_updated.is_some(), false);
}

#[test]
fn it_creates_client() {
    // Create instance of GoodData HTTP Connector
    let connector = gooddata_fs::gd::Connector::new(gooddata_fs::rest::url::SERVER.to_string());
    // Create instance of GoodData REST API Client
    let gd = gd::GoodDataClient::new(connector, None);
    assert_eq!(gd.projects.is_some(), false);
    assert_eq!(gd.user.is_some(), false);
    assert_eq!(gd.token.is_some(), false);
}

#[test]
fn it_creates_client_with_token() {
    // Create instance of GoodData HTTP Connector
    let connector = gooddata_fs::gd::Connector::new(gooddata_fs::rest::url::SERVER.to_string());
    let token = Some("xxx".to_string());
    // Create instance of GoodData REST API Client
    let gd = gd::GoodDataClient::new(connector, token);
    assert_eq!(gd.token.is_some(), true);
    assert_eq!(gd.projects.is_some(), false);
    assert_eq!(gd.user.is_some(), false);
}

#[test]
fn client_can_connect() {
    let gd = connect();
    assert_eq!(gd.projects.is_some(), false);
    assert_eq!(gd.user.is_some(), true);

}

#[test]
fn client_can_disconnect() {
    let mut gd = connect();
    gd.disconnect();
    assert_eq!(gd.projects.is_some(), false);
    assert_eq!(gd.user.is_some(), false);
}

#[test]
fn client_can_get_projects() {
    let mut gd = connect();
    gd.projects_fetch();
    assert_eq!(gd.projects.is_some(), true);
    assert_eq!(gd.user.is_some(), true);
    gd.disconnect();
}
