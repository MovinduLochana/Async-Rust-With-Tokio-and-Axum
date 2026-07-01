use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::{
    error, fmt,
    fmt::{Display, Formatter},
};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,

    // Auth Errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,

    // Model Errors
    TicketDeleteFailIdNotFound { id: u64 },
}

#[allow(non_camel_case_types)]
#[derive(Debug, strum_macros::AsRefStr)]
// #[serde(rename_all = "PascalCase")] // must have Serialized and Deserialized for serde macro to work
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> core::result::Result<(), fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!(">> {:<12} - {self:?}", "INTO_RES");

        // Axum placeholder response
        let mut res = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        res.extensions_mut().insert(self);

        res
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            // Auth
            Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailCtxNotInRequestExt => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // Model
            Self::TicketDeleteFailIdNotFound { .. } => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),

            _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR)
        }
    }
}
