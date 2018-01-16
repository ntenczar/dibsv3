extern crate rusqlite;
extern crate time;

use time::Timespec;
use rusqlite::Connection;

#[derive(Serialize, Deserialize, Debug)]
pub struct QueueRequest {
    name: String,
    channel: String
}

pub fn enqueue(req: QueueRequest) {
    println!("{:?}", req);
}
