pub type Result<T> = std::result::Result<T, ImporterError>;

#[derive(Debug)]
pub enum ImporterError {
    ImportError(String),
    ParsingError(String),
}

#[macro_export]
macro_rules! import_error {
    ($e:expr) => {
        Err(ImporterError::ImportError($e.to_string()))
    };
    ($($arg:tt)*) => {
        Err(ImporterError::ImportError(format!($($arg)*)))
    }
}

#[macro_export]
macro_rules! parse_error {
    ($e:expr) => {
        Err(ImporterError::ParsingError($e.to_string()))
    };
    ($($arg:tt)*) => {
        Err(ImporterError::ParsingError(format!($($arg)*)))
    }
}
