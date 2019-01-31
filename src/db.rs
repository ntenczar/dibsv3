use dotenv::dotenv;
use std::env;
use std::collections::{HashMap};

use models::{Queue};

pub struct DibsDB {
    pub slack_token: String,
    queues: HashMap<String, Queue>
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
        let queue = self.get_or_create_queue(queue_name);
        return queue.show();
    }

    pub fn enqueue(&mut self, user_name: String, queue_name: String) {
        let mut queue = self.get_or_create_queue(queue_name.clone());
        queue.enqueue(user_name);
        self.queues.insert(queue_name, queue);
    }

    pub fn dequeue(&mut self, user_name: String, queue_name: String) -> bool {
        let mut queue = self.get_or_create_queue(queue_name.clone());
        if queue.dequeue(user_name) {
            self.queues.insert(queue_name, queue);
            return true;
        }
        return false;
    }

    fn get_or_create_queue(&self, queue_name: String) -> Queue {
        let queue = self.queues.get(&queue_name);
        match queue {
            Some(q) => q.clone(),
            None => Queue::new(queue_name)
        }
    }
}
