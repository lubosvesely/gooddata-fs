extern crate clap;
extern crate gooddata_fs;
extern crate users;

use clap::{Arg, App};
use std::env;

use gooddata_fs::*;

const DESCRIPTION: &'static str = "GoodData as Filesystem"; // env!("CARGO_PKG_DESCRIPTION");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new(DESCRIPTION)
        .version(VERSION)
        .arg(Arg::with_name("username")
            .help("GoodData Username")
            .use_delimiter(false)
            .required(true)
            .index(1))
        .arg(Arg::with_name("password")
            .help("GoodData Password")
            .use_delimiter(false)
            .required(true)
            .index(2))
        .arg(Arg::with_name("mountpoint")
            .help("Mount Point")
            .required(true)
            .index(3))
        .arg(Arg::with_name("server")
            .help("Server to use")
            .takes_value(true)
            .short("s")
            .long("server")
            .default_value(rest::url::SERVER))
        .get_matches();

    let username = matches.value_of("username").unwrap().to_string();
    let password = matches.value_of("password").unwrap().to_string();
    let server = matches.value_of("server").unwrap().to_string();

    let mut gd = gooddata_fs::gd::GoodDataClient::new(server);
    gd.connect(username, password);

    // Mount GoodData
    let mountpoint = env::args_os().nth(3).unwrap();
    let fs = fs::GoodDataFS {
        client: gd,
        users_cache: users::UsersCache::new(),
    };
    fs.mount(mountpoint.to_str().unwrap().to_string());
}
