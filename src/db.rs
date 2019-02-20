use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use queue::Queue;

pub struct DibsDB {
    pub slack_token: String,
    queues: HashMap<String, Queue>,
    db_filename: String,
}

impl DibsDB {
    pub fn new() -> Self {
        dotenv().ok();

        let slack_token =
            env::var("SLACK_TOKEN").expect("SLACK_TOKEN is undefined.");
        let db_filename =
            env::var("DB_FILENAME").expect("DB_FILENAME is undefined.");

        let file = File::open(&db_filename);
        match file {
            Ok(mut f) => {
                let mut contents = String::new();
                f.read_to_string(&mut contents)
                    .expect("Could not read file to string!");

                let deserialized = serde_json::from_str(&contents)
                    .expect("Unable to deserialize db file!");

                return DibsDB {
                    slack_token: slack_token,
                    queues: deserialized,
                    db_filename: db_filename,
                };
            }
            Err(_) => {
                return DibsDB {
                    slack_token: slack_token,
                    queues: HashMap::new(),
                    db_filename: db_filename,
                };
            }
        }
    }

    fn save_to_disk(&self) -> std::io::Result<()> {
        let mut buf = File::create(&self.db_filename)?;
        let serialized = serde_json::to_vec(&self.queues)?;
        buf.write(&serialized)?;
        return Ok(());
    }

    fn try_save_to_disk(&self) {
        let res = self.save_to_disk();
        match res {
            Ok(_) => (),
            Err(_) => panic!("There is an issue saving the state to disk!"),
        }
    }

    pub fn show_queue(&self, queue_name: String) -> String {
        let queue = self.get_or_create_queue(queue_name);
        return queue.show();
    }

    pub fn enqueue(&mut self, user_name: String, queue_name: String) {
        let mut queue = self.get_or_create_queue(queue_name.clone());
        queue.enqueue(user_name);
        self.queues.insert(queue_name, queue);
        self.try_save_to_disk();
    }

    pub fn dequeue(&mut self, user_name: String, queue_name: String) -> bool {
        let mut queue = self.get_or_create_queue(queue_name.clone());
        if queue.dequeue(user_name) {
            self.queues.insert(queue_name, queue);
            self.try_save_to_disk();
            return true;
        }
        return false;
    }

    fn get_or_create_queue(&self, queue_name: String) -> Queue {
        let queue = self.queues.get(&queue_name);
        match queue {
            Some(q) => q.clone(),
            None => Queue::new(queue_name),
        }
    }
}
