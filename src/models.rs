use schema::{queues, users};

#[derive(Identifiable, Queryable, Debug, Clone, Associations)]
pub struct Queue {
    pub id: String,
    pub title: String,
    pub is_frozen: bool,
}

#[derive(Identifiable, Queryable, Debug)]
pub struct User {
    pub id: String,
    pub user_id: String,
    pub queue_id: String,
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
