/// A struct for error handling.
#[derive(Debug)]
pub struct Error;

/// result type for lcd display operations
pub type Result<T> = core::result::Result<T, Error>;
