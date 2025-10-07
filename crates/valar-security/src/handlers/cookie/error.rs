use crate::valarsecurity::http::{IntoCfSecurityResponse, Response};

#[derive(Debug)]
pub enum CookieError {
    MissingExtension,
}

impl IntoCfSecurityResponse for CookieError {
    fn into_cf_security_response(self) -> Response {
        todo!()
    }
}
