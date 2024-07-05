use thiserror::Error;

#[derive(Error, Debug)]
pub enum DockerError {
    #[error("Docker API error: {0}")]
    DockerAPIError(#[from] reqwest::Error),
    #[error("Parsing error: {0}")]
    ParsingError(#[from] serde_json::Error),
    #[error("Service update error: {0}")]
    ServiceUpdateError(String),
}
