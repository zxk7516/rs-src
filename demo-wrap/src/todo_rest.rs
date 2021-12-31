use std::sync::Arc;

use serde_json::json;
use serde_json::Value;
use warp::{
    reply::Json,
    Filter,
};

use crate::security::check_auth;
use crate::security::UserCtx;
use crate::with_db_pool;
use crate::DbPool;

pub fn todo_filter(
    db_pool: Arc<DbPool>
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let todo_base = warp::path("todos");
    let list = todo_base
        .and(warp::get())
        .and(warp::path::end())
        .and(check_auth())
        .and(with_db_pool(db_pool.clone()))
        .and_then(todo_list);

    let get = todo_base
        .and(warp::get())
        .and(check_auth())
        .and(with_db_pool(db_pool.clone()))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(todo_get);

    let create = todo_base
        .and(warp::post())
        .and(check_auth())
        .and(with_db_pool(db_pool.clone()))
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(todo_create);

    list.or(get).or(create)
}

async fn todo_list(
    _user_ctx: UserCtx,
    _db_pool: Arc<DbPool>,
) -> Result<Json, warp::Rejection> {
    let todos = json!([
        {"id":1, "title": "todo 1", "user_id": _user_ctx.user_id, },
        {"id":2, "title": "todo 2", "user_id": _user_ctx.user_id, },
    ]);
    let todos = warp::reply::json(&todos);
    Ok(todos)
}

async fn todo_get(
    _user_ctx: UserCtx,
    _db_pool: Arc<DbPool>,
    id: i64,
) -> Result<Json, warp::Rejection> {
    // TODO - get from DB
    let todo = json!( {"id":id, "title": format!("todo {}",id), "user_id": _user_ctx.user_id});
    let todo = warp::reply::json(&todo);
    Ok(todo)
}

async fn todo_create(
    _user_ctx: UserCtx,
    _db_pool: Arc<DbPool>,
    data: Value,
) -> Result<Json, warp::Rejection> {
    // TODO - write to DB
    let todo = data;
    let todo = warp::reply::json(&todo);
    Ok(todo)
}
