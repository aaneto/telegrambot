use optbuilder::OptionalBuilder;

use serde_derive::Serialize;

use crate::input::{ChatID, ReplyMarkup};

#[derive(OptionalBuilder, Default, Debug, Serialize)]
pub struct SendContact {
    pub chat_id: ChatID,
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub vcard: Option<String>,
    pub disable_notification: bool,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendContact {
    pub fn new(chat_id: ChatID, phone_number: String, first_name: String) -> Self {
        SendContact {
            chat_id,
            phone_number,
            first_name,
            ..Default::default()
        }
    }
}
