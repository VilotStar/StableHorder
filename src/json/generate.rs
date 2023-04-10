use serde::{Deserialize, Serialize};
use super::job;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub prompt: String,
    pub params: Params,
    pub nsfw: bool,
    #[serde(rename = "censor_nsfw")]
    pub censor_nsfw: bool,
    #[serde(rename = "trusted_workers")]
    pub trusted_workers: bool,
    pub models: Vec<String>,
    pub shared: bool,
    pub r2: bool,
    pub job_id: String,
    pub index: i64,
    pub gathered: bool,
    pub failed: bool,
}

impl From<job::Root> for Root {
    fn from(job: job::Root) -> Self {
        let params = Params { // preset values are set due to job not contaning fields (some fields are handled by server)
            steps: job.payload.ddim_steps,
            n: job.payload.n_iter,
            sampler_name: job.payload.sampler_name.clone(),
            width: job.payload.width,
            height: job.payload.height,
            cfg_scale: job.payload.cfg_scale,
            seed_variation: 1000,
            seed: job.payload.seed.clone(),
            karras: job.payload.karras,
            denoising_strength: 0.75,
            tiling: job.payload.tiling,
            hires_fix: job.payload.hires_fix,
            clip_skip: 1,
            post_processing: job.payload.post_processing.clone()
        };

        Self { 
            prompt: job.payload.prompt.clone(),
            params,
            nsfw: true,
            censor_nsfw: false,
            trusted_workers: false,
            models: vec!(job.model.clone()),
            shared: false,
            r2: true,
            job_id: "".to_string(),
            index: 0,
            gathered: false,
            failed: false
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub steps: i64,
    pub n: i64,
    #[serde(rename = "sampler_name")]
    pub sampler_name: String,
    pub width: i64,
    pub height: i64,
    #[serde(rename = "cfg_scale")]
    pub cfg_scale: f64,
    #[serde(rename = "seed_variation")]
    pub seed_variation: i64,
    pub seed: String,
    pub karras: bool,
    #[serde(rename = "denoising_strength")]
    pub denoising_strength: f64,
    pub tiling: bool,
    #[serde(rename = "hires_fix")]
    pub hires_fix: bool,
    #[serde(rename = "clip_skip")]
    pub clip_skip: i64,
    #[serde(rename = "post_processing")]
    pub post_processing: Vec<String>,
}
