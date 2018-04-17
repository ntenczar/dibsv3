use diesel::pg::Pg;
use diesel::{debug_query, delete, insert_into};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;
use uuid::Uuid;

use models::{NewQueue, NewUser, Queue, User};
use schema::{queues, users};

pub struct DibsDB {
    pub slack_token: String,
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
        let slack_token =
            env::var("SLACK_TOKEN").expect("SLACK_TOKEN is undefined.");
        return DibsDB {
            pool: pool,
            slack_token: slack_token,
        };
    }

    pub fn show(&self, queue_name: String) -> String {
        let queue = self.get_queue(queue_name.clone());
        match queue {
            Some(q) => {
                let users = self.get_users_for_queue(q.clone().id);
                return q.show(users);
            }
            None => format!("{} Queue is Empty.", queue_name),
        }
    }

    pub fn enqueue(&self, user_name: String, queue_name: String) {
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

    pub fn dequeue(&self, user_name: String, queue_name: String) {
        let conn = self.pool.get().unwrap();
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

        let query = insert_into(queues::table).values(&new_queue);
        debug_query!(query);

        return query.get_result(&conn).expect("Error creating new queue.");
    }

    fn get_queue(&self, name: String) -> Option<Queue> {
        let conn = self.pool.get().unwrap();
        let query = queues::dsl::queues.filter(queues::dsl::title.eq(name));
        debug_query!(query);
        match query
            .load::<Queue>(&conn)
            .expect("Failed to query queues table.")
            .first()
        {
            Some(q_ref) => Some((*q_ref).clone()),
            None => None,
        }
    }

    fn get_users_for_queue(&self, queue_id: String) -> Vec<User> {
        let conn = self.pool.get().unwrap();
        let query = users::dsl::users.filter(users::dsl::queue_id.eq(queue_id));
        debug_query!(query);
        return query
            .load::<User>(&conn)
            .expect("Failed to get users for queue.");
    }
}
