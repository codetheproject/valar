pub mod request;
pub mod response;
pub(super) mod valarsecurity;

#[cfg(test)]
mod tests {
    use crate::{
        api::cookie::{CookieHandler, CookieOption, CookiePayload, CookieState},
        prelude::*,
    };
    use anyhow::Result;
    use axum::{
        Router,
        body::Body,
        extract::Request,
        response::{IntoResponse, Response},
        routing::{get, post},
    };
    use http::{StatusCode, request::Parts};
    use tower::ServiceExt;

    #[allow(dead_code)]
    #[derive(Debug, serde::Deserialize)]
    pub struct JwtBody {
        token: String,
    }

    #[tokio::test]
    async fn test_context_if_it_compiles_it_works() -> Result<()> {
        let app = Router::new()
            .route(
                "/signup",
                get(|parts: Parts, mut cf_security: CFrameworkSecurity| async move {
                    let _response = cf_security
                        .with(parts)
                        .authenticate::<CookieHandler, _>(CookieState {})
                        .await;

                    // Change this later
                    ().into_response()
                }),
            )
            .layer(CFrameworkSecurity::default());

        let req = Request::builder()
            .uri("/")
            .body(Body::empty())?;

        let res = app.oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    async fn test_cookie_sign_in_with_axum_parts() -> Result<()> {
        let cf = CFrameworkSecurity::default().use_cookie();

        async fn authentication_handler(parts: Parts, mut ctx: CFrameworkSecurity) -> Result<Response, Response> {
            // cookie response
            match ctx
                .with(parts)
                .sign_in_with_cookie(CookieState {}, CookiePayload {})
                .await
            {
                Ok(_response) => println!("Signed in successful"),
                Err(_response) => println!("Signed out failed"),
            }

            // TODO: Response must be returned properly here
            Ok((StatusCode::OK, String::from("Yet to be implemented")).into_response())
        }

        let app = Router::new()
            .route("/", get(authentication_handler))
            .layer(cf);

        let req = Request::builder()
            .uri("/")
            .body(Body::empty())?;

        let res = app.oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::OK, "Response status mismatch and response: {:?}", res);

        Ok(())
    }

    #[tokio::test]
    async fn test_cookie_sign_in_with_axum_typed_request() -> Result<()> {
        let cf = CFrameworkSecurity::default().use_cookie();

        async fn authentication_handler(parts: Parts, mut ctx: CFrameworkSecurity) -> Result<Response, Response> {
            let _response = ctx
                .with(parts)
                .sign_in_with_cookie(CookieState {}, CookiePayload {})
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

            Ok(String::from("Yet to be implemented").into_response())
        }

        let app = Router::new()
            .route("/", post(authentication_handler))
            .layer(cf);

        let req = Request::builder()
            .uri("/")
            .body(Body::empty())?;

        let res = app.oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::OK);

        Ok(())
    }
}
