use chrono::prelude::*;

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub queue_time: DateTime<Utc>,
}

impl User {
    pub fn new(name: String) -> Self {
        return User {
            id: name,
            queue_time: Utc::now(),
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
