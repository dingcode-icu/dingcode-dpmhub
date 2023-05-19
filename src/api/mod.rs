pub mod model;

mod Agent {
    const TIME_OUT_D: u64 = 3;

    use ureq;
    use ureq::AgentBuilder;
    use core::time::Duration;
    use std::env;
    pub fn agent() -> ureq::Agent {
        AgentBuilder::new()
            .timeout(Duration::from_secs(TIME_OUT_D))
            .build()
    }

    pub fn chk_env() {
        let ret = dotenv::dotenv();
        if let Ok(p) = ret {
            log::info!("init env from file = <{}> suc!", p.display());
        }
    }

    pub fn core_host() -> String {
        chk_env();
        let ret = env::var("DPM_HOST").unwrap_or("https://api.picboo.ink".to_owned());
        ret
    }
}

pub mod ApiRsvr {
    use super::Agent;
    use crate::error::ApiError;

    
    fn url(path: &str) -> String {
        let u_str = format!("{}/{}", Agent::core_host(), path);
        log::info!("[api-rsvr]req url is {}", u_str);
        let u = url::Url::parse(u_str.as_str()).expect("[api-rsvr]path is not legal path to ");
        u.as_str().to_string()
    }

    pub fn get_pmlist(pm_type: &str) -> Result<Vec<super::model::DpmCellInfo>, ApiError> {
        let resp:Vec<super::model::DpmCellInfo> = Agent::agent().get(url(pm_type).as_str()).call()?.into_json()?;   
        Ok(resp)
    }
}
