#[derive(RustcDecodable, RustcEncodable)]
pub struct UserLoginBody {
    pub profile: String,
    pub state: String,
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct UserLogin {
    pub userLogin: UserLoginBody,
}
