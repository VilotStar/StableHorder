use StableHorder::{worker::Worker, json};

type JobJson = json::job::Root;
type GenerateJson = json::generate::Root;
type StatusJson = json::status::Root;

#[tokio::main]
async fn main() {
    let worker = Worker::new_from_toml("Worker.toml").unwrap();

    let job: JobJson = worker.request_job().await.unwrap();
    println!("{:?}", job);

    let payload = GenerateJson::from(job);
    println!("{:?}", payload);

    let generation = worker.request_gen(&payload).await.unwrap();


}