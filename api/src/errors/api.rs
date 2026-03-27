use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum ApiError {
    InternalServerError,
    Unauthorized,
    Conflict,
    NotFound,
    BadRequest(String),
    UnprocessableEntity(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error, detail): (StatusCode, &str, Option<String>) = match self {
            Self::InternalServerError      => (StatusCode::INTERNAL_SERVER_ERROR, "internal_server_error", None),
            Self::Unauthorized             => (StatusCode::UNAUTHORIZED, "unauthorized", None),
            Self::Conflict                 => (StatusCode::CONFLICT, "conflict", None),
            Self::NotFound                 => (StatusCode::NOT_FOUND, "not_found", None),
            Self::BadRequest(msg)          => (StatusCode::BAD_REQUEST, "bad_request", Some(msg)),
            Self::UnprocessableEntity(msg) => (StatusCode::UNPROCESSABLE_ENTITY, "unprocessable_entity", Some(msg)),
        };

        let body = match detail {
            Some(d) => serde_json::json!({ "error": error, "detail": d }),
            None    => serde_json::json!({ "error": error }),
        };

        (status, axum::Json(body)).into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        if let sqlx::Error::Database(ref db_err) = e
            && db_err.code().as_deref() == Some("23505")
        {
            return Self::Conflict;
        }
        tracing::error!("database error: {:?}", e);
        Self::InternalServerError
    }
}

impl From<bcrypt::BcryptError> for ApiError {
    fn from(e: bcrypt::BcryptError) -> Self {
        tracing::error!("bcrypt error: {:?}", e);
        Self::InternalServerError
    }
}

impl From<jsonwebtoken::errors::Error> for ApiError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        use jsonwebtoken::errors::ErrorKind;
        match e.kind() {
            ErrorKind::ExpiredSignature => {
                tracing::debug!("jwt expired");
                Self::Unauthorized
            }
            ErrorKind::InvalidToken
            | ErrorKind::InvalidSignature
            | ErrorKind::InvalidAlgorithm => {
                tracing::debug!("jwt invalid: {:?}", e);
                Self::Unauthorized
            }
            _ => {
                tracing::error!("jwt error: {:?}", e);
                Self::InternalServerError
            }
        }
    }
}