#[derive(Serialize, Deserialize, Debug)]
pub struct QueueRequest {
    pub name: String,
    pub channel: String,
}
