#![feature(
    proc_macro_hygiene,
    plugin,
    decl_macro,
    core_intrinsics,
    custom_attribute
)]
#[macro_use]
extern crate rocket;

extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate dotenv;
extern crate rocket_contrib;
extern crate uuid;

mod db;
mod queue;
mod user;
mod request;
use rocket::request::{Form, State};
use rocket::response::status::BadRequest;
use rocket_contrib::json::Json;
use std::sync::RwLock;

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
    db: &DibsDB,
    request: DibsRequest,
) -> Result<Json<SlackResponse>, BadRequest<String>> {
    let queue_name = request.channel_id;
    let response = SlackResponse {
        text: db.show_queue(queue_name),
        response_type: format!("ephemeral"),
    };
    return Ok(Json(response));
}

fn queue_request(
    db: &mut DibsDB,
    request: DibsRequest,
) -> Result<Json<SlackResponse>, BadRequest<String>> {
    let user_name = request.user_id;
    let queue_name = request.channel_id;
    db.enqueue(user_name.clone(), queue_name.clone());
    let response = SlackResponse {
        text: format!(
            "<@{}> has joined the queue. \n {}",
            user_name,
            db.show_queue(queue_name),
        ),
        response_type: format!("in_channel"),
    };
    return Ok(Json(response));
}

fn dequeue_request(
    db: &mut DibsDB,
    request: DibsRequest,
) -> Result<Json<SlackResponse>, BadRequest<String>> {
    let user_name = request.user_id;
    let queue_name = request.channel_id;
    db.dequeue(user_name.clone(), queue_name.clone());
    let response = SlackResponse {
        text: format!(
            "<@{}> has left the queue. \n {}",
            user_name,
            db.show_queue(queue_name)
        ),
        response_type: format!("in_channel"),
    };
    return Ok(Json(response));
}

#[post(
    "/",
    format = "application/x-www-form-urlencoded",
    data = "<dibs_request>"
)]
fn main_request(
    db_state: State<RwLock<DibsDB>>,
    dibs_request: Form<DibsRequest>,
) -> Result<Json<SlackResponse>, BadRequest<String>> {
    let request = dibs_request.into_inner();
    if request.token != db_state.read().unwrap().slack_token {
        return Err(BadRequest(Some(format!("Invalid slack token."))));
    }
    debug_request!(request);
    match request.text.as_ref() {
        "show" => {
            let db = db_state.read().unwrap();
            show_request(&db, request)
        }
        _ => {
            let db: &mut DibsDB = &mut db_state.write().unwrap();
            return match request.text.as_ref() {
                "" => queue_request(db, request),
                "queue" => queue_request(db, request),
                "dequeue" => dequeue_request(db, request),
                "done" => dequeue_request(db, request),
                _ => Err(BadRequest(Some(format!("not yet implemented")))),
            };
        }
    }
}

fn main() {
    let db = DibsDB::new();
    let db_lock = RwLock::new(db);
    rocket::ignite()
        .manage(db_lock)
        .mount("/", routes![main_request])
        .launch();
}
