#![feature(plugin, decl_macro, core_intrinsics, custom_derive)]
#![plugin(rocket_codegen)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

extern crate rocket;
extern crate rocket_contrib;

extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;
mod models;
mod request;
mod schema;

use rocket::response::status::BadRequest;
use rocket::request::{Form, State};

use db::DibsDB;
use request::DibsRequest;

macro_rules! debug_request {
    ($req:expr) => {
        if cfg!(debug_assertions) {
            println!("{:?}", &$req);
        }
    };
}

fn show_request(
    db: State<DibsDB>,
    request: DibsRequest,
) -> Result<String, BadRequest<String>> {
    let queue_name = request.channel_id;
    return Ok(db.show(queue_name));
}

fn queue_request(
    db: State<DibsDB>,
    request: DibsRequest,
) -> Result<String, BadRequest<String>> {
    let user_name = request.user_id;
    let queue_name = request.channel_id;
    db.enqueue(user_name, queue_name.clone());

    return Ok(db.show(queue_name));
}

fn dequeue_request(
    db: State<DibsDB>,
    request: DibsRequest,
) -> Result<String, BadRequest<String>> {
    let user_name = request.user_id;
    let queue_name = request.channel_id;
    db.dequeue(user_name, queue_name.clone());

    return Ok(db.show(queue_name));
}

#[post("/", format = "application/x-www-form-urlencoded",
       data = "<dibs_request>")]
fn main_request(
    db: State<DibsDB>,
    dibs_request: Form<DibsRequest>,
) -> Result<String, BadRequest<String>> {
    let request = dibs_request.into_inner();
    if request.token != db.slack_token {
        return Err(BadRequest(Some(format!("Invalid slack token."))));
    }
    debug_request!(request);
    match request.text.as_ref() {
        "show" => show_request(db, request),
        "" => queue_request(db, request),
        "queue" => queue_request(db, request),
        "dequeue" => dequeue_request(db, request),
        "done" => dequeue_request(db, request),
        _ => Ok(format!("not yet implemented")),
    }
}

fn main() {
    let db = DibsDB::new();
    rocket::ignite()
        .manage(db)
        .mount("/", routes![main_request])
        .launch();
}
