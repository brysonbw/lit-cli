use std::error::Error;

pub type UIResult<T> = Result<T, Box<dyn Error>>;
pub type CommandResult<T> = Result<T, Box<dyn Error>>;
pub type ValidateCommandResult<T> = Result<T, String>;
pub type StagingResult<T> = Result<T, Box<dyn Error + Send + Sync>>;
pub type ProcessCommitEntryResult = std::io::Result<()>;
