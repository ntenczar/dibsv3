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

fn show_request(dibs_request: DibsRequest) -> () {}
fn queue_request(dibs_request: DibsRequest) -> () {}
fn dequeue_request(dibs_request: DibsRequest) -> () {}

#[post("/", format = "application/x-www-form-urlencoded",
       data = "<dibs_request>")]
fn main_request(
    db: State<DibsDB>,
    dibs_request: Form<DibsRequest>,
) -> Result<&'static str, BadRequest<String>> {
    let request = dibs_request.into_inner();
    if request.token != db.slack_token {
        return Err(BadRequest(Some(format!("Invalid slack token."))));
    }
    println!("{:?}", request);
    return Ok("Success");
}

fn main() {
    let db = DibsDB::new();
    rocket::ignite()
        .manage(db)
        .mount("/", routes![main_request])
        .launch();
}
