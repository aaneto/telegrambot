use crate::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

use reqwest::r#async::multipart::Form;
use reqwest::r#async::RequestBuilder;
use reqwest::r#async::Response;

pub enum Method {
    GET,
    POST,
}

pub struct TelegramRequest {
    builder: RequestBuilder,
    bot: Bot,
}

impl TelegramRequest {
    pub fn new(method: Method, route: String, bot: Bot) -> Self {
        let client = &bot.connection.client;

        let request = match method {
            Method::GET => client.get(&route),
            Method::POST => client.post(&route),
        };

        TelegramRequest {
            builder: request,
            bot,
        }
    }

    pub fn with_body<B: Serialize + Sized>(mut self, body_data: B) -> Self {
        self.builder = self.builder.json(&body_data);

        self
    }

    pub fn with_multipart(mut self, form: Form) -> Self {
        self.builder = self.builder.multipart(form);

        self
    }

    pub fn with_query<Q: Serialize + Sized>(mut self, query_data: Q) -> Self {
        self.builder = self.builder.query(&query_data);

        self
    }

    pub fn with_uploader<U: Uploader>(self, tag: &str, file_uploader: U) -> Self {
        file_uploader.upload_into(tag, self)
    }

    pub fn execute<O: DeserializeOwned + std::fmt::Debug>(
        self,
    ) -> impl Future<Item = (Bot, O), Error = APIError> {
        let bot = self.bot;

        self.builder
            .send()
            .and_then(|mut response: Response| response.json())
            .map_err(APIError::from)
            .and_then(|api_response: APIResponse<O>| {
                let api_result: APIResult<O> = api_response.into();

                api_result
            })
            .map(move |data: O| (bot, data))
    }
}

impl Bot {
    pub fn get_me(self) -> impl Future<Item = (Self, User), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"getMe"), self).execute()
    }

    pub fn get_updates(
        self,
        get_updates: GetUpdates,
    ) -> impl Future<Item = (Self, Vec<Update>), Error = APIError> {
        TelegramRequest::new(Method::POST, self.get_route(&"getUpdates"), self)
            .with_body(get_updates)
            .execute()
    }

    pub fn send_message(
        self,
        send_message: SendMessage,
    ) -> impl Future<Item = (Self, Message), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"sendMessage"), self)
            .with_query(send_message)
            .execute()
    }

    pub fn get_chat<ID: Into<ChatID>>(
        self,
        id: ID,
    ) -> impl Future<Item = (Self, Chat), Error = APIError> {
        let get_chat = GetChat::new(id.into());

        TelegramRequest::new(Method::GET, self.get_route(&"getChat"), self)
            .with_query(get_chat)
            .execute()
    }

    pub fn set_chat_title<ID: Into<ChatID>>(
        self,
        id: ID,
        title: String,
    ) -> impl Future<Item = (Self, bool), Error = APIError> {
        let set_chat_title = SetChatTitle::new(id.into(), title);

        TelegramRequest::new(Method::GET, self.get_route(&"setChatTitle"), self)
            .with_query(set_chat_title)
            .execute()
    }

    pub fn set_chat_description<ID: Into<ChatID>>(
        self,
        id: ID,
        description: String,
    ) -> impl Future<Item = (Self, bool), Error = APIError> {
        let set_chat_description = SetChatDescription::new(id.into(), description);

        TelegramRequest::new(Method::GET, self.get_route(&"setChatDescription"), self)
            .with_query(set_chat_description)
            .execute()
    }

    pub fn pin_message<ID: Into<ChatID>>(
        self,
        id: ID,
        message_id: i64,
        disable_notification: bool,
    ) -> impl Future<Item = (Self, bool), Error = APIError> {
        let pin_message = PinMessage::new(id.into(), message_id, disable_notification);

        TelegramRequest::new(Method::GET, self.get_route(&"pinChatMessage"), self)
            .with_query(pin_message)
            .execute()
    }

    pub fn unpin_message<ID: Into<ChatID>>(
        self,
        id: ID,
    ) -> impl Future<Item = (Self, bool), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"unpinChatMessage"), self)
            .with_query(id.into())
            .execute()
    }

    pub fn leave_chat<ID: Into<ChatID>>(
        self,
        id: ID,
    ) -> impl Future<Item = (Self, bool), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"leaveChat"), self)
            .with_query(id.into())
            .execute()
    }

    pub fn get_chat_members_count<ID: Into<ChatID>>(
        self,
        id: ID,
    ) -> impl Future<Item = (Self, u64), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"getChatMembersCount"), self)
            .with_query(id.into())
            .execute()
    }

    pub fn send_location(
        self,
        send_location: SendLocation,
    ) -> impl Future<Item = (Self, Message), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"sendLocation"), self)
            .with_query(send_location)
            .execute()
    }

    pub fn get_file(
        self,
        file_id: String,
    ) -> impl Future<Item = (Self, FileInfo), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"getFile"), self)
            .with_query(GetFile::new(file_id))
            .execute()
    }

    pub fn send_contact(
        self,
        send_contact: SendContact,
    ) -> impl Future<Item = (Self, Message), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"sendContact"), self)
            .with_query(send_contact)
            .execute()
    }

    pub fn get_chat_admins<ID: Into<ChatID>>(
        self,
        chat_id: ID,
    ) -> impl Future<Item = (Self, Vec<ChatMember>), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"getChatAdministrators"), self)
            .with_query(chat_id.into())
            .execute()
    }

    pub fn get_chat_member<ID>(
        self,
        chat_id: ID,
        user_id: i64,
    ) -> impl Future<Item = (Self, ChatMember), Error = APIError>
    where
        ID: Into<ChatID>,
    {
        let get_chat_member = GetChatMember {
            user_id,
            chat_id: chat_id.into(),
        };

        TelegramRequest::new(Method::GET, self.get_route(&"getChatMember"), self)
            .with_query(get_chat_member)
            .execute()
    }

    /// Set sticker set for a group or chat.
    ///
    /// Will only work for groups of at least
    /// 100 members.
    pub fn set_chat_sticker_set<ID: Into<ChatID>>(
        self,
        chat_id: ID,
        sticker_set: &str,
    ) -> impl Future<Item = (Self, bool), Error = APIError> {
        let set_chat_sticker_set = SetChatStickerSet {
            sticker_set_name: sticker_set.to_string(),
            chat_id: chat_id.into(),
        };

        TelegramRequest::new(Method::GET, self.get_route(&"setChatStickerSet"), self)
            .with_query(set_chat_sticker_set)
            .execute()
    }

    /// Delete sticker set for a group or chat.
    ///
    /// Will only work if there is a sticker set defined.
    pub fn delete_chat_sticker_set<ID: Into<ChatID>>(
        self,
        chat_id: ID,
    ) -> impl Future<Item = (Self, bool), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"deleteChatStickerSet"), self)
            .with_query(chat_id.into())
            .execute()
    }

    /// Promote a chat member.
    ///
    /// Note that the promotions are of type Option<bool>
    /// because:
    ///
    /// None => Don't modify this permission
    /// Some(false) => Deny this permission
    /// Some(true) => Grant this permission
    ///
    /// 1. Admins cannot demote/promote the creator of the group.
    /// 2. Admins cannot demote/promote other admins.
    pub fn promote_chat_member(
        self,
        promote_member: PromoteChatMember,
    ) -> impl Future<Item = (Self, bool), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"promoteChatMember"), self)
            .with_query(promote_member)
            .execute()
    }

    pub fn restrict_chat_member(
        self,
        restrict_member: RestrictChatMember,
    ) -> impl Future<Item = (Self, bool), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"restrictChatMember"), self)
            .with_query(restrict_member)
            .execute()
    }

    pub fn delete_chat_photo<ID: Into<ChatID>>(
        self,
        chat_id: ID,
    ) -> impl Future<Item = (Self, bool), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"deleteChatPhoto"), self)
            .with_query(chat_id.into())
            .execute()
    }

    pub fn set_chat_photo<ID: Into<ChatID>>(
        self,
        chat_id: ID,
        file_uploader: FileUploader,
    ) -> impl Future<Item = (Self, bool), Error = APIError> {
        TelegramRequest::new(Method::POST, self.get_route(&"setChatPhoto"), self)
            .with_query(chat_id.into())
            .with_uploader("photo", file_uploader)
            .execute()
    }

    /// Send a photo in telegram.
    ///
    /// Photos can be uploaded by Id, Url and Post
    /// methods. Note that chat photo id's are only
    /// usable for downloading a chat photo, not here.
    pub fn send_photo<U: Uploader>(
        self,
        send_photo: SendPhoto,
        file_uploader: U,
    ) -> impl Future<Item = (Self, Message), Error = APIError> {
        TelegramRequest::new(Method::POST, self.get_route(&"sendPhoto"), self)
            .with_query(send_photo)
            .with_uploader("photo", file_uploader)
            .execute()
    }
}
