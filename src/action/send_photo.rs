use futures::Future;

use crate::input::SendPhoto;
use crate::object::Message;
use crate::error::BotError;
use crate::input::Uploader;
use crate::Bot;
use crate::telegram_request::{Method, TelegramRequest};

impl Bot {
    /// Send a photo in telegram.
    ///
    /// Photos can be uploaded by Id, Url and Post
    /// methods. Note that chat photo id's are only
    /// usable for downloading a chat photo, not here.
    pub fn send_photo<U: Uploader + Default>(
        self,
        send_photo: SendPhoto<U>,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        let (query, uploader) = send_photo.split();

        TelegramRequest::new(Method::POST, self.get_route(&"sendPhoto"), self)
            .with_query(query)
            .with_uploader("photo", uploader)
            .execute()
    }
}

#[cfg(test)]
mod tests {
    use crate::input::SendPhoto;
    use crate::object::Message;
    use crate::error::BotError;
    use crate::input::add_mime;
    use crate::input::file;
    use crate::Bot;
    use futures::Future;
    use std::env::var;
    use tokio::runtime::Runtime;

    fn send_photo_future(
        file_name: &str,
        mime_string: Option<&str>,
    ) -> impl Future<Item = (Bot, Message), Error = BotError> {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut uploader_res = file(file_name);

        if let Some(mime_str) = mime_string {
            uploader_res = uploader_res.and_then(add_mime(mime_str));
        }

        let arg = SendPhoto::new(chat_id, uploader_res.unwrap());
        bot.send_photo(arg)
    }

    #[test]
    fn send_photo() {
        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        if let Err(err) =
            runtime.block_on(send_photo_future("res/brownpuppy.png", Some("image/png")))
        {
            panic!("{:#?}", err);
        }
    }

    #[test]
    fn send_photo_without_mime() {
        let mut runtime = Runtime::new().expect("Unable to create a runtime");
        if let Err(err) = runtime.block_on(send_photo_future("res/brownpuppy.png", None)) {
            panic!("{:#?}", err);
        }
    }
}
