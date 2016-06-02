extern crate gooddata_fs;

use gooddata_fs::*;

fn connect() -> gd::GoodDataClient {
    let mut gd = gd::GoodDataClient::new();
    gd.connect("tomas.korcak+gem_tester@gooddata.com", "jindrisska");
    gd
}

#[test]
fn it_creates_client() {
    let gd = gd::GoodDataClient::new();
    assert_eq!(gd.projects.is_some(), false);
    assert_eq!(gd.user.is_some(), false);
    assert_eq!(gd.token_updated.is_some(), false);
}

#[test]
fn client_can_connect() {
    let gd = connect();

    assert_eq!(gd.projects.is_some(), false);
    assert_eq!(gd.user.is_some(), true);
    assert_eq!(gd.token_updated.is_some(), true);
}

#[test]
fn client_can_disconnect() {
    let mut gd = connect();
    gd.disconnect();

    assert_eq!(gd.projects.is_some(), false);
    assert_eq!(gd.user.is_some(), false);
    assert_eq!(gd.token_updated.is_some(), false);
}

#[test]
fn client_can_get_projects() {
    let mut gd = connect();
    gd.projects_fetch();

    assert_eq!(gd.projects.is_some(), true);
    assert_eq!(gd.user.is_some(), true);
    assert_eq!(gd.token_updated.is_some(), true);

    gd.disconnect();
}
