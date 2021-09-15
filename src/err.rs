use actix_web::ResponseError;

use std::fmt::{self, Debug, Display};

pub struct AnyhowResponse(anyhow::Error);

impl std::error::Error for AnyhowResponse {}
impl Display for AnyhowResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for AnyhowResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl ResponseError for AnyhowResponse {}

pub trait EResponse<T> {
    fn as_err_response(self) -> Result<T, AnyhowResponse>;
}

impl<T> EResponse<T> for anyhow::Result<T> {
    fn as_err_response(self) -> Result<T, AnyhowResponse> {
        self.map_err(|e| AnyhowResponse(e))
    }
}
