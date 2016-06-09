#![deny(warnings)]
#![allow(non_snake_case)]
#[allow(unused_imports)]

extern crate time;
extern crate hyper;
extern crate lru_cache;

use cookie::CookieJar;
use hyper::client::Client;
use hyper::client::response::Response;
use hyper::header::{Accept, Cookie, ContentType, SetCookie, UserAgent, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use rustc_serialize::json;
use rustc_serialize::json::DecoderError;
use rustc_serialize::Decodable;

use lru_cache::LruCache;
use std::io::Read;
use std::vec::Vec;

use helpers;
use object;
use rest::url;

const CACHE_SIZE: usize = 32 * 1024;

pub struct GoodDataClient {
    pub client: Client,
    pub server: String,
    pub token: Option<String>,
    pub jar: CookieJar<'static>,
    pub user: Option<object::AccountSetting>,
    pub projects: Option<Vec<object::Project>>,
    pub token_updated: Option<time::PreciseTime>,
    pub cache: LruCache<String, Response>,
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
    pub fn new(server: String, token: Option<String>) -> GoodDataClient {
        GoodDataClient {
            client: Client::new(),
            server: server,
            token: token,
            jar: CookieJar::new(helpers::random_string(32).as_bytes()),
            user: None,
            projects: None,
            token_updated: None,
            cache: LruCache::new(CACHE_SIZE),
        }
    }

    /// Get Projects
    pub fn projects(&self) -> &Option<Vec<object::Project>> {
        // self.projects_fetch();
        &self.projects
    }

    pub fn projects_fetch_if_none(&mut self) -> &Vec<object::Project> {
        match self.projects {
            Some(ref projects) => projects,
            None => {
                self.projects_fetch();
                self.projects().as_ref().unwrap()
            }
        }
    }

    pub fn projects_fetch(&mut self) -> &Option<Vec<object::Project>> {
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

        let projects: object::Projects = json::decode(&raw_projects[..]).unwrap();

        self.projects = Some(projects.projects);
        &self.projects
    }

    pub fn create_project(&mut self, project: object::ProjectCreate) {
        let payload = json::encode(&project).unwrap();

        println!("Creating project: {}", payload);
        let mut raw = self.post(url::PROJECTS.to_string(), payload);
        let content = self.get_content(&mut raw);
        println!("Project created: {}", content);
    }

    pub fn delete_project(&mut self, project: object::Project) {
        let title = project.project().meta().title().as_ref().unwrap();
        let uri = project.get_link("self");
        println!("Deleting project: {} ({})", title, uri);
        self.delete(uri);
    }

    /// Login to GoodData platform
    pub fn connect<S: Into<String>>(&mut self, username: S, password: S) {
        let payload = object::PostUserLogin {
            postUserLogin: object::PostUserLoginBody {
                login: Some(username.into()),
                password: Some(password.into()),
                remember: Some(false),
            },
        };

        self.cache = LruCache::new(CACHE_SIZE);

        let mut raw = self.post(url::LOGIN.to_string(), json::encode(&payload).unwrap());

        self.refresh_token();

        let content = self.get_content(&mut raw);

        let user: object::UserLogin = json::decode(&content[..]).unwrap();
        let uri = user.userLogin.profile;

        let mut raw = self.get(uri);
        let rawUser = self.get_content(&mut raw);

        let user: object::AccountSetting = json::decode(&rawUser[..]).unwrap();
        self.user = Some(user);
    }

    pub fn disconnect(&mut self) {
        println!("GoodDataClient::disconnect() - Disconnecting from GoodData Platform");

        self.user = None;
        self.token_updated = None;
        self.projects = None;
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

        println!("GoodDataClient::get() - Response: {:?}", raw);
        if !raw.is_ok() {
            return self.get(uriPath);
        }

        let mut res = raw.unwrap();

        // assert_eq!(res.status, hyper::Ok);
        if res.status != hyper::Ok {
            return res;
        }

        self.print_response(&mut res);
        self.update_cookie_jar(&res);

        // self.cache.insert(uriPath, res);

        return res;
    }

    /// HTTP Method GET Wrapper
    pub fn get_cached<S: Into<String>>(&mut self, path: S) -> &Response {
        let key: String = format!("{}", path.into());
        if self.cache.contains_key(&key) {
            let res: &Response = self.cache.get_mut(&key).unwrap();
            return res;
        }

        let res = self.get(key.clone());
        self.cache.insert(key.clone(), res);
        return self.cache.get_mut(&key.clone()).unwrap();
    }

    pub fn get_link_obj<Type: Decodable>(&mut self, link: String) -> Option<Type> {
        let mut res = self.get(link);

        if res.status != hyper::Ok {
            return None;
        }

        let raw = self.get_content(&mut res);
        let obj: Result<Type, DecoderError> = json::decode(&raw.to_string());
        match obj {
            Ok(obj) => Some(obj),
            Err(e) => None,
        }
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
            .header(Cookie::from_cookie_jar(&self.jar))
            .body(&payload[..])
            .send();


        println!("GoodDataClient::post() - Response: {:?}", raw);
        if !raw.is_ok() {
            return self.post(uriPath, payload);
        }

        let mut res = raw.unwrap();
        assert_eq!(res.status, hyper::Ok);

        self.print_response(&mut res);
        self.update_cookie_jar(&res);

        return res;
    }

    fn delete<S: Into<String>>(&mut self, path: S) -> hyper::client::response::Response {
        self.refresh_token_check();

        let uriPath = format!("{}", path.into());
        let uri = format!("{}{}", self.server, uriPath);

        let raw = self.client
            .delete(&uri[..])
            .header(UserAgent(GoodDataClient::user_agent().to_owned()))
            .header(Accept(vec![qitem(Mime(TopLevel::Application,
                                           SubLevel::Json,
                                           vec![(Attr::Charset, Value::Utf8)]))]))
            .header(Cookie::from_cookie_jar(&self.jar))
            .send();

        println!("GoodDataClient::delete() - Response: {:?}", raw);
        if !raw.is_ok() {
            return self.delete(uriPath);
        }

        let mut res = raw.unwrap();
        assert_eq!(res.status, hyper::Ok);

        self.print_response(&mut res);
        self.update_cookie_jar(&res);

        return res;
    }

    /// Get HTTP Response body
    pub fn get_content(&mut self, res: &mut hyper::client::Response) -> String {
        let mut buf = String::new();
        println!("{:?}", res.read_to_string(&mut buf));
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

        let uri = format!("{}{}", self.server, url::TOKEN);
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

    pub fn user(&self) -> &Option<object::AccountSetting> {
        &self.user
    }

    /// Construct User-Agent HTTP Header
    fn user_agent() -> String {
        const VERSION: &'static str = env!("CARGO_PKG_VERSION");
        return format!("gooddata-rust/{}", VERSION);
    }
}
