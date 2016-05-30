
#[macro_use]
extern crate log;

extern crate chrono;
extern crate cookie;
extern crate core;
extern crate env_logger;
extern crate fuse;
extern crate hyper;
extern crate libc;
extern crate regex;
extern crate rustc_serialize;
extern crate time;
extern crate users;

use std::env;

mod fs;
mod gd;

fn main() {
    let mut gd = gd::GoodDataClient::new();
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
