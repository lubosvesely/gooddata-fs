extern crate gooddata_fs;
extern crate users;

use std::env;

use gooddata_fs::*;

fn main() {
    let mut gd = gooddata_fs::gd::GoodDataClient::new();
    let username = env::args_os().nth(1).unwrap();
    let password = env::args_os().nth(2).unwrap();
    gd.connect(username.to_str().unwrap().to_string(),
               password.to_str().unwrap().to_string());

    // println!("{}", gd.projects().as_ref()[0].meta.title.unwrap());

    // Mount GoodData
    let mountpoint = env::args_os().nth(3).unwrap();
    let fs = fs::GoodDataFS {
        client: gd,
        users_cache: users::UsersCache::new(),
    };
    fs.mount(mountpoint.to_str().unwrap().to_string());
}
