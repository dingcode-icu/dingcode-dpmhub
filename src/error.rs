use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug)]
pub enum ApiError {

    //todo: Integration of all *remote* errors into the ApiFailed
    #[error("Api failed!{0}")]
    ApiFailed(String), 
    
    #[error("ilegal url path")]
    UrlError(#[from] ParseError), 
    
    #[error("request error")]
    ReqError(#[from] ureq::Error), 
    
    #[error("request error")]
    SerdeJson(#[from] std::io::Error)
}

