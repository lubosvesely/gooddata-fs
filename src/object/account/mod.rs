mod post_user_login;
mod projects;
mod user_login;

use gd::connector::Connector;
use rustc_serialize::json;
use std::collections::HashMap;
use object;
use rest::url;
use std::time::Duration;
use std::thread;

pub use self::post_user_login::*;
pub use self::projects::*;
pub use self::user_login::*;


#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct AccountSettingBody {
    pub country: Option<String>,
    pub firstName: Option<String>,
    pub language: Option<String>,
    pub ssoProvider: Option<String>,
    pub timezone: Option<String>,
    pub position: Option<String>,
    pub authenticationModes: Option<Vec<String>>,
    pub companyName: Option<String>,
    pub login: Option<String>,
    pub email: Option<String>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub lastName: Option<String>,
    pub phoneNumber: Option<String>,
    pub links: Option<HashMap<String, String>>,
}

impl AccountSettingBody {
    pub fn links(&self) -> &Option<HashMap<String, String>> {
        &self.links
    }
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct AccountSetting {
    pub accountSetting: AccountSettingBody,
}

impl Into<String> for AccountSetting {
    fn into(self) -> String {
        format!("{}\n", json::as_pretty_json(&self).to_string())
    }
}

impl AccountSetting {
    pub fn account_setting(&self) -> &AccountSettingBody {
        &self.accountSetting
    }

    pub fn get_link(&self, link_name: &str) -> String {
        self.account_setting().links().as_ref().unwrap()[link_name].to_string()
    }

    pub fn projects(&self, connector: &mut Connector) -> Option<Projects> {
    	connector.object_by_get::<Projects>(self.get_link("projects"))
    }

    pub fn project_create(&self, connector: &mut Connector, project_create: object::ProjectCreate) -> Option<object::Project> {
        let u = connector.object_by_post::<object::ProjectCreate, object::Uri>(url::PROJECTS.to_string(), project_create);
        if u.is_some() {
            let uri = u.unwrap().uri;
            println!("Uri: {}", uri);
            for _ in 1..10 {
                let p = connector.object_by_get::<object::Project>(uri.clone());
                if p.is_some() {
                    let project = p.unwrap();
                    if project.project.content.state.as_ref().unwrap() != &"PREPARING".to_string() {
                        return Some(project)
                    }
                }
                thread::sleep(Duration::from_millis(1000));
            }
        }
        return None
    }
    pub fn project_delete(&self, connector: &mut Connector, project_delete: object::Project) {
        let title = project_delete.project().meta().title().as_ref().unwrap();
        let uri = project_delete.get_link("self");
        println!("Deleting project: {} ({})", title, uri);
        connector.delete(uri);
    }
}
