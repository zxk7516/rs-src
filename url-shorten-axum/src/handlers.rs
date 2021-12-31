use std::convert::TryInto;

use axum::{
    extract::{
        Extension,
        Path,
    },
    http::{
        HeaderMap,
        StatusCode,
    },
    response::IntoResponse,
    Json,
};

use crate::dto::{
    CreateLinkResp,
    CreateShortLinkReq,
    IdentifyContentyById,
};
use crate::DBPool;

const EMPTY_STRING: String = String::new();

pub async fn create_short(
    Json(req): Json<CreateShortLinkReq>,
    Extension(pool): Extension<DBPool>,
) -> impl IntoResponse {
    match crate::models::create_short_link(&pool, &req.url).await {
        Ok(last_insert_id) => {
            let bytes = last_insert_id.to_le_bytes();
            let s = base64::encode(&bytes);
            (
                StatusCode::OK,
                Json(CreateLinkResp {
                    ok: true,
                    url: s,
                    id: last_insert_id,
                }),
            )
        }
        Err(_e) => {
            println!("{:?}", _e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CreateLinkResp {
                    ok: false,
                    url: EMPTY_STRING,
                    id: 0,
                }),
            )
        }
    }
}

pub async fn delete_short(
    Json(req): Json<IdentifyContentyById>,
    Extension(pool): Extension<DBPool>,
) -> impl IntoResponse {
    match crate::models::delete_short_link(&pool, req.id).await {
        Ok(_r) => (StatusCode::NO_CONTENT, Json(())),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(())),
    }
}

pub async fn get_short(
    Path(short_str): Path<String>,
    Extension(pool): Extension<DBPool>,
) -> impl IntoResponse {
    let mut resp_header = HeaderMap::new();
    let bytes = base64::decode(&short_str);
    match bytes {
        Ok(b) => {
            let b1: Result<[u8; 4], _> = b.try_into();
            match b1 {
                Ok(bb) => {
                    let id: i32 = i32::from_le_bytes(bb);
                    match crate::models::get_short_link(&pool, id).await {
                        Ok(_r) => {
                            resp_header.insert("location", _r.url.parse().unwrap());
                            (StatusCode::TEMPORARY_REDIRECT, resp_header)
                        }
                        Err(_e) => (StatusCode::NOT_FOUND, resp_header),
                    }
                }
                Err(_e) => (StatusCode::BAD_REQUEST, resp_header),
            }
        }
        Err(_e) => (StatusCode::BAD_REQUEST, resp_header),
    }
}
