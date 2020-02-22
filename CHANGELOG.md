# Changelog

## 0.6.0 (27.01.2020)

- Added Bot API 4.6 support.
- Added `ResponseError::can_retry()` method.
- Added `ParseMode::escape()` method.
- Added `get_full_name`, `get_link` and `get_mention` methods to `User` type.
- Removed deprecated `RestrictChatMember` methods.

## 0.5.1 (10.01.2020)

- Return socks proxy support.
- Deprecate some `RestrictChatMember` methods (to be removed in 0.6.0).

## 0.5.0 (07.01.2020)

- Added async/await support.
- Added Bot API 4.5 support.
- Added rustls support.
- Fixed some errors when parsing update.
- Temporarily disabled socks proxy support.

## 0.4.4 (31.07.2019)

- Added Bot API 4.4 support.
- Added information about response data to poll request error.
- `RestrictChatMember` methods:
  `can_send_messages`,
  `can_send_media_messages`,
  `can_send_other_messages`,
  `can_add_web_page_previews` are now deprecated and will be removed in 0.5.0.
  Use `with_permissions` method instead.

## 0.4.3 (07.07.2019)

- Ignore invalid updates in webhook handler.

## 0.4.2 (01.06.2019)

- Added Bot API 4.3 support.

## 0.4.1 (28.05.2019)

- Fixed ChatMemberAdministrator parsing.

## 0.4.0 (07.05.2019)

- Added Bot API 4.2 support.
- Added `prelude` module.
- Added `ChatMember::is_member()` method.
- Added `UpdatesStream::should_retry()` method.
- Added `Api::download_file()` method.
- Added option to set custom base url for Telegram API (see `Config` struct).
- Added multipart support (you can upload files using multipart/form-data now).
- Added `UserId` type.
- You can (de)serialize callback data using `serde_json` (see `CallbackQuery` and `InlineKeyboardButton` types).
- `Api::new()` now takes a `Config` object.
- `Api::execute()` takes a method by value instead of reference.
- `Method::get_request(&self)` replaced by `Method::into_request(self)`
- `RequestBuilder` is now private.
- `WebhookServiceFactory` is now private.
- Removed use of `Mutex` in `WebhookService`.
- Removed `WebhookServiceFactoryError`.
- Removed `InlineKeyboardButton::new()` method as the user must use exactly one of the option fields.

## 0.3.0 (12.03.2019)

- Dispatcher moved to [carapax](https://github.com/tg-rs/carapax).
- Added `Update::get_chat_id()`, `Update::get_chat_username()` and `Update::get_user()` methods.
- Added `Message::is_edited()` and `Message::get_chat_username()` methods.
- Added `Message.commands` property.
- Added `UpdatesStreamOptions`.
- Removed `Api::create()` and `Api::with_proxy()` in favor of `Api::new()`.
- Removed `Api::get_updates()`, use `tgbot::handle_updates()` instead.
- `WebhookService` is public now.
- Respect `retry_after` parameter on polling error.

## 0.2.0 (27.02.2019)

- Migrated from curl to hyper.
- Added dispatcher.
- Added webhooks support.

## 0.1.0 (23.12.2018)

- First release.
