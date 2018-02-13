#![feature(plugin, decl_macro, core_intrinsics)]
#![plugin(rocket_codegen)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;
mod models;
mod request;
mod schema;

use rocket_contrib::Json;
use rocket::response::status;
use rocket::request::State;

use db::DibsDB;
use request::QueueRequest;

#[post("/queue", format = "application/json", data = "<queue_request>")]
fn queue(
    db: State<DibsDB>,
    queue_request: Json<QueueRequest>,
) -> status::Accepted<()> {
    db.enqueue(queue_request);
    return status::Accepted::<()>(None);
}

fn main() {
    let db = DibsDB::new();
    rocket::ignite()
        .manage(db)
        .mount("/", routes![queue])
        .launch();
}
