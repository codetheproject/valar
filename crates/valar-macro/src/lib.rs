use crate::valarbot::valar_bot_telegram_command;
use proc_macro::TokenStream;

#[cfg(feature = "valar-bot-telegram")]
mod valarbot;

#[cfg(feature = "valar-bot-telegram")]
#[proc_macro_derive(Command)]
pub fn valar_bot_command(token_stream: TokenStream) -> TokenStream {
    valar_bot_telegram_command(token_stream)
}
