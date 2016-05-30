#[derive(RustcDecodable, RustcEncodable)]
pub struct PostUserLoginBody {
    pub login: String,
    pub password: String,
    pub remember: bool,
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct PostUserLogin {
    pub postUserLogin: PostUserLoginBody,
}
