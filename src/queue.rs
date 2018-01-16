#[derive(Serialize, Deserialize, Debug)]
pub struct QueueRequest {
    name: String,
    channel: String
}
