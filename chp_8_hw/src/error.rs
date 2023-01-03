use thiserror::Error;

/*
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
*/

#[derive(Debug, Error)]
pub enum ChannelError {
    #[error("Wrong channel (expected {expected:?}, got {found:?})")]
    ChannelOpError { expected: String, found: String },
    //to be used for malicious error
    #[error("Oh no")]
    ChannelScaryError,
}
/*
impl fmt::Debug for Cli {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {

        }
    }
}
*/
