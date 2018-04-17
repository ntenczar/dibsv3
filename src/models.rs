use std::time::SystemTime;
use chrono::prelude::*;

use schema::{queues, users};

#[derive(Identifiable, Queryable, Debug, Clone, Associations)]
pub struct Queue {
    pub id: String,
    pub title: String,
    pub is_frozen: bool,
    pub created_at: SystemTime,
}

impl Queue {
    pub fn show(&self, users: Vec<User>) -> String {
        let queue_name = format!("<#{}>", self.title);
        let header = format!("{} Queue \n============", queue_name);
        let mut body = String::from("");
        if users.len() == 0 {
            return format!("{} Queue is Empty.", queue_name);
        }
        let now: DateTime<Utc> = Utc::now();
        for u in users {
            body = format!("{} \n {}", u.show(now), body);
        }
        return format!("{} \n {}", header, body);
    }
}

#[derive(Identifiable, Queryable, Debug)]
pub struct User {
    pub id: String,
    pub user_id: String,
    pub queue_id: String,
    pub created_at: SystemTime,
}

impl User {
    pub fn show(&self, now: DateTime<Utc>) -> String {
        let created_at: DateTime<Utc> = self.created_at.into();
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
        return format!("<@{}> in queue for {}", self.user_id, time_in_queue);
    }
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub user_id: &'a str,
    pub queue_id: &'a str,
}

#[derive(Insertable, Debug)]
#[table_name = "queues"]
pub struct NewQueue<'a> {
    pub id: &'a str,
    pub title: &'a str,
}
