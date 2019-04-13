#![forbid(unsafe_code)]

pub mod api;
pub mod macros;
pub mod util;

pub mod prelude {
    pub use crate::api::args::*;
    pub use crate::api::datatypes::*;
    pub use crate::api::error::APIError;
    pub use crate::api::APIResponse;
    pub use crate::api::APIResult;
    pub use crate::api::Bot;
    pub use crate::util::*;
    pub use futures;
    pub use futures::Future;
    pub use reqwest;
    pub use tokio;
}

#[cfg(test)]
mod tests {
    //! General tests for the telegrambot wrapper.
    //!
    //! Here should be included functionality tests
    //! for bots. That includes running actual API
    //! calls and expecting an return.
    use crate::api::Bot;
    use crate::util::get_argv;
    use std::error::Error;
    use tokio::runtime::Runtime;

    #[test]
    fn test_get_me() {
        let api_key = get_argv("API_KEY").expect("Cannot find API_KEY in ENV");
        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        if let Err(error) = runtime.block_on(bot.get_me()) {
            panic!(error.description().to_owned());
        }
    }

}
