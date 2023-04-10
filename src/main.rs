use StableHoarder::{worker::Worker, json};

#[tokio::main]
async fn main() {
    let worker = Worker::new_from_toml("Worker.toml").unwrap();
    
    let job: json::job::Root = worker.request_job().await.json().await.unwrap();
    println!("{:?}", job);

    let payload = json::generate::Root::from(job);
    println!("{:?}", payload);

    worker.request_img(&payload);
}
