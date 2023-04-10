use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub payload: Payload,
    pub id: String,
    pub skipped: Value, // will create new struct if i find out wtf this is used for
    pub model: String,
    #[serde(rename = "source_image")]
    pub source_image: Option<String>,
    #[serde(rename = "source_processing")]
    pub source_processing: String,
    #[serde(rename = "source_mask")]
    pub source_mask: Option<String>,
    #[serde(rename = "r2_upload")]
    pub r2_upload: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub prompt: String,
    #[serde(rename = "ddim_steps")]
    pub ddim_steps: i64,
    #[serde(rename = "n_iter")]
    pub n_iter: i64,
    #[serde(rename = "sampler_name")]
    pub sampler_name: String,
    #[serde(rename = "cfg_scale")]
    pub cfg_scale: f64,
    pub seed: String,
    pub height: i64,
    pub width: i64,
    #[serde(rename = "post_processing")]
    pub post_processing: Vec<String>,
    pub karras: bool,
    pub tiling: bool,
    #[serde(rename = "hires_fix")]
    pub hires_fix: bool,
    #[serde(rename = "image_is_control")]
    pub image_is_control: bool,
    #[serde(rename = "return_control_map")]
    pub return_control_map: bool,
}