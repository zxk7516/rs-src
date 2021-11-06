#[macro_use]
extern crate rocket;


use rocket::{State, response::{Redirect, status::{BadRequest, NotFound}}};
use dashmap::DashMap;
use rand::{self, Rng, thread_rng};




#[get("/")]
fn index() -> &'static str {
    "hello world"
}


#[get("/api/shorten?<url>")]
fn shorten(url:String, state: &State<DashMap<u32,String>>) -> Result<String, BadRequest<&str>> {
    if url.is_empty() {
        Err(BadRequest(Some("URL is empty")))
    }else{
        let key: u32  = thread_rng().gen();
        state.insert(key, url);
        Ok(key.to_string())
    }
}


#[get("/<key>")]
fn redirect(key:u32,state:&State<DashMap<u32,String>>) -> Result<Redirect,NotFound<&str>> {
    state.get(&key)
        .map(|url| Redirect::to(url.clone()))
        .ok_or(NotFound("Invalid or expire link!"))
}


#[launch]
fn rocket() -> _ {
    let state :DashMap<u32,String>= DashMap::new();
    rocket::build()
        .manage(state)
        .mount("/", routes![index,shorten,redirect])
}


