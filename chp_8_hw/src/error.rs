use anyhow::Result;

use thiserror::Error;

use std::io;

pub struct Cli {
    command: String,
}

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Unexpected command type `{0}`")]
    UnexpectedCommand(String),
    #[error("IO error")]
    UnexpectedIO(#[from] io::Error),
    #[error("Unexpected operation (expected: {expected:?}. got {found:?}) ")]
    UnexpectedOperation { expected: String, found: String },
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl Cli {
    fn get_cli() -> Result<String> {
        unimplemented!()
    }
}

/*
impl fmt::Debug for Cli {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {

        }
    }
}
*/
