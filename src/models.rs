use std::collections::{HashMap};
use chrono::prelude::*;

#[derive(Debug, Clone)]
pub struct Queue {
    pub id: String,
    pub is_frozen: bool,
    pub created_at: DateTime<Utc>,
    pub users: HashMap<String, User>
}

impl Queue {
    pub fn new(name: String) -> Self {
        return Queue {
            id: name,
            is_frozen: false,
            created_at: Utc::now(),
            users: HashMap::new()
        };
    }

    pub fn show(&self) -> String {
        let queue_name = format!("<#{}>", self.id);
        let header = format!("{} queue \n============", queue_name);
        let mut body = String::from("");
        if self.users.len() == 0 {
            return format!("{} queue is empty.", queue_name);
        }
        let now: DateTime<Utc> = Utc::now();
        let mut position = 1;
        for (_id, u) in &self.users {
            body = format!("{} {}. {} \n", body, position, u.show(now));
            position += 1;
        }
        return format!("{} \n {}", header, body);
    }

    pub fn enqueue(&mut self, user_name: String) {
        // TODO(nate): check that not already queued?
        let user = User::new(user_name.clone());
        self.users.insert(user_name, user);
    }

    pub fn dequeue(&mut self, user_name: String) -> bool {
        match self.users.remove(&user_name) {
            Some(_u) => true,
            None => false
        }
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub queue_time: DateTime<Utc>
}

impl User {
    pub fn new(name: String) -> Self {
        return User {
            id: name,
            queue_time: Utc::now()
        };
    }

    pub fn show(&self, now: DateTime<Utc>) -> String {
        let created_at: DateTime<Utc> = self.queue_time;
        let diff = now.signed_duration_since(created_at);
        let hours = diff.num_hours();
        let minutes = diff.num_minutes() - (hours * 60);
        let seconds = diff.num_seconds() - (hours * 3600) - (minutes * 60);
        let time_in_queue;
        if hours == 0 {
            time_in_queue = format!("{}m{}s", minutes, seconds);
        } else {
            time_in_queue = format!("{}h{}m{}s", hours, minutes, seconds);
        }
        return format!("<@{}> in queue for {}", self.id, time_in_queue);
    }
}
