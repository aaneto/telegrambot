use futures::Future;

use crate::api::args::ChatID;
use crate::api::args::ForwardMessage;
use crate::api::datatypes::Message;
use crate::api::error::APIError;
use crate::api::Bot;
use crate::api::Method;
use crate::api::TelegramRequest;

impl Bot {
    pub fn forward_message<ID: Into<ChatID>>(
        self,
        chat_id: ID,
        from_chat_id: ID,
        message_id: i64,
        disable_notification: bool,
    ) -> impl Future<Item = (Self, Message), Error = APIError> {
        let args = ForwardMessage {
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_id,
            disable_notification,
        };

        TelegramRequest::new(Method::GET, self.get_route(&"forwardMessage"), self)
            .with_query(args)
            .execute()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::Bot;
    use std::env::var;
    use tokio::runtime::Runtime;

    #[test]
    fn resend_message() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");

        let message_id: i64 = var("MESSAGE_ID")
            .expect("Cannot find MESSAGE_ID in ENV")
            .parse()
            .expect("MESSAGE_ID is not an valid ID.");

        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        if let Err(err) = runtime.block_on(bot.forward_message(chat_id, chat_id, message_id, false))
        {
            panic!("{:#?}", err);
        }
    }
}