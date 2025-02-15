mod exclusive;
pub use exclusive::*;

mod version;
pub use version::*;

use crate::ManagerError;
use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum ArenaError {
    #[error("manager error: {0}")]
    ManagerError(#[from] ManagerError),
}
