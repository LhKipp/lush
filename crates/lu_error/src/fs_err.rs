use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug, Deserialize, Serialize)]
pub enum FsErr {
    #[error("Reading of file {0} failed with message\n:{1}")]
    ReadFailed(PathBuf, String),
    #[error("Writing of file {0} failed with message\n:{1}")]
    WriteFailed(PathBuf, String),
}
