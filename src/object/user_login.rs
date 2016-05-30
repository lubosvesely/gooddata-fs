#[derive(RustcDecodable, RustcEncodable)]
pub struct UserLoginBody {
    pub profile: String,
    pub state: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct UserLogin {
    pub userLogin: UserLoginBody,
}
