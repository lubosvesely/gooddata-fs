extern crate gooddata_fs;

use gooddata_fs::*;

#[test]
fn it_creates_client() {
    let gd = gd::GoodDataClient::new();
    assert_eq!(gd.projects.is_some(), false);
    assert_eq!(gd.user.is_some(), false);
    assert_eq!(gd.token_updated.is_some(), false);
}
