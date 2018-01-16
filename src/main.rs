#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![world]).launch();
}
