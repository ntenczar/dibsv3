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

use rocket_contrib::Json;
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

#[derive(Serialize, Deserialize)]
struct SlackResponse {
    text: String,
    response_type: String,
}

fn show_request(
    db: State<DibsDB>,
    request: DibsRequest,
) -> Result<Json<SlackResponse>, BadRequest<String>> {
    let queue_name = request.channel_id;
    let response = SlackResponse {
        text: db.show(queue_name),
        response_type: format!("ephemeral"),
    };
    return Ok(Json(response));
}

fn queue_request(
    db: State<DibsDB>,
    request: DibsRequest,
) -> Result<Json<SlackResponse>, BadRequest<String>> {
    let user_name = request.user_id;
    let queue_name = request.channel_id;
    db.enqueue(user_name.clone(), queue_name.clone());
    let response = SlackResponse {
        text: format!(
            "<@{}> has joined the queue. \n {}",
            user_name,
            db.show(queue_name),
        ),
        response_type: format!("in_channel"),
    };
    return Ok(Json(response));
}

fn dequeue_request(
    db: State<DibsDB>,
    request: DibsRequest,
) -> Result<Json<SlackResponse>, BadRequest<String>> {
    let user_name = request.user_id;
    let queue_name = request.channel_id;
    db.dequeue(user_name.clone(), queue_name.clone());
    let response = SlackResponse {
        text: format!(
            "<@{}> has left the queue. \n {}",
            user_name,
            db.show(queue_name)
        ),
        response_type: format!("in_channel"),
    };
    return Ok(Json(response));
}

#[post("/", format = "application/x-www-form-urlencoded",
       data = "<dibs_request>")]
fn main_request(
    db: State<DibsDB>,
    dibs_request: Form<DibsRequest>,
) -> Result<Json<SlackResponse>, BadRequest<String>> {
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
        _ => Err(BadRequest(Some(format!("not yet implemented")))),
    }
}

fn main() {
    let db = DibsDB::new();
    rocket::ignite()
        .manage(db)
        .mount("/", routes![main_request])
        .launch();
}
