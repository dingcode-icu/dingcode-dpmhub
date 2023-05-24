use log::{warn, info};
use std::{env};


const CORE_JSON:&str = "res/core.json";

///app运行全局配置上下文
/// 
/// ## dpm_config
/// ```
/// {
///     bin_install_path: string    bin安装路径
/// }
/// ```

#[derive(Debug, serde::Deserialize)]
pub struct ConfigContext {
    pub(crate) title: String,
    pub(crate) api_host: String, 

    //dpm sad   
}

impl Default for ConfigContext {
    fn default() -> Self {
        let str_config = include_str!("res/core.json");
        let val: ConfigContext = serde_json::from_str(str_config).expect(&format!("[config.rs]{} is not a legal json ", CORE_JSON));
        info!("Server config value is {:?}", val);
        return val;
    }
}



lazy_static! {
    pub static ref CFG: ConfigContext = ConfigContext::default();
}