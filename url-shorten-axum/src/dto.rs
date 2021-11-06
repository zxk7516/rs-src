use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize)]
pub struct IdentifyContentyById {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct CreateShortLinkReq {
    pub url: String,
}

#[derive(Serialize)]
pub struct CreateUserResp {
    pub ok: bool,
    pub id: i32,
}

#[derive(Serialize)]
pub struct CreateLinkResp {
    pub ok: bool,
    pub url: String,
    pub id: i32,
}
