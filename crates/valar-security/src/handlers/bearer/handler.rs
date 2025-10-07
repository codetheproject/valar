#![allow(dead_code, unused_variables)]
use crate::{
    handlers::bearer::{error::BearerError, option::BearerOption, payload::BearerPayload, response::BearerResponse, state::BearerState},
    valarsecurity::{
        handler::{Handler, SignInHandler, SignOutHandler},
        http::IntoCfSecurityRequest,
    },
};
use std::sync::Arc;

// Cookie must be easy to clone we could use the inner scope
#[derive(Clone)]
pub struct BearerHandler {
    inner: Arc<InnerBearerHandler>,
}

impl BearerHandler {
    pub fn new(bearer_option: BearerOption) -> Self {
        let inner = Arc::new(InnerBearerHandler::new(bearer_option));
        Self { inner }
    }
}

#[derive(new)]
struct InnerBearerHandler {
    cookie_option: BearerOption,
}

impl<Request, State> Handler<Request, State> for BearerHandler
where
    Request: IntoCfSecurityRequest + Sync + Send,
    State: Into<BearerState> + Send + Sync,
{
    type Response = BearerResponse;
    type Error = BearerError;

    const NAME: &'static str = "Cookie";

    async fn authenticate(&self, request: Request, state: State) -> Result<Self::Response, Self::Error> {
        let request = request.into_cf_security_request();

        // implement authentication logic here

        Ok(BearerResponse::default())
    }

    async fn forbid(&self, request: Request, state: State) -> Result<Self::Response, Self::Error> {
        let request = request.into_cf_security_request();

        // implement forbid logic here

        Ok(BearerResponse::default())
    }

    async fn challenge(&self, request: Request, state: State) -> Result<Self::Response, Self::Error> {
        let request = request.into_cf_security_request();

        // implement challenge logic here

        Ok(BearerResponse::default())
    }
}

impl<Request, State> SignOutHandler<Request, State> for BearerHandler
where
    Request: IntoCfSecurityRequest + Send + Sync,
    State: Into<BearerState> + Sync + Send,
{
    async fn sign_out(&self, request: Request, _state: State) -> Result<Self::Response, Self::Error> {
        let request = request.into_cf_security_request();

        // implement sign out  logic here

        Ok(BearerResponse::default())
    }
}

impl<Request, State, Payload> SignInHandler<Request, State, Payload> for BearerHandler
where
    Request: IntoCfSecurityRequest + Send + Sync,
    State: Into<BearerState> + Sync + Send,
    Payload: Into<BearerPayload> + Send + Sync,
{
    async fn sign_in(&self, request: Request, state: State, payload: Payload) -> Result<Self::Response, Self::Error> {
        let request = request.into_cf_security_request();

        // implement sign in  logic here

        Ok(BearerResponse::default())
    }
}
