use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::json;
type JobJson = json::job::Root;
type GenerateJson = json::generate::Root;
type StatusJson = json::status::Root;

#[derive(Debug)]
pub enum WorkerErr {
    FileRead,
    TomlParse,
    InvalidProxy,
    ClientCreation,
    Request(Error),
    JsonParse(Error)
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

    pub async fn get_img(&self, id: String) -> Result<StatusJson, WorkerErr> {
        let check_url = format!("{}/api/v2/generate/check/{}", &self.data.horde_url, &id);
        loop {
            let res = match self.gen_client.get(&check_url).send().await {
                Ok(res) => res,
                Err(err) => return Err(WorkerErr::Request(err))
            };
            let json: Value = match res.json().await {
                Ok(val) => val,
                Err(err) => return Err(WorkerErr::JsonParse(err))
            };
            let finished = json.get("finished").unwrap().to_string();
            if finished.parse::<u8>().unwrap() == 1 {
                break;
            }
        };
        let status_url = format!("{}/api/v2/generate/status/{}", &self.data.horde_url, &id);
        let res = match self.gen_client.get(&status_url).send().await {
            Ok(res) => res,
            Err(err) => return Err(WorkerErr::Request(err))
        };
        return match res.json().await {
            Ok(json) =>  Ok(json),
            Err(err) => Err(WorkerErr::JsonParse(err))
        }
    }

    pub async fn request_gen(&self, payload: &GenerateJson) -> Result<String, WorkerErr> {
        let url = format!("{}/api/v2/generate/async", &self.data.horde_url);
        let res = match self.gen_client.post(url).header("apikey", &self.data.gen_info.key).json(payload).send().await {
            Ok(res) => res,
            Err(err) => return Err(WorkerErr::Request(err))
        };
        let json: Value = match res.json().await {
            Ok(val) => val,
            Err(err) => return Err(WorkerErr::JsonParse(err))
        };
        Ok(json.get("id").unwrap().to_string())
    }

    pub async fn request_job(&self) -> Result<JobJson, WorkerErr> {
        let url = format!("{}/api/v2/generate/pop", &self.data.horde_url);
        let res = match self.rec_client.post(url).header("apikey", &self.data.rec_info.key).json(&self.data.payload).send().await {
            Ok(res) => res,
            Err(err) => return Err(WorkerErr::Request(err))
        };
        match res.json().await {
            Ok(json) => Ok(json),
            Err(err) => return Err(WorkerErr::JsonParse(err))
        }
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