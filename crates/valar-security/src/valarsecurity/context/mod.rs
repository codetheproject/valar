#![cfg_attr(debug_assertions, allow(dead_code, unused_variables))]

pub mod map;

#[derive(Debug)]
pub struct ContextLayer {}

#[derive(Debug)]
pub struct Context {
    layers: ContextLayer,
}

#[derive(Debug)]
pub struct HandlerContext {
    // These context are to hold
    forbid: Context,
    sign_in: Context,
    sign_out: Context,
    challenge: Context,
    authenticate: Context,
}

#[derive(Debug)]
pub struct AuthenticationContext {
    // This context runs first and run on every call
    default_context: Context,

    // This contain handler context and run only on specific actions
    handler_context: HandlerContext,
}


// These are just templates
impl Default for AuthenticationContext {
    fn default() -> Self {
        Self {
            default_context: Context { layers: ContextLayer {} },
            handler_context: HandlerContext {
                forbid: Context { layers: ContextLayer {} },
                sign_in: Context { layers: ContextLayer {} },
                sign_out: Context { layers: ContextLayer {} },
                challenge: Context { layers: ContextLayer {} },
                authenticate: Context { layers: ContextLayer {} },
            },
        }
    }
}

impl Clone for AuthenticationContext {
    fn clone(&self) -> Self {
        Self {
            default_context: Context { layers: ContextLayer {} },
            handler_context: HandlerContext {
                forbid: Context { layers: ContextLayer {} },
                sign_in: Context { layers: ContextLayer {} },
                sign_out: Context { layers: ContextLayer {} },
                challenge: Context { layers: ContextLayer {} },
                authenticate: Context { layers: ContextLayer {} },
            },
        }
    }
}
