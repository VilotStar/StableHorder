use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub generations: Vec<Generation>,
    pub shared: bool,
    pub finished: i64,
    pub processing: i64,
    pub restarted: i64,
    pub waiting: i64,
    pub done: bool,
    pub faulted: bool,
    #[serde(rename = "wait_time")]
    pub wait_time: i64,
    #[serde(rename = "queue_position")]
    pub queue_position: i64,
    pub kudos: f64,
    #[serde(rename = "is_possible")]
    pub is_possible: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Generation {
    pub img: String,
    pub seed: String,
    pub id: String,
    pub censored: bool,
    #[serde(rename = "worker_id")]
    pub worker_id: String,
    #[serde(rename = "worker_name")]
    pub worker_name: String,
    pub model: String,
    pub state: String,
}
