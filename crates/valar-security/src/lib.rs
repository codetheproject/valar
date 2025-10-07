//! Valar - Security
//!
//! # Overview
//!
//! # TODO
#![forbid(unsafe_code)]
// Silence the noise in development!
// #![cfg_attr(debug_assertions, allow(dead_code, unused_variables, unused_imports))]
// Docs and linting rules
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
#![cfg_attr(test, allow(clippy::float_cmp))]
#![cfg_attr(not(test), deny(clippy::print_stdout, clippy::dbg_macro))]
// - Lint for missing docs
#![cfg_attr(not(debug_assertions), deny(missing_docs))]
// Use this page for export and documentation only but we gonna include some tests

// Extern crate
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate derive_new;

// pub mod authorization;
mod handlers;
// Valar framework security is built ontop of the http crates
mod adapter;
mod valarsecurity;
// This pattern is used to control visibility of internal modules

pub mod api {
    use super::*;
    pub use valarsecurity::{
        CFrameworkSecurity, Error, WithRequest, authenticate, get_handler_from_map,
        http::{IntoCfSecurityRequest, IntoCfSecurityResponse, Response},
        sign_in,
    };

    // This expose all of the context api
    pub mod context {
        pub use crate::valarsecurity::context::{AuthenticationContext, Context, ContextLayer, HandlerContext, map::Map};
    }

    pub mod http {
        pub use crate::valarsecurity::http::{IntoCfSecurityRequest, IntoCfSecurityResponse, Response};
    }

    // cookie
    pub mod cookie {
        pub use super::handlers::cookie::{
            CookieHandlerExt, CookieHandlerExtWithRequest, error::CookieError, handler::CookieHandler, option::CookieOption,
            payload::CookiePayload, response::CookieResponse, state::CookieState,
        };
    }
}
pub mod prelude {
    use super::*;
    pub use api::CFrameworkSecurity;

    // cookie
    pub use api::cookie::{CookieHandlerExt, CookieHandlerExtWithRequest};
}
