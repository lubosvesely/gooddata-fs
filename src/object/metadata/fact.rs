#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ExprItem {
    pub data: Option<String>,
    // pub type: Option<String>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct FactcContent {
    pub expr: Option<Vec<ExprItem>>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct FactBody {
    pub content: FactcContent,
    pub meta: super::MetadataMeta,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Fact {
    pub fact: FactBody,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ObjectsFactBody {
    pub paging: super::MetadataPaging,
    pub items: Vec<Fact>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ObjectsFact {
    pub objects: ObjectsFactBody,
}
