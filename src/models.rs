#[derive(Queryable, Debug)]
pub struct Queue {
    id: i32,
    title: String,
    is_frozen: bool,
}

impl Queue {
    pub fn new(channel_name: String) -> Queue {
        return Queue {
            id: 0,
            title: channel_name,
            is_frozen: false,
        };
    }
}

#[derive(Queryable, Debug)]
pub struct QueueUser {
    user_id: String,
    timestamp: String,
}
