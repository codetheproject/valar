use crate::api::{
    CFrameworkSecurity,
    context::{AuthenticationContext, Map},
};
use axum::{
    extract::{FromRequestParts, Request},
    response::Response,
};
use http::{StatusCode, request::Parts};
use pin_project_lite::pin_project;
use std::{pin::Pin, task::Poll};
use tower_service::Service;

#[derive(new, Clone)]
pub struct CFrameworkService<S> {
    inner: S,
    handler_map: Map,
    context: AuthenticationContext,
}

pin_project! {
    #[derive(Debug, new)]
    pub struct ServiceResponseFuture<F> {
        #[pin]
        future: F
    }
}

impl<S> Service<Request> for CFrameworkService<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ServiceResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request) -> Self::Future {
        // Insert a CframeworkSecurity back inside the extension
        let prev = request
            .extensions_mut()
            .insert(CFrameworkSecurity::new(self.handler_map.clone(), self.context.clone()));
        debug_assert!(prev.is_none(), "Context already present in request extensions");
        ServiceResponseFuture::new(self.inner.call(request))
    }
}

impl<S> tower_layer::Layer<S> for CFrameworkSecurity {
    type Service = CFrameworkService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CFrameworkService::new(inner, self.handler_map().clone(), self.context().clone())
    }
}

impl<F> Future for ServiceResponseFuture<F>
where
    F: Future,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        // Forward the inner future's poll result directly.
        this.future.poll(cx)
    }
}

impl<S> FromRequestParts<S> for CFrameworkSecurity
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<CFrameworkSecurity>()
            .cloned()
            .ok_or_else(|| {
                //TODO: Update this error message later
                error!("Can't extract CFrameworkSecurity. Is `CFrameworkSecurity` added as layer in your `Router`");
                (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error occured, Please check the log!")
            })
    }
}
