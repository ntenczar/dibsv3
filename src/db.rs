use dotenv::dotenv;
use std::env;
use std::collections::{HashMap};
use uuid::Uuid;

use models::{Queue};

pub struct DibsDB {
    pub slack_token: String,
    queues: HashMap<String, Queue>
}

fn get_uuid() -> String {
    format!("{}", Uuid::new_v4())
}

impl DibsDB {
    pub fn new() -> Self {
        dotenv().ok();

        let slack_token =
            env::var("SLACK_TOKEN").expect("SLACK_TOKEN is undefined.");
        return DibsDB {
            slack_token: slack_token,
            queues: HashMap::new()
        };
    }

    pub fn show_queue(&self, queue_name: String) -> String {
        let queue = self.queues.get(&queue_name);
        match queue {
            Some(q) => {
                return q.show();
            }
            None => format!("{} Queue is Empty.", queue_name),
        }
    }

    pub fn enqueue(&self, user_name: String, queue_name: String) {
        panic!("not yet implemented!");
    }

    pub fn dequeue(&self, user_name: String, queue_name: String) {
        panic!("not yet implemented!");
    }

    fn get_or_create_queue(&self, queue_name: String) -> Queue {
        panic!("not yet implemented!");
    }

    fn create_queue(&self, queue_name: String) -> Queue {
        panic!("not yet implemented!");
    }

    fn get_queue(&self, name: String) -> Option<Queue> {
        panic!("not yet implemented!");
    }
}
