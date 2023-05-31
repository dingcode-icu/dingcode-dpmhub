use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DpmCellInfo {
    pub name: String,
    pub ver: String,
    pub descript: String,
    pub license: Option<String>,
    pub md5: Option<String>,
    pub cont_size: Option<u64>,
    pub url: Option<String>,
    pub runtime: String,
    pub min_runtime_ver: Option<u32>,
    pub max_runtime_ver: Option<u32>,
}


#[derive(PartialEq, Clone, Debug)]
pub enum ERequestStatu {
    Idle,
    Requesting,
    Error,
}
