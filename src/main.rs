extern crate clap;
extern crate gooddata_fs;
extern crate users;

use clap::{Arg, App};

use gooddata_fs::*;

const DESCRIPTION: &'static str = "GoodData as Filesystem"; // env!("CARGO_PKG_DESCRIPTION");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    // Specify program options
    let matches = App::new(DESCRIPTION)
        .version(VERSION)
        .arg(Arg::with_name("server")
            .help("Server to use")
            .takes_value(true)
            .short("s")
            .long("server")
            .default_value(rest::url::SERVER))
        .arg(Arg::with_name("token")
            .help("Token for creating of projects")
            .takes_value(true)
            .short("t")
            .long("token"))
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
        .get_matches();

    // Parse required program options
    let username = matches.value_of("username").unwrap().to_string();
    let password = matches.value_of("password").unwrap().to_string();
    let mountpoint = matches.value_of("mountpoint").unwrap().to_string();
    let server = matches.value_of("server").unwrap().to_string();
    let token = matches.value_of("token").map(|token| token.to_string());

    // Create instance of GoodData REST API Client
    let mut gd = gooddata_fs::gd::GoodDataClient::new(server, token);
    gd.connect(username, password);

    // Create GoodData Filesystem instance
    let fs = fs::GoodDataFS {
        client: gd,
        users_cache: users::UsersCache::new(),
    };

    // Mount GoodData Filesystem
    fs.mount(mountpoint);
}
