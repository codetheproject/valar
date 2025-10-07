use crate::api::{IntoCfSecurityResponse, Response};

#[derive(Debug)]
pub enum BearerError {
    MissingExtension,
}

impl IntoCfSecurityResponse for BearerError {
    fn into_cf_security_response(self) -> Response {
        todo!()
    }
}
