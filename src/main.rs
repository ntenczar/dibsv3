#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;

extern crate rocket;
extern crate rocket_contrib;

mod queue;

use queue::QueueRequest;
use rocket_contrib::Json;

#[post("/queue", format = "application/json", data = "<queue_request>")]
fn queue(queue_request: Json<QueueRequest>) -> &'static str {
    println!("{:?}", queue_request);
    return "watman";
}

fn main() {
    rocket::ignite().mount("/", routes![queue]).launch();
}
