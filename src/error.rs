use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Api failed!")]
    ApiFailed, 
    
    #[error("ilegal url path")]
    UrlError(#[from] ParseError), 
    
    #[error("request error")]
    ReqError(#[from] ureq::Error), 
    
    #[error("request error")]
    SerdeJson(#[from] std::io::Error)
}

