use std::collections::HashMap;

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct AccountSetting {
    pub accountSetting: AccountSettingBody,
}
