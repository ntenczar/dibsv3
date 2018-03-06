use rocket_contrib::Json;
use diesel::pg::Pg;
use diesel::{debug_query, delete, insert_into, sql_query};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;
use uuid::Uuid;

use models::{NewQueue, NewUser, Queue, User};
use request::QueueRequest;
use schema::{queues, users};

pub struct DibsDB {
    pool: Pool<ConnectionManager<PgConnection>>,
}

fn get_uuid() -> String {
    format!("{}", Uuid::new_v4())
}

macro_rules! debug_query {
    ($query:expr) => {
        if cfg!(debug_assertions) {
            let debug = debug_query::<Pg, _>(&$query);
            println!("{:?}", debug);
        }
    };
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
        let user_name = req_json.0.name;
        let queue_name = req_json.0.channel;
        let queue: Queue = self.get_or_create_queue(queue_name.clone());
        let conn = self.pool.get().unwrap();
        let user: NewUser = NewUser {
            id: &get_uuid(),
            user_id: &user_name,
            queue_id: &queue.id,
        };
        let query = insert_into(users::table).values(&user);
        debug_query!(query);
        let _user: User = query.get_result(&conn).expect(&format!(
            "Error inserting user {} into queue {}.",
            user_name, queue_name
        ));
    }

    pub fn dequeue(&self, req_json: Json<QueueRequest>) {
        let conn = self.pool.get().unwrap();
        let user_name = req_json.0.name;
        let queue_name = req_json.0.channel;
        let queue: Queue = self.get_or_create_queue(queue_name.clone());
        let query = delete(
            users::dsl::users.filter(
                users::dsl::user_id
                    .eq(user_name)
                    .and(users::dsl::queue_id.eq(queue.id)),
            ),
        );
        debug_query!(query);
        query.execute(&conn).expect("Error dequeueing user.");
    }

    fn get_or_create_queue(&self, queue_name: String) -> Queue {
        match self.get_queue(queue_name.clone()) {
            Some(q) => q,
            None => self.create_queue(queue_name),
        }
    }

    fn create_queue(&self, queue_name: String) -> Queue {
        let conn = self.pool.get().unwrap();
        let new_queue = NewQueue {
            id: &get_uuid(),
            title: &queue_name,
        };

        return insert_into(queues::table)
            .values(&new_queue)
            .get_result(&conn)
            .expect("Error creating new queue.");
    }

    fn get_queue(&self, name: String) -> Option<Queue> {
        let conn = self.pool.get().unwrap();
        match queues::dsl::queues
            .filter(queues::dsl::title.eq(name))
            .load::<Queue>(&conn)
            .expect("Failed to query queues table.")
            .first()
        {
            Some(q_ref) => Some((*q_ref).clone()),
            None => None,
        }
    }
}
