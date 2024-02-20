#[derive(serde::Deserialize, Debug)]
pub struct Envs {
    #[serde(default = "default_ans_min")]
    pub ans_min: u32,
    #[serde(default = "default_ans_max")]
    pub ans_max: u32,
    pub ans: Option<u32>,
}

fn default_ans_min() -> u32 {
    1
}

fn default_ans_max() -> u32 {
    100
}
