use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use strum_macros::Display;

pub type BootResult = core::result::Result<(), BootError>;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
#[allow(dead_code)]
pub enum CustomError {
    NotFoundError,
    NoFrequentNumber,
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    NumberParsingError,
    AuthFailCtxNotInRequestExt,
}

#[derive(Debug, Display)]
pub enum BootError {
    CronJobInit,
    CronJobRun,
    Api,
}

// region:    --- Error Boilerplate
impl core::fmt::Display for CustomError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for CustomError {}
// endregion: --- Error Boilerplate

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        response
    }
}

impl CustomError {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::NotFoundError => (StatusCode::NOT_FOUND, ClientError::NOT_FOUND),
            Self::NumberParsingError =>(StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR),
            Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailCtxNotInRequestExt => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    NOT_FOUND,
    NO_AUTH,
    SERVICE_ERROR,
}
