use futures::Future;

use crate::error::BotError;
use crate::input::ChatID;
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
    pub fn delete_chat_photo<ID: Into<ChatID>>(
        self,
        chat_id: ID,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"deleteChatPhoto"), self)
            .with_query(chat_id.into())
            .execute()
    }
}
