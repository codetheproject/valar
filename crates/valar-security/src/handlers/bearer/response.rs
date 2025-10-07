use crate::valarsecurity::http::{IntoCfSecurityResponse, Response};

#[derive(Debug, Default)]
pub struct BearerResponse {}

impl IntoCfSecurityResponse for BearerResponse {
    fn into_cf_security_response(self) -> Response {
        todo!()
    }
}
