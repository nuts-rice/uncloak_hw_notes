use anyhow;
use thiserror::Error;

use std::io;
use std::string::FromUtf8Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unexpected command type `{0}`")]
    UnexpectedCommand(String),
}
