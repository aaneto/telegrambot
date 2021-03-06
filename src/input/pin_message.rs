use serde_derive::Serialize;

use crate::input::ChatID;

#[derive(Debug, Default, Serialize)]
pub struct PinMessage {
    pub chat_id: ChatID,
    pub message_id: i64,
    pub disable_notification: bool,
}

impl PinMessage {
    pub fn new(chat_id: ChatID, message_id: i64, disable_notification: bool) -> Self {
        Self {
            chat_id,
            message_id,
            disable_notification,
        }
    }
}
