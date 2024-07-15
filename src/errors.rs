use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    InternalError(String),
    FetchError(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match self {
            CustomError::InternalError(err) => format!("Internal error: {err}"),
            CustomError::FetchError(err) => format!("Fetch error: {err}"),
        };
        write!(f, "{}", printable.as_str())
    }
}
