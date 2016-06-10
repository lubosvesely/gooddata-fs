#![deny(warnings)]
#![allow(non_snake_case)]
#[allow(unused_imports)]

use std::vec::Vec;

use object;
use rest::url;
use super::connector::Connector;


pub struct GoodDataClient {
    pub connector: Connector,
    pub token: Option<String>,
    pub user: Option<object::AccountSetting>,
    pub projects: Option<Vec<object::Project>>,
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
    pub fn new(connector: Connector, token: Option<String>) -> GoodDataClient {
        GoodDataClient {
            connector: connector,
            token: token,
            user: None,
            projects: None,
        }
    }

    /// Get Connector
    pub fn connector(&self) -> &Connector {
        &self.connector
    }

    /// Get Projects
    pub fn projects(&self) -> &Option<Vec<object::Project>> {
        // self.projects_fetch();
        &self.projects
    }

    /// Get user
    pub fn user(&self) -> &Option<object::AccountSetting> {
        &self.user
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

    pub fn create_project(&mut self, project_create: object::ProjectCreate) {
        let project = self.user.as_ref().unwrap().project_create(&mut self.connector, project_create);
        match project {
            Some(p) => self.projects.as_mut().unwrap().push(p),
            None => {}
        }
    }

    pub fn delete_project(&mut self, project_delete: object::Project) {
        let res = self.user.as_ref().unwrap().project_delete(&mut self.connector, project_delete);
    }

    pub fn projects_fetch(&mut self) {
        let projects = self.user.as_ref().unwrap().projects(&mut self.connector);
        self.projects = match projects {
            Some(p) => Some(p.projects),
            None => None
        }
    }

    pub fn report_csv(&mut self, report_definition: String) -> String {
        let payload = object::ReportReq {
            report_req: object::ReportReqBody {
                reportDefinition: report_definition
            }
        };
        let uri = self.connector.object_by_post::<object::ReportReq, object::Uri>(url::PROJECT_EXECUTE_RAW.to_string(), payload);
        let mut result = self.connector.get(uri.unwrap().uri);
        self.connector.get_content(&mut result)
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
        let user_login = self.connector.object_by_post::<object::PostUserLogin, object::UserLogin>(url::LOGIN.to_string(), payload);
        let profile_link = user_login.unwrap().userLogin.profile;

        self.connector.refresh_token();
        let user = self.connector.object_by_get::<object::AccountSetting>(profile_link).unwrap();
        self.user = Some(user);
        // let csv = self.report_csv("/gdc/md/GoodSalesDemo/obj/30834".to_string());
        // println!("CSV: {}", csv);
    }

    pub fn disconnect(&mut self) {
        println!("GoodDataClient::disconnect() - Disconnecting from GoodData Platform");
        self.user = None;
        self.projects = None;
    }
}
