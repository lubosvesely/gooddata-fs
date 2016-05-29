#![deny(warnings)]
#![allow(non_snake_case)]
#[allow(unused_imports)]

extern crate time;
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

#[derive(RustcDecodable, RustcEncodable)]
pub struct ProjectContent {
    pub environment: Option<String>,
    pub cluster: Option<String>,
    pub guidedNavigation: Option<String>,
    pub isPublic: Option<String>,
    pub driver: Option<String>,
    pub state: Option<String>,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct ProjectMeta {
    pub created: Option<String>,
    pub summary: Option<String>,
    pub updated: Option<String>,
    pub author: Option<String>,
    pub title: Option<String>,
    pub contributor: Option<String>,
}

#[allow(dead_code)]
impl ProjectMeta {
    pub fn created(&self) -> &Option<String> {
        &self.created
    }

    pub fn summary(&self) -> &Option<String> {
        &self.summary
    }

    pub fn updated(&self) -> &Option<String> {
        &self.updated
    }

    pub fn author(&self) -> &Option<String> {
        &self.author
    }

    pub fn title(&self) -> &Option<String> {
        &self.title
    }

    pub fn contributor(&self) -> &Option<String> {
        &self.contributor
    }
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct ProjectBody {
    pub content: ProjectContent,
    pub links: Option<HashMap<String, String>>,
    pub meta: ProjectMeta,
}

impl ProjectBody {
    pub fn meta(&self) -> &ProjectMeta {
        &self.meta
    }
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Project {
    pub project: ProjectBody,
}

impl Project {
    pub fn project(&self) -> &ProjectBody {
        &self.project
    }
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Projects {
    pub projects: Vec<Project>,
}

pub struct GoodDataClient {
    pub client: Client,
    pub server: String,
    pub jar: CookieJar<'static>,
    pub user: Option<AccountSetting>,
    pub projects: Option<Vec<Project>>,
    pub token_updated: Option<time::PreciseTime>,
}

impl Drop for GoodDataClient {
    fn drop(&mut self) {
        self.disconnect();
    }
}

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
            projects: None,
            token_updated: None,
        }
    }

    /// Get Projects
    pub fn projects(&self) -> &Option<Vec<Project>> {
        // self.projects_fetch();
        &self.projects
    }

    pub fn projects_fetch(&mut self) -> &Option<Vec<Project>> {
        let uri = format!("{}",
                          self.user
                              .as_ref()
                              .unwrap()
                              .accountSetting
                              .links
                              .as_ref()
                              .unwrap()
                              .get("projects")
                              .unwrap());
        let mut res = self.get(&uri[..]);
        let raw_projects = self.get_content(&mut res);

        let projects: Projects = json::decode(&raw_projects[..]).unwrap();

        self.projects = Some(projects.projects);
        &self.projects
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
        self.refresh_token_check();

        let uriPath = format!("{}", path.into());
        let uri = format!("{}{}", self.server, uriPath);
        let raw = self.client
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
            .send();

        println!("{:?}", raw);
        if !raw.is_ok() {
            return self.get(uriPath);
        }

        let mut res = raw.unwrap();
        assert_eq!(res.status, hyper::Ok);
        println!("{:?}", res);

        self.print_response(&mut res);
        self.update_cookie_jar(&res);

        return res;
    }

    /// HTTP Method POST Wrapper
    fn post<S: Into<String>>(&mut self, path: S, body: S) -> hyper::client::response::Response {
        self.refresh_token_check();

        let uriPath = format!("{}", path.into());
        let uri = format!("{}{}", self.server, uriPath);
        let payload = body.into();

        let raw = self.client
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
            .send();


        println!("{:?}", raw);
        if !raw.is_ok() {
            return self.post(uriPath, payload);
        }

        let mut res = raw.unwrap();
        assert_eq!(res.status, hyper::Ok);
        println!("{:?}", res);

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
                self.jar.add(cookie.clone());
            }
        }
    }

    /// Refresh GoodData TT (Temporary Token)
    fn refresh_token(&mut self) {
        // Refresh token
        // self.get("/gdc/account/token");

        let uri = format!("{}/gdc/account/token", self.server);
        let raw = self.client
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
            .send();

        println!("{:?}", raw);
        if !raw.is_ok() {
            return self.refresh_token();
        }

        let mut res = raw.unwrap();
        assert_eq!(res.status, hyper::Ok);
        println!("{:?}", res);

        self.print_response(&mut res);
        self.update_cookie_jar(&res);

        self.token_updated = Some(time::PreciseTime::now());
    }

    fn refresh_token_check(&mut self) {
        if self.token_updated.is_some() &&
           self.token_updated.unwrap().to(time::PreciseTime::now()) >
           time::Duration::seconds(4 * 60) {
            self.refresh_token();
        }
    }

    pub fn user(&self) -> &Option<AccountSetting> {
        &self.user
    }

    /// Construct User-Agent HTTP Header
    fn user_agent() -> String {
        const VERSION: &'static str = env!("CARGO_PKG_VERSION");
        return format!("gooddata-rust/{}", VERSION);
    }
}
