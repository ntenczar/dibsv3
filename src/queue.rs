use rocket_contrib::Json;
use r2d2_redis::RedisConnectionManager;
use r2d2;

#[derive(Serialize, Deserialize, Debug)]
pub struct QueueRequest {
    name: String,
    channel: String,
}

pub struct Queue {
    pool: r2d2::Pool<RedisConnectionManager>,
}

impl Queue {
    pub fn new() -> Queue {
        // TODO(nt): refactor to a config where you can run redis anywhere
        let manager = RedisConnectionManager::new("redis://localhost").unwrap();
        let pool = r2d2::Pool::builder().build(manager).unwrap();
        return Queue { pool: pool };
    }
}

pub fn enqueue(req: Json<QueueRequest>) {
    println!("{:?}", req);
}
