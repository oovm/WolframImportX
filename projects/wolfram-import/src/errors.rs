use wolfram_library_link::wstp::Error;

pub type Result<T> = std::result::Result<T, ImporterError>;

#[derive(Debug)]
pub struct ImporterError {}
