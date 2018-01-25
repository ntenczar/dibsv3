#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;
extern crate rocket;
extern crate rocket_contrib;

mod queue;

use rocket_contrib::Json;
use rocket::response::status;

use queue::{Queue, QueueRequest};

#[post("/queue", format = "application/json", data = "<queue_request>")]
fn queue(queue_request: Json<QueueRequest>) -> status::Accepted<()> {
    queue::enqueue(queue_request);
    return status::Accepted::<()>(None);
}

fn main() {
    let queue = Queue::new();
    rocket::ignite()
        .manage(queue)
        .mount("/", routes![queue])
        .launch();
}
