pub(crate) enum RloxError {
    InvalidInput(String),
    UnexpectedEof(String),
    UnhandledError(String),
}

impl From<RloxError> for Box<dyn std::error::Error> {
    fn from(e: RloxError) -> Self {
        match e {
            RloxError::InvalidInput(msg) => Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, msg)),
            RloxError::UnexpectedEof(msg) => Box::new(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, msg)),
            RloxError::UnhandledError(msg) => Box::new(std::io::Error::new(std::io::ErrorKind::Other, msg)),
        }
    }
}