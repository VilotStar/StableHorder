use reqwest::{header::HeaderMap, Proxy, Response, Client};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::json;

#[derive(Debug, Deserialize)]
pub enum WorkerErr {
    FileRead,
    TomlParse,
    InvalidProxy,
    ClientCreation
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkerData {
    payload: PopPayload,
    gen_info: ApiInfo,
    rec_info: ApiInfo,
    bridge_version: u32,
    bridge_agent: String,
    horde_url: String
}

pub struct Worker {
    data: WorkerData,
    rec_client: Client,
    gen_client: Client
}

impl Worker {
    pub fn new(payload: PopPayload,
           gen_info: ApiInfo,
           rec_info: ApiInfo,
           bridge_version: u32,
           bridge_agent: String,
           horde_url: String) -> Result<Worker, WorkerErr> {
        
        let worker_data = WorkerData {
            payload,
            gen_info,
            rec_info,
            bridge_version,
            bridge_agent,
            horde_url
        };

        let (rec_client, gen_client) = match Self::init(&worker_data) {
            Ok(clients) => clients,
            Err(err) => return Err(err)
        };

        Ok(Self {
            data: worker_data,
            rec_client,
            gen_client
        })
    }

    pub fn new_from_toml(path: &str) -> Result<Worker, WorkerErr> {
        let content = match std::fs::read_to_string(path) {
            Ok(str) => str,
            Err(_) => return Err(WorkerErr::FileRead)
        };

        let worker_data: WorkerData = match toml::from_str(&content) {
            Ok(data) => data,
            Err(_) => return Err(WorkerErr::TomlParse)
        };

        let (rec_client, gen_client) = match Self::init(&worker_data) {
            Ok(clients) => clients,
            Err(err) => return Err(err)
        };

        Ok(Self {
            data: worker_data,
            rec_client,
            gen_client
        })
    }

    fn init(data: &WorkerData) -> Result<(Client, Client), WorkerErr> {
        let rec_proxy = match reqwest::Proxy::all(&data.rec_info.proxy) {
            Ok(proxy) => proxy,
            Err(_) => return Err(WorkerErr::InvalidProxy)
        };

        let rec_client = match reqwest::Client::builder().proxy(rec_proxy).build() {
            Ok(client) => client,
            Err(_) => return Err(WorkerErr::ClientCreation)
        };

        let gen_proxy = match reqwest::Proxy::all(&data.gen_info.proxy) {
            Ok(proxy) => proxy,
            Err(_) => return Err(WorkerErr::InvalidProxy)
        };

        let gen_client = match reqwest::Client::builder().proxy(gen_proxy).build() {
            Ok(client) => client,
            Err(_) => return Err(WorkerErr::ClientCreation)
        };

        Ok((rec_client, gen_client))
    }

    pub async fn request_img(self, payload: &json::generate::Root) -> String {
        let url = format!("{}/api/v2/generate/async", &self.data.horde_url);
        let res = (&(self.gen_client)).post(url)
            .header("apikey", &self.data.gen_info.key)
            .json(payload)
            .send().await.unwrap();
        let json: Value = res.json().await.unwrap();
        json["id"].to_string()
    }

    pub async fn request_job(self) -> Response {
        let url = format!("{}/api/v2/generate/pop", &self.data.horde_url);
        self.rec_client.post(url)
            .header("apikey", &self.data.rec_info.key)
            .json(&self.data.payload)
            .send().await.unwrap()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiInfo {
    key: String,
    proxy: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PopPayload {
    name: String,
    max_pixels: u32,
    priority_usernames: Vec<String>,
    nsfw: bool,
    blacklist: Vec<String>,
    models: Vec<String>,
    allow_img2img: bool,
    allow_inpainting: bool,
    allow_unsafe_ip: bool,
    threads: u32,
    allow_post_processing: bool,
    allow_controlnet: bool,
    require_upfront_kudos: bool
}