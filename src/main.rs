#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;

extern crate rocket;
extern crate rocket_contrib;
extern crate rusqlite;
extern crate time;

mod queue;

use queue::QueueRequest;
use rocket_contrib::Json;
use rocket::response::status;

#[post("/queue", format = "application/json", data = "<queue_request>")]
fn queue(queue_request: Json<QueueRequest>) -> status::Accepted<()> {
    queue::enqueue(queue_request);
    return status::Accepted::<()>(None);
}

fn main() {
    rocket::ignite().mount("/", routes![queue]).launch();
}
