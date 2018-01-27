use rocket_contrib::Json;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct QueueRequest {
    name: String,
    channel: String,
}

pub struct DibsDB {
    pool: Pool<ConnectionManager<PgConnection>>,
}

#[derive(Queryable, Debug)]
struct Queue {
    id: i32,
    title: String,
    users: Vec<QueueUser>,
    is_frozen: bool,
}

impl Queue {
    pub fn new(channel_name: String) -> Queue {
        return Queue {
            id: 0,
            title: channel_name,
            users: vec![],
            is_frozen: false,
        };
    }
}

#[derive(Queryable, Debug)]
struct QueueUser {
    user_id: String,
    timestamp: String,
}

impl DibsDB {
    pub fn new() -> DibsDB {
        dotenv().ok();

        let database_url =
            env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        return DibsDB { pool: pool };
    }

    pub fn enqueue(&self, req_json: Json<QueueRequest>) {
        let name = req_json.0.name;
        let channel = req_json.0.channel;
        let queue = self.get_or_create_queue(name.clone());

        println!("{:?}", name);
        println!("{:?}", channel);
        println!("{:?}", queue);
    }

    fn get_or_create_queue(&self, name: String) -> Queue {
        match self.get_queue(name.clone()) {
            Some(q) => q,
            None => Queue::new(name),
        }
    }

    fn get_queue(&self, name: String) -> Option<Queue> {
        return None;
    }
}
