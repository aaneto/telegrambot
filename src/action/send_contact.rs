use futures::Future;

use crate::input::SendContact;
use crate::object::Message;
use crate::error::BotError;
use crate::Bot;
use crate::telegram_request::{Method, TelegramRequest};

impl Bot {
    pub fn send_contact(
        self,
        send_contact: SendContact,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"sendContact"), self)
            .with_query(send_contact)
            .execute()
    }
}
