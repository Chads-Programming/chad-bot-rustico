use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    InternalError(String),
    FetchError(String),
    OutOfFunds(String),
    BadArguments(String),
    Timeout(String),
    NotFound(String),
    AlreadyMemberExists(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match self {
            CustomError::InternalError(err) => err,
            CustomError::FetchError(err) => err,
            CustomError::BadArguments(err) => err,
            CustomError::Timeout(err) => err,
            CustomError::NotFound(err) => err,
            CustomError::AlreadyMemberExists(err) => err,
            CustomError::OutOfFunds(err) => err,
        };
        write!(f, "{}", printable.as_str())
    }
}

impl From<sqlx::Error> for CustomError {
    fn from(sqlx_error: sqlx::Error) -> Self {
        match sqlx_error {
            sqlx::Error::Configuration(err) => {
                CustomError::InternalError(format!("[Configuration error] {err:?}"))
            }
            sqlx::Error::Database(err) => CustomError::InternalError(format!("[Database] {err:?}")),
            sqlx::Error::Io(err) => CustomError::InternalError(format!("[IO]: {err:?}")),
            sqlx::Error::Tls(err) => CustomError::InternalError(format!("[Tls]: {err:?}")),
            sqlx::Error::Protocol(err) => CustomError::InternalError(format!("[Protocol]: {err}")),
            sqlx::Error::RowNotFound => CustomError::NotFound("[RowNotFound]: error".to_string()),
            sqlx::Error::TypeNotFound { type_name } => {
                CustomError::BadArguments(format!("[TypeNotFound]: {type_name}"))
            }
            sqlx::Error::ColumnIndexOutOfBounds { index, len } => CustomError::InternalError(
                format!("[ColumnIndexOutOfBounds]: index => {index}, len => {len}"),
            ),
            sqlx::Error::ColumnNotFound(err) => {
                CustomError::BadArguments(format!("[ColumnNotFound]: {err}"))
            }
            sqlx::Error::ColumnDecode { index, source } => CustomError::InternalError(format!(
                "[ColumnDecode]: index => {index}, source => {source}"
            )),
            sqlx::Error::Decode(err) => CustomError::InternalError(format!("[Decode]: {err:?}")),
            sqlx::Error::AnyDriverError(err) => {
                CustomError::InternalError(format!("[AnyDriverError]: {err:?}"))
            }
            sqlx::Error::PoolTimedOut => {
                CustomError::Timeout("[PoolTimedOut]: Pool Timeout".to_string())
            }
            sqlx::Error::PoolClosed => {
                CustomError::InternalError("[PoolClosed]: Pool closed".to_string())
            }
            sqlx::Error::WorkerCrashed => {
                CustomError::InternalError("[WorkerCrashed]: worker has crashed".to_string())
            }
            sqlx::Error::Migrate(err) => CustomError::InternalError(format!("[Migrate]: {err:?}")),
            _ => todo!(),
        }
    }
}
