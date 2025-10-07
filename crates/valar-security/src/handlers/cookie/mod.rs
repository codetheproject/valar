//! # Cookie Authentication
//!
//! # Overview
//! - Cookie authentication is a method of authentication that involves using cookies to store user credentials.

use crate::{
    api::http::IntoCfSecurityRequest,
    handlers::cookie::{
        error::CookieError, handler::CookieHandler, option::CookieOption, payload::CookiePayload, response::CookieResponse,
        state::CookieState,
    },
    valarsecurity::{CFrameworkSecurity, WithRequest, authenticate, sign_in},
};

pub(crate) mod error;
pub(crate) mod handler;
pub(crate) mod option;
pub(crate) mod payload;
pub(crate) mod response;
pub(crate) mod state;

pub trait CookieHandlerExt {
    fn use_cookie(self) -> Self;
    fn configure_cookie<F>(self, callback: F) -> Self
    where
        F: FnOnce(CookieOption) -> CookieOption;
}

#[rustfmt::skip]
pub trait CookieHandlerExtWithRequest<State: Into<CookieState>, Payload: Into<CookiePayload>> {
    fn authenticate_with_cookie(self, state: State) -> impl Future<Output = Result<CookieResponse, CookieError>>;
    fn forbid_with_cookie(self, state: State) -> impl Future<Output = Result<CookieResponse, CookieError>>;
    fn challenge_with_cookie(self, state: State) -> impl Future<Output = Result<CookieResponse, CookieError>>;

    fn sign_out_with_cookie(self, state: State) -> impl Future<Output = Result<CookieResponse, CookieError>>;
    fn sign_in_with_cookie(self, state: State, payload: Payload) -> impl Future<Output = Result<CookieResponse, CookieError>>;
}

impl CookieHandlerExt for CFrameworkSecurity {
    fn use_cookie(self) -> Self {
        self.register_handler(CookieHandler::new(CookieOption::default()))
    }

    fn configure_cookie<F>(self, callback: F) -> Self
    where
        F: FnOnce(CookieOption) -> CookieOption,
    {
        // THis is wrong we are expected to modify
        // self.register_handler(CookieHandler::new(callback(CookieOption::default())))

        //  Reads from the handler map
        // Mutates the existing CookieHandler if present
        // Falls back to default if missing
        // Replaces the handler cleanly

        todo!()
    }
}

impl<'a, Request, State, Payload> CookieHandlerExtWithRequest<State, Payload> for WithRequest<'a, Request>
where
    State: Into<CookieState> + Send + Sync + 'static,
    Payload: Into<CookiePayload> + Send + Sync + 'static,
    Request: IntoCfSecurityRequest + Send + Sync + 'static,
{
    async fn authenticate_with_cookie(self, state: State) -> Result<CookieResponse, CookieError> {
        let (request, handler_map) = self.unpack();
        let handler = handler_map
            .get::<CookieHandler>()
            .ok_or(CookieError::MissingExtension)?;

        authenticate(handler, request, state).await
    }

    async fn forbid_with_cookie(self, _state: State) -> Result<CookieResponse, CookieError> {
        todo!()
    }

    async fn challenge_with_cookie(self, _state: State) -> Result<CookieResponse, CookieError> {
        todo!()
    }

    async fn sign_out_with_cookie(self, _state: State) -> Result<CookieResponse, CookieError> {
        todo!()
    }

    async fn sign_in_with_cookie(self, state: State, payload: Payload) -> Result<CookieResponse, CookieError> {
        let (request, handler_map) = self.unpack();
        let handler = handler_map
            .get::<CookieHandler>()
            .ok_or(CookieError::MissingExtension)?;

        sign_in(handler, request, state, payload).await
    }
}
