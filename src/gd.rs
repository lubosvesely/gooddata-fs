#![deny(warnings)]
#![allow(non_snake_case)]
#[allow(unused_imports)]

extern crate hyper;

use cookie::CookieJar;
use hyper::client::Client;
use hyper::client::response::Response;
use hyper::header::{Accept, Cookie, ContentType, SetCookie, UserAgent, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use rustc_serialize::json;

use std::collections::HashMap;
use std::io::Read;
use std::vec::Vec;

#[derive(RustcDecodable, RustcEncodable)]
pub struct AccountSettingBody {
    pub country: Option<String>,
    pub firstName: Option<String>,
    pub language: Option<String>,
    pub ssoProvider: Option<String>,
    pub timezone: Option<String>,
    pub position: Option<String>,
    pub authenticationModes: Vec<String>,
    pub companyName: Option<String>,
    pub login: Option<String>,
    pub email: Option<String>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub lastName: Option<String>,
    pub phoneNumber: Option<String>,
    pub links: Option<HashMap<String, String>>,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct AccountSetting {
    pub accountSetting: AccountSettingBody,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct PostUserLoginBody {
    pub login: String,
    pub password: String,
    pub remember: bool,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct PostUserLogin {
    pub postUserLogin: PostUserLoginBody,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct UserLoginBody {
    pub profile: String,
    pub state: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct UserLogin {
    pub userLogin: UserLoginBody,
}

pub struct GoodDataClient {
    pub client: Client,
    pub server: String,
    pub jar: CookieJar<'static>,
    pub user: Option<AccountSetting>,
}

// impl Drop for GoodDataClient {
// fn drop(&mut self) {
// self.disconnect();
// }
// }
//

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unreachable_code)]
impl GoodDataClient {
    /// Create Instance of GoodData Client
    pub fn new() -> GoodDataClient {
        GoodDataClient {
            client: Client::new(),
            server: "https://secure.gooddata.com".to_string(),
            jar: CookieJar::new(b"f8f9eaf1ecdedff5e5b749c58115441e"),
            user: None,
        }
    }

    pub fn drop(&mut self) {
        println!("NOTE: Logging out is not implemented yet!");
    }

    /// Get Projects
    pub fn projects(&mut self) -> String {
        let mut res = self.get("/gdc/md");
        return self.get_content(&mut res);
    }

    /// Login to GoodData platform
    pub fn connect<S: Into<String>>(&mut self, username: S, password: S) {
        let payload = PostUserLogin {
            postUserLogin: PostUserLoginBody {
                login: username.into(),
                password: password.into(),
                remember: false,
            },
        };

        let mut raw = self.post("/gdc/account/login".to_string(),
                                json::encode(&payload).unwrap());

        self.refresh_token();

        let content = self.get_content(&mut raw);

        let user: UserLogin = json::decode(&content[..]).unwrap();
        let uri = user.userLogin.profile;

        let mut raw = self.get(uri);
        let rawUser = self.get_content(&mut raw);

        let user: AccountSetting = json::decode(&rawUser[..]).unwrap();
        self.user = Some(user);
    }

    pub fn disconnect(&mut self) {
        println!("Disconnecting from GoodData Platform");
    }

    /// HTTP Method GET Wrapper
    pub fn get<S: Into<String>>(&mut self, path: S) -> Response {
        let uri = format!("{}{}", self.server, path.into());
        let mut res = self.client
            .get(&uri[..])
            .header(ContentType(Mime(TopLevel::Application,
                                     SubLevel::Json,
                                     vec![(Attr::Charset, Value::Utf8)])))
            .header(Accept(vec![
                             qitem(Mime(TopLevel::Application, SubLevel::Json,
                             vec![(Attr::Charset, Value::Utf8)])),
            ]))
            .header(UserAgent(GoodDataClient::user_agent().to_owned()))
            .header(Cookie::from_cookie_jar(&self.jar))
            .send()
            .unwrap();
        assert_eq!(res.status, hyper::Ok);
        println!("{:?}", res);

        self.print_response(&mut res);
        self.update_cookie_jar(&res);

        return res;
    }

    /// HTTP Method POST Wrapper
    fn post<S: Into<String>>(&mut self, path: S, body: S) -> hyper::client::response::Response {
        let uri = format!("{}{}", self.server, path.into());
        let payload = body.into();

        let mut res = self.client
            .post(&uri[..])
            .header(ContentType(Mime(TopLevel::Application,
                                     SubLevel::Json,
                                     vec![(Attr::Charset, Value::Utf8)])))
            .header(UserAgent(GoodDataClient::user_agent().to_owned()))
            .header(Accept(vec![
                            qitem(Mime(TopLevel::Application, SubLevel::Json,
                            vec![(Attr::Charset, Value::Utf8)])),
            ]))
            .body(&payload[..])
            .send()
            .unwrap();
        assert_eq!(res.status, hyper::Ok);

        self.print_response(&mut res);
        self.update_cookie_jar(&res);

        return res;
    }

    /// Get HTTP Response body
    pub fn get_content(&mut self, res: &mut hyper::client::Response) -> String {
        let mut buf = String::new();
        match res.read_to_string(&mut buf) {
            Ok(_) => (),
            Err(_) => panic!("I give up."),
        };

        return buf;
    }

    /// Print HTTP Response
    pub fn print_response(&mut self, res: &mut hyper::client::Response) {
        return;

        let obj = res;

        println!("{:?}", obj);

        let content = self.get_content(obj);
        println!("{}", content);
    }

    /// Update Cookies in Jar from HTTP Response
    fn update_cookie_jar(&mut self, res: &hyper::client::Response) {
        for setCookie in res.headers.get::<SetCookie>().iter() {
            for cookie in setCookie.iter() {
                // println!("{:?}", cookie);
                self.jar.add(cookie.clone());
            }
        }
    }

    /// Refresh GoodData TT (Temporary Token)
    fn refresh_token(&mut self) {
        // Refresh token
        self.get("/gdc/account/token");
    }

    pub fn user(&self) -> &Option<AccountSetting> {
        &self.user
    }

    /// Construct User-Agent HTTP Header
    fn user_agent() -> String {
        const VERSION: &'static str = env!("CARGO_PKG_VERSION");
        return format!("gooddata-ruby/{}", VERSION);
    }
}
