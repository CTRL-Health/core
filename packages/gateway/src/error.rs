use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,

    //Auth errors
    AuthFailedNoAuthTokenCookie,
    AuthFailedTokenWrongFormat,
    AuthFailedCtxNotInRequestExt,

    // Model errors
    TicketDeleteFailIdNotFound { id: u64},
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            // Auth
            Self::AuthFailedNoAuthTokenCookie
            | Self::AuthFailedTokenWrongFormat
            | Self::AuthFailedCtxNotInRequestExt => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            }


            // Model
            Self::TicketDeleteFailIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }

            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            )
        }
    }
}


#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}