use crate::Result;
use hyper::StatusCode;
use std::{error, fmt, string};

pub enum ErrorCode {
    NotFound,
    DatabaseConnectionError,
    DatabaseError,
    UserNotFound,
    WrongPassword,
    UserNameTaken,
    NotAuthenticated,
    SerdeJsonError,
    Utf8Error,
    InternalError,
    InvalidInput,
    ResourceNotFound,
}

impl ErrorCode {
    pub fn status_code(&self) -> StatusCode {
        match &self {
            ErrorCode::NotFound => StatusCode::NOT_FOUND,
            ErrorCode::UserNotFound
            | ErrorCode::WrongPassword
            | ErrorCode::UserNameTaken
            | ErrorCode::SerdeJsonError
            | ErrorCode::Utf8Error
            | ErrorCode::InvalidInput
            | ErrorCode::ResourceNotFound => StatusCode::BAD_REQUEST,
            ErrorCode::NotAuthenticated => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match &self {
            ErrorCode::NotFound => "NotFound",
            ErrorCode::DatabaseConnectionError => "DatabaseConnectionError",
            ErrorCode::DatabaseError => "DatabaseError",
            ErrorCode::UserNotFound => "UserNotFound",
            ErrorCode::WrongPassword => "WrongPassword",
            ErrorCode::UserNameTaken => "UserNameTaken",
            ErrorCode::NotAuthenticated => "NotAuthenticated",
            ErrorCode::SerdeJsonError => "SerdeJsonError",
            ErrorCode::Utf8Error => "Utf8Error",
            ErrorCode::InternalError => "InternalError",
            ErrorCode::InvalidInput => "InvalidInput",
            ErrorCode::ResourceNotFound => "ResourceNotFound",
        }
    }

    pub fn default(self) -> Error {
        let message = match &self {
            ErrorCode::NotFound => "404 Not Found",
            ErrorCode::UserNotFound => "User not found",
            ErrorCode::WrongPassword => "Wrong password",
            ErrorCode::UserNameTaken => "Username is already in use",
            ErrorCode::NotAuthenticated => "User is not authenticated",
            any => any.to_string(),
        };
        self.message2(message.to_string())
    }

    pub fn message(self, message: &str) -> Error {
        self.message2(message.to_string())
    }

    fn message2(self, message: String) -> Error {
        Error {
            error_code: self,
            message,
        }
    }
}

pub struct Error {
    pub error_code: ErrorCode,
    pub message: String,
}

impl Error {
    pub fn err<T>(self) -> Result<T> {
        Err(self)
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.error_code.to_string(), self.message)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.error_code.to_string(), self.message)
    }
}

impl From<()> for Error {
    fn from(_: ()) -> Self {
        ErrorCode::InternalError.default()
    }
}

impl From<r2d2::Error> for Error {
    fn from(error: r2d2::Error) -> Self {
        ErrorCode::DatabaseConnectionError.message2(error.to_string())
    }
}

impl From<postgres::Error> for Error {
    fn from(error: postgres::Error) -> Self {
        ErrorCode::DatabaseError.message2(error.to_string())
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(error: string::FromUtf8Error) -> Self {
        ErrorCode::Utf8Error.message2(error.to_string())
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Self {
        ErrorCode::SerdeJsonError.message2(error.to_string())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        ErrorCode::InvalidInput.message2(error.to_string())
    }
}

impl From<sled::Error> for Error {
    fn from(error: sled::Error) -> Self {
        ErrorCode::DatabaseError.message2(error.to_string())
    }
}

impl From<http::Error> for Error {
    fn from(error: http::Error) -> Self {
        ErrorCode::InternalError.message2(error.to_string())
    }
}
