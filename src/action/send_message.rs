use futures::Future;

use crate::error::BotError;
use crate::input::SendMessage;
use crate::object::Message;
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
    pub fn send_message(
        self,
        send_message: SendMessage,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"sendMessage"), self)
            .with_query(send_message)
            .execute()
    }
}
