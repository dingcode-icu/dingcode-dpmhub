use std::{collections::HashMap, path::PathBuf};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DpmCellInfo {
    //remote+loc
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
    //loc 
    pub scripts: Option<HashMap<String, String>>, 
    pub install_path: Option<PathBuf>
}


#[derive(PartialEq, Clone, Debug)]
pub enum ERequestStatu {
    Idle,
    Requesting,
    Error,
}

#[derive(PartialEq, Clone, Debug)]
pub enum ECmdStatu {
    Idle,
    Running,
    Error,
}