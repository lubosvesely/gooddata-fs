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
