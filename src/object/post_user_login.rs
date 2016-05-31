#[derive(RustcDecodable, RustcEncodable)]
pub struct PostUserLoginBody {
    pub login: Option<String>,
    pub password: Option<String>,
    pub remember: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct PostUserLogin {
    pub postUserLogin: PostUserLoginBody,
}
