use thiserror::Error;

#[derive(Error, Debug)]
pub enum CLIError {
    #[error("Invalid CLI Args")]
    InvalidCLIArgs,
    #[error("File read error")]
    FileRead,
    #[error("Parse File error")]
    ParseCSV,
    #[error("HTTP error when calling MixMax")]
    HTTP,
}
