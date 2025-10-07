use crate::valarsecurity::Error;
use axum::response::{IntoResponse, Response};

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        todo!()
    }
}
