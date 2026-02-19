// this file is auto-generated

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ChatId {
    Integer(i64),
    String(String),
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Attachment {
    InputFile(InputFile),
    String(String),
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum MessageOrTrue {
    Message(Message),
    True(crate::True),
}
/// This <a href="https://core.telegram.org/bots/api#available-types">object</a> represents an incoming update.<br>At most <strong>one</strong> of the optional parameters can be present in any given update.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Update {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using <a href="https://core.telegram.org/bots/api#setwebhook">webhooks</a>, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// New incoming message of any kind - text, photo, sticker, etc.
    pub message: Option<Message>,
    /// New version of a message that is known to the bot and was edited. This update may at times be triggered by changes to message fields that are either unavailable or not actively used by your bot.
    pub edited_message: Option<Message>,
    /// New incoming channel post of any kind - text, photo, sticker, etc.
    pub channel_post: Option<Message>,
    /// New version of a channel post that is known to the bot and was edited. This update may at times be triggered by changes to message fields that are either unavailable or not actively used by your bot.
    pub edited_channel_post: Option<Message>,
    /// The bot was connected to or disconnected from a business account, or a user edited an existing connection with the bot
    pub business_connection: Option<BusinessConnection>,
    /// New message from a connected business account
    pub business_message: Option<Message>,
    /// New version of a message from a connected business account
    pub edited_business_message: Option<Message>,
    /// Messages were deleted from a connected business account
    pub deleted_business_messages: Option<BusinessMessagesDeleted>,
    /// A reaction to a message was changed by a user. The bot must be an administrator in the chat and must explicitly specify <code>"message_reaction"</code> in the list of <em>allowed_updates</em> to receive these updates. The update isn't received for reactions set by bots.
    pub message_reaction: Option<MessageReactionUpdated>,
    /// Reactions to a message with anonymous reactions were changed. The bot must be an administrator in the chat and must explicitly specify <code>"message_reaction_count"</code> in the list of <em>allowed_updates</em> to receive these updates. The updates are grouped and can be sent with delay up to a few minutes.
    pub message_reaction_count: Option<MessageReactionCountUpdated>,
    /// New incoming <a href="https://core.telegram.org/bots/api#inline-mode">inline</a> query
    pub inline_query: Option<InlineQuery>,
    /// The result of an <a href="https://core.telegram.org/bots/api#inline-mode">inline</a> query that was chosen by a user and sent to their chat partner. Please see our documentation on the <a href="/bots/inline#collecting-feedback">feedback collecting</a> for details on how to enable these updates for your bot.
    pub chosen_inline_result: Option<ChosenInlineResult>,
    /// New incoming callback query
    pub callback_query: Option<CallbackQuery>,
    /// New incoming shipping query. Only for invoices with flexible price
    pub shipping_query: Option<ShippingQuery>,
    /// New incoming pre-checkout query. Contains full information about checkout
    pub pre_checkout_query: Option<PreCheckoutQuery>,
    /// A user purchased paid media with a non-empty payload sent by the bot in a non-channel chat
    pub purchased_paid_media: Option<PaidMediaPurchased>,
    /// New poll state. Bots receive only updates about manually stopped polls and polls, which are sent by the bot
    pub poll: Option<Poll>,
    /// A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself.
    pub poll_answer: Option<PollAnswer>,
    /// The bot's chat member status was updated in a chat. For private chats, this update is received only when the bot is blocked or unblocked by the user.
    pub my_chat_member: Option<ChatMemberUpdated>,
    /// A chat member's status was updated in a chat. The bot must be an administrator in the chat and must explicitly specify <code>"chat_member"</code> in the list of <em>allowed_updates</em> to receive these updates.
    pub chat_member: Option<ChatMemberUpdated>,
    /// A request to join the chat has been sent. The bot must have the <em>can_invite_users</em> administrator right in the chat to receive these updates.
    pub chat_join_request: Option<ChatJoinRequest>,
    /// A chat boost was added or changed. The bot must be an administrator in the chat to receive these updates.
    pub chat_boost: Option<ChatBoostUpdated>,
    /// A boost was removed from a chat. The bot must be an administrator in the chat to receive these updates.
    pub removed_chat_boost: Option<ChatBoostRemoved>,
}
/// Use this method to receive incoming updates using long polling (<a href="https://en.wikipedia.org/wiki/Push_technology#Long_polling">wiki</a>). Returns an Array of <a href="https://core.telegram.org/bots/api#update">Update</a> objects.
#[derive(macros::Method)]
#[method(name = "getUpdates", response(Vec<Update>))]
pub struct GetUpdatesRequest {
    /// Identifier of the first update to be returned. Must be greater by one than the highest among the identifiers of previously received updates. By default, updates starting with the earliest unconfirmed update are returned. An update is considered confirmed as soon as <a href="https://core.telegram.org/bots/api#getupdates">getUpdates</a> is called with an <em>offset</em> higher than its <em>update_id</em>. The negative offset can be specified to retrieve updates starting from <em>-offset</em> update from the end of the updates queue. All previous updates will be forgotten.
    pub offset: Option<i64>,
    /// Limits the number of updates to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    pub limit: Option<i64>,
    /// Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling. Should be positive, short polling should be used for testing purposes only.
    pub timeout: Option<i64>,
    /// A JSON-serialized list of the update types you want your bot to receive. For example, specify <code>["message", "edited_channel_post", "callback_query"]</code> to only receive updates of these types. See <a href="https://core.telegram.org/bots/api#update">Update</a> for a complete list of available update types. Specify an empty list to receive all update types except <em>chat_member</em>, <em>message_reaction</em>, and <em>message_reaction_count</em> (default). If not specified, the previous setting will be used.<br><br>Please note that this parameter doesn't affect updates created before the call to getUpdates, so unwanted updates may be received for a short period of time.
    pub allowed_updates: Option<Vec<String>>,
}
/// Use this method to specify a URL and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified URL, containing a JSON-serialized <a href="https://core.telegram.org/bots/api#update">Update</a>. In case of an unsuccessful request (a request with response <a href="https://en.wikipedia.org/wiki/List_of_HTTP_status_codes">HTTP status code</a> different from <code>2XY</code>), we will repeat the request and give up after a reasonable amount of attempts. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setWebhook", response(crate::True))]
pub struct SetWebhookRequest {
    /// HTTPS URL to send updates to. Use an empty string to remove webhook integration
    pub url: String,
    /// Upload your public key certificate so that the root certificate in use can be checked. See our <a href="/bots/self-signed">self-signed guide</a> for details.
    pub certificate: Option<InputFile>,
    /// The fixed IP address which will be used to send webhook requests instead of the IP address resolved through DNS
    pub ip_address: Option<String>,
    /// The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100. Defaults to <em>40</em>. Use lower values to limit the load on your bot's server, and higher values to increase your bot's throughput.
    pub max_connections: Option<i64>,
    /// A JSON-serialized list of the update types you want your bot to receive. For example, specify <code>["message", "edited_channel_post", "callback_query"]</code> to only receive updates of these types. See <a href="https://core.telegram.org/bots/api#update">Update</a> for a complete list of available update types. Specify an empty list to receive all update types except <em>chat_member</em>, <em>message_reaction</em>, and <em>message_reaction_count</em> (default). If not specified, the previous setting will be used.<br>Please note that this parameter doesn't affect updates created before the call to the setWebhook, so unwanted updates may be received for a short period of time.
    pub allowed_updates: Option<Vec<String>>,
    /// Pass <em>True</em> to drop all pending updates
    pub drop_pending_updates: Option<bool>,
    /// A secret token to be sent in a header “X-Telegram-Bot-Api-Secret-Token” in every webhook request, 1-256 characters. Only characters <code>A-Z</code>, <code>a-z</code>, <code>0-9</code>, <code>_</code> and <code>-</code> are allowed. The header is useful to ensure that the request comes from a webhook set by you.
    pub secret_token: Option<String>,
}
/// Use this method to remove webhook integration if you decide to switch back to <a href="https://core.telegram.org/bots/api#getupdates">getUpdates</a>. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "deleteWebhook", response(crate::True))]
pub struct DeleteWebhookRequest {
    /// Pass <em>True</em> to drop all pending updates
    pub drop_pending_updates: Option<bool>,
}
/// Use this method to get current webhook status. Requires no parameters. On success, returns a <a href="https://core.telegram.org/bots/api#webhookinfo">WebhookInfo</a> object. If the bot is using <a href="https://core.telegram.org/bots/api#getupdates">getUpdates</a>, will return an object with the <em>url</em> field empty.
#[derive(macros::Method)]
#[method(name = "getWebhookInfo", response(WebhookInfo))]
pub struct GetWebhookInfoRequest;
/// Describes the current status of a webhook.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WebhookInfo {
    /// Webhook URL, may be empty if webhook is not set up
    pub url: String,
    /// <em>True</em>, if a custom certificate was provided for webhook certificate checks
    pub has_custom_certificate: bool,
    /// Number of updates awaiting delivery
    pub pending_update_count: i64,
    /// Currently used webhook IP address
    pub ip_address: Option<String>,
    /// Unix time for the most recent error that happened when trying to deliver an update via webhook
    pub last_error_date: Option<i64>,
    /// Error message in human-readable format for the most recent error that happened when trying to deliver an update via webhook
    pub last_error_message: Option<String>,
    /// Unix time of the most recent error that happened when trying to synchronize available updates with Telegram datacenters
    pub last_synchronization_error_date: Option<i64>,
    /// The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery
    pub max_connections: Option<i64>,
    /// A list of update types the bot is subscribed to. Defaults to all update types except <em>chat_member</em>
    pub allowed_updates: Option<Vec<String>>,
}
/// This object represents a Telegram user or bot.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    /// Unique identifier for this user or bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// <em>True</em>, if this user is a bot
    pub is_bot: bool,
    /// User's or bot's first name
    pub first_name: String,
    /// User's or bot's last name
    pub last_name: Option<String>,
    /// User's or bot's username
    pub username: Option<String>,
    /// <a href="https://en.wikipedia.org/wiki/IETF_language_tag">IETF language tag</a> of the user's language
    pub language_code: Option<String>,
    /// <em>True</em>, if this user is a Telegram Premium user
    pub is_premium: Option<crate::True>,
    /// <em>True</em>, if this user added the bot to the attachment menu
    pub added_to_attachment_menu: Option<crate::True>,
    /// <em>True</em>, if the bot can be invited to groups. Returned only in <a href="https://core.telegram.org/bots/api#getme">getMe</a>.
    pub can_join_groups: Option<bool>,
    /// <em>True</em>, if <a href="/bots/features#privacy-mode">privacy mode</a> is disabled for the bot. Returned only in <a href="https://core.telegram.org/bots/api#getme">getMe</a>.
    pub can_read_all_group_messages: Option<bool>,
    /// <em>True</em>, if the bot supports inline queries. Returned only in <a href="https://core.telegram.org/bots/api#getme">getMe</a>.
    pub supports_inline_queries: Option<bool>,
    /// <em>True</em>, if the bot can be connected to a Telegram Business account to receive its messages. Returned only in <a href="https://core.telegram.org/bots/api#getme">getMe</a>.
    pub can_connect_to_business: Option<bool>,
    /// <em>True</em>, if the bot has a main Web App. Returned only in <a href="https://core.telegram.org/bots/api#getme">getMe</a>.
    pub has_main_web_app: Option<bool>,
    /// <em>True</em>, if the bot has forum topic mode enabled in private chats. Returned only in <a href="https://core.telegram.org/bots/api#getme">getMe</a>.
    pub has_topics_enabled: Option<bool>,
    /// <em>True</em>, if the bot allows users to create and delete topics in private chats. Returned only in <a href="https://core.telegram.org/bots/api#getme">getMe</a>.
    pub allows_users_to_create_topics: Option<bool>,
}
/// This object represents a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Chat {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Type of the chat, can be either “private”, “group”, “supergroup” or “channel”
    pub r#type: String,
    /// Title, for supergroups, channels and group chats
    pub title: Option<String>,
    /// Username, for private chats, supergroups and channels if available
    pub username: Option<String>,
    /// First name of the other party in a private chat
    pub first_name: Option<String>,
    /// Last name of the other party in a private chat
    pub last_name: Option<String>,
    /// <em>True</em>, if the supergroup chat is a forum (has <a href="https://telegram.org/blog/topics-in-groups-collectible-usernames#topics-in-groups">topics</a> enabled)
    pub is_forum: Option<crate::True>,
    /// <em>True</em>, if the chat is the direct messages chat of a channel
    pub is_direct_messages: Option<crate::True>,
}
/// This object contains full information about a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatFullInfo {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Type of the chat, can be either “private”, “group”, “supergroup” or “channel”
    pub r#type: String,
    /// Title, for supergroups, channels and group chats
    pub title: Option<String>,
    /// Username, for private chats, supergroups and channels if available
    pub username: Option<String>,
    /// First name of the other party in a private chat
    pub first_name: Option<String>,
    /// Last name of the other party in a private chat
    pub last_name: Option<String>,
    /// <em>True</em>, if the supergroup chat is a forum (has <a href="https://telegram.org/blog/topics-in-groups-collectible-usernames#topics-in-groups">topics</a> enabled)
    pub is_forum: Option<crate::True>,
    /// <em>True</em>, if the chat is the direct messages chat of a channel
    pub is_direct_messages: Option<crate::True>,
    /// Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See <a href="https://core.telegram.org/bots/api#accent-colors">accent colors</a> for more details.
    pub accent_color_id: i64,
    /// The maximum number of reactions that can be set on a message in the chat
    pub max_reaction_count: i64,
    /// Chat photo
    pub photo: Option<ChatPhoto>,
    /// If non-empty, the list of all <a href="https://telegram.org/blog/topics-in-groups-collectible-usernames#collectible-usernames">active chat usernames</a>; for private chats, supergroups and channels
    pub active_usernames: Option<Vec<String>>,
    /// For private chats, the date of birth of the user
    pub birthdate: Option<Birthdate>,
    /// For private chats with business accounts, the intro of the business
    pub business_intro: Option<BusinessIntro>,
    /// For private chats with business accounts, the location of the business
    pub business_location: Option<BusinessLocation>,
    /// For private chats with business accounts, the opening hours of the business
    pub business_opening_hours: Option<BusinessOpeningHours>,
    /// For private chats, the personal channel of the user
    pub personal_chat: Option<Chat>,
    /// Information about the corresponding channel chat; for direct messages chats only
    pub parent_chat: Option<Chat>,
    /// List of available reactions allowed in the chat. If omitted, then all <a href="https://core.telegram.org/bots/api#reactiontypeemoji">emoji reactions</a> are allowed.
    pub available_reactions: Option<Vec<ReactionType>>,
    /// Custom emoji identifier of the emoji chosen by the chat for the reply header and link preview background
    pub background_custom_emoji_id: Option<String>,
    /// Identifier of the accent color for the chat's profile background. See <a href="https://core.telegram.org/bots/api#profile-accent-colors">profile accent colors</a> for more details.
    pub profile_accent_color_id: Option<i64>,
    /// Custom emoji identifier of the emoji chosen by the chat for its profile background
    pub profile_background_custom_emoji_id: Option<String>,
    /// Custom emoji identifier of the emoji status of the chat or the other party in a private chat
    pub emoji_status_custom_emoji_id: Option<String>,
    /// Expiration date of the emoji status of the chat or the other party in a private chat, in Unix time, if any
    pub emoji_status_expiration_date: Option<i64>,
    /// Bio of the other party in a private chat
    pub bio: Option<String>,
    /// <em>True</em>, if privacy settings of the other party in the private chat allows to use <code>tg://user?id=<user_id></code> links only in chats with the user
    pub has_private_forwards: Option<crate::True>,
    /// <em>True</em>, if the privacy settings of the other party restrict sending voice and video note messages in the private chat
    pub has_restricted_voice_and_video_messages: Option<crate::True>,
    /// <em>True</em>, if users need to join the supergroup before they can send messages
    pub join_to_send_messages: Option<crate::True>,
    /// <em>True</em>, if all users directly joining the supergroup without using an invite link need to be approved by supergroup administrators
    pub join_by_request: Option<crate::True>,
    /// Description, for groups, supergroups and channel chats
    pub description: Option<String>,
    /// Primary invite link, for groups, supergroups and channel chats
    pub invite_link: Option<String>,
    /// The most recent pinned message (by sending date)
    pub pinned_message: Option<Message>,
    /// Default chat member permissions, for groups and supergroups
    pub permissions: Option<ChatPermissions>,
    /// Information about types of gifts that are accepted by the chat or by the corresponding user for private chats
    pub accepted_gift_types: AcceptedGiftTypes,
    /// <em>True</em>, if paid media messages can be sent or forwarded to the channel chat. The field is available only for channel chats.
    pub can_send_paid_media: Option<crate::True>,
    /// For supergroups, the minimum allowed delay between consecutive messages sent by each unprivileged user; in seconds
    pub slow_mode_delay: Option<i64>,
    /// For supergroups, the minimum number of boosts that a non-administrator user needs to add in order to ignore slow mode and chat permissions
    pub unrestrict_boost_count: Option<i64>,
    /// The time after which all messages sent to the chat will be automatically deleted; in seconds
    pub message_auto_delete_time: Option<i64>,
    /// <em>True</em>, if aggressive anti-spam checks are enabled in the supergroup. The field is only available to chat administrators.
    pub has_aggressive_anti_spam_enabled: Option<crate::True>,
    /// <em>True</em>, if non-administrators can only get the list of bots and administrators in the chat
    pub has_hidden_members: Option<crate::True>,
    /// <em>True</em>, if messages from the chat can't be forwarded to other chats
    pub has_protected_content: Option<crate::True>,
    /// <em>True</em>, if new chat members will have access to old messages; available only to chat administrators
    pub has_visible_history: Option<crate::True>,
    /// For supergroups, name of the group sticker set
    pub sticker_set_name: Option<String>,
    /// <em>True</em>, if the bot can change the group sticker set
    pub can_set_sticker_set: Option<crate::True>,
    /// For supergroups, the name of the group's custom emoji sticker set. Custom emoji from this set can be used by all users and bots in the group.
    pub custom_emoji_sticker_set_name: Option<String>,
    /// Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    pub linked_chat_id: Option<i64>,
    /// For supergroups, the location to which the supergroup is connected
    pub location: Option<ChatLocation>,
    /// For private chats, the rating of the user if any
    pub rating: Option<UserRating>,
    /// For private chats, the first audio added to the profile of the user
    pub first_profile_audio: Option<Audio>,
    /// The color scheme based on a unique gift that must be used for the chat's name, message replies and link previews
    pub unique_gift_colors: Option<UniqueGiftColors>,
    /// The number of Telegram Stars a general user have to pay to send a message to the chat
    pub paid_message_star_count: Option<i64>,
}
/// This object represents a message.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Message {
    /// Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    pub message_id: i64,
    /// Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    pub message_thread_id: Option<i64>,
    /// Information about the direct messages chat topic that contains the message
    pub direct_messages_topic: Option<DirectMessagesTopic>,
    /// Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    pub from: Option<User>,
    /// Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field <em>from</em> contains a fake sender user in non-channel chats.
    pub sender_chat: Option<Chat>,
    /// If the sender of the message boosted the chat, the number of boosts added by the user
    pub sender_boost_count: Option<i64>,
    /// The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    pub sender_business_bot: Option<User>,
    /// Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    pub date: i64,
    /// Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    pub business_connection_id: Option<String>,
    /// Chat the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// <em>True</em>, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    pub is_topic_message: Option<crate::True>,
    /// <em>True</em>, if the message is a channel post that was automatically forwarded to the connected discussion group
    pub is_automatic_forward: Option<crate::True>,
    /// For replies in the same chat and message thread, the original message. Note that the <a href="https://core.telegram.org/bots/api#message">Message</a> object in this field will not contain further <em>reply_to_message</em> fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Information about the message that is being replied to, which may come from another chat or forum topic
    pub external_reply: Option<ExternalReplyInfo>,
    /// For replies that quote part of the original message, the quoted part of the message
    pub quote: Option<TextQuote>,
    /// For replies to a story, the original story
    pub reply_to_story: Option<Story>,
    /// Identifier of the specific checklist task that is being replied to
    pub reply_to_checklist_task_id: Option<i64>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Date the message was last edited in Unix time
    pub edit_date: Option<i64>,
    /// <em>True</em>, if the message can't be forwarded
    pub has_protected_content: Option<crate::True>,
    /// <em>True</em>, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    pub is_from_offline: Option<crate::True>,
    /// <em>True</em>, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    pub is_paid_post: Option<crate::True>,
    /// The unique identifier of a media message group this message belongs to
    pub media_group_id: Option<String>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<String>,
    /// The number of Telegram Stars that were paid by the sender of the message to send it
    pub paid_star_count: Option<i64>,
    /// For text messages, the actual UTF-8 text of the message
    pub text: Option<String>,
    /// For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    pub entities: Option<Vec<MessageEntity>>,
    /// Options used for link preview generation for the message, if it is a text message and link preview options were changed
    pub link_preview_options: Option<LinkPreviewOptions>,
    /// Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    pub suggested_post_info: Option<SuggestedPostInfo>,
    /// Unique identifier of the message effect added to the message
    pub effect_id: Option<String>,
    /// Message is an animation, information about the animation. For backward compatibility, when this field is set, the <em>document</em> field will also be set
    pub animation: Option<Animation>,
    /// Message is an audio file, information about the file
    pub audio: Option<Audio>,
    /// Message is a general file, information about the file
    pub document: Option<Document>,
    /// Message contains paid media; information about the paid media
    pub paid_media: Option<PaidMediaInfo>,
    /// Message is a photo, available sizes of the photo
    pub photo: Option<Vec<PhotoSize>>,
    /// Message is a sticker, information about the sticker
    pub sticker: Option<Sticker>,
    /// Message is a forwarded story
    pub story: Option<Story>,
    /// Message is a video, information about the video
    pub video: Option<Video>,
    /// Message is a <a href="https://telegram.org/blog/video-messages-and-telescope">video note</a>, information about the video message
    pub video_note: Option<VideoNote>,
    /// Message is a voice message, information about the file
    pub voice: Option<Voice>,
    /// Caption for the animation, audio, document, paid media, photo, video or voice
    pub caption: Option<String>,
    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// <em>True</em>, if the caption must be shown above the message media
    pub show_caption_above_media: Option<crate::True>,
    /// <em>True</em>, if the message media is covered by a spoiler animation
    pub has_media_spoiler: Option<crate::True>,
    /// Message is a checklist
    pub checklist: Option<Checklist>,
    /// Message is a shared contact, information about the contact
    pub contact: Option<Contact>,
    /// Message is a dice with random value
    pub dice: Option<Dice>,
    /// Message is a game, information about the game. <a href="https://core.telegram.org/bots/api#games">More about games »</a>
    pub game: Option<Game>,
    /// Message is a native poll, information about the poll
    pub poll: Option<Poll>,
    /// Message is a venue, information about the venue. For backward compatibility, when this field is set, the <em>location</em> field will also be set
    pub venue: Option<Venue>,
    /// Message is a shared location, information about the location
    pub location: Option<Location>,
    /// New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)
    pub new_chat_members: Option<Vec<User>>,
    /// A member was removed from the group, information about them (this member may be the bot itself)
    pub left_chat_member: Option<User>,
    /// Service message: chat owner has left
    pub chat_owner_left: Option<ChatOwnerLeft>,
    /// Service message: chat owner has changed
    pub chat_owner_changed: Option<ChatOwnerChanged>,
    /// A chat title was changed to this value
    pub new_chat_title: Option<String>,
    /// A chat photo was change to this value
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    /// Service message: the chat photo was deleted
    pub delete_chat_photo: Option<crate::True>,
    /// Service message: the group has been created
    pub group_chat_created: Option<crate::True>,
    /// Service message: the supergroup has been created. This field can't be received in a message coming through updates, because bot can't be a member of a supergroup when it is created. It can only be found in reply_to_message if someone replies to a very first message in a directly created supergroup.
    pub supergroup_chat_created: Option<crate::True>,
    /// Service message: the channel has been created. This field can't be received in a message coming through updates, because bot can't be a member of a channel when it is created. It can only be found in reply_to_message if someone replies to a very first message in a channel.
    pub channel_chat_created: Option<crate::True>,
    /// Service message: auto-delete timer settings changed in the chat
    pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
    /// The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub migrate_to_chat_id: Option<i64>,
    /// The supergroup has been migrated from a group with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub migrate_from_chat_id: Option<i64>,
    /// Specified message was pinned. Note that the <a href="https://core.telegram.org/bots/api#message">Message</a> object in this field will not contain further <em>reply_to_message</em> fields even if it itself is a reply.
    pub pinned_message: Option<Box<MaybeInaccessibleMessage>>,
    /// Message is an invoice for a <a href="https://core.telegram.org/bots/api#payments">payment</a>, information about the invoice. <a href="https://core.telegram.org/bots/api#payments">More about payments »</a>
    pub invoice: Option<Invoice>,
    /// Message is a service message about a successful payment, information about the payment. <a href="https://core.telegram.org/bots/api#payments">More about payments »</a>
    pub successful_payment: Option<SuccessfulPayment>,
    /// Message is a service message about a refunded payment, information about the payment. <a href="https://core.telegram.org/bots/api#payments">More about payments »</a>
    pub refunded_payment: Option<RefundedPayment>,
    /// Service message: users were shared with the bot
    pub users_shared: Option<UsersShared>,
    /// Service message: a chat was shared with the bot
    pub chat_shared: Option<ChatShared>,
    /// Service message: a regular gift was sent or received
    pub gift: Option<GiftInfo>,
    /// Service message: a unique gift was sent or received
    pub unique_gift: Option<UniqueGiftInfo>,
    /// Service message: upgrade of a gift was purchased after the gift was sent
    pub gift_upgrade_sent: Option<GiftInfo>,
    /// The domain name of the website on which the user has logged in. <a href="/widgets/login">More about Telegram Login »</a>
    pub connected_website: Option<String>,
    /// Service message: the user allowed the bot to write messages after adding it to the attachment or side menu, launching a Web App from a link, or accepting an explicit request from a Web App sent by the method <a href="/bots/webapps#initializing-mini-apps">requestWriteAccess</a>
    pub write_access_allowed: Option<WriteAccessAllowed>,
    /// Telegram Passport data
    pub passport_data: Option<PassportData>,
    /// Service message. A user in the chat triggered another user's proximity alert while sharing Live Location.
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,
    /// Service message: user boosted the chat
    pub boost_added: Option<ChatBoostAdded>,
    /// Service message: chat background set
    pub chat_background_set: Option<ChatBackground>,
    /// Service message: some tasks in a checklist were marked as done or not done
    pub checklist_tasks_done: Option<Box<ChecklistTasksDone>>,
    /// Service message: tasks were added to a checklist
    pub checklist_tasks_added: Option<Box<ChecklistTasksAdded>>,
    /// Service message: the price for paid messages in the corresponding direct messages chat of a channel has changed
    pub direct_message_price_changed: Option<DirectMessagePriceChanged>,
    /// Service message: forum topic created
    pub forum_topic_created: Option<ForumTopicCreated>,
    /// Service message: forum topic edited
    pub forum_topic_edited: Option<ForumTopicEdited>,
    /// Service message: forum topic closed
    pub forum_topic_closed: Option<ForumTopicClosed>,
    /// Service message: forum topic reopened
    pub forum_topic_reopened: Option<ForumTopicReopened>,
    /// Service message: the 'General' forum topic hidden
    pub general_forum_topic_hidden: Option<GeneralForumTopicHidden>,
    /// Service message: the 'General' forum topic unhidden
    pub general_forum_topic_unhidden: Option<GeneralForumTopicUnhidden>,
    /// Service message: a scheduled giveaway was created
    pub giveaway_created: Option<GiveawayCreated>,
    /// The message is a scheduled giveaway message
    pub giveaway: Option<Giveaway>,
    /// A giveaway with public winners was completed
    pub giveaway_winners: Option<GiveawayWinners>,
    /// Service message: a giveaway without public winners was completed
    pub giveaway_completed: Option<Box<GiveawayCompleted>>,
    /// Service message: the price for paid messages has changed in the chat
    pub paid_message_price_changed: Option<PaidMessagePriceChanged>,
    /// Service message: a suggested post was approved
    pub suggested_post_approved: Option<Box<SuggestedPostApproved>>,
    /// Service message: approval of a suggested post has failed
    pub suggested_post_approval_failed: Option<Box<SuggestedPostApprovalFailed>>,
    /// Service message: a suggested post was declined
    pub suggested_post_declined: Option<Box<SuggestedPostDeclined>>,
    /// Service message: payment for a suggested post was received
    pub suggested_post_paid: Option<Box<SuggestedPostPaid>>,
    /// Service message: payment for a suggested post was refunded
    pub suggested_post_refunded: Option<Box<SuggestedPostRefunded>>,
    /// Service message: video chat scheduled
    pub video_chat_scheduled: Option<VideoChatScheduled>,
    /// Service message: video chat started
    pub video_chat_started: Option<VideoChatStarted>,
    /// Service message: video chat ended
    pub video_chat_ended: Option<VideoChatEnded>,
    /// Service message: new participants invited to a video chat
    pub video_chat_participants_invited: Option<VideoChatParticipantsInvited>,
    /// Service message: data sent by a Web App
    pub web_app_data: Option<WebAppData>,
    /// <a href="/bots/features#inline-keyboards">Inline keyboard</a> attached to the message. <code>login_url</code> buttons are represented as ordinary <code>url</code> buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// This object represents a unique message identifier.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageId {
    /// Unique message identifier. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    pub message_id: i64,
}
/// This object describes a message that was deleted or is otherwise inaccessible to the bot.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InaccessibleMessage {
    /// Chat the message belonged to
    pub chat: Chat,
    /// Unique message identifier inside the chat
    pub message_id: i64,
    /// Always 0. The field can be used to differentiate regular and inaccessible messages.
    pub date: i64,
}
/// This object describes a message that can be inaccessible to the bot.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum MaybeInaccessibleMessage {
    Message(Message),
    InaccessibleMessage(InaccessibleMessage),
}
/// This object represents one special entity in a text message. For example, hashtags, usernames, URLs, etc.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageEntity {
    /// Type of the entity. Currently, can be “mention” (<code>@username</code>), “hashtag” (<code>#hashtag</code> or <code>#hashtag@chatusername</code>), “cashtag” (<code>$USD</code> or <code>$USD@chatusername</code>), “bot_command” (<code>/start@jobs_bot</code>), “url” (<code>https://telegram.org</code>), “email” (<code>do-not-reply@telegram.org</code>), “phone_number” (<code>+1-212-555-0123</code>), “bold” (<strong>bold text</strong>), “italic” (<em>italic text</em>), “underline” (underlined text), “strikethrough” (strikethrough text), “spoiler” (spoiler message), “blockquote” (block quotation), “expandable_blockquote” (collapsed-by-default block quotation), “code” (monowidth string), “pre” (monowidth block), “text_link” (for clickable text URLs), “text_mention” (for users <a href="https://telegram.org/blog/edit#new-mentions">without usernames</a>), “custom_emoji” (for inline custom emoji stickers)
    pub r#type: String,
    /// Offset in <a href="/api/entities#entity-length">UTF-16 code units</a> to the start of the entity
    pub offset: i64,
    /// Length of the entity in <a href="/api/entities#entity-length">UTF-16 code units</a>
    pub length: i64,
    /// For “text_link” only, URL that will be opened after user taps on the text
    pub url: Option<String>,
    /// For “text_mention” only, the mentioned user
    pub user: Option<User>,
    /// For “pre” only, the programming language of the entity text
    pub language: Option<String>,
    /// For “custom_emoji” only, unique identifier of the custom emoji. Use <a href="https://core.telegram.org/bots/api#getcustomemojistickers">getCustomEmojiStickers</a> to get full information about the sticker
    pub custom_emoji_id: Option<String>,
}
/// This object contains information about the quoted part of a message that is replied to by the given message.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TextQuote {
    /// Text of the quoted part of a message that is replied to by the given message
    pub text: String,
    /// Special entities that appear in the quote. Currently, only <em>bold</em>, <em>italic</em>, <em>underline</em>, <em>strikethrough</em>, <em>spoiler</em>, and <em>custom_emoji</em> entities are kept in quotes.
    pub entities: Option<Vec<MessageEntity>>,
    /// Approximate quote position in the original message in UTF-16 code units as specified by the sender
    pub position: i64,
    /// <em>True</em>, if the quote was chosen manually by the message sender. Otherwise, the quote was added automatically by the server.
    pub is_manual: Option<crate::True>,
}
/// This object contains information about a message that is being replied to, which may come from another chat or forum topic.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExternalReplyInfo {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    pub message_id: Option<i64>,
    /// Options used for link preview generation for the original message, if it is a text message
    pub link_preview_options: Option<LinkPreviewOptions>,
    /// Message is an animation, information about the animation
    pub animation: Option<Animation>,
    /// Message is an audio file, information about the file
    pub audio: Option<Audio>,
    /// Message is a general file, information about the file
    pub document: Option<Document>,
    /// Message contains paid media; information about the paid media
    pub paid_media: Option<PaidMediaInfo>,
    /// Message is a photo, available sizes of the photo
    pub photo: Option<Vec<PhotoSize>>,
    /// Message is a sticker, information about the sticker
    pub sticker: Option<Sticker>,
    /// Message is a forwarded story
    pub story: Option<Story>,
    /// Message is a video, information about the video
    pub video: Option<Video>,
    /// Message is a <a href="https://telegram.org/blog/video-messages-and-telescope">video note</a>, information about the video message
    pub video_note: Option<VideoNote>,
    /// Message is a voice message, information about the file
    pub voice: Option<Voice>,
    /// <em>True</em>, if the message media is covered by a spoiler animation
    pub has_media_spoiler: Option<crate::True>,
    /// Message is a checklist
    pub checklist: Option<Checklist>,
    /// Message is a shared contact, information about the contact
    pub contact: Option<Contact>,
    /// Message is a dice with random value
    pub dice: Option<Dice>,
    /// Message is a game, information about the game. <a href="https://core.telegram.org/bots/api#games">More about games »</a>
    pub game: Option<Game>,
    /// Message is a scheduled giveaway, information about the giveaway
    pub giveaway: Option<Giveaway>,
    /// A giveaway with public winners was completed
    pub giveaway_winners: Option<GiveawayWinners>,
    /// Message is an invoice for a <a href="https://core.telegram.org/bots/api#payments">payment</a>, information about the invoice. <a href="https://core.telegram.org/bots/api#payments">More about payments »</a>
    pub invoice: Option<Invoice>,
    /// Message is a shared location, information about the location
    pub location: Option<Location>,
    /// Message is a native poll, information about the poll
    pub poll: Option<Poll>,
    /// Message is a venue, information about the venue
    pub venue: Option<Venue>,
}
/// Describes reply parameters for the message that is being sent.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReplyParameters {
    /// Identifier of the message that will be replied to in the current chat, or in the chat <em>chat_id</em> if it is specified
    pub message_id: i64,
    /// If the message to be replied to is from a different chat, unique identifier for the chat or username of the channel (in the format <code>@channelusername</code>). Not supported for messages sent on behalf of a business account and messages from channel direct messages chats.
    pub chat_id: Option<ChatId>,
    /// Pass <em>True</em> if the message should be sent even if the specified message to be replied to is not found. Always <em>False</em> for replies in another chat or forum topic. Always <em>True</em> for messages sent on behalf of a business account.
    pub allow_sending_without_reply: Option<bool>,
    /// Quoted part of the message to be replied to; 0-1024 characters after entities parsing. The quote must be an exact substring of the message to be replied to, including <em>bold</em>, <em>italic</em>, <em>underline</em>, <em>strikethrough</em>, <em>spoiler</em>, and <em>custom_emoji</em> entities. The message will fail to send if the quote isn't found in the original message.
    pub quote: Option<String>,
    /// Mode for parsing entities in the quote. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub quote_parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the quote. It can be specified instead of <em>quote_parse_mode</em>.
    pub quote_entities: Option<Vec<MessageEntity>>,
    /// Position of the quote in the original message in UTF-16 code units
    pub quote_position: Option<i64>,
    /// Identifier of the specific checklist task to be replied to
    pub checklist_task_id: Option<i64>,
}
/// This object describes the origin of a message.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum MessageOrigin {
    User(MessageOriginUser),
    HiddenUser(MessageOriginHiddenUser),
    Chat(MessageOriginChat),
    Channel(MessageOriginChannel),
}
/// The message was originally sent by a known user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageOriginUser {
    /// Type of the message origin, always “user”
    pub r#type: String,
    /// Date the message was sent originally in Unix time
    pub date: i64,
    /// User that sent the message originally
    pub sender_user: User,
}
/// The message was originally sent by an unknown user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageOriginHiddenUser {
    /// Type of the message origin, always “hidden_user”
    pub r#type: String,
    /// Date the message was sent originally in Unix time
    pub date: i64,
    /// Name of the user that sent the message originally
    pub sender_user_name: String,
}
/// The message was originally sent on behalf of a chat to a group chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageOriginChat {
    /// Type of the message origin, always “chat”
    pub r#type: String,
    /// Date the message was sent originally in Unix time
    pub date: i64,
    /// Chat that sent the message originally
    pub sender_chat: Chat,
    /// For messages originally sent by an anonymous chat administrator, original message author signature
    pub author_signature: Option<String>,
}
/// The message was originally sent to a channel chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageOriginChannel {
    /// Type of the message origin, always “channel”
    pub r#type: String,
    /// Date the message was sent originally in Unix time
    pub date: i64,
    /// Channel chat to which the message was originally sent
    pub chat: Chat,
    /// Unique message identifier inside the chat
    pub message_id: i64,
    /// Signature of the original post author
    pub author_signature: Option<String>,
}
/// This object represents one size of a photo or a <a href="https://core.telegram.org/bots/api#document">file</a> / <a href="https://core.telegram.org/bots/api#sticker">sticker</a> thumbnail.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PhotoSize {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Photo width
    pub width: i64,
    /// Photo height
    pub height: i64,
    /// File size in bytes
    pub file_size: Option<i64>,
}
/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video without sound).
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Animation {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width as defined by the sender
    pub width: i64,
    /// Video height as defined by the sender
    pub height: i64,
    /// Duration of the video in seconds as defined by the sender
    pub duration: i64,
    /// Animation thumbnail as defined by the sender
    pub thumbnail: Option<PhotoSize>,
    /// Original animation filename as defined by the sender
    pub file_name: Option<String>,
    /// MIME type of the file as defined by the sender
    pub mime_type: Option<String>,
    /// File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    pub file_size: Option<i64>,
}
/// This object represents an audio file to be treated as music by the Telegram clients.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Audio {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Duration of the audio in seconds as defined by the sender
    pub duration: i64,
    /// Performer of the audio as defined by the sender or by audio tags
    pub performer: Option<String>,
    /// Title of the audio as defined by the sender or by audio tags
    pub title: Option<String>,
    /// Original filename as defined by the sender
    pub file_name: Option<String>,
    /// MIME type of the file as defined by the sender
    pub mime_type: Option<String>,
    /// File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    pub file_size: Option<i64>,
    /// Thumbnail of the album cover to which the music file belongs
    pub thumbnail: Option<PhotoSize>,
}
/// This object represents a general file (as opposed to <a href="https://core.telegram.org/bots/api#photosize">photos</a>, <a href="https://core.telegram.org/bots/api#voice">voice messages</a> and <a href="https://core.telegram.org/bots/api#audio">audio files</a>).
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Document {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Document thumbnail as defined by the sender
    pub thumbnail: Option<PhotoSize>,
    /// Original filename as defined by the sender
    pub file_name: Option<String>,
    /// MIME type of the file as defined by the sender
    pub mime_type: Option<String>,
    /// File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    pub file_size: Option<i64>,
}
/// This object represents a story.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Story {
    /// Chat that posted the story
    pub chat: Chat,
    /// Unique identifier for the story in the chat
    pub id: i64,
}
/// This object represents a video file of a specific quality.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VideoQuality {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width
    pub width: i64,
    /// Video height
    pub height: i64,
    /// Codec that was used to encode the video, for example, “h264”, “h265”, or “av01”
    pub codec: String,
    /// File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    pub file_size: Option<i64>,
}
/// This object represents a video file.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Video {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width as defined by the sender
    pub width: i64,
    /// Video height as defined by the sender
    pub height: i64,
    /// Duration of the video in seconds as defined by the sender
    pub duration: i64,
    /// Video thumbnail
    pub thumbnail: Option<PhotoSize>,
    /// Available sizes of the cover of the video in the message
    pub cover: Option<Vec<PhotoSize>>,
    /// Timestamp in seconds from which the video will play in the message
    pub start_timestamp: Option<i64>,
    /// List of available qualities of the video
    pub qualities: Option<Vec<VideoQuality>>,
    /// Original filename as defined by the sender
    pub file_name: Option<String>,
    /// MIME type of the file as defined by the sender
    pub mime_type: Option<String>,
    /// File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    pub file_size: Option<i64>,
}
/// This object represents a <a href="https://telegram.org/blog/video-messages-and-telescope">video message</a> (available in Telegram apps as of <a href="https://telegram.org/blog/video-messages-and-telescope">v.4.0</a>).
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VideoNote {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width and height (diameter of the video message) as defined by the sender
    pub length: i64,
    /// Duration of the video in seconds as defined by the sender
    pub duration: i64,
    /// Video thumbnail
    pub thumbnail: Option<PhotoSize>,
    /// File size in bytes
    pub file_size: Option<i64>,
}
/// This object represents a voice note.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Voice {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Duration of the audio in seconds as defined by the sender
    pub duration: i64,
    /// MIME type of the file as defined by the sender
    pub mime_type: Option<String>,
    /// File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    pub file_size: Option<i64>,
}
/// Describes the paid media added to a message.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaidMediaInfo {
    /// The number of Telegram Stars that must be paid to buy access to the media
    pub star_count: i64,
    /// Information about the paid media
    pub paid_media: Vec<PaidMedia>,
}
/// This object describes paid media.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum PaidMedia {
    Preview(PaidMediaPreview),
    Photo(PaidMediaPhoto),
    Video(PaidMediaVideo),
}
/// The paid media isn't available before the payment.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaidMediaPreview {
    /// Type of the paid media, always “preview”
    pub r#type: String,
    /// Media width as defined by the sender
    pub width: Option<i64>,
    /// Media height as defined by the sender
    pub height: Option<i64>,
    /// Duration of the media in seconds as defined by the sender
    pub duration: Option<i64>,
}
/// The paid media is a photo.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaidMediaPhoto {
    /// Type of the paid media, always “photo”
    pub r#type: String,
    /// The photo
    pub photo: Vec<PhotoSize>,
}
/// The paid media is a video.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaidMediaVideo {
    /// Type of the paid media, always “video”
    pub r#type: String,
    /// The video
    pub video: Video,
}
/// This object represents a phone contact.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Contact {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    pub last_name: Option<String>,
    /// Contact's user identifier in Telegram. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    pub user_id: Option<i64>,
    /// Additional data about the contact in the form of a <a href="https://en.wikipedia.org/wiki/VCard">vCard</a>
    pub vcard: Option<String>,
}
/// This object represents an animated emoji that displays a random value.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Dice {
    /// Emoji on which the dice throw animation is based
    pub emoji: String,
    /// Value of the dice, 1-6 for “<img class="emoji" src="//telegram.org/img/emoji/40/F09F8EB2.png" width="20" height="20" alt="🎲" />”, “<img class="emoji" src="//telegram.org/img/emoji/40/F09F8EAF.png" width="20" height="20" alt="🎯" />” and “<img class="emoji" src="//telegram.org/img/emoji/40/F09F8EB3.png" width="20" height="20" alt="🎳" />” base emoji, 1-5 for “<img class="emoji" src="//telegram.org/img/emoji/40/F09F8F80.png" width="20" height="20" alt="🏀" />” and “<img class="emoji" src="//telegram.org/img/emoji/40/E29ABD.png" width="20" height="20" alt="⚽" />” base emoji, 1-64 for “<img class="emoji" src="//telegram.org/img/emoji/40/F09F8EB0.png" width="20" height="20" alt="🎰" />” base emoji
    pub value: i64,
}
/// This object contains information about one answer option in a poll.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PollOption {
    /// Option text, 1-100 characters
    pub text: String,
    /// Special entities that appear in the option <em>text</em>. Currently, only custom emoji entities are allowed in poll option texts
    pub text_entities: Option<Vec<MessageEntity>>,
    /// Number of users that voted for this option
    pub voter_count: i64,
}
/// This object contains information about one answer option in a poll to be sent.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputPollOption {
    /// Option text, 1-100 characters
    pub text: String,
    /// Mode for parsing entities in the text. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details. Currently, only custom emoji entities are allowed
    pub text_parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the poll option text. It can be specified instead of <em>text_parse_mode</em>
    pub text_entities: Option<Vec<MessageEntity>>,
}
/// This object represents an answer of a user in a non-anonymous poll.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PollAnswer {
    /// Unique poll identifier
    pub poll_id: String,
    /// The chat that changed the answer to the poll, if the voter is anonymous
    pub voter_chat: Option<Chat>,
    /// The user that changed the answer to the poll, if the voter isn't anonymous
    pub user: Option<User>,
    /// 0-based identifiers of chosen answer options. May be empty if the vote was retracted.
    pub option_ids: Vec<i64>,
}
/// This object contains information about a poll.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Poll {
    /// Unique poll identifier
    pub id: String,
    /// Poll question, 1-300 characters
    pub question: String,
    /// Special entities that appear in the <em>question</em>. Currently, only custom emoji entities are allowed in poll questions
    pub question_entities: Option<Vec<MessageEntity>>,
    /// List of poll options
    pub options: Vec<PollOption>,
    /// Total number of users that voted in the poll
    pub total_voter_count: i64,
    /// <em>True</em>, if the poll is closed
    pub is_closed: bool,
    /// <em>True</em>, if the poll is anonymous
    pub is_anonymous: bool,
    /// Poll type, currently can be “regular” or “quiz”
    pub r#type: String,
    /// <em>True</em>, if the poll allows multiple answers
    pub allows_multiple_answers: bool,
    /// 0-based identifier of the correct answer option. Available only for polls in the quiz mode, which are closed, or was sent (not forwarded) by the bot or to the private chat with the bot.
    pub correct_option_id: Option<i64>,
    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters
    pub explanation: Option<String>,
    /// Special entities like usernames, URLs, bot commands, etc. that appear in the <em>explanation</em>
    pub explanation_entities: Option<Vec<MessageEntity>>,
    /// Amount of time in seconds the poll will be active after creation
    pub open_period: Option<i64>,
    /// Point in time (Unix timestamp) when the poll will be automatically closed
    pub close_date: Option<i64>,
}
/// Describes a task in a checklist.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChecklistTask {
    /// Unique identifier of the task
    pub id: i64,
    /// Text of the task
    pub text: String,
    /// Special entities that appear in the task text
    pub text_entities: Option<Vec<MessageEntity>>,
    /// User that completed the task; omitted if the task wasn't completed by a user
    pub completed_by_user: Option<User>,
    /// Chat that completed the task; omitted if the task wasn't completed by a chat
    pub completed_by_chat: Option<Chat>,
    /// Point in time (Unix timestamp) when the task was completed; 0 if the task wasn't completed
    pub completion_date: Option<i64>,
}
/// Describes a checklist.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Checklist {
    /// Title of the checklist
    pub title: String,
    /// Special entities that appear in the checklist title
    pub title_entities: Option<Vec<MessageEntity>>,
    /// List of tasks in the checklist
    pub tasks: Vec<ChecklistTask>,
    /// <em>True</em>, if users other than the creator of the list can add tasks to the list
    pub others_can_add_tasks: Option<crate::True>,
    /// <em>True</em>, if users other than the creator of the list can mark tasks as done or not done
    pub others_can_mark_tasks_as_done: Option<crate::True>,
}
/// Describes a task to add to a checklist.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputChecklistTask {
    /// Unique identifier of the task; must be positive and unique among all task identifiers currently present in the checklist
    pub id: i64,
    /// Text of the task; 1-100 characters after entities parsing
    pub text: String,
    /// Mode for parsing entities in the text. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the text, which can be specified instead of parse_mode. Currently, only <em>bold</em>, <em>italic</em>, <em>underline</em>, <em>strikethrough</em>, <em>spoiler</em>, and <em>custom_emoji</em> entities are allowed.
    pub text_entities: Option<Vec<MessageEntity>>,
}
/// Describes a checklist to create.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputChecklist {
    /// Title of the checklist; 1-255 characters after entities parsing
    pub title: String,
    /// Mode for parsing entities in the title. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the title, which can be specified instead of parse_mode. Currently, only <em>bold</em>, <em>italic</em>, <em>underline</em>, <em>strikethrough</em>, <em>spoiler</em>, and <em>custom_emoji</em> entities are allowed.
    pub title_entities: Option<Vec<MessageEntity>>,
    /// List of 1-30 tasks in the checklist
    pub tasks: Vec<InputChecklistTask>,
    /// Pass <em>True</em> if other users can add tasks to the checklist
    pub others_can_add_tasks: Option<bool>,
    /// Pass <em>True</em> if other users can mark tasks as done or not done in the checklist
    pub others_can_mark_tasks_as_done: Option<bool>,
}
/// Describes a service message about checklist tasks marked as done or not done.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChecklistTasksDone {
    /// Message containing the checklist whose tasks were marked as done or not done. Note that the <a href="https://core.telegram.org/bots/api#message">Message</a> object in this field will not contain the <em>reply_to_message</em> field even if it itself is a reply.
    pub checklist_message: Option<Box<Message>>,
    /// Identifiers of the tasks that were marked as done
    pub marked_as_done_task_ids: Option<Vec<i64>>,
    /// Identifiers of the tasks that were marked as not done
    pub marked_as_not_done_task_ids: Option<Vec<i64>>,
}
/// Describes a service message about tasks added to a checklist.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChecklistTasksAdded {
    /// Message containing the checklist to which the tasks were added. Note that the <a href="https://core.telegram.org/bots/api#message">Message</a> object in this field will not contain the <em>reply_to_message</em> field even if it itself is a reply.
    pub checklist_message: Option<Box<Message>>,
    /// List of tasks added to the checklist
    pub tasks: Vec<ChecklistTask>,
}
/// This object represents a point on the map.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Location {
    /// Latitude as defined by the sender
    pub latitude: f64,
    /// Longitude as defined by the sender
    pub longitude: f64,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,
    /// Time relative to the message sending date, during which the location can be updated; in seconds. For active live locations only.
    pub live_period: Option<i64>,
    /// The direction in which user is moving, in degrees; 1-360. For active live locations only.
    pub heading: Option<i64>,
    /// The maximum distance for proximity alerts about approaching another chat member, in meters. For sent live locations only.
    pub proximity_alert_radius: Option<i64>,
}
/// This object represents a venue.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Venue {
    /// Venue location. Can't be a live location
    pub location: Location,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue
    pub google_place_id: Option<String>,
    /// Google Places type of the venue. (See <a href="https://developers.google.com/places/web-service/supported_types">supported types</a>.)
    pub google_place_type: Option<String>,
}
/// Describes data sent from a <a href="/bots/webapps">Web App</a> to the bot.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WebAppData {
    /// The data. Be aware that a bad client can send arbitrary data in this field.
    pub data: String,
    /// Text of the <em>web_app</em> keyboard button from which the Web App was opened. Be aware that a bad client can send arbitrary data in this field.
    pub button_text: String,
}
/// This object represents the content of a service message, sent whenever a user in the chat triggers a proximity alert set by another user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProximityAlertTriggered {
    /// User that triggered the alert
    pub traveler: User,
    /// User that set the alert
    pub watcher: User,
    /// The distance between the users
    pub distance: i64,
}
/// This object represents a service message about a change in auto-delete timer settings.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageAutoDeleteTimerChanged {
    /// New auto-delete time for messages in the chat; in seconds
    pub message_auto_delete_time: i64,
}
/// This object represents a service message about a user boosting a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatBoostAdded {
    /// Number of boosts added by the user
    pub boost_count: i64,
}
/// This object describes the way a background is filled based on the selected colors.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BackgroundFill {
    Solid(BackgroundFillSolid),
    Gradient(BackgroundFillGradient),
    FreeformGradient(BackgroundFillFreeformGradient),
}
/// The background is filled using the selected color.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BackgroundFillSolid {
    /// Type of the background fill, always “solid”
    pub r#type: String,
    /// The color of the background fill in the RGB24 format
    pub color: i64,
}
/// The background is a gradient fill.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BackgroundFillGradient {
    /// Type of the background fill, always “gradient”
    pub r#type: String,
    /// Top color of the gradient in the RGB24 format
    pub top_color: i64,
    /// Bottom color of the gradient in the RGB24 format
    pub bottom_color: i64,
    /// Clockwise rotation angle of the background fill in degrees; 0-359
    pub rotation_angle: i64,
}
/// The background is a freeform gradient that rotates after every message in the chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BackgroundFillFreeformGradient {
    /// Type of the background fill, always “freeform_gradient”
    pub r#type: String,
    /// A list of the 3 or 4 base colors that are used to generate the freeform gradient in the RGB24 format
    pub colors: Vec<i64>,
}
/// This object describes the type of a background.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BackgroundType {
    Fill(BackgroundTypeFill),
    Wallpaper(BackgroundTypeWallpaper),
    Pattern(BackgroundTypePattern),
    ChatTheme(BackgroundTypeChatTheme),
}
/// The background is automatically filled based on the selected colors.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BackgroundTypeFill {
    /// Type of the background, always “fill”
    pub r#type: String,
    /// The background fill
    pub fill: BackgroundFill,
    /// Dimming of the background in dark themes, as a percentage; 0-100
    pub dark_theme_dimming: i64,
}
/// The background is a wallpaper in the JPEG format.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BackgroundTypeWallpaper {
    /// Type of the background, always “wallpaper”
    pub r#type: String,
    /// Document with the wallpaper
    pub document: Document,
    /// Dimming of the background in dark themes, as a percentage; 0-100
    pub dark_theme_dimming: i64,
    /// <em>True</em>, if the wallpaper is downscaled to fit in a 450x450 square and then box-blurred with radius 12
    pub is_blurred: Option<crate::True>,
    /// <em>True</em>, if the background moves slightly when the device is tilted
    pub is_moving: Option<crate::True>,
}
/// The background is a .PNG or .TGV (gzipped subset of SVG with MIME type “application/x-tgwallpattern”) pattern to be combined with the background fill chosen by the user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BackgroundTypePattern {
    /// Type of the background, always “pattern”
    pub r#type: String,
    /// Document with the pattern
    pub document: Document,
    /// The background fill that is combined with the pattern
    pub fill: BackgroundFill,
    /// Intensity of the pattern when it is shown above the filled background; 0-100
    pub intensity: i64,
    /// <em>True</em>, if the background fill must be applied only to the pattern itself. All other pixels are black in this case. For dark themes only
    pub is_inverted: Option<crate::True>,
    /// <em>True</em>, if the background moves slightly when the device is tilted
    pub is_moving: Option<crate::True>,
}
/// The background is taken directly from a built-in chat theme.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BackgroundTypeChatTheme {
    /// Type of the background, always “chat_theme”
    pub r#type: String,
    /// Name of the chat theme, which is usually an emoji
    pub theme_name: String,
}
/// This object represents a chat background.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatBackground {
    /// Type of the background
    pub r#type: BackgroundType,
}
/// This object represents a service message about a new forum topic created in the chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForumTopicCreated {
    /// Name of the topic
    pub name: String,
    /// Color of the topic icon in RGB format
    pub icon_color: i64,
    /// Unique identifier of the custom emoji shown as the topic icon
    pub icon_custom_emoji_id: Option<String>,
    /// <em>True</em>, if the name of the topic wasn't specified explicitly by its creator and likely needs to be changed by the bot
    pub is_name_implicit: Option<crate::True>,
}
/// This object represents a service message about a forum topic closed in the chat. Currently holds no information.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForumTopicClosed;
/// This object represents a service message about an edited forum topic.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForumTopicEdited {
    /// New name of the topic, if it was edited
    pub name: Option<String>,
    /// New identifier of the custom emoji shown as the topic icon, if it was edited; an empty string if the icon was removed
    pub icon_custom_emoji_id: Option<String>,
}
/// This object represents a service message about a forum topic reopened in the chat. Currently holds no information.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForumTopicReopened;
/// This object represents a service message about General forum topic hidden in the chat. Currently holds no information.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GeneralForumTopicHidden;
/// This object represents a service message about General forum topic unhidden in the chat. Currently holds no information.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GeneralForumTopicUnhidden;
/// This object contains information about a user that was shared with the bot using a <a href="https://core.telegram.org/bots/api#keyboardbuttonrequestusers">KeyboardButtonRequestUsers</a> button.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SharedUser {
    /// Identifier of the shared user. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so 64-bit integers or double-precision float types are safe for storing these identifiers. The bot may not have access to the user and could be unable to use this identifier, unless the user is already known to the bot by some other means.
    pub user_id: i64,
    /// First name of the user, if the name was requested by the bot
    pub first_name: Option<String>,
    /// Last name of the user, if the name was requested by the bot
    pub last_name: Option<String>,
    /// Username of the user, if the username was requested by the bot
    pub username: Option<String>,
    /// Available sizes of the chat photo, if the photo was requested by the bot
    pub photo: Option<Vec<PhotoSize>>,
}
/// This object contains information about the users whose identifiers were shared with the bot using a <a href="https://core.telegram.org/bots/api#keyboardbuttonrequestusers">KeyboardButtonRequestUsers</a> button.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UsersShared {
    /// Identifier of the request
    pub request_id: i64,
    /// Information about users shared with the bot.
    pub users: Vec<SharedUser>,
}
/// This object contains information about a chat that was shared with the bot using a <a href="https://core.telegram.org/bots/api#keyboardbuttonrequestchat">KeyboardButtonRequestChat</a> button.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatShared {
    /// Identifier of the request
    pub request_id: i64,
    /// Identifier of the shared chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. The bot may not have access to the chat and could be unable to use this identifier, unless the chat is already known to the bot by some other means.
    pub chat_id: i64,
    /// Title of the chat, if the title was requested by the bot.
    pub title: Option<String>,
    /// Username of the chat, if the username was requested by the bot and available.
    pub username: Option<String>,
    /// Available sizes of the chat photo, if the photo was requested by the bot
    pub photo: Option<Vec<PhotoSize>>,
}
/// This object represents a service message about a user allowing a bot to write messages after adding it to the attachment menu, launching a Web App from a link, or accepting an explicit request from a Web App sent by the method <a href="/bots/webapps#initializing-mini-apps">requestWriteAccess</a>.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WriteAccessAllowed {
    /// <em>True</em>, if the access was granted after the user accepted an explicit request from a Web App sent by the method <a href="/bots/webapps#initializing-mini-apps">requestWriteAccess</a>
    pub from_request: Option<bool>,
    /// Name of the Web App, if the access was granted when the Web App was launched from a link
    pub web_app_name: Option<String>,
    /// <em>True</em>, if the access was granted when the bot was added to the attachment or side menu
    pub from_attachment_menu: Option<bool>,
}
/// This object represents a service message about a video chat scheduled in the chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VideoChatScheduled {
    /// Point in time (Unix timestamp) when the video chat is supposed to be started by a chat administrator
    pub start_date: i64,
}
/// This object represents a service message about a video chat started in the chat. Currently holds no information.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VideoChatStarted;
/// This object represents a service message about a video chat ended in the chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VideoChatEnded {
    /// Video chat duration in seconds
    pub duration: i64,
}
/// This object represents a service message about new members invited to a video chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VideoChatParticipantsInvited {
    /// New members that were invited to the video chat
    pub users: Vec<User>,
}
/// Describes a service message about a change in the price of paid messages within a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaidMessagePriceChanged {
    /// The new number of Telegram Stars that must be paid by non-administrator users of the supergroup chat for each sent message
    pub paid_message_star_count: i64,
}
/// Describes a service message about a change in the price of direct messages sent to a channel chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DirectMessagePriceChanged {
    /// <em>True</em>, if direct messages are enabled for the channel chat; false otherwise
    pub are_direct_messages_enabled: bool,
    /// The new number of Telegram Stars that must be paid by users for each direct message sent to the channel. Does not apply to users who have been exempted by administrators. Defaults to 0.
    pub direct_message_star_count: Option<i64>,
}
/// Describes a service message about the approval of a suggested post.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuggestedPostApproved {
    /// Message containing the suggested post. Note that the <a href="https://core.telegram.org/bots/api#message">Message</a> object in this field will not contain the <em>reply_to_message</em> field even if it itself is a reply.
    pub suggested_post_message: Option<Box<Message>>,
    /// Amount paid for the post
    pub price: Option<SuggestedPostPrice>,
    /// Date when the post will be published
    pub send_date: i64,
}
/// Describes a service message about the failed approval of a suggested post. Currently, only caused by insufficient user funds at the time of approval.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuggestedPostApprovalFailed {
    /// Message containing the suggested post whose approval has failed. Note that the <a href="https://core.telegram.org/bots/api#message">Message</a> object in this field will not contain the <em>reply_to_message</em> field even if it itself is a reply.
    pub suggested_post_message: Option<Box<Message>>,
    /// Expected price of the post
    pub price: SuggestedPostPrice,
}
/// Describes a service message about the rejection of a suggested post.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuggestedPostDeclined {
    /// Message containing the suggested post. Note that the <a href="https://core.telegram.org/bots/api#message">Message</a> object in this field will not contain the <em>reply_to_message</em> field even if it itself is a reply.
    pub suggested_post_message: Option<Box<Message>>,
    /// Comment with which the post was declined
    pub comment: Option<String>,
}
/// Describes a service message about a successful payment for a suggested post.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuggestedPostPaid {
    /// Message containing the suggested post. Note that the <a href="https://core.telegram.org/bots/api#message">Message</a> object in this field will not contain the <em>reply_to_message</em> field even if it itself is a reply.
    pub suggested_post_message: Option<Box<Message>>,
    /// Currency in which the payment was made. Currently, one of “XTR” for Telegram Stars or “TON” for toncoins
    pub currency: String,
    /// The amount of the currency that was received by the channel in nanotoncoins; for payments in toncoins only
    pub amount: Option<i64>,
    /// The amount of Telegram Stars that was received by the channel; for payments in Telegram Stars only
    pub star_amount: Option<StarAmount>,
}
/// Describes a service message about a payment refund for a suggested post.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuggestedPostRefunded {
    /// Message containing the suggested post. Note that the <a href="https://core.telegram.org/bots/api#message">Message</a> object in this field will not contain the <em>reply_to_message</em> field even if it itself is a reply.
    pub suggested_post_message: Option<Box<Message>>,
    /// Reason for the refund. Currently, one of “post_deleted” if the post was deleted within 24 hours of being posted or removed from scheduled messages without being posted, or “payment_refunded” if the payer refunded their payment.
    pub reason: String,
}
/// This object represents a service message about the creation of a scheduled giveaway.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GiveawayCreated {
    /// The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only
    pub prize_star_count: Option<i64>,
}
/// This object represents a message about a scheduled giveaway.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Giveaway {
    /// The list of chats which the user must join to participate in the giveaway
    pub chats: Vec<Chat>,
    /// Point in time (Unix timestamp) when winners of the giveaway will be selected
    pub winners_selection_date: i64,
    /// The number of users which are supposed to be selected as winners of the giveaway
    pub winner_count: i64,
    /// <em>True</em>, if only users who join the chats after the giveaway started should be eligible to win
    pub only_new_members: Option<crate::True>,
    /// <em>True</em>, if the list of giveaway winners will be visible to everyone
    pub has_public_winners: Option<crate::True>,
    /// Description of additional giveaway prize
    pub prize_description: Option<String>,
    /// A list of two-letter <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO 3166-1 alpha-2</a> country codes indicating the countries from which eligible users for the giveaway must come. If empty, then all users can participate in the giveaway. Users with a phone number that was bought on Fragment can always participate in giveaways.
    pub country_codes: Option<Vec<String>>,
    /// The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only
    pub prize_star_count: Option<i64>,
    /// The number of months the Telegram Premium subscription won from the giveaway will be active for; for Telegram Premium giveaways only
    pub premium_subscription_month_count: Option<i64>,
}
/// This object represents a message about the completion of a giveaway with public winners.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GiveawayWinners {
    /// The chat that created the giveaway
    pub chat: Chat,
    /// Identifier of the message with the giveaway in the chat
    pub giveaway_message_id: i64,
    /// Point in time (Unix timestamp) when winners of the giveaway were selected
    pub winners_selection_date: i64,
    /// Total number of winners in the giveaway
    pub winner_count: i64,
    /// List of up to 100 winners of the giveaway
    pub winners: Vec<User>,
    /// The number of other chats the user had to join in order to be eligible for the giveaway
    pub additional_chat_count: Option<i64>,
    /// The number of Telegram Stars that were split between giveaway winners; for Telegram Star giveaways only
    pub prize_star_count: Option<i64>,
    /// The number of months the Telegram Premium subscription won from the giveaway will be active for; for Telegram Premium giveaways only
    pub premium_subscription_month_count: Option<i64>,
    /// Number of undistributed prizes
    pub unclaimed_prize_count: Option<i64>,
    /// <em>True</em>, if only users who had joined the chats after the giveaway started were eligible to win
    pub only_new_members: Option<crate::True>,
    /// <em>True</em>, if the giveaway was canceled because the payment for it was refunded
    pub was_refunded: Option<crate::True>,
    /// Description of additional giveaway prize
    pub prize_description: Option<String>,
}
/// This object represents a service message about the completion of a giveaway without public winners.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GiveawayCompleted {
    /// Number of winners in the giveaway
    pub winner_count: i64,
    /// Number of undistributed prizes
    pub unclaimed_prize_count: Option<i64>,
    /// Message with the giveaway that was completed, if it wasn't deleted
    pub giveaway_message: Option<Box<Message>>,
    /// <em>True</em>, if the giveaway is a Telegram Star giveaway. Otherwise, currently, the giveaway is a Telegram Premium giveaway.
    pub is_star_giveaway: Option<crate::True>,
}
/// Describes the options used for link preview generation.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LinkPreviewOptions {
    /// <em>True</em>, if the link preview is disabled
    pub is_disabled: Option<bool>,
    /// URL to use for the link preview. If empty, then the first URL found in the message text will be used
    pub url: Option<String>,
    /// <em>True</em>, if the media in the link preview is supposed to be shrunk; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview
    pub prefer_small_media: Option<bool>,
    /// <em>True</em>, if the media in the link preview is supposed to be enlarged; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview
    pub prefer_large_media: Option<bool>,
    /// <em>True</em>, if the link preview must be shown above the message text; otherwise, the link preview will be shown below the message text
    pub show_above_text: Option<bool>,
}
/// Describes the price of a suggested post.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuggestedPostPrice {
    /// Currency in which the post will be paid. Currently, must be one of “XTR” for Telegram Stars or “TON” for toncoins
    pub currency: String,
    /// The amount of the currency that will be paid for the post in the <em>smallest units</em> of the currency, i.e. Telegram Stars or nanotoncoins. Currently, price in Telegram Stars must be between 5 and 100000, and price in nanotoncoins must be between 10000000 and 10000000000000.
    pub amount: i64,
}
/// Contains information about a suggested post.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuggestedPostInfo {
    /// State of the suggested post. Currently, it can be one of “pending”, “approved”, “declined”.
    pub state: String,
    /// Proposed price of the post. If the field is omitted, then the post is unpaid.
    pub price: Option<SuggestedPostPrice>,
    /// Proposed send date of the post. If the field is omitted, then the post can be published at any time within 30 days at the sole discretion of the user or administrator who approves it.
    pub send_date: Option<i64>,
}
/// Contains parameters of a post that is being suggested by the bot.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuggestedPostParameters {
    /// Proposed price for the post. If the field is omitted, then the post is unpaid.
    pub price: Option<SuggestedPostPrice>,
    /// Proposed send date of the post. If specified, then the date must be between 300 second and 2678400 seconds (30 days) in the future. If the field is omitted, then the post can be published at any time within 30 days at the sole discretion of the user who approves it.
    pub send_date: Option<i64>,
}
/// Describes a topic of a direct messages chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DirectMessagesTopic {
    /// Unique identifier of the topic. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    pub topic_id: i64,
    /// Information about the user that created the topic. Currently, it is always present
    pub user: Option<User>,
}
/// This object represent a user's profile pictures.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has
    pub total_count: i64,
    /// Requested profile pictures (in up to 4 sizes each)
    pub photos: Vec<Vec<PhotoSize>>,
}
/// This object represents the audios displayed on a user's profile.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserProfileAudios {
    /// Total number of profile audios for the target user
    pub total_count: i64,
    /// Requested profile audios
    pub audios: Vec<Audio>,
}
/// This object represents a file ready to be downloaded. The file can be downloaded via the link <code>https://api.telegram.org/file/bot<token>/<file_path></code>. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling <a href="https://core.telegram.org/bots/api#getfile">getFile</a>.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct File {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    pub file_size: Option<i64>,
    /// File path. Use <code>https://api.telegram.org/file/bot<token>/<file_path></code> to get the file.
    pub file_path: Option<String>,
}
/// Describes a <a href="/bots/webapps">Web App</a>.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WebAppInfo {
    /// An HTTPS URL of a Web App to be opened with additional data as specified in <a href="/bots/webapps#initializing-mini-apps">Initializing Web Apps</a>
    pub url: String,
}
/// This object represents a <a href="/bots/features#keyboards">custom keyboard</a> with reply options (see <a href="/bots/features#keyboards">Introduction to bots</a> for details and examples). Not supported in channels and for messages sent on behalf of a Telegram Business account.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReplyKeyboardMarkup {
    /// Array of button rows, each represented by an Array of <a href="https://core.telegram.org/bots/api#keyboardbutton">KeyboardButton</a> objects
    pub keyboard: Vec<Vec<KeyboardButton>>,
    /// Requests clients to always show the keyboard when the regular keyboard is hidden. Defaults to <em>false</em>, in which case the custom keyboard can be hidden and opened with a keyboard icon.
    pub is_persistent: Option<bool>,
    /// Requests clients to resize the keyboard vertically for optimal fit (e.g., make the keyboard smaller if there are just two rows of buttons). Defaults to <em>false</em>, in which case the custom keyboard is always of the same height as the app's standard keyboard.
    pub resize_keyboard: Option<bool>,
    /// Requests clients to hide the keyboard as soon as it's been used. The keyboard will still be available, but clients will automatically display the usual letter-keyboard in the chat - the user can press a special button in the input field to see the custom keyboard again. Defaults to <em>false</em>.
    pub one_time_keyboard: Option<bool>,
    /// The placeholder to be shown in the input field when the keyboard is active; 1-64 characters
    pub input_field_placeholder: Option<String>,
    /// Use this parameter if you want to show the keyboard to specific users only. Targets: 1) users that are @mentioned in the <em>text</em> of the <a href="https://core.telegram.org/bots/api#message">Message</a> object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message.<br><br><em>Example:</em> A user requests to change the bot's language, bot replies to the request with a keyboard to select the new language. Other users in the group don't see the keyboard.
    pub selective: Option<bool>,
}
/// This object represents one button of the reply keyboard. At most one of the fields other than <em>text</em>, <em>icon_custom_emoji_id</em>, and <em>style</em> must be used to specify the type of the button. For simple text buttons, <em>String</em> can be used instead of this object to specify the button text.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct KeyboardButton {
    /// Text of the button. If none of the fields other than <em>text</em>, <em>icon_custom_emoji_id</em>, and <em>style</em> are used, it will be sent as a message when the button is pressed
    pub text: String,
    /// Unique identifier of the custom emoji shown before the text of the button. Can only be used by bots that purchased additional usernames on <a href="https://fragment.com">Fragment</a> or in the messages directly sent by the bot to private, group and supergroup chats if the owner of the bot has a Telegram Premium subscription.
    pub icon_custom_emoji_id: Option<String>,
    /// Style of the button. Must be one of “danger” (red), “success” (green) or “primary” (blue). If omitted, then an app-specific style is used.
    pub style: Option<String>,
    /// If specified, pressing the button will open a list of suitable users. Identifiers of selected users will be sent to the bot in a “users_shared” service message. Available in private chats only.
    pub request_users: Option<KeyboardButtonRequestUsers>,
    /// If specified, pressing the button will open a list of suitable chats. Tapping on a chat will send its identifier to the bot in a “chat_shared” service message. Available in private chats only.
    pub request_chat: Option<KeyboardButtonRequestChat>,
    /// If <em>True</em>, the user's phone number will be sent as a contact when the button is pressed. Available in private chats only.
    pub request_contact: Option<bool>,
    /// If <em>True</em>, the user's current location will be sent when the button is pressed. Available in private chats only.
    pub request_location: Option<bool>,
    /// If specified, the user will be asked to create a poll and send it to the bot when the button is pressed. Available in private chats only.
    pub request_poll: Option<KeyboardButtonPollType>,
    /// If specified, the described <a href="/bots/webapps">Web App</a> will be launched when the button is pressed. The Web App will be able to send a “web_app_data” service message. Available in private chats only.
    pub web_app: Option<WebAppInfo>,
}
/// This object defines the criteria used to request suitable users. Information about the selected users will be shared with the bot when the corresponding button is pressed. <a href="/bots/features#chat-and-user-selection">More about requesting users »</a>
#[derive(serde::Serialize, serde::Deserialize)]
pub struct KeyboardButtonRequestUsers {
    /// Signed 32-bit identifier of the request that will be received back in the <a href="https://core.telegram.org/bots/api#usersshared">UsersShared</a> object. Must be unique within the message
    pub request_id: i64,
    /// Pass <em>True</em> to request bots, pass <em>False</em> to request regular users. If not specified, no additional restrictions are applied.
    pub user_is_bot: Option<bool>,
    /// Pass <em>True</em> to request premium users, pass <em>False</em> to request non-premium users. If not specified, no additional restrictions are applied.
    pub user_is_premium: Option<bool>,
    /// The maximum number of users to be selected; 1-10. Defaults to 1.
    pub max_quantity: Option<i64>,
    /// Pass <em>True</em> to request the users' first and last names
    pub request_name: Option<bool>,
    /// Pass <em>True</em> to request the users' usernames
    pub request_username: Option<bool>,
    /// Pass <em>True</em> to request the users' photos
    pub request_photo: Option<bool>,
}
/// This object defines the criteria used to request a suitable chat. Information about the selected chat will be shared with the bot when the corresponding button is pressed. The bot will be granted requested rights in the chat if appropriate. <a href="/bots/features#chat-and-user-selection">More about requesting chats »</a>.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct KeyboardButtonRequestChat {
    /// Signed 32-bit identifier of the request, which will be received back in the <a href="https://core.telegram.org/bots/api#chatshared">ChatShared</a> object. Must be unique within the message
    pub request_id: i64,
    /// Pass <em>True</em> to request a channel chat, pass <em>False</em> to request a group or a supergroup chat.
    pub chat_is_channel: bool,
    /// Pass <em>True</em> to request a forum supergroup, pass <em>False</em> to request a non-forum chat. If not specified, no additional restrictions are applied.
    pub chat_is_forum: Option<bool>,
    /// Pass <em>True</em> to request a supergroup or a channel with a username, pass <em>False</em> to request a chat without a username. If not specified, no additional restrictions are applied.
    pub chat_has_username: Option<bool>,
    /// Pass <em>True</em> to request a chat owned by the user. Otherwise, no additional restrictions are applied.
    pub chat_is_created: Option<bool>,
    /// A JSON-serialized object listing the required administrator rights of the user in the chat. The rights must be a superset of <em>bot_administrator_rights</em>. If not specified, no additional restrictions are applied.
    pub user_administrator_rights: Option<ChatAdministratorRights>,
    /// A JSON-serialized object listing the required administrator rights of the bot in the chat. The rights must be a subset of <em>user_administrator_rights</em>. If not specified, no additional restrictions are applied.
    pub bot_administrator_rights: Option<ChatAdministratorRights>,
    /// Pass <em>True</em> to request a chat with the bot as a member. Otherwise, no additional restrictions are applied.
    pub bot_is_member: Option<bool>,
    /// Pass <em>True</em> to request the chat's title
    pub request_title: Option<bool>,
    /// Pass <em>True</em> to request the chat's username
    pub request_username: Option<bool>,
    /// Pass <em>True</em> to request the chat's photo
    pub request_photo: Option<bool>,
}
/// This object represents type of a poll, which is allowed to be created and sent when the corresponding button is pressed.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct KeyboardButtonPollType {
    /// If <em>quiz</em> is passed, the user will be allowed to create only polls in the quiz mode. If <em>regular</em> is passed, only regular polls will be allowed. Otherwise, the user will be allowed to create a poll of any type.
    pub r#type: Option<String>,
}
/// Upon receiving a message with this object, Telegram clients will remove the current custom keyboard and display the default letter-keyboard. By default, custom keyboards are displayed until a new keyboard is sent by a bot. An exception is made for one-time keyboards that are hidden immediately after the user presses a button (see <a href="https://core.telegram.org/bots/api#replykeyboardmarkup">ReplyKeyboardMarkup</a>). Not supported in channels and for messages sent on behalf of a Telegram Business account.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReplyKeyboardRemove {
    /// Requests clients to remove the custom keyboard (user will not be able to summon this keyboard; if you want to hide the keyboard from sight but keep it accessible, use <em>one_time_keyboard</em> in <a href="https://core.telegram.org/bots/api#replykeyboardmarkup">ReplyKeyboardMarkup</a>)
    pub remove_keyboard: crate::True,
    /// Use this parameter if you want to remove the keyboard for specific users only. Targets: 1) users that are @mentioned in the <em>text</em> of the <a href="https://core.telegram.org/bots/api#message">Message</a> object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message.<br><br><em>Example:</em> A user votes in a poll, bot returns confirmation message in reply to the vote and removes the keyboard for that user, while still showing the keyboard with poll options to users who haven't voted yet.
    pub selective: Option<bool>,
}
/// This object represents an <a href="/bots/features#inline-keyboards">inline keyboard</a> that appears right next to the message it belongs to.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an Array of <a href="https://core.telegram.org/bots/api#inlinekeyboardbutton">InlineKeyboardButton</a> objects
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}
/// This object represents one button of an inline keyboard. Exactly one of the fields other than <em>text</em>, <em>icon_custom_emoji_id</em>, and <em>style</em> must be used to specify the type of the button.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineKeyboardButton {
    /// Label text on the button
    pub text: String,
    /// Unique identifier of the custom emoji shown before the text of the button. Can only be used by bots that purchased additional usernames on <a href="https://fragment.com">Fragment</a> or in the messages directly sent by the bot to private, group and supergroup chats if the owner of the bot has a Telegram Premium subscription.
    pub icon_custom_emoji_id: Option<String>,
    /// Style of the button. Must be one of “danger” (red), “success” (green) or “primary” (blue). If omitted, then an app-specific style is used.
    pub style: Option<String>,
    /// HTTP or tg:// URL to be opened when the button is pressed. Links <code>tg://user?id=<user_id></code> can be used to mention a user by their identifier without using a username, if this is allowed by their privacy settings.
    pub url: Option<String>,
    /// Data to be sent in a <a href="https://core.telegram.org/bots/api#callbackquery">callback query</a> to the bot when the button is pressed, 1-64 bytes
    pub callback_data: Option<String>,
    /// Description of the <a href="/bots/webapps">Web App</a> that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method <a href="https://core.telegram.org/bots/api#answerwebappquery">answerWebAppQuery</a>. Available only in private chats between a user and the bot. Not supported for messages sent on behalf of a Telegram Business account.
    pub web_app: Option<WebAppInfo>,
    /// An HTTPS URL used to automatically authorize the user. Can be used as a replacement for the <a href="/widgets/login">Telegram Login Widget</a>.
    pub login_url: Option<LoginUrl>,
    /// If set, pressing the button will prompt the user to select one of their chats, open that chat and insert the bot's username and the specified inline query in the input field. May be empty, in which case just the bot's username will be inserted. Not supported for messages sent in channel direct messages chats and on behalf of a Telegram Business account.
    pub switch_inline_query: Option<String>,
    /// If set, pressing the button will insert the bot's username and the specified inline query in the current chat's input field. May be empty, in which case only the bot's username will be inserted.<br><br>This offers a quick way for the user to open your bot in inline mode in the same chat - good for selecting something from multiple options. Not supported in channels and for messages sent in channel direct messages chats and on behalf of a Telegram Business account.
    pub switch_inline_query_current_chat: Option<String>,
    /// If set, pressing the button will prompt the user to select one of their chats of the specified type, open that chat and insert the bot's username and the specified inline query in the input field. Not supported for messages sent in channel direct messages chats and on behalf of a Telegram Business account.
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,
    /// Description of the button that copies the specified text to the clipboard.
    pub copy_text: Option<CopyTextButton>,
    /// Description of the game that will be launched when the user presses the button.<br><br><strong>NOTE:</strong> This type of button <strong>must</strong> always be the first button in the first row.
    pub callback_game: Option<CallbackGame>,
    /// Specify <em>True</em>, to send a <a href="https://core.telegram.org/bots/api#payments">Pay button</a>. Substrings “<img class="emoji" src="//telegram.org/img/emoji/40/E2AD90.png" width="20" height="20" alt="⭐" />” and “XTR” in the buttons's text will be replaced with a Telegram Star icon.<br><br><strong>NOTE:</strong> This type of button <strong>must</strong> always be the first button in the first row and can only be used in invoice messages.
    pub pay: Option<bool>,
}
/// This object represents a parameter of the inline keyboard button used to automatically authorize a user. Serves as a great replacement for the <a href="/widgets/login">Telegram Login Widget</a> when the user is coming from Telegram. All the user needs to do is tap/click a button and confirm that they want to log in:
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginUrl {
    /// An HTTPS URL to be opened with user authorization data added to the query string when the button is pressed. If the user refuses to provide authorization data, the original URL without information about the user will be opened. The data added is the same as described in <a href="/widgets/login#receiving-authorization-data">Receiving authorization data</a>.<br><br><strong>NOTE:</strong> You <strong>must</strong> always check the hash of the received data to verify the authentication and the integrity of the data as described in <a href="/widgets/login#checking-authorization">Checking authorization</a>.
    pub url: String,
    /// New text of the button in forwarded messages.
    pub forward_text: Option<String>,
    /// Username of a bot, which will be used for user authorization. See <a href="/widgets/login#setting-up-a-bot">Setting up a bot</a> for more details. If not specified, the current bot's username will be assumed. The <em>url</em>'s domain must be the same as the domain linked with the bot. See <a href="/widgets/login#linking-your-domain-to-the-bot">Linking your domain to the bot</a> for more details.
    pub bot_username: Option<String>,
    /// Pass <em>True</em> to request the permission for your bot to send messages to the user.
    pub request_write_access: Option<bool>,
}
/// This object represents an inline button that switches the current user to inline mode in a chosen chat, with an optional default inline query.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchInlineQueryChosenChat {
    /// The default inline query to be inserted in the input field. If left empty, only the bot's username will be inserted
    pub query: Option<String>,
    /// <em>True</em>, if private chats with users can be chosen
    pub allow_user_chats: Option<bool>,
    /// <em>True</em>, if private chats with bots can be chosen
    pub allow_bot_chats: Option<bool>,
    /// <em>True</em>, if group and supergroup chats can be chosen
    pub allow_group_chats: Option<bool>,
    /// <em>True</em>, if channel chats can be chosen
    pub allow_channel_chats: Option<bool>,
}
/// This object represents an inline keyboard button that copies specified text to the clipboard.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CopyTextButton {
    /// The text to be copied to the clipboard; 1-256 characters
    pub text: String,
}
/// This object represents an incoming callback query from a callback button in an <a href="/bots/features#inline-keyboards">inline keyboard</a>. If the button that originated the query was attached to a message sent by the bot, the field <em>message</em> will be present. If the button was attached to a message sent via the bot (in <a href="https://core.telegram.org/bots/api#inline-mode">inline mode</a>), the field <em>inline_message_id</em> will be present. Exactly one of the fields <em>data</em> or <em>game_short_name</em> will be present.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CallbackQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Message sent by the bot with the callback button that originated the query
    pub message: Option<MaybeInaccessibleMessage>,
    /// Identifier of the message sent via the bot in inline mode, that originated the query.
    pub inline_message_id: Option<String>,
    /// Global identifier, uniquely corresponding to the chat to which the message with the callback button was sent. Useful for high scores in <a href="https://core.telegram.org/bots/api#games">games</a>.
    pub chat_instance: String,
    /// Data associated with the callback button. Be aware that the message originated the query can contain no callback buttons with this data.
    pub data: Option<String>,
    /// Short name of a <a href="https://core.telegram.org/bots/api#games">Game</a> to be returned, serves as the unique identifier for the game
    pub game_short_name: Option<String>,
}
/// Upon receiving a message with this object, Telegram clients will display a reply interface to the user (act as if the user has selected the bot's message and tapped 'Reply'). This can be extremely useful if you want to create user-friendly step-by-step interfaces without having to sacrifice <a href="/bots/features#privacy-mode">privacy mode</a>. Not supported in channels and for messages sent on behalf of a Telegram Business account.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForceReply {
    /// Shows reply interface to the user, as if they manually selected the bot's message and tapped 'Reply'
    pub force_reply: crate::True,
    /// The placeholder to be shown in the input field when the reply is active; 1-64 characters
    pub input_field_placeholder: Option<String>,
    /// Use this parameter if you want to force reply from specific users only. Targets: 1) users that are @mentioned in the <em>text</em> of the <a href="https://core.telegram.org/bots/api#message">Message</a> object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message.
    pub selective: Option<bool>,
}
/// This object represents a chat photo.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatPhoto {
    /// File identifier of small (160x160) chat photo. This file_id can be used only for photo download and only for as long as the photo is not changed.
    pub small_file_id: String,
    /// Unique file identifier of small (160x160) chat photo, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub small_file_unique_id: String,
    /// File identifier of big (640x640) chat photo. This file_id can be used only for photo download and only for as long as the photo is not changed.
    pub big_file_id: String,
    /// Unique file identifier of big (640x640) chat photo, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub big_file_unique_id: String,
}
/// Represents an invite link for a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatInviteLink {
    /// The invite link. If the link was created by another chat administrator, then the second part of the link will be replaced with “…”.
    pub invite_link: String,
    /// Creator of the link
    pub creator: User,
    /// <em>True</em>, if users joining the chat via the link need to be approved by chat administrators
    pub creates_join_request: bool,
    /// <em>True</em>, if the link is primary
    pub is_primary: bool,
    /// <em>True</em>, if the link is revoked
    pub is_revoked: bool,
    /// Invite link name
    pub name: Option<String>,
    /// Point in time (Unix timestamp) when the link will expire or has been expired
    pub expire_date: Option<i64>,
    /// The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    pub member_limit: Option<i64>,
    /// Number of pending join requests created using this link
    pub pending_join_request_count: Option<i64>,
    /// The number of seconds the subscription will be active for before the next payment
    pub subscription_period: Option<i64>,
    /// The amount of Telegram Stars a user must pay initially and after each subsequent subscription period to be a member of the chat using the link
    pub subscription_price: Option<i64>,
}
/// Represents the rights of an administrator in a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatAdministratorRights {
    /// <em>True</em>, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// <em>True</em>, if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages, ignore slow mode, and send messages to the chat without paying Telegram Stars. Implied by any other administrator privilege.
    pub can_manage_chat: bool,
    /// <em>True</em>, if the administrator can delete messages of other users
    pub can_delete_messages: bool,
    /// <em>True</em>, if the administrator can manage video chats
    pub can_manage_video_chats: bool,
    /// <em>True</em>, if the administrator can restrict, ban or unban chat members, or access supergroup statistics
    pub can_restrict_members: bool,
    /// <em>True</em>, if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by the user)
    pub can_promote_members: bool,
    /// <em>True</em>, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: bool,
    /// <em>True</em>, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,
    /// <em>True</em>, if the administrator can post stories to the chat
    pub can_post_stories: bool,
    /// <em>True</em>, if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive
    pub can_edit_stories: bool,
    /// <em>True</em>, if the administrator can delete stories posted by other users
    pub can_delete_stories: bool,
    /// <em>True</em>, if the administrator can post messages in the channel, approve suggested posts, or access channel statistics; for channels only
    pub can_post_messages: Option<bool>,
    /// <em>True</em>, if the administrator can edit messages of other users and can pin messages; for channels only
    pub can_edit_messages: Option<bool>,
    /// <em>True</em>, if the user is allowed to pin messages; for groups and supergroups only
    pub can_pin_messages: Option<bool>,
    /// <em>True</em>, if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only
    pub can_manage_topics: Option<bool>,
    /// <em>True</em>, if the administrator can manage direct messages of the channel and decline suggested posts; for channels only
    pub can_manage_direct_messages: Option<bool>,
}
/// This object represents changes in the status of a chat member.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatMemberUpdated {
    /// Chat the user belongs to
    pub chat: Chat,
    /// Performer of the action, which resulted in the change
    pub from: User,
    /// Date the change was done in Unix time
    pub date: i64,
    /// Previous information about the chat member
    pub old_chat_member: ChatMember,
    /// New information about the chat member
    pub new_chat_member: ChatMember,
    /// Chat invite link, which was used by the user to join the chat; for joining by invite link events only.
    pub invite_link: Option<ChatInviteLink>,
    /// <em>True</em>, if the user joined the chat after sending a direct join request without using an invite link and being approved by an administrator
    pub via_join_request: Option<bool>,
    /// <em>True</em>, if the user joined the chat via a chat folder invite link
    pub via_chat_folder_invite_link: Option<bool>,
}
/// This object contains information about one member of a chat.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ChatMember {
    Owner(ChatMemberOwner),
    Administrator(ChatMemberAdministrator),
    Member(ChatMemberMember),
    Restricted(ChatMemberRestricted),
    Left(ChatMemberLeft),
    Banned(ChatMemberBanned),
}
/// Represents a <a href="https://core.telegram.org/bots/api#chatmember">chat member</a> that owns the chat and has all administrator privileges.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatMemberOwner {
    /// The member's status in the chat, always “creator”
    pub status: String,
    /// Information about the user
    pub user: User,
    /// <em>True</em>, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// Custom title for this user
    pub custom_title: Option<String>,
}
/// Represents a <a href="https://core.telegram.org/bots/api#chatmember">chat member</a> that has some additional privileges.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatMemberAdministrator {
    /// The member's status in the chat, always “administrator”
    pub status: String,
    /// Information about the user
    pub user: User,
    /// <em>True</em>, if the bot is allowed to edit administrator privileges of that user
    pub can_be_edited: bool,
    /// <em>True</em>, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// <em>True</em>, if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages, ignore slow mode, and send messages to the chat without paying Telegram Stars. Implied by any other administrator privilege.
    pub can_manage_chat: bool,
    /// <em>True</em>, if the administrator can delete messages of other users
    pub can_delete_messages: bool,
    /// <em>True</em>, if the administrator can manage video chats
    pub can_manage_video_chats: bool,
    /// <em>True</em>, if the administrator can restrict, ban or unban chat members, or access supergroup statistics
    pub can_restrict_members: bool,
    /// <em>True</em>, if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by the user)
    pub can_promote_members: bool,
    /// <em>True</em>, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: bool,
    /// <em>True</em>, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,
    /// <em>True</em>, if the administrator can post stories to the chat
    pub can_post_stories: bool,
    /// <em>True</em>, if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive
    pub can_edit_stories: bool,
    /// <em>True</em>, if the administrator can delete stories posted by other users
    pub can_delete_stories: bool,
    /// <em>True</em>, if the administrator can post messages in the channel, approve suggested posts, or access channel statistics; for channels only
    pub can_post_messages: Option<bool>,
    /// <em>True</em>, if the administrator can edit messages of other users and can pin messages; for channels only
    pub can_edit_messages: Option<bool>,
    /// <em>True</em>, if the user is allowed to pin messages; for groups and supergroups only
    pub can_pin_messages: Option<bool>,
    /// <em>True</em>, if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only
    pub can_manage_topics: Option<bool>,
    /// <em>True</em>, if the administrator can manage direct messages of the channel and decline suggested posts; for channels only
    pub can_manage_direct_messages: Option<bool>,
    /// Custom title for this user
    pub custom_title: Option<String>,
}
/// Represents a <a href="https://core.telegram.org/bots/api#chatmember">chat member</a> that has no additional privileges or restrictions.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatMemberMember {
    /// The member's status in the chat, always “member”
    pub status: String,
    /// Information about the user
    pub user: User,
    /// Date when the user's subscription will expire; Unix time
    pub until_date: Option<i64>,
}
/// Represents a <a href="https://core.telegram.org/bots/api#chatmember">chat member</a> that is under certain restrictions in the chat. Supergroups only.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatMemberRestricted {
    /// The member's status in the chat, always “restricted”
    pub status: String,
    /// Information about the user
    pub user: User,
    /// <em>True</em>, if the user is a member of the chat at the moment of the request
    pub is_member: bool,
    /// <em>True</em>, if the user is allowed to send text messages, contacts, giveaways, giveaway winners, invoices, locations and venues
    pub can_send_messages: bool,
    /// <em>True</em>, if the user is allowed to send audios
    pub can_send_audios: bool,
    /// <em>True</em>, if the user is allowed to send documents
    pub can_send_documents: bool,
    /// <em>True</em>, if the user is allowed to send photos
    pub can_send_photos: bool,
    /// <em>True</em>, if the user is allowed to send videos
    pub can_send_videos: bool,
    /// <em>True</em>, if the user is allowed to send video notes
    pub can_send_video_notes: bool,
    /// <em>True</em>, if the user is allowed to send voice notes
    pub can_send_voice_notes: bool,
    /// <em>True</em>, if the user is allowed to send polls and checklists
    pub can_send_polls: bool,
    /// <em>True</em>, if the user is allowed to send animations, games, stickers and use inline bots
    pub can_send_other_messages: bool,
    /// <em>True</em>, if the user is allowed to add web page previews to their messages
    pub can_add_web_page_previews: bool,
    /// <em>True</em>, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: bool,
    /// <em>True</em>, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,
    /// <em>True</em>, if the user is allowed to pin messages
    pub can_pin_messages: bool,
    /// <em>True</em>, if the user is allowed to create forum topics
    pub can_manage_topics: bool,
    /// Date when restrictions will be lifted for this user; Unix time. If 0, then the user is restricted forever
    pub until_date: i64,
}
/// Represents a <a href="https://core.telegram.org/bots/api#chatmember">chat member</a> that isn't currently a member of the chat, but may join it themselves.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatMemberLeft {
    /// The member's status in the chat, always “left”
    pub status: String,
    /// Information about the user
    pub user: User,
}
/// Represents a <a href="https://core.telegram.org/bots/api#chatmember">chat member</a> that was banned in the chat and can't return to the chat or view chat messages.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatMemberBanned {
    /// The member's status in the chat, always “kicked”
    pub status: String,
    /// Information about the user
    pub user: User,
    /// Date when restrictions will be lifted for this user; Unix time. If 0, then the user is banned forever
    pub until_date: i64,
}
/// Represents a join request sent to a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatJoinRequest {
    /// Chat to which the request was sent
    pub chat: Chat,
    /// User that sent the join request
    pub from: User,
    /// Identifier of a private chat with the user who sent the join request. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. The bot can use this identifier for 5 minutes to send messages until the join request is processed, assuming no other administrator contacted the user.
    pub user_chat_id: i64,
    /// Date the request was sent in Unix time
    pub date: i64,
    /// Bio of the user.
    pub bio: Option<String>,
    /// Chat invite link that was used by the user to send the join request
    pub invite_link: Option<ChatInviteLink>,
}
/// Describes actions that a non-administrator user is allowed to take in a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatPermissions {
    /// <em>True</em>, if the user is allowed to send text messages, contacts, giveaways, giveaway winners, invoices, locations and venues
    pub can_send_messages: Option<bool>,
    /// <em>True</em>, if the user is allowed to send audios
    pub can_send_audios: Option<bool>,
    /// <em>True</em>, if the user is allowed to send documents
    pub can_send_documents: Option<bool>,
    /// <em>True</em>, if the user is allowed to send photos
    pub can_send_photos: Option<bool>,
    /// <em>True</em>, if the user is allowed to send videos
    pub can_send_videos: Option<bool>,
    /// <em>True</em>, if the user is allowed to send video notes
    pub can_send_video_notes: Option<bool>,
    /// <em>True</em>, if the user is allowed to send voice notes
    pub can_send_voice_notes: Option<bool>,
    /// <em>True</em>, if the user is allowed to send polls and checklists
    pub can_send_polls: Option<bool>,
    /// <em>True</em>, if the user is allowed to send animations, games, stickers and use inline bots
    pub can_send_other_messages: Option<bool>,
    /// <em>True</em>, if the user is allowed to add web page previews to their messages
    pub can_add_web_page_previews: Option<bool>,
    /// <em>True</em>, if the user is allowed to change the chat title, photo and other settings. Ignored in public supergroups
    pub can_change_info: Option<bool>,
    /// <em>True</em>, if the user is allowed to invite new users to the chat
    pub can_invite_users: Option<bool>,
    /// <em>True</em>, if the user is allowed to pin messages. Ignored in public supergroups
    pub can_pin_messages: Option<bool>,
    /// <em>True</em>, if the user is allowed to create forum topics. If omitted defaults to the value of can_pin_messages
    pub can_manage_topics: Option<bool>,
}
/// Describes the birthdate of a user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Birthdate {
    /// Day of the user's birth; 1-31
    pub day: i64,
    /// Month of the user's birth; 1-12
    pub month: i64,
    /// Year of the user's birth
    pub year: Option<i64>,
}
/// Contains information about the start page settings of a Telegram Business account.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BusinessIntro {
    /// Title text of the business intro
    pub title: Option<String>,
    /// Message text of the business intro
    pub message: Option<String>,
    /// Sticker of the business intro
    pub sticker: Option<Sticker>,
}
/// Contains information about the location of a Telegram Business account.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BusinessLocation {
    /// Address of the business
    pub address: String,
    /// Location of the business
    pub location: Option<Location>,
}
/// Describes an interval of time during which a business is open.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BusinessOpeningHoursInterval {
    /// The minute's sequence number in a week, starting on Monday, marking the start of the time interval during which the business is open; 0 - 7 * 24 * 60
    pub opening_minute: i64,
    /// The minute's sequence number in a week, starting on Monday, marking the end of the time interval during which the business is open; 0 - 8 * 24 * 60
    pub closing_minute: i64,
}
/// Describes the opening hours of a business.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BusinessOpeningHours {
    /// Unique name of the time zone for which the opening hours are defined
    pub time_zone_name: String,
    /// List of time intervals describing business opening hours
    pub opening_hours: Vec<BusinessOpeningHoursInterval>,
}
/// This object describes the rating of a user based on their Telegram Star spendings.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserRating {
    /// Current level of the user, indicating their reliability when purchasing digital goods and services. A higher level suggests a more trustworthy customer; a negative level is likely reason for concern.
    pub level: i64,
    /// Numerical value of the user's rating; the higher the rating, the better
    pub rating: i64,
    /// The rating value required to get the current level
    pub current_level_rating: i64,
    /// The rating value required to get to the next level; omitted if the maximum level was reached
    pub next_level_rating: Option<i64>,
}
/// Describes the position of a clickable area within a story.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StoryAreaPosition {
    /// The abscissa of the area's center, as a percentage of the media width
    pub x_percentage: f64,
    /// The ordinate of the area's center, as a percentage of the media height
    pub y_percentage: f64,
    /// The width of the area's rectangle, as a percentage of the media width
    pub width_percentage: f64,
    /// The height of the area's rectangle, as a percentage of the media height
    pub height_percentage: f64,
    /// The clockwise rotation angle of the rectangle, in degrees; 0-360
    pub rotation_angle: f64,
    /// The radius of the rectangle corner rounding, as a percentage of the media width
    pub corner_radius_percentage: f64,
}
/// Describes the physical address of a location.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LocationAddress {
    /// The two-letter ISO 3166-1 alpha-2 country code of the country where the location is located
    pub country_code: String,
    /// State of the location
    pub state: Option<String>,
    /// City of the location
    pub city: Option<String>,
    /// Street address of the location
    pub street: Option<String>,
}
/// Describes the type of a clickable area on a story.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum StoryAreaType {
    Location(StoryAreaTypeLocation),
    SuggestedReaction(StoryAreaTypeSuggestedReaction),
    Link(StoryAreaTypeLink),
    Weather(StoryAreaTypeWeather),
    UniqueGift(StoryAreaTypeUniqueGift),
}
/// Describes a story area pointing to a location. Currently, a story can have up to 10 location areas.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StoryAreaTypeLocation {
    /// Type of the area, always “location”
    pub r#type: String,
    /// Location latitude in degrees
    pub latitude: f64,
    /// Location longitude in degrees
    pub longitude: f64,
    /// Address of the location
    pub address: Option<LocationAddress>,
}
/// Describes a story area pointing to a suggested reaction. Currently, a story can have up to 5 suggested reaction areas.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StoryAreaTypeSuggestedReaction {
    /// Type of the area, always “suggested_reaction”
    pub r#type: String,
    /// Type of the reaction
    pub reaction_type: ReactionType,
    /// Pass <em>True</em> if the reaction area has a dark background
    pub is_dark: Option<bool>,
    /// Pass <em>True</em> if reaction area corner is flipped
    pub is_flipped: Option<bool>,
}
/// Describes a story area pointing to an HTTP or tg:// link. Currently, a story can have up to 3 link areas.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StoryAreaTypeLink {
    /// Type of the area, always “link”
    pub r#type: String,
    /// HTTP or tg:// URL to be opened when the area is clicked
    pub url: String,
}
/// Describes a story area containing weather information. Currently, a story can have up to 3 weather areas.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StoryAreaTypeWeather {
    /// Type of the area, always “weather”
    pub r#type: String,
    /// Temperature, in degree Celsius
    pub temperature: f64,
    /// Emoji representing the weather
    pub emoji: String,
    /// A color of the area background in the ARGB format
    pub background_color: i64,
}
/// Describes a story area pointing to a unique gift. Currently, a story can have at most 1 unique gift area.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StoryAreaTypeUniqueGift {
    /// Type of the area, always “unique_gift”
    pub r#type: String,
    /// Unique name of the gift
    pub name: String,
}
/// Describes a clickable area on a story media.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StoryArea {
    /// Position of the area
    pub position: StoryAreaPosition,
    /// Type of the area
    pub r#type: StoryAreaType,
}
/// Represents a location to which a chat is connected.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatLocation {
    /// The location to which the supergroup is connected. Can't be a live location.
    pub location: Location,
    /// Location address; 1-64 characters, as defined by the chat owner
    pub address: String,
}
/// This object describes the type of a reaction.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ReactionType {
    Emoji(ReactionTypeEmoji),
    CustomEmoji(ReactionTypeCustomEmoji),
    Paid(ReactionTypePaid),
}
/// The reaction is based on an emoji.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReactionTypeEmoji {
    /// Type of the reaction, always “emoji”
    pub r#type: String,
    /// Reaction emoji. Currently, it can be one of "<img class="emoji" src="//telegram.org/img/emoji/40/E29DA4.png" width="20" height="20" alt="❤" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F918D.png" width="20" height="20" alt="👍" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F918E.png" width="20" height="20" alt="👎" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F94A5.png" width="20" height="20" alt="🔥" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09FA5B0.png" width="20" height="20" alt="🥰" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F918F.png" width="20" height="20" alt="👏" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F9881.png" width="20" height="20" alt="😁" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09FA494.png" width="20" height="20" alt="🤔" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09FA4AF.png" width="20" height="20" alt="🤯" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F98B1.png" width="20" height="20" alt="😱" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09FA4AC.png" width="20" height="20" alt="🤬" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F98A2.png" width="20" height="20" alt="😢" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F8E89.png" width="20" height="20" alt="🎉" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09FA4A9.png" width="20" height="20" alt="🤩" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09FA4AE.png" width="20" height="20" alt="🤮" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F92A9.png" width="20" height="20" alt="💩" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F998F.png" width="20" height="20" alt="🙏" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F918C.png" width="20" height="20" alt="👌" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F958A.png" width="20" height="20" alt="🕊" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09FA4A1.png" width="20" height="20" alt="🤡" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09FA5B1.png" width="20" height="20" alt="🥱" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09FA5B4.png" width="20" height="20" alt="🥴" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F988D.png" width="20" height="20" alt="😍" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F90B3.png" width="20" height="20" alt="🐳" />", "<img class="emoji" src="//telegram.org/img/emoji/40/E29DA4E2808DF09F94A5.png" width="20" height="20" alt="❤‍🔥" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F8C9A.png" width="20" height="20" alt="🌚" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F8CAD.png" width="20" height="20" alt="🌭" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F92AF.png" width="20" height="20" alt="💯" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09FA4A3.png" width="20" height="20" alt="🤣" />", "<img class="emoji" src="//telegram.org/img/emoji/40/E29AA1.png" width="20" height="20" alt="⚡" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F8D8C.png" width="20" height="20" alt="🍌" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F8F86.png" width="20" height="20" alt="🏆" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F9294.png" width="20" height="20" alt="💔" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09FA4A8.png" width="20" height="20" alt="🤨" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F9890.png" width="20" height="20" alt="😐" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F8D93.png" width="20" height="20" alt="🍓" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F8DBE.png" width="20" height="20" alt="🍾" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F928B.png" width="20" height="20" alt="💋" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F9695.png" width="20" height="20" alt="🖕" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F9888.png" width="20" height="20" alt="😈" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F98B4.png" width="20" height="20" alt="😴" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F98AD.png" width="20" height="20" alt="😭" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09FA493.png" width="20" height="20" alt="🤓" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F91BB.png" width="20" height="20" alt="👻" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F91A8E2808DF09F92BB.png" width="20" height="20" alt="👨‍💻" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F9180.png" width="20" height="20" alt="👀" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F8E83.png" width="20" height="20" alt="🎃" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F9988.png" width="20" height="20" alt="🙈" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F9887.png" width="20" height="20" alt="😇" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F98A8.png" width="20" height="20" alt="😨" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09FA49D.png" width="20" height="20" alt="🤝" />", "<img class="emoji" src="//telegram.org/img/emoji/40/E29C8D.png" width="20" height="20" alt="✍" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09FA497.png" width="20" height="20" alt="🤗" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09FABA1.png" width="20" height="20" alt="🫡" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F8E85.png" width="20" height="20" alt="🎅" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F8E84.png" width="20" height="20" alt="🎄" />", "<img class="emoji" src="//telegram.org/img/emoji/40/E29883.png" width="20" height="20" alt="☃" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F9285.png" width="20" height="20" alt="💅" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09FA4AA.png" width="20" height="20" alt="🤪" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F97BF.png" width="20" height="20" alt="🗿" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F8692.png" width="20" height="20" alt="🆒" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F9298.png" width="20" height="20" alt="💘" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F9989.png" width="20" height="20" alt="🙉" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09FA684.png" width="20" height="20" alt="🦄" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F9898.png" width="20" height="20" alt="😘" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F928A.png" width="20" height="20" alt="💊" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F998A.png" width="20" height="20" alt="🙊" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F988E.png" width="20" height="20" alt="😎" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F91BE.png" width="20" height="20" alt="👾" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09FA4B7E2808DE29982.png" width="20" height="20" alt="🤷‍♂" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09FA4B7.png" width="20" height="20" alt="🤷" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09FA4B7E2808DE29980.png" width="20" height="20" alt="🤷‍♀" />", "<img class="emoji" src="//telegram.org/img/emoji/40/F09F98A1.png" width="20" height="20" alt="😡" />"
    pub emoji: String,
}
/// The reaction is based on a custom emoji.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReactionTypeCustomEmoji {
    /// Type of the reaction, always “custom_emoji”
    pub r#type: String,
    /// Custom emoji identifier
    pub custom_emoji_id: String,
}
/// The reaction is paid.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReactionTypePaid {
    /// Type of the reaction, always “paid”
    pub r#type: String,
}
/// Represents a reaction added to a message along with the number of times it was added.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReactionCount {
    /// Type of the reaction
    pub r#type: ReactionType,
    /// Number of times the reaction was added
    pub total_count: i64,
}
/// This object represents a change of a reaction on a message performed by a user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageReactionUpdated {
    /// The chat containing the message the user reacted to
    pub chat: Chat,
    /// Unique identifier of the message inside the chat
    pub message_id: i64,
    /// The user that changed the reaction, if the user isn't anonymous
    pub user: Option<User>,
    /// The chat on behalf of which the reaction was changed, if the user is anonymous
    pub actor_chat: Option<Chat>,
    /// Date of the change in Unix time
    pub date: i64,
    /// Previous list of reaction types that were set by the user
    pub old_reaction: Vec<ReactionType>,
    /// New list of reaction types that have been set by the user
    pub new_reaction: Vec<ReactionType>,
}
/// This object represents reaction changes on a message with anonymous reactions.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageReactionCountUpdated {
    /// The chat containing the message
    pub chat: Chat,
    /// Unique message identifier inside the chat
    pub message_id: i64,
    /// Date of the change in Unix time
    pub date: i64,
    /// List of reactions that are present on the message
    pub reactions: Vec<ReactionCount>,
}
/// This object represents a forum topic.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForumTopic {
    /// Unique identifier of the forum topic
    pub message_thread_id: i64,
    /// Name of the topic
    pub name: String,
    /// Color of the topic icon in RGB format
    pub icon_color: i64,
    /// Unique identifier of the custom emoji shown as the topic icon
    pub icon_custom_emoji_id: Option<String>,
    /// <em>True</em>, if the name of the topic wasn't specified explicitly by its creator and likely needs to be changed by the bot
    pub is_name_implicit: Option<crate::True>,
}
/// This object describes the background of a gift.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GiftBackground {
    /// Center color of the background in RGB format
    pub center_color: i64,
    /// Edge color of the background in RGB format
    pub edge_color: i64,
    /// Text color of the background in RGB format
    pub text_color: i64,
}
/// This object represents a gift that can be sent by the bot.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Gift {
    /// Unique identifier of the gift
    pub id: String,
    /// The sticker that represents the gift
    pub sticker: Sticker,
    /// The number of Telegram Stars that must be paid to send the sticker
    pub star_count: i64,
    /// The number of Telegram Stars that must be paid to upgrade the gift to a unique one
    pub upgrade_star_count: Option<i64>,
    /// <em>True</em>, if the gift can only be purchased by Telegram Premium subscribers
    pub is_premium: Option<crate::True>,
    /// <em>True</em>, if the gift can be used (after being upgraded) to customize a user's appearance
    pub has_colors: Option<crate::True>,
    /// The total number of gifts of this type that can be sent by all users; for limited gifts only
    pub total_count: Option<i64>,
    /// The number of remaining gifts of this type that can be sent by all users; for limited gifts only
    pub remaining_count: Option<i64>,
    /// The total number of gifts of this type that can be sent by the bot; for limited gifts only
    pub personal_total_count: Option<i64>,
    /// The number of remaining gifts of this type that can be sent by the bot; for limited gifts only
    pub personal_remaining_count: Option<i64>,
    /// Background of the gift
    pub background: Option<GiftBackground>,
    /// The total number of different unique gifts that can be obtained by upgrading the gift
    pub unique_gift_variant_count: Option<i64>,
    /// Information about the chat that published the gift
    pub publisher_chat: Option<Chat>,
}
/// This object represent a list of gifts.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Gifts {
    /// The list of gifts
    pub gifts: Vec<Gift>,
}
/// This object describes the model of a unique gift.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UniqueGiftModel {
    /// Name of the model
    pub name: String,
    /// The sticker that represents the unique gift
    pub sticker: Sticker,
    /// The number of unique gifts that receive this model for every 1000 gift upgrades. Always 0 for crafted gifts.
    pub rarity_per_mille: i64,
    /// Rarity of the model if it is a crafted model. Currently, can be “uncommon”, “rare”, “epic”, or “legendary”.
    pub rarity: Option<String>,
}
/// This object describes the symbol shown on the pattern of a unique gift.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UniqueGiftSymbol {
    /// Name of the symbol
    pub name: String,
    /// The sticker that represents the unique gift
    pub sticker: Sticker,
    /// The number of unique gifts that receive this model for every 1000 gifts upgraded
    pub rarity_per_mille: i64,
}
/// This object describes the colors of the backdrop of a unique gift.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UniqueGiftBackdropColors {
    /// The color in the center of the backdrop in RGB format
    pub center_color: i64,
    /// The color on the edges of the backdrop in RGB format
    pub edge_color: i64,
    /// The color to be applied to the symbol in RGB format
    pub symbol_color: i64,
    /// The color for the text on the backdrop in RGB format
    pub text_color: i64,
}
/// This object describes the backdrop of a unique gift.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UniqueGiftBackdrop {
    /// Name of the backdrop
    pub name: String,
    /// Colors of the backdrop
    pub colors: UniqueGiftBackdropColors,
    /// The number of unique gifts that receive this backdrop for every 1000 gifts upgraded
    pub rarity_per_mille: i64,
}
/// This object contains information about the color scheme for a user's name, message replies and link previews based on a unique gift.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UniqueGiftColors {
    /// Custom emoji identifier of the unique gift's model
    pub model_custom_emoji_id: String,
    /// Custom emoji identifier of the unique gift's symbol
    pub symbol_custom_emoji_id: String,
    /// Main color used in light themes; RGB format
    pub light_theme_main_color: i64,
    /// List of 1-3 additional colors used in light themes; RGB format
    pub light_theme_other_colors: Vec<i64>,
    /// Main color used in dark themes; RGB format
    pub dark_theme_main_color: i64,
    /// List of 1-3 additional colors used in dark themes; RGB format
    pub dark_theme_other_colors: Vec<i64>,
}
/// This object describes a unique gift that was upgraded from a regular gift.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UniqueGift {
    /// Identifier of the regular gift from which the gift was upgraded
    pub gift_id: String,
    /// Human-readable name of the regular gift from which this unique gift was upgraded
    pub base_name: String,
    /// Unique name of the gift. This name can be used in <code>https://t.me/nft/...</code> links and story areas
    pub name: String,
    /// Unique number of the upgraded gift among gifts upgraded from the same regular gift
    pub number: i64,
    /// Model of the gift
    pub model: UniqueGiftModel,
    /// Symbol of the gift
    pub symbol: UniqueGiftSymbol,
    /// Backdrop of the gift
    pub backdrop: UniqueGiftBackdrop,
    /// <em>True</em>, if the original regular gift was exclusively purchaseable by Telegram Premium subscribers
    pub is_premium: Option<crate::True>,
    /// <em>True</em>, if the gift was used to craft another gift and isn't available anymore
    pub is_burned: Option<crate::True>,
    /// <em>True</em>, if the gift is assigned from the TON blockchain and can't be resold or transferred in Telegram
    pub is_from_blockchain: Option<crate::True>,
    /// The color scheme that can be used by the gift's owner for the chat's name, replies to messages and link previews; for business account gifts and gifts that are currently on sale only
    pub colors: Option<UniqueGiftColors>,
    /// Information about the chat that published the gift
    pub publisher_chat: Option<Chat>,
}
/// Describes a service message about a regular gift that was sent or received.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GiftInfo {
    /// Information about the gift
    pub gift: Gift,
    /// Unique identifier of the received gift for the bot; only present for gifts received on behalf of business accounts
    pub owned_gift_id: Option<String>,
    /// Number of Telegram Stars that can be claimed by the receiver by converting the gift; omitted if conversion to Telegram Stars is impossible
    pub convert_star_count: Option<i64>,
    /// Number of Telegram Stars that were prepaid for the ability to upgrade the gift
    pub prepaid_upgrade_star_count: Option<i64>,
    /// <em>True</em>, if the gift's upgrade was purchased after the gift was sent
    pub is_upgrade_separate: Option<crate::True>,
    /// <em>True</em>, if the gift can be upgraded to a unique gift
    pub can_be_upgraded: Option<crate::True>,
    /// Text of the message that was added to the gift
    pub text: Option<String>,
    /// Special entities that appear in the text
    pub entities: Option<Vec<MessageEntity>>,
    /// <em>True</em>, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
    pub is_private: Option<crate::True>,
    /// Unique number reserved for this gift when upgraded. See the <em>number</em> field in <a href="https://core.telegram.org/bots/api#uniquegift">UniqueGift</a>
    pub unique_gift_number: Option<i64>,
}
/// Describes a service message about a unique gift that was sent or received.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UniqueGiftInfo {
    /// Information about the gift
    pub gift: UniqueGift,
    /// Origin of the gift. Currently, either “upgrade” for gifts upgraded from regular gifts, “transfer” for gifts transferred from other users or channels, “resale” for gifts bought from other users, “gifted_upgrade” for upgrades purchased after the gift was sent, or “offer” for gifts bought or sold through gift purchase offers
    pub origin: String,
    /// For gifts bought from other users, the currency in which the payment for the gift was done. Currently, one of “XTR” for Telegram Stars or “TON” for toncoins.
    pub last_resale_currency: Option<String>,
    /// For gifts bought from other users, the price paid for the gift in either Telegram Stars or nanotoncoins
    pub last_resale_amount: Option<i64>,
    /// Unique identifier of the received gift for the bot; only present for gifts received on behalf of business accounts
    pub owned_gift_id: Option<String>,
    /// Number of Telegram Stars that must be paid to transfer the gift; omitted if the bot cannot transfer the gift
    pub transfer_star_count: Option<i64>,
    /// Point in time (Unix timestamp) when the gift can be transferred. If it is in the past, then the gift can be transferred now
    pub next_transfer_date: Option<i64>,
}
/// This object describes a gift received and owned by a user or a chat.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum OwnedGift {
    Regular(OwnedGiftRegular),
    Unique(OwnedGiftUnique),
}
/// Describes a regular gift owned by a user or a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OwnedGiftRegular {
    /// Type of the gift, always “regular”
    pub r#type: String,
    /// Information about the regular gift
    pub gift: Gift,
    /// Unique identifier of the gift for the bot; for gifts received on behalf of business accounts only
    pub owned_gift_id: Option<String>,
    /// Sender of the gift if it is a known user
    pub sender_user: Option<User>,
    /// Date the gift was sent in Unix time
    pub send_date: i64,
    /// Text of the message that was added to the gift
    pub text: Option<String>,
    /// Special entities that appear in the text
    pub entities: Option<Vec<MessageEntity>>,
    /// <em>True</em>, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
    pub is_private: Option<crate::True>,
    /// <em>True</em>, if the gift is displayed on the account's profile page; for gifts received on behalf of business accounts only
    pub is_saved: Option<crate::True>,
    /// <em>True</em>, if the gift can be upgraded to a unique gift; for gifts received on behalf of business accounts only
    pub can_be_upgraded: Option<crate::True>,
    /// <em>True</em>, if the gift was refunded and isn't available anymore
    pub was_refunded: Option<crate::True>,
    /// Number of Telegram Stars that can be claimed by the receiver instead of the gift; omitted if the gift cannot be converted to Telegram Stars; for gifts received on behalf of business accounts only
    pub convert_star_count: Option<i64>,
    /// Number of Telegram Stars that were paid for the ability to upgrade the gift
    pub prepaid_upgrade_star_count: Option<i64>,
    /// <em>True</em>, if the gift's upgrade was purchased after the gift was sent; for gifts received on behalf of business accounts only
    pub is_upgrade_separate: Option<crate::True>,
    /// Unique number reserved for this gift when upgraded. See the <em>number</em> field in <a href="https://core.telegram.org/bots/api#uniquegift">UniqueGift</a>
    pub unique_gift_number: Option<i64>,
}
/// Describes a unique gift received and owned by a user or a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OwnedGiftUnique {
    /// Type of the gift, always “unique”
    pub r#type: String,
    /// Information about the unique gift
    pub gift: UniqueGift,
    /// Unique identifier of the received gift for the bot; for gifts received on behalf of business accounts only
    pub owned_gift_id: Option<String>,
    /// Sender of the gift if it is a known user
    pub sender_user: Option<User>,
    /// Date the gift was sent in Unix time
    pub send_date: i64,
    /// <em>True</em>, if the gift is displayed on the account's profile page; for gifts received on behalf of business accounts only
    pub is_saved: Option<crate::True>,
    /// <em>True</em>, if the gift can be transferred to another owner; for gifts received on behalf of business accounts only
    pub can_be_transferred: Option<crate::True>,
    /// Number of Telegram Stars that must be paid to transfer the gift; omitted if the bot cannot transfer the gift
    pub transfer_star_count: Option<i64>,
    /// Point in time (Unix timestamp) when the gift can be transferred. If it is in the past, then the gift can be transferred now
    pub next_transfer_date: Option<i64>,
}
/// Contains the list of gifts received and owned by a user or a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OwnedGifts {
    /// The total number of gifts owned by the user or the chat
    pub total_count: i64,
    /// The list of gifts
    pub gifts: Vec<OwnedGift>,
    /// Offset for the next request. If empty, then there are no more results
    pub next_offset: Option<String>,
}
/// This object describes the types of gifts that can be gifted to a user or a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AcceptedGiftTypes {
    /// <em>True</em>, if unlimited regular gifts are accepted
    pub unlimited_gifts: bool,
    /// <em>True</em>, if limited regular gifts are accepted
    pub limited_gifts: bool,
    /// <em>True</em>, if unique gifts or gifts that can be upgraded to unique for free are accepted
    pub unique_gifts: bool,
    /// <em>True</em>, if a Telegram Premium subscription is accepted
    pub premium_subscription: bool,
    /// <em>True</em>, if transfers of unique gifts from channels are accepted
    pub gifts_from_channels: bool,
}
/// Describes an amount of Telegram Stars.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StarAmount {
    /// Integer amount of Telegram Stars, rounded to 0; can be negative
    pub amount: i64,
    /// The number of 1/1000000000 shares of Telegram Stars; from -999999999 to 999999999; can be negative if and only if <em>amount</em> is non-positive
    pub nanostar_amount: Option<i64>,
}
/// This object represents a bot command.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotCommand {
    /// Text of the command; 1-32 characters. Can contain only lowercase English letters, digits and underscores.
    pub command: String,
    /// Description of the command; 1-256 characters.
    pub description: String,
}
/// This object represents the scope to which bot commands are applied.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BotCommandScope {
    Default(BotCommandScopeDefault),
    AllPrivateChats(BotCommandScopeAllPrivateChats),
    AllGroupChats(BotCommandScopeAllGroupChats),
    AllChatAdministrators(BotCommandScopeAllChatAdministrators),
    Chat(BotCommandScopeChat),
    ChatAdministrators(BotCommandScopeChatAdministrators),
    ChatMember(BotCommandScopeChatMember),
}
/// Represents the default <a href="https://core.telegram.org/bots/api#botcommandscope">scope</a> of bot commands. Default commands are used if no commands with a <a href="https://core.telegram.org/bots/api#determining-list-of-commands">narrower scope</a> are specified for the user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotCommandScopeDefault {
    /// Scope type, must be <em>default</em>
    pub r#type: String,
}
/// Represents the <a href="https://core.telegram.org/bots/api#botcommandscope">scope</a> of bot commands, covering all private chats.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotCommandScopeAllPrivateChats {
    /// Scope type, must be <em>all_private_chats</em>
    pub r#type: String,
}
/// Represents the <a href="https://core.telegram.org/bots/api#botcommandscope">scope</a> of bot commands, covering all group and supergroup chats.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotCommandScopeAllGroupChats {
    /// Scope type, must be <em>all_group_chats</em>
    pub r#type: String,
}
/// Represents the <a href="https://core.telegram.org/bots/api#botcommandscope">scope</a> of bot commands, covering all group and supergroup chat administrators.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotCommandScopeAllChatAdministrators {
    /// Scope type, must be <em>all_chat_administrators</em>
    pub r#type: String,
}
/// Represents the <a href="https://core.telegram.org/bots/api#botcommandscope">scope</a> of bot commands, covering a specific chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotCommandScopeChat {
    /// Scope type, must be <em>chat</em>
    pub r#type: String,
    /// Unique identifier for the target chat or username of the target supergroup (in the format <code>@supergroupusername</code>). Channel direct messages chats and channel chats aren't supported.
    pub chat_id: ChatId,
}
/// Represents the <a href="https://core.telegram.org/bots/api#botcommandscope">scope</a> of bot commands, covering all administrators of a specific group or supergroup chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotCommandScopeChatAdministrators {
    /// Scope type, must be <em>chat_administrators</em>
    pub r#type: String,
    /// Unique identifier for the target chat or username of the target supergroup (in the format <code>@supergroupusername</code>). Channel direct messages chats and channel chats aren't supported.
    pub chat_id: ChatId,
}
/// Represents the <a href="https://core.telegram.org/bots/api#botcommandscope">scope</a> of bot commands, covering a specific member of a group or supergroup chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotCommandScopeChatMember {
    /// Scope type, must be <em>chat_member</em>
    pub r#type: String,
    /// Unique identifier for the target chat or username of the target supergroup (in the format <code>@supergroupusername</code>). Channel direct messages chats and channel chats aren't supported.
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
}
/// This object represents the bot's name.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotName {
    /// The bot's name
    pub name: String,
}
/// This object represents the bot's description.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotDescription {
    /// The bot's description
    pub description: String,
}
/// This object represents the bot's short description.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotShortDescription {
    /// The bot's short description
    pub short_description: String,
}
/// This object describes the bot's menu button in a private chat.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum MenuButton {
    Commands(MenuButtonCommands),
    WebApp(MenuButtonWebApp),
    Default(MenuButtonDefault),
}
/// Represents a menu button, which opens the bot's list of commands.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MenuButtonCommands {
    /// Type of the button, must be <em>commands</em>
    pub r#type: String,
}
/// Represents a menu button, which launches a <a href="/bots/webapps">Web App</a>.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MenuButtonWebApp {
    /// Type of the button, must be <em>web_app</em>
    pub r#type: String,
    /// Text on the button
    pub text: String,
    /// Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method <a href="https://core.telegram.org/bots/api#answerwebappquery">answerWebAppQuery</a>. Alternatively, a <code>t.me</code> link to a Web App of the bot can be specified in the object instead of the Web App's URL, in which case the Web App will be opened as if the user pressed the link.
    pub web_app: WebAppInfo,
}
/// Describes that no specific value for the menu button was set.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MenuButtonDefault {
    /// Type of the button, must be <em>default</em>
    pub r#type: String,
}
/// This object describes the source of a chat boost.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ChatBoostSource {
    Premium(ChatBoostSourcePremium),
    GiftCode(ChatBoostSourceGiftCode),
    Giveaway(ChatBoostSourceGiveaway),
}
/// The boost was obtained by subscribing to Telegram Premium or by gifting a Telegram Premium subscription to another user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatBoostSourcePremium {
    /// Source of the boost, always “premium”
    pub source: String,
    /// User that boosted the chat
    pub user: User,
}
/// The boost was obtained by the creation of Telegram Premium gift codes to boost a chat. Each such code boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatBoostSourceGiftCode {
    /// Source of the boost, always “gift_code”
    pub source: String,
    /// User for which the gift code was created
    pub user: User,
}
/// The boost was obtained by the creation of a Telegram Premium or a Telegram Star giveaway. This boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription for Telegram Premium giveaways and <em>prize_star_count</em> / 500 times for one year for Telegram Star giveaways.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatBoostSourceGiveaway {
    /// Source of the boost, always “giveaway”
    pub source: String,
    /// Identifier of a message in the chat with the giveaway; the message could have been deleted already. May be 0 if the message isn't sent yet.
    pub giveaway_message_id: i64,
    /// User that won the prize in the giveaway if any; for Telegram Premium giveaways only
    pub user: Option<User>,
    /// The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only
    pub prize_star_count: Option<i64>,
    /// <em>True</em>, if the giveaway was completed, but there was no user to win the prize
    pub is_unclaimed: Option<crate::True>,
}
/// This object contains information about a chat boost.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatBoost {
    /// Unique identifier of the boost
    pub boost_id: String,
    /// Point in time (Unix timestamp) when the chat was boosted
    pub add_date: i64,
    /// Point in time (Unix timestamp) when the boost will automatically expire, unless the booster's Telegram Premium subscription is prolonged
    pub expiration_date: i64,
    /// Source of the added boost
    pub source: ChatBoostSource,
}
/// This object represents a boost added to a chat or changed.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatBoostUpdated {
    /// Chat which was boosted
    pub chat: Chat,
    /// Information about the chat boost
    pub boost: ChatBoost,
}
/// This object represents a boost removed from a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatBoostRemoved {
    /// Chat which was boosted
    pub chat: Chat,
    /// Unique identifier of the boost
    pub boost_id: String,
    /// Point in time (Unix timestamp) when the boost was removed
    pub remove_date: i64,
    /// Source of the removed boost
    pub source: ChatBoostSource,
}
/// Describes a service message about the chat owner leaving the chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatOwnerLeft {
    /// The user which will be the new owner of the chat if the previous owner does not return to the chat
    pub new_owner: Option<User>,
}
/// Describes a service message about an ownership change in the chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatOwnerChanged {
    /// The new owner of the chat
    pub new_owner: User,
}
/// This object represents a list of boosts added to a chat by a user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserChatBoosts {
    /// The list of boosts added to the chat by the user
    pub boosts: Vec<ChatBoost>,
}
/// Represents the rights of a business bot.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BusinessBotRights {
    /// <em>True</em>, if the bot can send and edit messages in the private chats that had incoming messages in the last 24 hours
    pub can_reply: Option<crate::True>,
    /// <em>True</em>, if the bot can mark incoming private messages as read
    pub can_read_messages: Option<crate::True>,
    /// <em>True</em>, if the bot can delete messages sent by the bot
    pub can_delete_sent_messages: Option<crate::True>,
    /// <em>True</em>, if the bot can delete all private messages in managed chats
    pub can_delete_all_messages: Option<crate::True>,
    /// <em>True</em>, if the bot can edit the first and last name of the business account
    pub can_edit_name: Option<crate::True>,
    /// <em>True</em>, if the bot can edit the bio of the business account
    pub can_edit_bio: Option<crate::True>,
    /// <em>True</em>, if the bot can edit the profile photo of the business account
    pub can_edit_profile_photo: Option<crate::True>,
    /// <em>True</em>, if the bot can edit the username of the business account
    pub can_edit_username: Option<crate::True>,
    /// <em>True</em>, if the bot can change the privacy settings pertaining to gifts for the business account
    pub can_change_gift_settings: Option<crate::True>,
    /// <em>True</em>, if the bot can view gifts and the amount of Telegram Stars owned by the business account
    pub can_view_gifts_and_stars: Option<crate::True>,
    /// <em>True</em>, if the bot can convert regular gifts owned by the business account to Telegram Stars
    pub can_convert_gifts_to_stars: Option<crate::True>,
    /// <em>True</em>, if the bot can transfer and upgrade gifts owned by the business account
    pub can_transfer_and_upgrade_gifts: Option<crate::True>,
    /// <em>True</em>, if the bot can transfer Telegram Stars received by the business account to its own account, or use them to upgrade and transfer gifts
    pub can_transfer_stars: Option<crate::True>,
    /// <em>True</em>, if the bot can post, edit and delete stories on behalf of the business account
    pub can_manage_stories: Option<crate::True>,
}
/// Describes the connection of the bot with a business account.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BusinessConnection {
    /// Unique identifier of the business connection
    pub id: String,
    /// Business account user that created the business connection
    pub user: User,
    /// Identifier of a private chat with the user who created the business connection. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    pub user_chat_id: i64,
    /// Date the connection was established in Unix time
    pub date: i64,
    /// Rights of the business bot
    pub rights: Option<BusinessBotRights>,
    /// <em>True</em>, if the connection is active
    pub is_enabled: bool,
}
/// This object is received when messages are deleted from a connected business account.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BusinessMessagesDeleted {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Information about a chat in the business account. The bot may not have access to the chat or the corresponding user.
    pub chat: Chat,
    /// The list of identifiers of deleted messages in the chat of the business account
    pub message_ids: Vec<i64>,
}
/// Describes why a request was unsuccessful.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResponseParameters {
    /// The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub migrate_to_chat_id: Option<i64>,
    /// In case of exceeding flood control, the number of seconds left to wait before the request can be repeated
    pub retry_after: Option<i64>,
}
/// This object represents the content of a media message to be sent.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum InputMedia {
    Animation(InputMediaAnimation),
    Document(InputMediaDocument),
    Audio(InputMediaAudio),
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
}
/// Represents a photo to be sent.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputMediaPhoto {
    /// Type of the result, must be <em>photo</em>
    pub r#type: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub media: String,
    /// Caption of the photo to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass <em>True</em>, if the caption must be shown above the message media
    pub show_caption_above_media: Option<bool>,
    /// Pass <em>True</em> if the photo needs to be covered with a spoiler animation
    pub has_spoiler: Option<bool>,
}
/// Represents a video to be sent.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputMediaVideo {
    /// Type of the result, must be <em>video</em>
    pub r#type: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub media: String,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub thumbnail: Option<String>,
    /// Cover for the video in the message. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub cover: Option<String>,
    /// Start timestamp for the video in the message
    pub start_timestamp: Option<i64>,
    /// Caption of the video to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the video caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass <em>True</em>, if the caption must be shown above the message media
    pub show_caption_above_media: Option<bool>,
    /// Video width
    pub width: Option<i64>,
    /// Video height
    pub height: Option<i64>,
    /// Video duration in seconds
    pub duration: Option<i64>,
    /// Pass <em>True</em> if the uploaded video is suitable for streaming
    pub supports_streaming: Option<bool>,
    /// Pass <em>True</em> if the video needs to be covered with a spoiler animation
    pub has_spoiler: Option<bool>,
}
/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputMediaAnimation {
    /// Type of the result, must be <em>animation</em>
    pub r#type: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub media: String,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub thumbnail: Option<String>,
    /// Caption of the animation to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the animation caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass <em>True</em>, if the caption must be shown above the message media
    pub show_caption_above_media: Option<bool>,
    /// Animation width
    pub width: Option<i64>,
    /// Animation height
    pub height: Option<i64>,
    /// Animation duration in seconds
    pub duration: Option<i64>,
    /// Pass <em>True</em> if the animation needs to be covered with a spoiler animation
    pub has_spoiler: Option<bool>,
}
/// Represents an audio file to be treated as music to be sent.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputMediaAudio {
    /// Type of the result, must be <em>audio</em>
    pub r#type: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub media: String,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub thumbnail: Option<String>,
    /// Caption of the audio to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the audio caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Duration of the audio in seconds
    pub duration: Option<i64>,
    /// Performer of the audio
    pub performer: Option<String>,
    /// Title of the audio
    pub title: Option<String>,
}
/// Represents a general file to be sent.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputMediaDocument {
    /// Type of the result, must be <em>document</em>
    pub r#type: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub media: String,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub thumbnail: Option<String>,
    /// Caption of the document to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the document caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Disables automatic server-side content type detection for files uploaded using multipart/form-data. Always <em>True</em>, if the document is sent as part of an album.
    pub disable_content_type_detection: Option<bool>,
}
/// This object represents the contents of a file to be uploaded. Must be posted using multipart/form-data in the usual way that files are uploaded via the browser.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputFile;
/// This object describes the paid media to be sent.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum InputPaidMedia {
    Photo(InputPaidMediaPhoto),
    Video(InputPaidMediaVideo),
}
/// The paid media to send is a photo.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputPaidMediaPhoto {
    /// Type of the media, must be <em>photo</em>
    pub r#type: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub media: String,
}
/// The paid media to send is a video.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputPaidMediaVideo {
    /// Type of the media, must be <em>video</em>
    pub r#type: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub media: String,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub thumbnail: Option<String>,
    /// Cover for the video in the message. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub cover: Option<String>,
    /// Start timestamp for the video in the message
    pub start_timestamp: Option<i64>,
    /// Video width
    pub width: Option<i64>,
    /// Video height
    pub height: Option<i64>,
    /// Video duration in seconds
    pub duration: Option<i64>,
    /// Pass <em>True</em> if the uploaded video is suitable for streaming
    pub supports_streaming: Option<bool>,
}
/// This object describes a profile photo to set.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum InputProfilePhoto {
    Static(InputProfilePhotoStatic),
    Animated(InputProfilePhotoAnimated),
}
/// A static profile photo in the .JPG format.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputProfilePhotoStatic {
    /// Type of the profile photo, must be <em>static</em>
    pub r#type: String,
    /// The static profile photo. Profile photos can't be reused and can only be uploaded as a new file, so you can pass “attach://<file_attach_name>” if the photo was uploaded using multipart/form-data under <file_attach_name>. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub photo: String,
}
/// An animated profile photo in the MPEG4 format.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputProfilePhotoAnimated {
    /// Type of the profile photo, must be <em>animated</em>
    pub r#type: String,
    /// The animated profile photo. Profile photos can't be reused and can only be uploaded as a new file, so you can pass “attach://<file_attach_name>” if the photo was uploaded using multipart/form-data under <file_attach_name>. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub animation: String,
    /// Timestamp in seconds of the frame that will be used as the static profile photo. Defaults to 0.0.
    pub main_frame_timestamp: Option<f64>,
}
/// This object describes the content of a story to post.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum InputStoryContent {
    Photo(InputStoryContentPhoto),
    Video(InputStoryContentVideo),
}
/// Describes a photo to post as a story.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputStoryContentPhoto {
    /// Type of the content, must be <em>photo</em>
    pub r#type: String,
    /// The photo to post as a story. The photo must be of the size 1080x1920 and must not exceed 10 MB. The photo can't be reused and can only be uploaded as a new file, so you can pass “attach://<file_attach_name>” if the photo was uploaded using multipart/form-data under <file_attach_name>. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub photo: String,
}
/// Describes a video to post as a story.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputStoryContentVideo {
    /// Type of the content, must be <em>video</em>
    pub r#type: String,
    /// The video to post as a story. The video must be of the size 720x1280, streamable, encoded with H.265 codec, with key frames added each second in the MPEG4 format, and must not exceed 30 MB. The video can't be reused and can only be uploaded as a new file, so you can pass “attach://<file_attach_name>” if the video was uploaded using multipart/form-data under <file_attach_name>. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub video: String,
    /// Precise duration of the video in seconds; 0-60
    pub duration: Option<f64>,
    /// Timestamp in seconds of the frame that will be used as the static cover for the story. Defaults to 0.0.
    pub cover_frame_timestamp: Option<f64>,
    /// Pass <em>True</em> if the video has no sound
    pub is_animation: Option<bool>,
}
/// A simple method for testing your bot's authentication token. Requires no parameters. Returns basic information about the bot in form of a <a href="https://core.telegram.org/bots/api#user">User</a> object.
#[derive(macros::Method)]
#[method(name = "getMe", response(User))]
pub struct GetMeRequest;
/// Use this method to log out from the cloud Bot API server before launching the bot locally. You <strong>must</strong> log out the bot before running it locally, otherwise there is no guarantee that the bot will receive updates. After a successful call, you can immediately log in on a local server, but will not be able to log in back to the cloud Bot API server for 10 minutes. Returns <em>True</em> on success. Requires no parameters.
#[derive(macros::Method)]
#[method(name = "logOut", response(crate::True))]
pub struct LogOutRequest;
/// Use this method to close the bot instance before moving it from one local server to another. You need to delete the webhook before calling this method to ensure that the bot isn't launched again after server restart. The method will return error 429 in the first 10 minutes after the bot is launched. Returns <em>True</em> on success. Requires no parameters.
#[derive(macros::Method)]
#[method(name = "close", response(crate::True))]
pub struct CloseRequest;
/// Use this method to send text messages. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendMessage", response(Message))]
pub struct SendMessageRequest {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    pub direct_messages_topic_id: Option<i64>,
    /// Text of the message to be sent, 1-4096 characters after entities parsing
    pub text: String,
    /// Mode for parsing entities in the message text. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in message text, which can be specified instead of <em>parse_mode</em>
    pub entities: Option<Vec<MessageEntity>>,
    /// Link preview generation options for the message
    pub link_preview_options: Option<LinkPreviewOptions>,
    /// Sends the message <a href="https://telegram.org/blog/channels-2-0#silent-messages">silently</a>. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Pass <em>True</em> to allow up to 1000 messages per second, ignoring <a href="https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once">broadcasting limits</a> for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an <a href="/bots/features#inline-keyboards">inline keyboard</a>, <a href="/bots/features#keyboards">custom reply keyboard</a>, instructions to remove a reply keyboard or to force a reply from the user
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to forward messages of any kind. Service messages and messages with protected content can't be forwarded. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "forwardMessage", response(Message))]
pub struct ForwardMessageRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be forwarded; required if the message is forwarded to a direct messages chat
    pub direct_messages_topic_id: Option<i64>,
    /// Unique identifier for the chat where the original message was sent (or channel username in the format <code>@channelusername</code>)
    pub from_chat_id: ChatId,
    /// New start timestamp for the forwarded video in the message
    pub video_start_timestamp: Option<i64>,
    /// Sends the message <a href="https://telegram.org/blog/channels-2-0#silent-messages">silently</a>. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the forwarded message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; only available when forwarding to private chats
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    /// Message identifier in the chat specified in <em>from_chat_id</em>
    pub message_id: i64,
}
/// Use this method to forward multiple messages of any kind. If some of the specified messages can't be found or forwarded, they are skipped. Service messages and messages with protected content can't be forwarded. Album grouping is kept for forwarded messages. On success, an array of <a href="https://core.telegram.org/bots/api#messageid">MessageId</a> of the sent messages is returned.
#[derive(macros::Method)]
#[method(name = "forwardMessages", response(MessageId))]
pub struct ForwardMessagesRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the messages will be forwarded; required if the messages are forwarded to a direct messages chat
    pub direct_messages_topic_id: Option<i64>,
    /// Unique identifier for the chat where the original messages were sent (or channel username in the format <code>@channelusername</code>)
    pub from_chat_id: ChatId,
    /// A JSON-serialized list of 1-100 identifiers of messages in the chat <em>from_chat_id</em> to forward. The identifiers must be specified in a strictly increasing order.
    pub message_ids: Vec<i64>,
    /// Sends the messages <a href="https://telegram.org/blog/channels-2-0#silent-messages">silently</a>. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the forwarded messages from forwarding and saving
    pub protect_content: Option<bool>,
}
/// Use this method to copy messages of any kind. Service messages, paid media messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz <a href="https://core.telegram.org/bots/api#poll">poll</a> can be copied only if the value of the field <em>correct_option_id</em> is known to the bot. The method is analogous to the method <a href="https://core.telegram.org/bots/api#forwardmessage">forwardMessage</a>, but the copied message doesn't have a link to the original message. Returns the <a href="https://core.telegram.org/bots/api#messageid">MessageId</a> of the sent message on success.
#[derive(macros::Method)]
#[method(name = "copyMessage", response(MessageId))]
pub struct CopyMessageRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    pub direct_messages_topic_id: Option<i64>,
    /// Unique identifier for the chat where the original message was sent (or channel username in the format <code>@channelusername</code>)
    pub from_chat_id: ChatId,
    /// Message identifier in the chat specified in <em>from_chat_id</em>
    pub message_id: i64,
    /// New start timestamp for the copied video in the message
    pub video_start_timestamp: Option<i64>,
    /// New caption for media, 0-1024 characters after entities parsing. If not specified, the original caption is kept
    pub caption: Option<String>,
    /// Mode for parsing entities in the new caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the new caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass <em>True</em>, if the caption must be shown above the message media. Ignored if a new caption isn't specified.
    pub show_caption_above_media: Option<bool>,
    /// Sends the message <a href="https://telegram.org/blog/channels-2-0#silent-messages">silently</a>. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Pass <em>True</em> to allow up to 1000 messages per second, ignoring <a href="https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once">broadcasting limits</a> for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; only available when copying to private chats
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an <a href="/bots/features#inline-keyboards">inline keyboard</a>, <a href="/bots/features#keyboards">custom reply keyboard</a>, instructions to remove a reply keyboard or to force a reply from the user
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to copy messages of any kind. If some of the specified messages can't be found or copied, they are skipped. Service messages, paid media messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz <a href="https://core.telegram.org/bots/api#poll">poll</a> can be copied only if the value of the field <em>correct_option_id</em> is known to the bot. The method is analogous to the method <a href="https://core.telegram.org/bots/api#forwardmessages">forwardMessages</a>, but the copied messages don't have a link to the original message. Album grouping is kept for copied messages. On success, an array of <a href="https://core.telegram.org/bots/api#messageid">MessageId</a> of the sent messages is returned.
#[derive(macros::Method)]
#[method(name = "copyMessages", response(MessageId))]
pub struct CopyMessagesRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the messages will be sent; required if the messages are sent to a direct messages chat
    pub direct_messages_topic_id: Option<i64>,
    /// Unique identifier for the chat where the original messages were sent (or channel username in the format <code>@channelusername</code>)
    pub from_chat_id: ChatId,
    /// A JSON-serialized list of 1-100 identifiers of messages in the chat <em>from_chat_id</em> to copy. The identifiers must be specified in a strictly increasing order.
    pub message_ids: Vec<i64>,
    /// Sends the messages <a href="https://telegram.org/blog/channels-2-0#silent-messages">silently</a>. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent messages from forwarding and saving
    pub protect_content: Option<bool>,
    /// Pass <em>True</em> to copy the messages without their captions
    pub remove_caption: Option<bool>,
}
/// Use this method to send photos. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendPhoto", response(Message))]
pub struct SendPhotoRequest {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    pub direct_messages_topic_id: Option<i64>,
    /// Photo to send. Pass a file_id as String to send a photo that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a photo from the Internet, or upload a new photo using multipart/form-data. The photo must be at most 10 MB in size. The photo's width and height must not exceed 10000 in total. Width and height ratio must be at most 20. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub photo: Attachment,
    /// Photo caption (may also be used when resending photos by <em>file_id</em>), 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass <em>True</em>, if the caption must be shown above the message media
    pub show_caption_above_media: Option<bool>,
    /// Pass <em>True</em> if the photo needs to be covered with a spoiler animation
    pub has_spoiler: Option<bool>,
    /// Sends the message <a href="https://telegram.org/blog/channels-2-0#silent-messages">silently</a>. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Pass <em>True</em> to allow up to 1000 messages per second, ignoring <a href="https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once">broadcasting limits</a> for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an <a href="/bots/features#inline-keyboards">inline keyboard</a>, <a href="/bots/features#keyboards">custom reply keyboard</a>, instructions to remove a reply keyboard or to force a reply from the user
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to send audio files, if you want Telegram clients to display them in the music player. Your audio must be in the .MP3 or .M4A format. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned. Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.
#[derive(macros::Method)]
#[method(name = "sendAudio", response(Message))]
pub struct SendAudioRequest {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    pub direct_messages_topic_id: Option<i64>,
    /// Audio file to send. Pass a file_id as String to send an audio file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an audio file from the Internet, or upload a new one using multipart/form-data. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub audio: Attachment,
    /// Audio caption, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the audio caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Duration of the audio in seconds
    pub duration: Option<i64>,
    /// Performer
    pub performer: Option<String>,
    /// Track name
    pub title: Option<String>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub thumbnail: Option<Attachment>,
    /// Sends the message <a href="https://telegram.org/blog/channels-2-0#silent-messages">silently</a>. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Pass <em>True</em> to allow up to 1000 messages per second, ignoring <a href="https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once">broadcasting limits</a> for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an <a href="/bots/features#inline-keyboards">inline keyboard</a>, <a href="/bots/features#keyboards">custom reply keyboard</a>, instructions to remove a reply keyboard or to force a reply from the user
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to send general files. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned. Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.
#[derive(macros::Method)]
#[method(name = "sendDocument", response(Message))]
pub struct SendDocumentRequest {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    pub direct_messages_topic_id: Option<i64>,
    /// File to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub document: Attachment,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub thumbnail: Option<Attachment>,
    /// Document caption (may also be used when resending documents by <em>file_id</em>), 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the document caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Disables automatic server-side content type detection for files uploaded using multipart/form-data
    pub disable_content_type_detection: Option<bool>,
    /// Sends the message <a href="https://telegram.org/blog/channels-2-0#silent-messages">silently</a>. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Pass <em>True</em> to allow up to 1000 messages per second, ignoring <a href="https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once">broadcasting limits</a> for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an <a href="/bots/features#inline-keyboards">inline keyboard</a>, <a href="/bots/features#keyboards">custom reply keyboard</a>, instructions to remove a reply keyboard or to force a reply from the user
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to send video files, Telegram clients support MPEG4 videos (other formats may be sent as <a href="https://core.telegram.org/bots/api#document">Document</a>). On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned. Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
#[derive(macros::Method)]
#[method(name = "sendVideo", response(Message))]
pub struct SendVideoRequest {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    pub direct_messages_topic_id: Option<i64>,
    /// Video to send. Pass a file_id as String to send a video that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a video from the Internet, or upload a new video using multipart/form-data. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub video: Attachment,
    /// Duration of sent video in seconds
    pub duration: Option<i64>,
    /// Video width
    pub width: Option<i64>,
    /// Video height
    pub height: Option<i64>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub thumbnail: Option<Attachment>,
    /// Cover for the video in the message. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub cover: Option<Attachment>,
    /// Start timestamp for the video in the message
    pub start_timestamp: Option<i64>,
    /// Video caption (may also be used when resending videos by <em>file_id</em>), 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the video caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass <em>True</em>, if the caption must be shown above the message media
    pub show_caption_above_media: Option<bool>,
    /// Pass <em>True</em> if the video needs to be covered with a spoiler animation
    pub has_spoiler: Option<bool>,
    /// Pass <em>True</em> if the uploaded video is suitable for streaming
    pub supports_streaming: Option<bool>,
    /// Sends the message <a href="https://telegram.org/blog/channels-2-0#silent-messages">silently</a>. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Pass <em>True</em> to allow up to 1000 messages per second, ignoring <a href="https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once">broadcasting limits</a> for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an <a href="/bots/features#inline-keyboards">inline keyboard</a>, <a href="/bots/features#keyboards">custom reply keyboard</a>, instructions to remove a reply keyboard or to force a reply from the user
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
#[derive(macros::Method)]
#[method(name = "sendAnimation", response(Message))]
pub struct SendAnimationRequest {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    pub direct_messages_topic_id: Option<i64>,
    /// Animation to send. Pass a file_id as String to send an animation that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an animation from the Internet, or upload a new animation using multipart/form-data. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub animation: Attachment,
    /// Duration of sent animation in seconds
    pub duration: Option<i64>,
    /// Animation width
    pub width: Option<i64>,
    /// Animation height
    pub height: Option<i64>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub thumbnail: Option<Attachment>,
    /// Animation caption (may also be used when resending animation by <em>file_id</em>), 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the animation caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass <em>True</em>, if the caption must be shown above the message media
    pub show_caption_above_media: Option<bool>,
    /// Pass <em>True</em> if the animation needs to be covered with a spoiler animation
    pub has_spoiler: Option<bool>,
    /// Sends the message <a href="https://telegram.org/blog/channels-2-0#silent-messages">silently</a>. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Pass <em>True</em> to allow up to 1000 messages per second, ignoring <a href="https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once">broadcasting limits</a> for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an <a href="/bots/features#inline-keyboards">inline keyboard</a>, <a href="/bots/features#keyboards">custom reply keyboard</a>, instructions to remove a reply keyboard or to force a reply from the user
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message. For this to work, your audio must be in an .OGG file encoded with OPUS, or in .MP3 format, or in .M4A format (other formats may be sent as <a href="https://core.telegram.org/bots/api#audio">Audio</a> or <a href="https://core.telegram.org/bots/api#document">Document</a>). On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future.
#[derive(macros::Method)]
#[method(name = "sendVoice", response(Message))]
pub struct SendVoiceRequest {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    pub direct_messages_topic_id: Option<i64>,
    /// Audio file to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub voice: Attachment,
    /// Voice message caption, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the voice message caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Duration of the voice message in seconds
    pub duration: Option<i64>,
    /// Sends the message <a href="https://telegram.org/blog/channels-2-0#silent-messages">silently</a>. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Pass <em>True</em> to allow up to 1000 messages per second, ignoring <a href="https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once">broadcasting limits</a> for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an <a href="/bots/features#inline-keyboards">inline keyboard</a>, <a href="/bots/features#keyboards">custom reply keyboard</a>, instructions to remove a reply keyboard or to force a reply from the user
    pub reply_markup: Option<ReplyMarkup>,
}
/// As of <a href="https://telegram.org/blog/video-messages-and-telescope">v.4.0</a>, Telegram clients support rounded square MPEG4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendVideoNote", response(Message))]
pub struct SendVideoNoteRequest {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    pub direct_messages_topic_id: Option<i64>,
    /// Video note to send. Pass a file_id as String to send a video note that exists on the Telegram servers (recommended) or upload a new video using multipart/form-data. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>. Sending video notes by a URL is currently unsupported
    pub video_note: Attachment,
    /// Duration of sent video in seconds
    pub duration: Option<i64>,
    /// Video width and height, i.e. diameter of the video message
    pub length: Option<i64>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub thumbnail: Option<Attachment>,
    /// Sends the message <a href="https://telegram.org/blog/channels-2-0#silent-messages">silently</a>. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Pass <em>True</em> to allow up to 1000 messages per second, ignoring <a href="https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once">broadcasting limits</a> for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an <a href="/bots/features#inline-keyboards">inline keyboard</a>, <a href="/bots/features#keyboards">custom reply keyboard</a>, instructions to remove a reply keyboard or to force a reply from the user
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to send paid media. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendPaidMedia", response(Message))]
pub struct SendPaidMediaRequest {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>). If the chat is a channel, all Telegram Star proceeds from this media will be credited to the chat's balance. Otherwise, they will be credited to the bot's balance.
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    pub direct_messages_topic_id: Option<i64>,
    /// The number of Telegram Stars that must be paid to buy access to the media; 1-25000
    pub star_count: i64,
    /// A JSON-serialized array describing the media to be sent; up to 10 items
    pub media: Vec<InputPaidMedia>,
    /// Bot-defined paid media payload, 0-128 bytes. This will not be displayed to the user, use it for your internal processes.
    pub payload: Option<String>,
    /// Media caption, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the media caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass <em>True</em>, if the caption must be shown above the message media
    pub show_caption_above_media: Option<bool>,
    /// Sends the message <a href="https://telegram.org/blog/channels-2-0#silent-messages">silently</a>. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Pass <em>True</em> to allow up to 1000 messages per second, ignoring <a href="https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once">broadcasting limits</a> for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    pub allow_paid_broadcast: Option<bool>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an <a href="/bots/features#inline-keyboards">inline keyboard</a>, <a href="/bots/features#keyboards">custom reply keyboard</a>, instructions to remove a reply keyboard or to force a reply from the user
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of <a href="https://core.telegram.org/bots/api#message">Message</a> objects that were sent is returned.
#[derive(macros::Method)]
#[method(name = "sendMediaGroup", response(Message))]
pub struct SendMediaGroupRequest {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the messages will be sent; required if the messages are sent to a direct messages chat
    pub direct_messages_topic_id: Option<i64>,
    /// A JSON-serialized array describing messages to be sent, must include 2-10 items
    pub media: Vec<InputMediaAudio>,
    /// Sends messages <a href="https://telegram.org/blog/channels-2-0#silent-messages">silently</a>. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent messages from forwarding and saving
    pub protect_content: Option<bool>,
    /// Pass <em>True</em> to allow up to 1000 messages per second, ignoring <a href="https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once">broadcasting limits</a> for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    pub message_effect_id: Option<String>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
}
/// Use this method to send point on the map. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendLocation", response(Message))]
pub struct SendLocationRequest {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    pub direct_messages_topic_id: Option<i64>,
    /// Latitude of the location
    pub latitude: f64,
    /// Longitude of the location
    pub longitude: f64,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,
    /// Period in seconds during which the location will be updated (see <a href="https://telegram.org/blog/live-locations">Live Locations</a>, should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely.
    pub live_period: Option<i64>,
    /// For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    pub heading: Option<i64>,
    /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    pub proximity_alert_radius: Option<i64>,
    /// Sends the message <a href="https://telegram.org/blog/channels-2-0#silent-messages">silently</a>. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Pass <em>True</em> to allow up to 1000 messages per second, ignoring <a href="https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once">broadcasting limits</a> for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an <a href="/bots/features#inline-keyboards">inline keyboard</a>, <a href="/bots/features#keyboards">custom reply keyboard</a>, instructions to remove a reply keyboard or to force a reply from the user
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to send information about a venue. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendVenue", response(Message))]
pub struct SendVenueRequest {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    pub direct_messages_topic_id: Option<i64>,
    /// Latitude of the venue
    pub latitude: f64,
    /// Longitude of the venue
    pub longitude: f64,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue
    pub google_place_id: Option<String>,
    /// Google Places type of the venue. (See <a href="https://developers.google.com/places/web-service/supported_types">supported types</a>.)
    pub google_place_type: Option<String>,
    /// Sends the message <a href="https://telegram.org/blog/channels-2-0#silent-messages">silently</a>. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Pass <em>True</em> to allow up to 1000 messages per second, ignoring <a href="https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once">broadcasting limits</a> for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an <a href="/bots/features#inline-keyboards">inline keyboard</a>, <a href="/bots/features#keyboards">custom reply keyboard</a>, instructions to remove a reply keyboard or to force a reply from the user
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to send phone contacts. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendContact", response(Message))]
pub struct SendContactRequest {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    pub direct_messages_topic_id: Option<i64>,
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    pub last_name: Option<String>,
    /// Additional data about the contact in the form of a <a href="https://en.wikipedia.org/wiki/VCard">vCard</a>, 0-2048 bytes
    pub vcard: Option<String>,
    /// Sends the message <a href="https://telegram.org/blog/channels-2-0#silent-messages">silently</a>. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Pass <em>True</em> to allow up to 1000 messages per second, ignoring <a href="https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once">broadcasting limits</a> for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an <a href="/bots/features#inline-keyboards">inline keyboard</a>, <a href="/bots/features#keyboards">custom reply keyboard</a>, instructions to remove a reply keyboard or to force a reply from the user
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to send a native poll. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendPoll", response(Message))]
pub struct SendPollRequest {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>). Polls can't be sent to channel direct messages chats.
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Poll question, 1-300 characters
    pub question: String,
    /// Mode for parsing entities in the question. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details. Currently, only custom emoji entities are allowed
    pub question_parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the poll question. It can be specified instead of <em>question_parse_mode</em>
    pub question_entities: Option<Vec<MessageEntity>>,
    /// A JSON-serialized list of 2-12 answer options
    pub options: Vec<InputPollOption>,
    /// <em>True</em>, if the poll needs to be anonymous, defaults to <em>True</em>
    pub is_anonymous: Option<bool>,
    /// Poll type, “quiz” or “regular”, defaults to “regular”
    pub r#type: Option<String>,
    /// <em>True</em>, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to <em>False</em>
    pub allows_multiple_answers: Option<bool>,
    /// 0-based identifier of the correct answer option, required for polls in quiz mode
    pub correct_option_id: Option<i64>,
    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing
    pub explanation: Option<String>,
    /// Mode for parsing entities in the explanation. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub explanation_parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the poll explanation. It can be specified instead of <em>explanation_parse_mode</em>
    pub explanation_entities: Option<Vec<MessageEntity>>,
    /// Amount of time in seconds the poll will be active after creation, 5-600. Can't be used together with <em>close_date</em>.
    pub open_period: Option<i64>,
    /// Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 600 seconds in the future. Can't be used together with <em>open_period</em>.
    pub close_date: Option<i64>,
    /// Pass <em>True</em> if the poll needs to be immediately closed. This can be useful for poll preview.
    pub is_closed: Option<bool>,
    /// Sends the message <a href="https://telegram.org/blog/channels-2-0#silent-messages">silently</a>. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Pass <em>True</em> to allow up to 1000 messages per second, ignoring <a href="https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once">broadcasting limits</a> for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    pub message_effect_id: Option<String>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an <a href="/bots/features#inline-keyboards">inline keyboard</a>, <a href="/bots/features#keyboards">custom reply keyboard</a>, instructions to remove a reply keyboard or to force a reply from the user
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to send a checklist on behalf of a connected business account. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendChecklist", response(Message))]
pub struct SendChecklistRequest {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: String,
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// A JSON-serialized object for the checklist to send
    pub checklist: InputChecklist,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Unique identifier of the message effect to be added to the message
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object for description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
    /// A JSON-serialized object for an <a href="/bots/features#inline-keyboards">inline keyboard</a>
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// Use this method to send an animated emoji that will display a random value. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendDice", response(Message))]
pub struct SendDiceRequest {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    pub direct_messages_topic_id: Option<i64>,
    /// Emoji on which the dice throw animation is based. Currently, must be one of “<img class="emoji" src="//telegram.org/img/emoji/40/F09F8EB2.png" width="20" height="20" alt="🎲" />”, “<img class="emoji" src="//telegram.org/img/emoji/40/F09F8EAF.png" width="20" height="20" alt="🎯" />”, “<img class="emoji" src="//telegram.org/img/emoji/40/F09F8F80.png" width="20" height="20" alt="🏀" />”, “<img class="emoji" src="//telegram.org/img/emoji/40/E29ABD.png" width="20" height="20" alt="⚽" />”, “<img class="emoji" src="//telegram.org/img/emoji/40/F09F8EB3.png" width="20" height="20" alt="🎳" />”, or “<img class="emoji" src="//telegram.org/img/emoji/40/F09F8EB0.png" width="20" height="20" alt="🎰" />”. Dice can have values 1-6 for “<img class="emoji" src="//telegram.org/img/emoji/40/F09F8EB2.png" width="20" height="20" alt="🎲" />”, “<img class="emoji" src="//telegram.org/img/emoji/40/F09F8EAF.png" width="20" height="20" alt="🎯" />” and “<img class="emoji" src="//telegram.org/img/emoji/40/F09F8EB3.png" width="20" height="20" alt="🎳" />”, values 1-5 for “<img class="emoji" src="//telegram.org/img/emoji/40/F09F8F80.png" width="20" height="20" alt="🏀" />” and “<img class="emoji" src="//telegram.org/img/emoji/40/E29ABD.png" width="20" height="20" alt="⚽" />”, and values 1-64 for “<img class="emoji" src="//telegram.org/img/emoji/40/F09F8EB0.png" width="20" height="20" alt="🎰" />”. Defaults to “<img class="emoji" src="//telegram.org/img/emoji/40/F09F8EB2.png" width="20" height="20" alt="🎲" />”
    pub emoji: Option<String>,
    /// Sends the message <a href="https://telegram.org/blog/channels-2-0#silent-messages">silently</a>. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding
    pub protect_content: Option<bool>,
    /// Pass <em>True</em> to allow up to 1000 messages per second, ignoring <a href="https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once">broadcasting limits</a> for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an <a href="/bots/features#inline-keyboards">inline keyboard</a>, <a href="/bots/features#keyboards">custom reply keyboard</a>, instructions to remove a reply keyboard or to force a reply from the user
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to stream a partial message to a user while the message is being generated; supported only for bots with forum topic mode enabled. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "sendMessageDraft", response(crate::True))]
pub struct SendMessageDraftRequest {
    /// Unique identifier for the target private chat
    pub chat_id: i64,
    /// Unique identifier for the target message thread
    pub message_thread_id: Option<i64>,
    /// Unique identifier of the message draft; must be non-zero. Changes of drafts with the same identifier are animated
    pub draft_id: i64,
    /// Text of the message to be sent, 1-4096 characters after entities parsing
    pub text: String,
    /// Mode for parsing entities in the message text. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in message text, which can be specified instead of <em>parse_mode</em>
    pub entities: Option<Vec<MessageEntity>>,
}
/// Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "sendChatAction", response(crate::True))]
pub struct SendChatActionRequest {
    /// Unique identifier of the business connection on behalf of which the action will be sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target supergroup (in the format <code>@supergroupusername</code>). Channel chats and channel direct messages chats aren't supported.
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread or topic of a forum; for supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Type of action to broadcast. Choose one, depending on what the user is about to receive: <em>typing</em> for <a href="https://core.telegram.org/bots/api#sendmessage">text messages</a>, <em>upload_photo</em> for <a href="https://core.telegram.org/bots/api#sendphoto">photos</a>, <em>record_video</em> or <em>upload_video</em> for <a href="https://core.telegram.org/bots/api#sendvideo">videos</a>, <em>record_voice</em> or <em>upload_voice</em> for <a href="https://core.telegram.org/bots/api#sendvoice">voice notes</a>, <em>upload_document</em> for <a href="https://core.telegram.org/bots/api#senddocument">general files</a>, <em>choose_sticker</em> for <a href="https://core.telegram.org/bots/api#sendsticker">stickers</a>, <em>find_location</em> for <a href="https://core.telegram.org/bots/api#sendlocation">location data</a>, <em>record_video_note</em> or <em>upload_video_note</em> for <a href="https://core.telegram.org/bots/api#sendvideonote">video notes</a>.
    pub action: String,
}
/// Use this method to change the chosen reactions on a message. Service messages of some types can't be reacted to. Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel. Bots can't use paid reactions. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setMessageReaction", response(crate::True))]
pub struct SetMessageReactionRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Identifier of the target message. If the message belongs to a media group, the reaction is set to the first non-deleted message in the group instead.
    pub message_id: i64,
    /// A JSON-serialized list of reaction types to set on the message. Currently, as non-premium users, bots can set up to one reaction per message. A custom emoji reaction can be used if it is either already present on the message or explicitly allowed by chat administrators. Paid reactions can't be used by bots.
    pub reaction: Option<Vec<ReactionType>>,
    /// Pass <em>True</em> to set the reaction with a big animation
    pub is_big: Option<bool>,
}
/// Use this method to get a list of profile pictures for a user. Returns a <a href="https://core.telegram.org/bots/api#userprofilephotos">UserProfilePhotos</a> object.
#[derive(macros::Method)]
#[method(name = "getUserProfilePhotos", response(UserProfilePhotos))]
pub struct GetUserProfilePhotosRequest {
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Sequential number of the first photo to be returned. By default, all photos are returned.
    pub offset: Option<i64>,
    /// Limits the number of photos to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    pub limit: Option<i64>,
}
/// Use this method to get a list of profile audios for a user. Returns a <a href="https://core.telegram.org/bots/api#userprofileaudios">UserProfileAudios</a> object.
#[derive(macros::Method)]
#[method(name = "getUserProfileAudios", response(UserProfileAudios))]
pub struct GetUserProfileAudiosRequest {
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Sequential number of the first audio to be returned. By default, all audios are returned.
    pub offset: Option<i64>,
    /// Limits the number of audios to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    pub limit: Option<i64>,
}
/// Changes the emoji status for a given user that previously allowed the bot to manage their emoji status via the Mini App method <a href="/bots/webapps#initializing-mini-apps">requestEmojiStatusAccess</a>. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setUserEmojiStatus", response(crate::True))]
pub struct SetUserEmojiStatusRequest {
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Custom emoji identifier of the emoji status to set. Pass an empty string to remove the status.
    pub emoji_status_custom_emoji_id: Option<String>,
    /// Expiration date of the emoji status, if any
    pub emoji_status_expiration_date: Option<i64>,
}
/// Use this method to get basic information about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a <a href="https://core.telegram.org/bots/api#file">File</a> object is returned. The file can then be downloaded via the link <code>https://api.telegram.org/file/bot<token>/<file_path></code>, where <code><file_path></code> is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling <a href="https://core.telegram.org/bots/api#getfile">getFile</a> again.
#[derive(macros::Method)]
#[method(name = "getFile", response(File))]
pub struct GetFileRequest {
    /// File identifier to get information about
    pub file_id: String,
}
/// Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless <a href="https://core.telegram.org/bots/api#unbanchatmember">unbanned</a> first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "banChatMember", response(crate::True))]
pub struct BanChatMemberRequest {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Date when the user will be unbanned; Unix time. If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever. Applied for supergroups and channels only.
    pub until_date: Option<i64>,
    /// Pass <em>True</em> to delete all messages from the chat for the user that is being removed. If <em>False</em>, the user will be able to see messages in the group that were sent before the user was removed. Always <em>True</em> for supergroups and channels.
    pub revoke_messages: Option<bool>,
}
/// Use this method to unban a previously banned user in a supergroup or channel. The user will <strong>not</strong> return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it. So if the user is a member of the chat they will also be <strong>removed</strong> from the chat. If you don't want this, use the parameter <em>only_if_banned</em>. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "unbanChatMember", response(crate::True))]
pub struct UnbanChatMemberRequest {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Do nothing if the user is not banned
    pub only_if_banned: Option<bool>,
}
/// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass <em>True</em> for all permissions to lift restrictions from a user. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "restrictChatMember", response(crate::True))]
pub struct RestrictChatMemberRequest {
    /// Unique identifier for the target chat or username of the target supergroup (in the format <code>@supergroupusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// A JSON-serialized object for new user permissions
    pub permissions: ChatPermissions,
    /// Pass <em>True</em> if chat permissions are set independently. Otherwise, the <em>can_send_other_messages</em> and <em>can_add_web_page_previews</em> permissions will imply the <em>can_send_messages</em>, <em>can_send_audios</em>, <em>can_send_documents</em>, <em>can_send_photos</em>, <em>can_send_videos</em>, <em>can_send_video_notes</em>, and <em>can_send_voice_notes</em> permissions; the <em>can_send_polls</em> permission will imply the <em>can_send_messages</em> permission.
    pub use_independent_chat_permissions: Option<bool>,
    /// Date when restrictions will be lifted for the user; Unix time. If user is restricted for more than 366 days or less than 30 seconds from the current time, they are considered to be restricted forever
    pub until_date: Option<i64>,
}
/// Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass <em>False</em> for all boolean parameters to demote a user. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "promoteChatMember", response(crate::True))]
pub struct PromoteChatMemberRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Pass <em>True</em> if the administrator's presence in the chat is hidden
    pub is_anonymous: Option<bool>,
    /// Pass <em>True</em> if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages, ignore slow mode, and send messages to the chat without paying Telegram Stars. Implied by any other administrator privilege.
    pub can_manage_chat: Option<bool>,
    /// Pass <em>True</em> if the administrator can delete messages of other users
    pub can_delete_messages: Option<bool>,
    /// Pass <em>True</em> if the administrator can manage video chats
    pub can_manage_video_chats: Option<bool>,
    /// Pass <em>True</em> if the administrator can restrict, ban or unban chat members, or access supergroup statistics. For backward compatibility, defaults to <em>True</em> for promotions of channel administrators
    pub can_restrict_members: Option<bool>,
    /// Pass <em>True</em> if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by him)
    pub can_promote_members: Option<bool>,
    /// Pass <em>True</em> if the administrator can change chat title, photo and other settings
    pub can_change_info: Option<bool>,
    /// Pass <em>True</em> if the administrator can invite new users to the chat
    pub can_invite_users: Option<bool>,
    /// Pass <em>True</em> if the administrator can post stories to the chat
    pub can_post_stories: Option<bool>,
    /// Pass <em>True</em> if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive
    pub can_edit_stories: Option<bool>,
    /// Pass <em>True</em> if the administrator can delete stories posted by other users
    pub can_delete_stories: Option<bool>,
    /// Pass <em>True</em> if the administrator can post messages in the channel, approve suggested posts, or access channel statistics; for channels only
    pub can_post_messages: Option<bool>,
    /// Pass <em>True</em> if the administrator can edit messages of other users and can pin messages; for channels only
    pub can_edit_messages: Option<bool>,
    /// Pass <em>True</em> if the administrator can pin messages; for supergroups only
    pub can_pin_messages: Option<bool>,
    /// Pass <em>True</em> if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only
    pub can_manage_topics: Option<bool>,
    /// Pass <em>True</em> if the administrator can manage direct messages within the channel and decline suggested posts; for channels only
    pub can_manage_direct_messages: Option<bool>,
}
/// Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setChatAdministratorCustomTitle", response(crate::True))]
pub struct SetChatAdministratorCustomTitleRequest {
    /// Unique identifier for the target chat or username of the target supergroup (in the format <code>@supergroupusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// New custom title for the administrator; 0-16 characters, emoji are not allowed
    pub custom_title: String,
}
/// Use this method to ban a channel chat in a supergroup or a channel. Until the chat is <a href="https://core.telegram.org/bots/api#unbanchatsenderchat">unbanned</a>, the owner of the banned chat won't be able to send messages on behalf of <strong>any of their channels</strong>. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "banChatSenderChat", response(crate::True))]
pub struct BanChatSenderChatRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier of the target sender chat
    pub sender_chat_id: i64,
}
/// Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "unbanChatSenderChat", response(crate::True))]
pub struct UnbanChatSenderChatRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier of the target sender chat
    pub sender_chat_id: i64,
}
/// Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the <em>can_restrict_members</em> administrator rights. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setChatPermissions", response(crate::True))]
pub struct SetChatPermissionsRequest {
    /// Unique identifier for the target chat or username of the target supergroup (in the format <code>@supergroupusername</code>)
    pub chat_id: ChatId,
    /// A JSON-serialized object for new default chat permissions
    pub permissions: ChatPermissions,
    /// Pass <em>True</em> if chat permissions are set independently. Otherwise, the <em>can_send_other_messages</em> and <em>can_add_web_page_previews</em> permissions will imply the <em>can_send_messages</em>, <em>can_send_audios</em>, <em>can_send_documents</em>, <em>can_send_photos</em>, <em>can_send_videos</em>, <em>can_send_video_notes</em>, and <em>can_send_voice_notes</em> permissions; the <em>can_send_polls</em> permission will imply the <em>can_send_messages</em> permission.
    pub use_independent_chat_permissions: Option<bool>,
}
/// Use this method to generate a new primary invite link for a chat; any previously generated primary link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the new invite link as <em>String</em> on success.
#[derive(macros::Method)]
#[method(name = "exportChatInviteLink", response(String))]
pub struct ExportChatInviteLinkRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
}
/// Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method <a href="https://core.telegram.org/bots/api#revokechatinvitelink">revokeChatInviteLink</a>. Returns the new invite link as <a href="https://core.telegram.org/bots/api#chatinvitelink">ChatInviteLink</a> object.
#[derive(macros::Method)]
#[method(name = "createChatInviteLink", response(ChatInviteLink))]
pub struct CreateChatInviteLinkRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Invite link name; 0-32 characters
    pub name: Option<String>,
    /// Point in time (Unix timestamp) when the link will expire
    pub expire_date: Option<i64>,
    /// The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    pub member_limit: Option<i64>,
    /// <em>True</em>, if users joining the chat via the link need to be approved by chat administrators. If <em>True</em>, <em>member_limit</em> can't be specified
    pub creates_join_request: Option<bool>,
}
/// Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a <a href="https://core.telegram.org/bots/api#chatinvitelink">ChatInviteLink</a> object.
#[derive(macros::Method)]
#[method(name = "editChatInviteLink", response(ChatInviteLink))]
pub struct EditChatInviteLinkRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// The invite link to edit
    pub invite_link: String,
    /// Invite link name; 0-32 characters
    pub name: Option<String>,
    /// Point in time (Unix timestamp) when the link will expire
    pub expire_date: Option<i64>,
    /// The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    pub member_limit: Option<i64>,
    /// <em>True</em>, if users joining the chat via the link need to be approved by chat administrators. If <em>True</em>, <em>member_limit</em> can't be specified
    pub creates_join_request: Option<bool>,
}
/// Use this method to create a <a href="https://telegram.org/blog/superchannels-star-reactions-subscriptions#star-subscriptions">subscription invite link</a> for a channel chat. The bot must have the <em>can_invite_users</em> administrator rights. The link can be edited using the method <a href="https://core.telegram.org/bots/api#editchatsubscriptioninvitelink">editChatSubscriptionInviteLink</a> or revoked using the method <a href="https://core.telegram.org/bots/api#revokechatinvitelink">revokeChatInviteLink</a>. Returns the new invite link as a <a href="https://core.telegram.org/bots/api#chatinvitelink">ChatInviteLink</a> object.
#[derive(macros::Method)]
#[method(name = "createChatSubscriptionInviteLink", response(ChatInviteLink))]
pub struct CreateChatSubscriptionInviteLinkRequest {
    /// Unique identifier for the target channel chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Invite link name; 0-32 characters
    pub name: Option<String>,
    /// The number of seconds the subscription will be active for before the next payment. Currently, it must always be 2592000 (30 days).
    pub subscription_period: i64,
    /// The amount of Telegram Stars a user must pay initially and after each subsequent subscription period to be a member of the chat; 1-10000
    pub subscription_price: i64,
}
/// Use this method to edit a subscription invite link created by the bot. The bot must have the <em>can_invite_users</em> administrator rights. Returns the edited invite link as a <a href="https://core.telegram.org/bots/api#chatinvitelink">ChatInviteLink</a> object.
#[derive(macros::Method)]
#[method(name = "editChatSubscriptionInviteLink", response(ChatInviteLink))]
pub struct EditChatSubscriptionInviteLinkRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// The invite link to edit
    pub invite_link: String,
    /// Invite link name; 0-32 characters
    pub name: Option<String>,
}
/// Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the revoked invite link as <a href="https://core.telegram.org/bots/api#chatinvitelink">ChatInviteLink</a> object.
#[derive(macros::Method)]
#[method(name = "revokeChatInviteLink", response(ChatInviteLink))]
pub struct RevokeChatInviteLinkRequest {
    /// Unique identifier of the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// The invite link to revoke
    pub invite_link: String,
}
/// Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the <em>can_invite_users</em> administrator right. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "approveChatJoinRequest", response(crate::True))]
pub struct ApproveChatJoinRequestRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
}
/// Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the <em>can_invite_users</em> administrator right. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "declineChatJoinRequest", response(crate::True))]
pub struct DeclineChatJoinRequestRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
}
/// Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setChatPhoto", response(crate::True))]
pub struct SetChatPhotoRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// New chat photo, uploaded using multipart/form-data
    pub photo: InputFile,
}
/// Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "deleteChatPhoto", response(crate::True))]
pub struct DeleteChatPhotoRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
}
/// Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setChatTitle", response(crate::True))]
pub struct SetChatTitleRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// New chat title, 1-128 characters
    pub title: String,
}
/// Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setChatDescription", response(crate::True))]
pub struct SetChatDescriptionRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// New chat description, 0-255 characters
    pub description: Option<String>,
}
/// Use this method to add a message to the list of pinned messages in a chat. In private chats and channel direct messages chats, all non-service messages can be pinned. Conversely, the bot must be an administrator with the 'can_pin_messages' right or the 'can_edit_messages' right to pin messages in groups and channels respectively. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "pinChatMessage", response(crate::True))]
pub struct PinChatMessageRequest {
    /// Unique identifier of the business connection on behalf of which the message will be pinned
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Identifier of a message to pin
    pub message_id: i64,
    /// Pass <em>True</em> if it is not necessary to send a notification to all chat members about the new pinned message. Notifications are always disabled in channels and private chats.
    pub disable_notification: Option<bool>,
}
/// Use this method to remove a message from the list of pinned messages in a chat. In private chats and channel direct messages chats, all messages can be unpinned. Conversely, the bot must be an administrator with the 'can_pin_messages' right or the 'can_edit_messages' right to unpin messages in groups and channels respectively. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "unpinChatMessage", response(crate::True))]
pub struct UnpinChatMessageRequest {
    /// Unique identifier of the business connection on behalf of which the message will be unpinned
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Identifier of the message to unpin. Required if <em>business_connection_id</em> is specified. If not specified, the most recent pinned message (by sending date) will be unpinned.
    pub message_id: Option<i64>,
}
/// Use this method to clear the list of pinned messages in a chat. In private chats and channel direct messages chats, no additional rights are required to unpin all pinned messages. Conversely, the bot must be an administrator with the 'can_pin_messages' right or the 'can_edit_messages' right to unpin all pinned messages in groups and channels respectively. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "unpinAllChatMessages", response(crate::True))]
pub struct UnpinAllChatMessagesRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
}
/// Use this method for your bot to leave a group, supergroup or channel. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "leaveChat", response(crate::True))]
pub struct LeaveChatRequest {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format <code>@channelusername</code>). Channel direct messages chats aren't supported; leave the corresponding channel instead.
    pub chat_id: ChatId,
}
/// Use this method to get up-to-date information about the chat. Returns a <a href="https://core.telegram.org/bots/api#chatfullinfo">ChatFullInfo</a> object on success.
#[derive(macros::Method)]
#[method(name = "getChat", response(ChatFullInfo))]
pub struct GetChatRequest {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
}
/// Use this method to get a list of administrators in a chat, which aren't bots. Returns an Array of <a href="https://core.telegram.org/bots/api#chatmember">ChatMember</a> objects.
#[derive(macros::Method)]
#[method(name = "getChatAdministrators", response(Vec<ChatMember>))]
pub struct GetChatAdministratorsRequest {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
}
/// Use this method to get the number of members in a chat. Returns <em>Int</em> on success.
#[derive(macros::Method)]
#[method(name = "getChatMemberCount", response(i64))]
pub struct GetChatMemberCountRequest {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
}
/// Use this method to get information about a member of a chat. The method is only guaranteed to work for other users if the bot is an administrator in the chat. Returns a <a href="https://core.telegram.org/bots/api#chatmember">ChatMember</a> object on success.
#[derive(macros::Method)]
#[method(name = "getChatMember", response(ChatMember))]
pub struct GetChatMemberRequest {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
}
/// Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field <em>can_set_sticker_set</em> optionally returned in <a href="https://core.telegram.org/bots/api#getchat">getChat</a> requests to check if the bot can use this method. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setChatStickerSet", response(crate::True))]
pub struct SetChatStickerSetRequest {
    /// Unique identifier for the target chat or username of the target supergroup (in the format <code>@supergroupusername</code>)
    pub chat_id: ChatId,
    /// Name of the sticker set to be set as the group sticker set
    pub sticker_set_name: String,
}
/// Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field <em>can_set_sticker_set</em> optionally returned in <a href="https://core.telegram.org/bots/api#getchat">getChat</a> requests to check if the bot can use this method. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "deleteChatStickerSet", response(crate::True))]
pub struct DeleteChatStickerSetRequest {
    /// Unique identifier for the target chat or username of the target supergroup (in the format <code>@supergroupusername</code>)
    pub chat_id: ChatId,
}
/// Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters. Returns an Array of <a href="https://core.telegram.org/bots/api#sticker">Sticker</a> objects.
#[derive(macros::Method)]
#[method(name = "getForumTopicIconStickers", response(Vec<Sticker>))]
pub struct GetForumTopicIconStickersRequest;
/// Use this method to create a topic in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator right. Returns information about the created topic as a <a href="https://core.telegram.org/bots/api#forumtopic">ForumTopic</a> object.
#[derive(macros::Method)]
#[method(name = "createForumTopic", response(ForumTopic))]
pub struct CreateForumTopicRequest {
    /// Unique identifier for the target chat or username of the target supergroup (in the format <code>@supergroupusername</code>)
    pub chat_id: ChatId,
    /// Topic name, 1-128 characters
    pub name: String,
    /// Color of the topic icon in RGB format. Currently, must be one of 7322096 (0x6FB9F0), 16766590 (0xFFD67E), 13338331 (0xCB86DB), 9367192 (0x8EEE98), 16749490 (0xFF93B2), or 16478047 (0xFB6F5F)
    pub icon_color: Option<i64>,
    /// Unique identifier of the custom emoji shown as the topic icon. Use <a href="https://core.telegram.org/bots/api#getforumtopiciconstickers">getForumTopicIconStickers</a> to get all allowed custom emoji identifiers.
    pub icon_custom_emoji_id: Option<String>,
}
/// Use this method to edit name and icon of a topic in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights, unless it is the creator of the topic. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "editForumTopic", response(crate::True))]
pub struct EditForumTopicRequest {
    /// Unique identifier for the target chat or username of the target supergroup (in the format <code>@supergroupusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
    /// New topic name, 0-128 characters. If not specified or empty, the current name of the topic will be kept
    pub name: Option<String>,
    /// New unique identifier of the custom emoji shown as the topic icon. Use <a href="https://core.telegram.org/bots/api#getforumtopiciconstickers">getForumTopicIconStickers</a> to get all allowed custom emoji identifiers. Pass an empty string to remove the icon. If not specified, the current icon will be kept
    pub icon_custom_emoji_id: Option<String>,
}
/// Use this method to close an open topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights, unless it is the creator of the topic. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "closeForumTopic", response(crate::True))]
pub struct CloseForumTopicRequest {
    /// Unique identifier for the target chat or username of the target supergroup (in the format <code>@supergroupusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
}
/// Use this method to reopen a closed topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights, unless it is the creator of the topic. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "reopenForumTopic", response(crate::True))]
pub struct ReopenForumTopicRequest {
    /// Unique identifier for the target chat or username of the target supergroup (in the format <code>@supergroupusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
}
/// Use this method to delete a forum topic along with all its messages in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the <em>can_delete_messages</em> administrator rights. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "deleteForumTopic", response(crate::True))]
pub struct DeleteForumTopicRequest {
    /// Unique identifier for the target chat or username of the target supergroup (in the format <code>@supergroupusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
}
/// Use this method to clear the list of pinned messages in a forum topic in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the <em>can_pin_messages</em> administrator right in the supergroup. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "unpinAllForumTopicMessages", response(crate::True))]
pub struct UnpinAllForumTopicMessagesRequest {
    /// Unique identifier for the target chat or username of the target supergroup (in the format <code>@supergroupusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
}
/// Use this method to edit the name of the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "editGeneralForumTopic", response(crate::True))]
pub struct EditGeneralForumTopicRequest {
    /// Unique identifier for the target chat or username of the target supergroup (in the format <code>@supergroupusername</code>)
    pub chat_id: ChatId,
    /// New topic name, 1-128 characters
    pub name: String,
}
/// Use this method to close an open 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "closeGeneralForumTopic", response(crate::True))]
pub struct CloseGeneralForumTopicRequest {
    /// Unique identifier for the target chat or username of the target supergroup (in the format <code>@supergroupusername</code>)
    pub chat_id: ChatId,
}
/// Use this method to reopen a closed 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights. The topic will be automatically unhidden if it was hidden. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "reopenGeneralForumTopic", response(crate::True))]
pub struct ReopenGeneralForumTopicRequest {
    /// Unique identifier for the target chat or username of the target supergroup (in the format <code>@supergroupusername</code>)
    pub chat_id: ChatId,
}
/// Use this method to hide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights. The topic will be automatically closed if it was open. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "hideGeneralForumTopic", response(crate::True))]
pub struct HideGeneralForumTopicRequest {
    /// Unique identifier for the target chat or username of the target supergroup (in the format <code>@supergroupusername</code>)
    pub chat_id: ChatId,
}
/// Use this method to unhide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "unhideGeneralForumTopic", response(crate::True))]
pub struct UnhideGeneralForumTopicRequest {
    /// Unique identifier for the target chat or username of the target supergroup (in the format <code>@supergroupusername</code>)
    pub chat_id: ChatId,
}
/// Use this method to clear the list of pinned messages in a General forum topic. The bot must be an administrator in the chat for this to work and must have the <em>can_pin_messages</em> administrator right in the supergroup. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "unpinAllGeneralForumTopicMessages", response(crate::True))]
pub struct UnpinAllGeneralForumTopicMessagesRequest {
    /// Unique identifier for the target chat or username of the target supergroup (in the format <code>@supergroupusername</code>)
    pub chat_id: ChatId,
}
/// Use this method to send answers to callback queries sent from <a href="/bots/features#inline-keyboards">inline keyboards</a>. The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, <em>True</em> is returned.
#[derive(macros::Method)]
#[method(name = "answerCallbackQuery", response(crate::True))]
pub struct AnswerCallbackQueryRequest {
    /// Unique identifier for the query to be answered
    pub callback_query_id: String,
    /// Text of the notification. If not specified, nothing will be shown to the user, 0-200 characters
    pub text: Option<String>,
    /// If <em>True</em>, an alert will be shown by the client instead of a notification at the top of the chat screen. Defaults to <em>false</em>.
    pub show_alert: Option<bool>,
    /// URL that will be opened by the user's client. If you have created a <a href="https://core.telegram.org/bots/api#game">Game</a> and accepted the conditions via <a href="https://t.me/botfather">@BotFather</a>, specify the URL that opens your game - note that this will only work if the query comes from a <a href="https://core.telegram.org/bots/api#inlinekeyboardbutton"><em>callback_game</em></a> button.<br><br>Otherwise, you may use links like <code>t.me/your_bot?start=XXXX</code> that open your bot with a parameter.
    pub url: Option<String>,
    /// The maximum amount of time in seconds that the result of the callback query may be cached client-side. Telegram apps will support caching starting in version 3.14. Defaults to 0.
    pub cache_time: Option<i64>,
}
/// Use this method to get the list of boosts added to a chat by a user. Requires administrator rights in the chat. Returns a <a href="https://core.telegram.org/bots/api#userchatboosts">UserChatBoosts</a> object.
#[derive(macros::Method)]
#[method(name = "getUserChatBoosts", response(UserChatBoosts))]
pub struct GetUserChatBoostsRequest {
    /// Unique identifier for the chat or username of the channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
}
/// Use this method to get information about the connection of the bot with a business account. Returns a <a href="https://core.telegram.org/bots/api#businessconnection">BusinessConnection</a> object on success.
#[derive(macros::Method)]
#[method(name = "getBusinessConnection", response(BusinessConnection))]
pub struct GetBusinessConnectionRequest {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
}
/// Use this method to change the list of the bot's commands. See <a href="/bots/features#commands">this manual</a> for more details about bot commands. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setMyCommands", response(crate::True))]
pub struct SetMyCommandsRequest {
    /// A JSON-serialized list of bot commands to be set as the list of the bot's commands. At most 100 commands can be specified.
    pub commands: Vec<BotCommand>,
    /// A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to <a href="https://core.telegram.org/bots/api#botcommandscopedefault">BotCommandScopeDefault</a>.
    pub scope: Option<BotCommandScope>,
    /// A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands
    pub language_code: Option<String>,
}
/// Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, <a href="https://core.telegram.org/bots/api#determining-list-of-commands">higher level commands</a> will be shown to affected users. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "deleteMyCommands", response(crate::True))]
pub struct DeleteMyCommandsRequest {
    /// A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to <a href="https://core.telegram.org/bots/api#botcommandscopedefault">BotCommandScopeDefault</a>.
    pub scope: Option<BotCommandScope>,
    /// A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands
    pub language_code: Option<String>,
}
/// Use this method to get the current list of the bot's commands for the given scope and user language. Returns an Array of <a href="https://core.telegram.org/bots/api#botcommand">BotCommand</a> objects. If commands aren't set, an empty list is returned.
#[derive(macros::Method)]
#[method(name = "getMyCommands", response(Vec<BotCommand>))]
pub struct GetMyCommandsRequest {
    /// A JSON-serialized object, describing scope of users. Defaults to <a href="https://core.telegram.org/bots/api#botcommandscopedefault">BotCommandScopeDefault</a>.
    pub scope: Option<BotCommandScope>,
    /// A two-letter ISO 639-1 language code or an empty string
    pub language_code: Option<String>,
}
/// Use this method to change the bot's name. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setMyName", response(crate::True))]
pub struct SetMyNameRequest {
    /// New bot name; 0-64 characters. Pass an empty string to remove the dedicated name for the given language.
    pub name: Option<String>,
    /// A two-letter ISO 639-1 language code. If empty, the name will be shown to all users for whose language there is no dedicated name.
    pub language_code: Option<String>,
}
/// Use this method to get the current bot name for the given user language. Returns <a href="https://core.telegram.org/bots/api#botname">BotName</a> on success.
#[derive(macros::Method)]
#[method(name = "getMyName", response(BotName))]
pub struct GetMyNameRequest {
    /// A two-letter ISO 639-1 language code or an empty string
    pub language_code: Option<String>,
}
/// Use this method to change the bot's description, which is shown in the chat with the bot if the chat is empty. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setMyDescription", response(crate::True))]
pub struct SetMyDescriptionRequest {
    /// New bot description; 0-512 characters. Pass an empty string to remove the dedicated description for the given language.
    pub description: Option<String>,
    /// A two-letter ISO 639-1 language code. If empty, the description will be applied to all users for whose language there is no dedicated description.
    pub language_code: Option<String>,
}
/// Use this method to get the current bot description for the given user language. Returns <a href="https://core.telegram.org/bots/api#botdescription">BotDescription</a> on success.
#[derive(macros::Method)]
#[method(name = "getMyDescription", response(BotDescription))]
pub struct GetMyDescriptionRequest {
    /// A two-letter ISO 639-1 language code or an empty string
    pub language_code: Option<String>,
}
/// Use this method to change the bot's short description, which is shown on the bot's profile page and is sent together with the link when users share the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setMyShortDescription", response(crate::True))]
pub struct SetMyShortDescriptionRequest {
    /// New short description for the bot; 0-120 characters. Pass an empty string to remove the dedicated short description for the given language.
    pub short_description: Option<String>,
    /// A two-letter ISO 639-1 language code. If empty, the short description will be applied to all users for whose language there is no dedicated short description.
    pub language_code: Option<String>,
}
/// Use this method to get the current bot short description for the given user language. Returns <a href="https://core.telegram.org/bots/api#botshortdescription">BotShortDescription</a> on success.
#[derive(macros::Method)]
#[method(name = "getMyShortDescription", response(BotShortDescription))]
pub struct GetMyShortDescriptionRequest {
    /// A two-letter ISO 639-1 language code or an empty string
    pub language_code: Option<String>,
}
/// Changes the profile photo of the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setMyProfilePhoto", response(crate::True))]
pub struct SetMyProfilePhotoRequest {
    /// The new profile photo to set
    pub photo: InputProfilePhoto,
}
/// Removes the profile photo of the bot. Requires no parameters. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "removeMyProfilePhoto", response(crate::True))]
pub struct RemoveMyProfilePhotoRequest;
/// Use this method to change the bot's menu button in a private chat, or the default menu button. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setChatMenuButton", response(crate::True))]
pub struct SetChatMenuButtonRequest {
    /// Unique identifier for the target private chat. If not specified, default bot's menu button will be changed
    pub chat_id: Option<i64>,
    /// A JSON-serialized object for the bot's new menu button. Defaults to <a href="https://core.telegram.org/bots/api#menubuttondefault">MenuButtonDefault</a>
    pub menu_button: Option<MenuButton>,
}
/// Use this method to get the current value of the bot's menu button in a private chat, or the default menu button. Returns <a href="https://core.telegram.org/bots/api#menubutton">MenuButton</a> on success.
#[derive(macros::Method)]
#[method(name = "getChatMenuButton", response(MenuButton))]
pub struct GetChatMenuButtonRequest {
    /// Unique identifier for the target private chat. If not specified, default bot's menu button will be returned
    pub chat_id: Option<i64>,
}
/// Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are free to modify the list before adding the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setMyDefaultAdministratorRights", response(crate::True))]
pub struct SetMyDefaultAdministratorRightsRequest {
    /// A JSON-serialized object describing new default administrator rights. If not specified, the default administrator rights will be cleared.
    pub rights: Option<ChatAdministratorRights>,
    /// Pass <em>True</em> to change the default administrator rights of the bot in channels. Otherwise, the default administrator rights of the bot for groups and supergroups will be changed.
    pub for_channels: Option<bool>,
}
/// Use this method to get the current default administrator rights of the bot. Returns <a href="https://core.telegram.org/bots/api#chatadministratorrights">ChatAdministratorRights</a> on success.
#[derive(macros::Method)]
#[method(name = "getMyDefaultAdministratorRights", response(ChatAdministratorRights))]
pub struct GetMyDefaultAdministratorRightsRequest {
    /// Pass <em>True</em> to get default administrator rights of the bot in channels. Otherwise, default administrator rights of the bot for groups and supergroups will be returned.
    pub for_channels: Option<bool>,
}
/// Returns the list of gifts that can be sent by the bot to users and channel chats. Requires no parameters. Returns a <a href="https://core.telegram.org/bots/api#gifts">Gifts</a> object.
#[derive(macros::Method)]
#[method(name = "getAvailableGifts", response(Gifts))]
pub struct GetAvailableGiftsRequest;
/// Sends a gift to the given user or channel chat. The gift can't be converted to Telegram Stars by the receiver. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "sendGift", response(crate::True))]
pub struct SendGiftRequest {
    /// Required if <em>chat_id</em> is not specified. Unique identifier of the target user who will receive the gift.
    pub user_id: Option<i64>,
    /// Required if <em>user_id</em> is not specified. Unique identifier for the chat or username of the channel (in the format <code>@channelusername</code>) that will receive the gift.
    pub chat_id: Option<ChatId>,
    /// Identifier of the gift; limited gifts can't be sent to channel chats
    pub gift_id: String,
    /// Pass <em>True</em> to pay for the gift upgrade from the bot's balance, thereby making the upgrade free for the receiver
    pub pay_for_upgrade: Option<bool>,
    /// Text that will be shown along with the gift; 0-128 characters
    pub text: Option<String>,
    /// Mode for parsing entities in the text. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details. Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom_emoji” are ignored.
    pub text_parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the gift text. It can be specified instead of <em>text_parse_mode</em>. Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom_emoji” are ignored.
    pub text_entities: Option<Vec<MessageEntity>>,
}
/// Gifts a Telegram Premium subscription to the given user. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "giftPremiumSubscription", response(crate::True))]
pub struct GiftPremiumSubscriptionRequest {
    /// Unique identifier of the target user who will receive a Telegram Premium subscription
    pub user_id: i64,
    /// Number of months the Telegram Premium subscription will be active for the user; must be one of 3, 6, or 12
    pub month_count: i64,
    /// Number of Telegram Stars to pay for the Telegram Premium subscription; must be 1000 for 3 months, 1500 for 6 months, and 2500 for 12 months
    pub star_count: i64,
    /// Text that will be shown along with the service message about the subscription; 0-128 characters
    pub text: Option<String>,
    /// Mode for parsing entities in the text. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details. Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom_emoji” are ignored.
    pub text_parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the gift text. It can be specified instead of <em>text_parse_mode</em>. Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom_emoji” are ignored.
    pub text_entities: Option<Vec<MessageEntity>>,
}
/// Verifies a user <a href="https://telegram.org/verify#third-party-verification">on behalf of the organization</a> which is represented by the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "verifyUser", response(crate::True))]
pub struct VerifyUserRequest {
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Custom description for the verification; 0-70 characters. Must be empty if the organization isn't allowed to provide a custom verification description.
    pub custom_description: Option<String>,
}
/// Verifies a chat <a href="https://telegram.org/verify#third-party-verification">on behalf of the organization</a> which is represented by the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "verifyChat", response(crate::True))]
pub struct VerifyChatRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>). Channel direct messages chats can't be verified.
    pub chat_id: ChatId,
    /// Custom description for the verification; 0-70 characters. Must be empty if the organization isn't allowed to provide a custom verification description.
    pub custom_description: Option<String>,
}
/// Removes verification from a user who is currently verified <a href="https://telegram.org/verify#third-party-verification">on behalf of the organization</a> represented by the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "removeUserVerification", response(crate::True))]
pub struct RemoveUserVerificationRequest {
    /// Unique identifier of the target user
    pub user_id: i64,
}
/// Removes verification from a chat that is currently verified <a href="https://telegram.org/verify#third-party-verification">on behalf of the organization</a> represented by the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "removeChatVerification", response(crate::True))]
pub struct RemoveChatVerificationRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
}
/// Marks incoming message as read on behalf of a business account. Requires the <em>can_read_messages</em> business bot right. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "readBusinessMessage", response(crate::True))]
pub struct ReadBusinessMessageRequest {
    /// Unique identifier of the business connection on behalf of which to read the message
    pub business_connection_id: String,
    /// Unique identifier of the chat in which the message was received. The chat must have been active in the last 24 hours.
    pub chat_id: i64,
    /// Unique identifier of the message to mark as read
    pub message_id: i64,
}
/// Delete messages on behalf of a business account. Requires the <em>can_delete_sent_messages</em> business bot right to delete messages sent by the bot itself, or the <em>can_delete_all_messages</em> business bot right to delete any message. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "deleteBusinessMessages", response(crate::True))]
pub struct DeleteBusinessMessagesRequest {
    /// Unique identifier of the business connection on behalf of which to delete the messages
    pub business_connection_id: String,
    /// A JSON-serialized list of 1-100 identifiers of messages to delete. All messages must be from the same chat. See <a href="https://core.telegram.org/bots/api#deletemessage">deleteMessage</a> for limitations on which messages can be deleted
    pub message_ids: Vec<i64>,
}
/// Changes the first and last name of a managed business account. Requires the <em>can_change_name</em> business bot right. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setBusinessAccountName", response(crate::True))]
pub struct SetBusinessAccountNameRequest {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// The new value of the first name for the business account; 1-64 characters
    pub first_name: String,
    /// The new value of the last name for the business account; 0-64 characters
    pub last_name: Option<String>,
}
/// Changes the username of a managed business account. Requires the <em>can_change_username</em> business bot right. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setBusinessAccountUsername", response(crate::True))]
pub struct SetBusinessAccountUsernameRequest {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// The new value of the username for the business account; 0-32 characters
    pub username: Option<String>,
}
/// Changes the bio of a managed business account. Requires the <em>can_change_bio</em> business bot right. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setBusinessAccountBio", response(crate::True))]
pub struct SetBusinessAccountBioRequest {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// The new value of the bio for the business account; 0-140 characters
    pub bio: Option<String>,
}
/// Changes the profile photo of a managed business account. Requires the <em>can_edit_profile_photo</em> business bot right. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setBusinessAccountProfilePhoto", response(crate::True))]
pub struct SetBusinessAccountProfilePhotoRequest {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// The new profile photo to set
    pub photo: InputProfilePhoto,
    /// Pass <em>True</em> to set the public photo, which will be visible even if the main photo is hidden by the business account's privacy settings. An account can have only one public photo.
    pub is_public: Option<bool>,
}
/// Removes the current profile photo of a managed business account. Requires the <em>can_edit_profile_photo</em> business bot right. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "removeBusinessAccountProfilePhoto", response(crate::True))]
pub struct RemoveBusinessAccountProfilePhotoRequest {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Pass <em>True</em> to remove the public photo, which is visible even if the main photo is hidden by the business account's privacy settings. After the main photo is removed, the previous profile photo (if present) becomes the main photo.
    pub is_public: Option<bool>,
}
/// Changes the privacy settings pertaining to incoming gifts in a managed business account. Requires the <em>can_change_gift_settings</em> business bot right. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setBusinessAccountGiftSettings", response(crate::True))]
pub struct SetBusinessAccountGiftSettingsRequest {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Pass <em>True</em>, if a button for sending a gift to the user or by the business account must always be shown in the input field
    pub show_gift_button: bool,
    /// Types of gifts accepted by the business account
    pub accepted_gift_types: AcceptedGiftTypes,
}
/// Returns the amount of Telegram Stars owned by a managed business account. Requires the <em>can_view_gifts_and_stars</em> business bot right. Returns <a href="https://core.telegram.org/bots/api#staramount">StarAmount</a> on success.
#[derive(macros::Method)]
#[method(name = "getBusinessAccountStarBalance", response(StarAmount))]
pub struct GetBusinessAccountStarBalanceRequest {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
}
/// Transfers Telegram Stars from the business account balance to the bot's balance. Requires the <em>can_transfer_stars</em> business bot right. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "transferBusinessAccountStars", response(crate::True))]
pub struct TransferBusinessAccountStarsRequest {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Number of Telegram Stars to transfer; 1-10000
    pub star_count: i64,
}
/// Returns the gifts received and owned by a managed business account. Requires the <em>can_view_gifts_and_stars</em> business bot right. Returns <a href="https://core.telegram.org/bots/api#ownedgifts">OwnedGifts</a> on success.
#[derive(macros::Method)]
#[method(name = "getBusinessAccountGifts", response(OwnedGifts))]
pub struct GetBusinessAccountGiftsRequest {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Pass <em>True</em> to exclude gifts that aren't saved to the account's profile page
    pub exclude_unsaved: Option<bool>,
    /// Pass <em>True</em> to exclude gifts that are saved to the account's profile page
    pub exclude_saved: Option<bool>,
    /// Pass <em>True</em> to exclude gifts that can be purchased an unlimited number of times
    pub exclude_unlimited: Option<bool>,
    /// Pass <em>True</em> to exclude gifts that can be purchased a limited number of times and can be upgraded to unique
    pub exclude_limited_upgradable: Option<bool>,
    /// Pass <em>True</em> to exclude gifts that can be purchased a limited number of times and can't be upgraded to unique
    pub exclude_limited_non_upgradable: Option<bool>,
    /// Pass <em>True</em> to exclude unique gifts
    pub exclude_unique: Option<bool>,
    /// Pass <em>True</em> to exclude gifts that were assigned from the TON blockchain and can't be resold or transferred in Telegram
    pub exclude_from_blockchain: Option<bool>,
    /// Pass <em>True</em> to sort results by gift price instead of send date. Sorting is applied before pagination.
    pub sort_by_price: Option<bool>,
    /// Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
    pub offset: Option<String>,
    /// The maximum number of gifts to be returned; 1-100. Defaults to 100
    pub limit: Option<i64>,
}
/// Returns the gifts owned and hosted by a user. Returns <a href="https://core.telegram.org/bots/api#ownedgifts">OwnedGifts</a> on success.
#[derive(macros::Method)]
#[method(name = "getUserGifts", response(OwnedGifts))]
pub struct GetUserGiftsRequest {
    /// Unique identifier of the user
    pub user_id: i64,
    /// Pass <em>True</em> to exclude gifts that can be purchased an unlimited number of times
    pub exclude_unlimited: Option<bool>,
    /// Pass <em>True</em> to exclude gifts that can be purchased a limited number of times and can be upgraded to unique
    pub exclude_limited_upgradable: Option<bool>,
    /// Pass <em>True</em> to exclude gifts that can be purchased a limited number of times and can't be upgraded to unique
    pub exclude_limited_non_upgradable: Option<bool>,
    /// Pass <em>True</em> to exclude gifts that were assigned from the TON blockchain and can't be resold or transferred in Telegram
    pub exclude_from_blockchain: Option<bool>,
    /// Pass <em>True</em> to exclude unique gifts
    pub exclude_unique: Option<bool>,
    /// Pass <em>True</em> to sort results by gift price instead of send date. Sorting is applied before pagination.
    pub sort_by_price: Option<bool>,
    /// Offset of the first entry to return as received from the previous request; use an empty string to get the first chunk of results
    pub offset: Option<String>,
    /// The maximum number of gifts to be returned; 1-100. Defaults to 100
    pub limit: Option<i64>,
}
/// Returns the gifts owned by a chat. Returns <a href="https://core.telegram.org/bots/api#ownedgifts">OwnedGifts</a> on success.
#[derive(macros::Method)]
#[method(name = "getChatGifts", response(OwnedGifts))]
pub struct GetChatGiftsRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Pass <em>True</em> to exclude gifts that aren't saved to the chat's profile page. Always <em>True</em>, unless the bot has the <em>can_post_messages</em> administrator right in the channel.
    pub exclude_unsaved: Option<bool>,
    /// Pass <em>True</em> to exclude gifts that are saved to the chat's profile page. Always <em>False</em>, unless the bot has the <em>can_post_messages</em> administrator right in the channel.
    pub exclude_saved: Option<bool>,
    /// Pass <em>True</em> to exclude gifts that can be purchased an unlimited number of times
    pub exclude_unlimited: Option<bool>,
    /// Pass <em>True</em> to exclude gifts that can be purchased a limited number of times and can be upgraded to unique
    pub exclude_limited_upgradable: Option<bool>,
    /// Pass <em>True</em> to exclude gifts that can be purchased a limited number of times and can't be upgraded to unique
    pub exclude_limited_non_upgradable: Option<bool>,
    /// Pass <em>True</em> to exclude gifts that were assigned from the TON blockchain and can't be resold or transferred in Telegram
    pub exclude_from_blockchain: Option<bool>,
    /// Pass <em>True</em> to exclude unique gifts
    pub exclude_unique: Option<bool>,
    /// Pass <em>True</em> to sort results by gift price instead of send date. Sorting is applied before pagination.
    pub sort_by_price: Option<bool>,
    /// Offset of the first entry to return as received from the previous request; use an empty string to get the first chunk of results
    pub offset: Option<String>,
    /// The maximum number of gifts to be returned; 1-100. Defaults to 100
    pub limit: Option<i64>,
}
/// Converts a given regular gift to Telegram Stars. Requires the <em>can_convert_gifts_to_stars</em> business bot right. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "convertGiftToStars", response(crate::True))]
pub struct ConvertGiftToStarsRequest {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Unique identifier of the regular gift that should be converted to Telegram Stars
    pub owned_gift_id: String,
}
/// Upgrades a given regular gift to a unique gift. Requires the <em>can_transfer_and_upgrade_gifts</em> business bot right. Additionally requires the <em>can_transfer_stars</em> business bot right if the upgrade is paid. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "upgradeGift", response(crate::True))]
pub struct UpgradeGiftRequest {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Unique identifier of the regular gift that should be upgraded to a unique one
    pub owned_gift_id: String,
    /// Pass <em>True</em> to keep the original gift text, sender and receiver in the upgraded gift
    pub keep_original_details: Option<bool>,
    /// The amount of Telegram Stars that will be paid for the upgrade from the business account balance. If <code>gift.prepaid_upgrade_star_count > 0</code>, then pass 0, otherwise, the <em>can_transfer_stars</em> business bot right is required and <code>gift.upgrade_star_count</code> must be passed.
    pub star_count: Option<i64>,
}
/// Transfers an owned unique gift to another user. Requires the <em>can_transfer_and_upgrade_gifts</em> business bot right. Requires <em>can_transfer_stars</em> business bot right if the transfer is paid. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "transferGift", response(crate::True))]
pub struct TransferGiftRequest {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Unique identifier of the regular gift that should be transferred
    pub owned_gift_id: String,
    /// Unique identifier of the chat which will own the gift. The chat must be active in the last 24 hours.
    pub new_owner_chat_id: i64,
    /// The amount of Telegram Stars that will be paid for the transfer from the business account balance. If positive, then the <em>can_transfer_stars</em> business bot right is required.
    pub star_count: Option<i64>,
}
/// Posts a story on behalf of a managed business account. Requires the <em>can_manage_stories</em> business bot right. Returns <a href="https://core.telegram.org/bots/api#story">Story</a> on success.
#[derive(macros::Method)]
#[method(name = "postStory", response(Story))]
pub struct PostStoryRequest {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Content of the story
    pub content: InputStoryContent,
    /// Period after which the story is moved to the archive, in seconds; must be one of <code>6 * 3600</code>, <code>12 * 3600</code>, <code>86400</code>, or <code>2 * 86400</code>
    pub active_period: i64,
    /// Caption of the story, 0-2048 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the story caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// A JSON-serialized list of clickable areas to be shown on the story
    pub areas: Option<Vec<StoryArea>>,
    /// Pass <em>True</em> to keep the story accessible after it expires
    pub post_to_chat_page: Option<bool>,
    /// Pass <em>True</em> if the content of the story must be protected from forwarding and screenshotting
    pub protect_content: Option<bool>,
}
/// Reposts a story on behalf of a business account from another business account. Both business accounts must be managed by the same bot, and the story on the source account must have been posted (or reposted) by the bot. Requires the <em>can_manage_stories</em> business bot right for both business accounts. Returns <a href="https://core.telegram.org/bots/api#story">Story</a> on success.
#[derive(macros::Method)]
#[method(name = "repostStory", response(Story))]
pub struct RepostStoryRequest {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Unique identifier of the chat which posted the story that should be reposted
    pub from_chat_id: i64,
    /// Unique identifier of the story that should be reposted
    pub from_story_id: i64,
    /// Period after which the story is moved to the archive, in seconds; must be one of <code>6 * 3600</code>, <code>12 * 3600</code>, <code>86400</code>, or <code>2 * 86400</code>
    pub active_period: i64,
    /// Pass <em>True</em> to keep the story accessible after it expires
    pub post_to_chat_page: Option<bool>,
    /// Pass <em>True</em> if the content of the story must be protected from forwarding and screenshotting
    pub protect_content: Option<bool>,
}
/// Edits a story previously posted by the bot on behalf of a managed business account. Requires the <em>can_manage_stories</em> business bot right. Returns <a href="https://core.telegram.org/bots/api#story">Story</a> on success.
#[derive(macros::Method)]
#[method(name = "editStory", response(Story))]
pub struct EditStoryRequest {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Unique identifier of the story to edit
    pub story_id: i64,
    /// Content of the story
    pub content: InputStoryContent,
    /// Caption of the story, 0-2048 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the story caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// A JSON-serialized list of clickable areas to be shown on the story
    pub areas: Option<Vec<StoryArea>>,
}
/// Deletes a story previously posted by the bot on behalf of a managed business account. Requires the <em>can_manage_stories</em> business bot right. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "deleteStory", response(crate::True))]
pub struct DeleteStoryRequest {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Unique identifier of the story to delete
    pub story_id: i64,
}
/// Use this method to edit text and <a href="https://core.telegram.org/bots/api#games">game</a> messages. On success, if the edited message is not an inline message, the edited <a href="https://core.telegram.org/bots/api#message">Message</a> is returned, otherwise <em>True</em> is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within <strong>48 hours</strong> from the time they were sent.
#[derive(macros::Method)]
#[method(name = "editMessageText", response(MessageOrTrue))]
pub struct EditMessageTextRequest {
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    pub business_connection_id: Option<String>,
    /// Required if <em>inline_message_id</em> is not specified. Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: Option<ChatId>,
    /// Required if <em>inline_message_id</em> is not specified. Identifier of the message to edit
    pub message_id: Option<i64>,
    /// Required if <em>chat_id</em> and <em>message_id</em> are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    /// New text of the message, 1-4096 characters after entities parsing
    pub text: String,
    /// Mode for parsing entities in the message text. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in message text, which can be specified instead of <em>parse_mode</em>
    pub entities: Option<Vec<MessageEntity>>,
    /// Link preview generation options for the message
    pub link_preview_options: Option<LinkPreviewOptions>,
    /// A JSON-serialized object for an <a href="/bots/features#inline-keyboards">inline keyboard</a>.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// Use this method to edit captions of messages. On success, if the edited message is not an inline message, the edited <a href="https://core.telegram.org/bots/api#message">Message</a> is returned, otherwise <em>True</em> is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within <strong>48 hours</strong> from the time they were sent.
#[derive(macros::Method)]
#[method(name = "editMessageCaption", response(MessageOrTrue))]
pub struct EditMessageCaptionRequest {
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    pub business_connection_id: Option<String>,
    /// Required if <em>inline_message_id</em> is not specified. Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: Option<ChatId>,
    /// Required if <em>inline_message_id</em> is not specified. Identifier of the message to edit
    pub message_id: Option<i64>,
    /// Required if <em>chat_id</em> and <em>message_id</em> are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    /// New caption of the message, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the message caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass <em>True</em>, if the caption must be shown above the message media. Supported only for animation, photo and video messages.
    pub show_caption_above_media: Option<bool>,
    /// A JSON-serialized object for an <a href="/bots/features#inline-keyboards">inline keyboard</a>.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// Use this method to edit animation, audio, document, photo, or video messages, or to add media to text messages. If a message is part of a message album, then it can be edited only to an audio for audio albums, only to a document for document albums and to a photo or a video otherwise. When an inline message is edited, a new file can't be uploaded; use a previously uploaded file via its file_id or specify a URL. On success, if the edited message is not an inline message, the edited <a href="https://core.telegram.org/bots/api#message">Message</a> is returned, otherwise <em>True</em> is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within <strong>48 hours</strong> from the time they were sent.
#[derive(macros::Method)]
#[method(name = "editMessageMedia", response(MessageOrTrue))]
pub struct EditMessageMediaRequest {
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    pub business_connection_id: Option<String>,
    /// Required if <em>inline_message_id</em> is not specified. Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: Option<ChatId>,
    /// Required if <em>inline_message_id</em> is not specified. Identifier of the message to edit
    pub message_id: Option<i64>,
    /// Required if <em>chat_id</em> and <em>message_id</em> are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    /// A JSON-serialized object for a new media content of the message
    pub media: InputMedia,
    /// A JSON-serialized object for a new <a href="/bots/features#inline-keyboards">inline keyboard</a>.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// Use this method to edit live location messages. A location can be edited until its <em>live_period</em> expires or editing is explicitly disabled by a call to <a href="https://core.telegram.org/bots/api#stopmessagelivelocation">stopMessageLiveLocation</a>. On success, if the edited message is not an inline message, the edited <a href="https://core.telegram.org/bots/api#message">Message</a> is returned, otherwise <em>True</em> is returned.
#[derive(macros::Method)]
#[method(name = "editMessageLiveLocation", response(MessageOrTrue))]
pub struct EditMessageLiveLocationRequest {
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    pub business_connection_id: Option<String>,
    /// Required if <em>inline_message_id</em> is not specified. Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: Option<ChatId>,
    /// Required if <em>inline_message_id</em> is not specified. Identifier of the message to edit
    pub message_id: Option<i64>,
    /// Required if <em>chat_id</em> and <em>message_id</em> are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    /// Latitude of new location
    pub latitude: f64,
    /// Longitude of new location
    pub longitude: f64,
    /// New period in seconds during which the location can be updated, starting from the message send date. If 0x7FFFFFFF is specified, then the location can be updated forever. Otherwise, the new value must not exceed the current <em>live_period</em> by more than a day, and the live location expiration date must remain within the next 90 days. If not specified, then <em>live_period</em> remains unchanged
    pub live_period: Option<i64>,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,
    /// Direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    pub heading: Option<i64>,
    /// The maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    pub proximity_alert_radius: Option<i64>,
    /// A JSON-serialized object for a new <a href="/bots/features#inline-keyboards">inline keyboard</a>.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// Use this method to stop updating a live location message before <em>live_period</em> expires. On success, if the message is not an inline message, the edited <a href="https://core.telegram.org/bots/api#message">Message</a> is returned, otherwise <em>True</em> is returned.
#[derive(macros::Method)]
#[method(name = "stopMessageLiveLocation", response(MessageOrTrue))]
pub struct StopMessageLiveLocationRequest {
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    pub business_connection_id: Option<String>,
    /// Required if <em>inline_message_id</em> is not specified. Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: Option<ChatId>,
    /// Required if <em>inline_message_id</em> is not specified. Identifier of the message with live location to stop
    pub message_id: Option<i64>,
    /// Required if <em>chat_id</em> and <em>message_id</em> are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    /// A JSON-serialized object for a new <a href="/bots/features#inline-keyboards">inline keyboard</a>.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// Use this method to edit a checklist on behalf of a connected business account. On success, the edited <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "editMessageChecklist", response(Message))]
pub struct EditMessageChecklistRequest {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: String,
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Unique identifier for the target message
    pub message_id: i64,
    /// A JSON-serialized object for the new checklist
    pub checklist: InputChecklist,
    /// A JSON-serialized object for the new <a href="/bots/features#inline-keyboards">inline keyboard</a> for the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// Use this method to edit only the reply markup of messages. On success, if the edited message is not an inline message, the edited <a href="https://core.telegram.org/bots/api#message">Message</a> is returned, otherwise <em>True</em> is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within <strong>48 hours</strong> from the time they were sent.
#[derive(macros::Method)]
#[method(name = "editMessageReplyMarkup", response(MessageOrTrue))]
pub struct EditMessageReplyMarkupRequest {
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    pub business_connection_id: Option<String>,
    /// Required if <em>inline_message_id</em> is not specified. Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: Option<ChatId>,
    /// Required if <em>inline_message_id</em> is not specified. Identifier of the message to edit
    pub message_id: Option<i64>,
    /// Required if <em>chat_id</em> and <em>message_id</em> are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    /// A JSON-serialized object for an <a href="/bots/features#inline-keyboards">inline keyboard</a>.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// Use this method to stop a poll which was sent by the bot. On success, the stopped <a href="https://core.telegram.org/bots/api#poll">Poll</a> is returned.
#[derive(macros::Method)]
#[method(name = "stopPoll", response(Poll))]
pub struct StopPollRequest {
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Identifier of the original message with the poll
    pub message_id: i64,
    /// A JSON-serialized object for a new message <a href="/bots/features#inline-keyboards">inline keyboard</a>.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// Use this method to approve a suggested post in a direct messages chat. The bot must have the 'can_post_messages' administrator right in the corresponding channel chat. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "approveSuggestedPost", response(crate::True))]
pub struct ApproveSuggestedPostRequest {
    /// Unique identifier for the target direct messages chat
    pub chat_id: i64,
    /// Identifier of a suggested post message to approve
    pub message_id: i64,
    /// Point in time (Unix timestamp) when the post is expected to be published; omit if the date has already been specified when the suggested post was created. If specified, then the date must be not more than 2678400 seconds (30 days) in the future
    pub send_date: Option<i64>,
}
/// Use this method to decline a suggested post in a direct messages chat. The bot must have the 'can_manage_direct_messages' administrator right in the corresponding channel chat. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "declineSuggestedPost", response(crate::True))]
pub struct DeclineSuggestedPostRequest {
    /// Unique identifier for the target direct messages chat
    pub chat_id: i64,
    /// Identifier of a suggested post message to decline
    pub message_id: i64,
    /// Comment for the creator of the suggested post; 0-128 characters
    pub comment: Option<String>,
}
/// Use this method to delete a message, including service messages, with the following limitations:<br>- A message can only be deleted if it was sent less than 48 hours ago.<br>- Service messages about a supergroup, channel, or forum topic creation can't be deleted.<br>- A dice message in a private chat can only be deleted if it was sent more than 24 hours ago.<br>- Bots can delete outgoing messages in private chats, groups, and supergroups.<br>- Bots can delete incoming messages in private chats.<br>- Bots granted <em>can_post_messages</em> permissions can delete outgoing messages in channels.<br>- If the bot is an administrator of a group, it can delete any message there.<br>- If the bot has <em>can_delete_messages</em> administrator right in a supergroup or a channel, it can delete any message there.<br>- If the bot has <em>can_manage_direct_messages</em> administrator right in a channel, it can delete any message in the corresponding direct messages chat.<br>Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "deleteMessage", response(crate::True))]
pub struct DeleteMessageRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Identifier of the message to delete
    pub message_id: i64,
}
/// Use this method to delete multiple messages simultaneously. If some of the specified messages can't be found, they are skipped. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "deleteMessages", response(crate::True))]
pub struct DeleteMessagesRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// A JSON-serialized list of 1-100 identifiers of messages to delete. See <a href="https://core.telegram.org/bots/api#deletemessage">deleteMessage</a> for limitations on which messages can be deleted
    pub message_ids: Vec<i64>,
}
/// This object represents a sticker.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Sticker {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Type of the sticker, currently one of “regular”, “mask”, “custom_emoji”. The type of the sticker is independent from its format, which is determined by the fields <em>is_animated</em> and <em>is_video</em>.
    pub r#type: String,
    /// Sticker width
    pub width: i64,
    /// Sticker height
    pub height: i64,
    /// <em>True</em>, if the sticker is <a href="https://telegram.org/blog/animated-stickers">animated</a>
    pub is_animated: bool,
    /// <em>True</em>, if the sticker is a <a href="https://telegram.org/blog/video-stickers-better-reactions">video sticker</a>
    pub is_video: bool,
    /// Sticker thumbnail in the .WEBP or .JPG format
    pub thumbnail: Option<PhotoSize>,
    /// Emoji associated with the sticker
    pub emoji: Option<String>,
    /// Name of the sticker set to which the sticker belongs
    pub set_name: Option<String>,
    /// For premium regular stickers, premium animation for the sticker
    pub premium_animation: Option<File>,
    /// For mask stickers, the position where the mask should be placed
    pub mask_position: Option<MaskPosition>,
    /// For custom emoji stickers, unique identifier of the custom emoji
    pub custom_emoji_id: Option<String>,
    /// <em>True</em>, if the sticker must be repainted to a text color in messages, the color of the Telegram Premium badge in emoji status, white color on chat photos, or another appropriate color in other places
    pub needs_repainting: Option<crate::True>,
    /// File size in bytes
    pub file_size: Option<i64>,
}
/// This object represents a sticker set.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StickerSet {
    /// Sticker set name
    pub name: String,
    /// Sticker set title
    pub title: String,
    /// Type of stickers in the set, currently one of “regular”, “mask”, “custom_emoji”
    pub sticker_type: String,
    /// List of all set stickers
    pub stickers: Vec<Sticker>,
    /// Sticker set thumbnail in the .WEBP, .TGS, or .WEBM format
    pub thumbnail: Option<PhotoSize>,
}
/// This object describes the position on faces where a mask should be placed by default.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MaskPosition {
    /// The part of the face relative to which the mask should be placed. One of “forehead”, “eyes”, “mouth”, or “chin”.
    pub point: String,
    /// Shift by X-axis measured in widths of the mask scaled to the face size, from left to right. For example, choosing -1.0 will place mask just to the left of the default mask position.
    pub x_shift: f64,
    /// Shift by Y-axis measured in heights of the mask scaled to the face size, from top to bottom. For example, 1.0 will place the mask just below the default mask position.
    pub y_shift: f64,
    /// Mask scaling coefficient. For example, 2.0 means double size.
    pub scale: f64,
}
/// This object describes a sticker to be added to a sticker set.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputSticker {
    /// The added sticker. Pass a <em>file_id</em> as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new file using multipart/form-data under <file_attach_name> name. Animated and video stickers can't be uploaded via HTTP URL. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub sticker: String,
    /// Format of the added sticker, must be one of “static” for a <strong>.WEBP</strong> or <strong>.PNG</strong> image, “animated” for a <strong>.TGS</strong> animation, “video” for a <strong>.WEBM</strong> video
    pub format: String,
    /// List of 1-20 emoji associated with the sticker
    pub emoji_list: Vec<String>,
    /// Position where the mask should be placed on faces. For “mask” stickers only.
    pub mask_position: Option<MaskPosition>,
    /// List of 0-20 search keywords for the sticker with total length of up to 64 characters. For “regular” and “custom_emoji” stickers only.
    pub keywords: Option<Vec<String>>,
}
/// Use this method to send static .WEBP, <a href="https://telegram.org/blog/animated-stickers">animated</a> .TGS, or <a href="https://telegram.org/blog/video-stickers-better-reactions">video</a> .WEBM stickers. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendSticker", response(Message))]
pub struct SendStickerRequest {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    pub direct_messages_topic_id: Option<i64>,
    /// Sticker to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a .WEBP sticker from the Internet, or upload a new .WEBP, .TGS, or .WEBM sticker using multipart/form-data. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>. Video and animated stickers can't be sent via an HTTP URL.
    pub sticker: Attachment,
    /// Emoji associated with the sticker; only for just uploaded stickers
    pub emoji: Option<String>,
    /// Sends the message <a href="https://telegram.org/blog/channels-2-0#silent-messages">silently</a>. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Pass <em>True</em> to allow up to 1000 messages per second, ignoring <a href="https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once">broadcasting limits</a> for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an <a href="/bots/features#inline-keyboards">inline keyboard</a>, <a href="/bots/features#keyboards">custom reply keyboard</a>, instructions to remove a reply keyboard or to force a reply from the user
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to get a sticker set. On success, a <a href="https://core.telegram.org/bots/api#stickerset">StickerSet</a> object is returned.
#[derive(macros::Method)]
#[method(name = "getStickerSet", response(StickerSet))]
pub struct GetStickerSetRequest {
    /// Name of the sticker set
    pub name: String,
}
/// Use this method to get information about custom emoji stickers by their identifiers. Returns an Array of <a href="https://core.telegram.org/bots/api#sticker">Sticker</a> objects.
#[derive(macros::Method)]
#[method(name = "getCustomEmojiStickers", response(Vec<Sticker>))]
pub struct GetCustomEmojiStickersRequest {
    /// A JSON-serialized list of custom emoji identifiers. At most 200 custom emoji identifiers can be specified.
    pub custom_emoji_ids: Vec<String>,
}
/// Use this method to upload a file with a sticker for later use in the <a href="https://core.telegram.org/bots/api#createnewstickerset">createNewStickerSet</a>, <a href="https://core.telegram.org/bots/api#addstickertoset">addStickerToSet</a>, or <a href="https://core.telegram.org/bots/api#replacestickerinset">replaceStickerInSet</a> methods (the file can be used multiple times). Returns the uploaded <a href="https://core.telegram.org/bots/api#file">File</a> on success.
#[derive(macros::Method)]
#[method(name = "uploadStickerFile", response(File))]
pub struct UploadStickerFileRequest {
    /// User identifier of sticker file owner
    pub user_id: i64,
    /// A file with the sticker in .WEBP, .PNG, .TGS, or .WEBM format. See <a href="/stickers"><a href="https://core.telegram.org/stickers">https://core.telegram.org/stickers</a></a> for technical requirements. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>
    pub sticker: InputFile,
    /// Format of the sticker, must be one of “static”, “animated”, “video”
    pub sticker_format: String,
}
/// Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "createNewStickerSet", response(crate::True))]
pub struct CreateNewStickerSetRequest {
    /// User identifier of created sticker set owner
    pub user_id: i64,
    /// Short name of sticker set, to be used in <code>t.me/addstickers/</code> URLs (e.g., <em>animals</em>). Can contain only English letters, digits and underscores. Must begin with a letter, can't contain consecutive underscores and must end in <code>"_by_<bot_username>"</code>. <code><bot_username></code> is case insensitive. 1-64 characters.
    pub name: String,
    /// Sticker set title, 1-64 characters
    pub title: String,
    /// A JSON-serialized list of 1-50 initial stickers to be added to the sticker set
    pub stickers: Vec<InputSticker>,
    /// Type of stickers in the set, pass “regular”, “mask”, or “custom_emoji”. By default, a regular sticker set is created.
    pub sticker_type: Option<String>,
    /// Pass <em>True</em> if stickers in the sticker set must be repainted to the color of text when used in messages, the accent color if used as emoji status, white on chat photos, or another appropriate color based on context; for custom emoji sticker sets only
    pub needs_repainting: Option<bool>,
}
/// Use this method to add a new sticker to a set created by the bot. Emoji sticker sets can have up to 200 stickers. Other sticker sets can have up to 120 stickers. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "addStickerToSet", response(crate::True))]
pub struct AddStickerToSetRequest {
    /// User identifier of sticker set owner
    pub user_id: i64,
    /// Sticker set name
    pub name: String,
    /// A JSON-serialized object with information about the added sticker. If exactly the same sticker had already been added to the set, then the set isn't changed.
    pub sticker: InputSticker,
}
/// Use this method to move a sticker in a set created by the bot to a specific position. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setStickerPositionInSet", response(crate::True))]
pub struct SetStickerPositionInSetRequest {
    /// File identifier of the sticker
    pub sticker: String,
    /// New sticker position in the set, zero-based
    pub position: i64,
}
/// Use this method to delete a sticker from a set created by the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "deleteStickerFromSet", response(crate::True))]
pub struct DeleteStickerFromSetRequest {
    /// File identifier of the sticker
    pub sticker: String,
}
/// Use this method to replace an existing sticker in a sticker set with a new one. The method is equivalent to calling <a href="https://core.telegram.org/bots/api#deletestickerfromset">deleteStickerFromSet</a>, then <a href="https://core.telegram.org/bots/api#addstickertoset">addStickerToSet</a>, then <a href="https://core.telegram.org/bots/api#setstickerpositioninset">setStickerPositionInSet</a>. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "replaceStickerInSet", response(crate::True))]
pub struct ReplaceStickerInSetRequest {
    /// User identifier of the sticker set owner
    pub user_id: i64,
    /// Sticker set name
    pub name: String,
    /// File identifier of the replaced sticker
    pub old_sticker: String,
    /// A JSON-serialized object with information about the added sticker. If exactly the same sticker had already been added to the set, then the set remains unchanged.
    pub sticker: InputSticker,
}
/// Use this method to change the list of emoji assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setStickerEmojiList", response(crate::True))]
pub struct SetStickerEmojiListRequest {
    /// File identifier of the sticker
    pub sticker: String,
    /// A JSON-serialized list of 1-20 emoji associated with the sticker
    pub emoji_list: Vec<String>,
}
/// Use this method to change search keywords assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setStickerKeywords", response(crate::True))]
pub struct SetStickerKeywordsRequest {
    /// File identifier of the sticker
    pub sticker: String,
    /// A JSON-serialized list of 0-20 search keywords for the sticker with total length of up to 64 characters
    pub keywords: Option<Vec<String>>,
}
/// Use this method to change the <a href="https://core.telegram.org/bots/api#maskposition">mask position</a> of a mask sticker. The sticker must belong to a sticker set that was created by the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setStickerMaskPosition", response(crate::True))]
pub struct SetStickerMaskPositionRequest {
    /// File identifier of the sticker
    pub sticker: String,
    /// A JSON-serialized object with the position where the mask should be placed on faces. Omit the parameter to remove the mask position.
    pub mask_position: Option<MaskPosition>,
}
/// Use this method to set the title of a created sticker set. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setStickerSetTitle", response(crate::True))]
pub struct SetStickerSetTitleRequest {
    /// Sticker set name
    pub name: String,
    /// Sticker set title, 1-64 characters
    pub title: String,
}
/// Use this method to set the thumbnail of a regular or mask sticker set. The format of the thumbnail file must match the format of the stickers in the set. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setStickerSetThumbnail", response(crate::True))]
pub struct SetStickerSetThumbnailRequest {
    /// Sticker set name
    pub name: String,
    /// User identifier of the sticker set owner
    pub user_id: i64,
    /// A <strong>.WEBP</strong> or <strong>.PNG</strong> image with the thumbnail, must be up to 128 kilobytes in size and have a width and height of exactly 100px, or a <strong>.TGS</strong> animation with a thumbnail up to 32 kilobytes in size (see <a href="/stickers#animation-requirements"><a href="https://core.telegram.org/stickers#animation-requirements">https://core.telegram.org/stickers#animation-requirements</a></a> for animated sticker technical requirements), or a <strong>.WEBM</strong> video with the thumbnail up to 32 kilobytes in size; see <a href="/stickers#video-requirements"><a href="https://core.telegram.org/stickers#video-requirements">https://core.telegram.org/stickers#video-requirements</a></a> for video sticker technical requirements. Pass a <em>file_id</em> as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. <a href="https://core.telegram.org/bots/api#sending-files">More information on Sending Files »</a>. Animated and video sticker set thumbnails can't be uploaded via HTTP URL. If omitted, then the thumbnail is dropped and the first sticker is used as the thumbnail.
    pub thumbnail: Option<Attachment>,
    /// Format of the thumbnail, must be one of “static” for a <strong>.WEBP</strong> or <strong>.PNG</strong> image, “animated” for a <strong>.TGS</strong> animation, or “video” for a <strong>.WEBM</strong> video
    pub format: String,
}
/// Use this method to set the thumbnail of a custom emoji sticker set. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setCustomEmojiStickerSetThumbnail", response(crate::True))]
pub struct SetCustomEmojiStickerSetThumbnailRequest {
    /// Sticker set name
    pub name: String,
    /// Custom emoji identifier of a sticker from the sticker set; pass an empty string to drop the thumbnail and use the first sticker as the thumbnail.
    pub custom_emoji_id: Option<String>,
}
/// Use this method to delete a sticker set that was created by the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "deleteStickerSet", response(crate::True))]
pub struct DeleteStickerSetRequest {
    /// Sticker set name
    pub name: String,
}
/// This object represents an incoming inline query. When the user sends an empty query, your bot could return some default or trending results.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Text of the query (up to 256 characters)
    pub query: String,
    /// Offset of the results to be returned, can be controlled by the bot
    pub offset: String,
    /// Type of the chat from which the inline query was sent. Can be either “sender” for a private chat with the inline query sender, “private”, “group”, “supergroup”, or “channel”. The chat type should be always known for requests sent from official clients and most third-party clients, unless the request was sent from a secret chat
    pub chat_type: Option<String>,
    /// Sender location, only for bots that request user location
    pub location: Option<Location>,
}
/// Use this method to send answers to an inline query. On success, <em>True</em> is returned.<br>No more than <strong>50</strong> results per query are allowed.
#[derive(macros::Method)]
#[method(name = "answerInlineQuery", response(crate::True))]
pub struct AnswerInlineQueryRequest {
    /// Unique identifier for the answered query
    pub inline_query_id: String,
    /// A JSON-serialized array of results for the inline query
    pub results: Vec<InlineQueryResult>,
    /// The maximum amount of time in seconds that the result of the inline query may be cached on the server. Defaults to 300.
    pub cache_time: Option<i64>,
    /// Pass <em>True</em> if results may be cached on the server side only for the user that sent the query. By default, results may be returned to any user who sends the same query.
    pub is_personal: Option<bool>,
    /// Pass the offset that a client should send in the next query with the same text to receive more results. Pass an empty string if there are no more results or if you don't support pagination. Offset length can't exceed 64 bytes.
    pub next_offset: Option<String>,
    /// A JSON-serialized object describing a button to be shown above inline query results
    pub button: Option<InlineQueryResultsButton>,
}
/// This object represents a button to be shown above inline query results. You <strong>must</strong> use exactly one of the optional fields.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultsButton {
    /// Label text on the button
    pub text: String,
    /// Description of the <a href="/bots/webapps">Web App</a> that will be launched when the user presses the button. The Web App will be able to switch back to the inline mode using the method <a href="/bots/webapps#initializing-mini-apps">switchInlineQuery</a> inside the Web App.
    pub web_app: Option<WebAppInfo>,
    /// <a href="/bots/features#deep-linking">Deep-linking</a> parameter for the /start message sent to the bot when a user presses the button. 1-64 characters, only <code>A-Z</code>, <code>a-z</code>, <code>0-9</code>, <code>_</code> and <code>-</code> are allowed.<br><br><em>Example:</em> An inline bot that sends YouTube videos can ask the user to connect the bot to their YouTube account to adapt search results accordingly. To do this, it displays a 'Connect your YouTube account' button above the results, or even before showing any. The user presses the button, switches to a private chat with the bot and, in doing so, passes a start parameter that instructs the bot to return an OAuth link. Once done, the bot can offer a <a href="https://core.telegram.org/bots/api#inlinekeyboardmarkup"><em>switch_inline</em></a> button so that the user can easily return to the chat where they wanted to use the bot's inline capabilities.
    pub start_parameter: Option<String>,
}
/// This object represents one result of an inline query.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum InlineQueryResult {
    CachedAudio(InlineQueryResultCachedAudio),
    CachedDocument(InlineQueryResultCachedDocument),
    CachedGif(InlineQueryResultCachedGif),
    CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
    CachedPhoto(InlineQueryResultCachedPhoto),
    CachedSticker(InlineQueryResultCachedSticker),
    CachedVideo(InlineQueryResultCachedVideo),
    CachedVoice(InlineQueryResultCachedVoice),
    Article(InlineQueryResultArticle),
    Audio(InlineQueryResultAudio),
    Contact(InlineQueryResultContact),
    Game(InlineQueryResultGame),
    Document(InlineQueryResultDocument),
    Gif(InlineQueryResultGif),
    Location(InlineQueryResultLocation),
    Mpeg4Gif(InlineQueryResultMpeg4Gif),
    Photo(InlineQueryResultPhoto),
    Venue(InlineQueryResultVenue),
    Video(InlineQueryResultVideo),
    Voice(InlineQueryResultVoice),
}
/// Represents a link to an article or web page.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultArticle {
    /// Type of the result, must be <em>article</em>
    pub r#type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Title of the result
    pub title: String,
    /// Content of the message to be sent
    pub input_message_content: InputMessageContent,
    /// <a href="/bots/features#inline-keyboards">Inline keyboard</a> attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// URL of the result
    pub url: Option<String>,
    /// Short description of the result
    pub description: Option<String>,
    /// Url of the thumbnail for the result
    pub thumbnail_url: Option<String>,
    /// Thumbnail width
    pub thumbnail_width: Option<i64>,
    /// Thumbnail height
    pub thumbnail_height: Option<i64>,
}
/// Represents a link to a photo. By default, this photo will be sent by the user with optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the photo.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultPhoto {
    /// Type of the result, must be <em>photo</em>
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL of the photo. Photo must be in <strong>JPEG</strong> format. Photo size must not exceed 5MB
    pub photo_url: String,
    /// URL of the thumbnail for the photo
    pub thumbnail_url: String,
    /// Width of the photo
    pub photo_width: Option<i64>,
    /// Height of the photo
    pub photo_height: Option<i64>,
    /// Title for the result
    pub title: Option<String>,
    /// Short description of the result
    pub description: Option<String>,
    /// Caption of the photo to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass <em>True</em>, if the caption must be shown above the message media
    pub show_caption_above_media: Option<bool>,
    /// <a href="/bots/features#inline-keyboards">Inline keyboard</a> attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the photo
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to an animated GIF file. By default, this animated GIF file will be sent by the user with optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the animation.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultGif {
    /// Type of the result, must be <em>gif</em>
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the GIF file
    pub gif_url: String,
    /// Width of the GIF
    pub gif_width: Option<i64>,
    /// Height of the GIF
    pub gif_height: Option<i64>,
    /// Duration of the GIF in seconds
    pub gif_duration: Option<i64>,
    /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
    pub thumbnail_url: String,
    /// MIME type of the thumbnail, must be one of “image/jpeg”, “image/gif”, or “video/mp4”. Defaults to “image/jpeg”
    pub thumbnail_mime_type: Option<String>,
    /// Title for the result
    pub title: Option<String>,
    /// Caption of the GIF file to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass <em>True</em>, if the caption must be shown above the message media
    pub show_caption_above_media: Option<bool>,
    /// <a href="/bots/features#inline-keyboards">Inline keyboard</a> attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the GIF animation
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default, this animated MPEG-4 file will be sent by the user with optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the animation.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultMpeg4Gif {
    /// Type of the result, must be <em>mpeg4_gif</em>
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the MPEG4 file
    pub mpeg4_url: String,
    /// Video width
    pub mpeg4_width: Option<i64>,
    /// Video height
    pub mpeg4_height: Option<i64>,
    /// Video duration in seconds
    pub mpeg4_duration: Option<i64>,
    /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
    pub thumbnail_url: String,
    /// MIME type of the thumbnail, must be one of “image/jpeg”, “image/gif”, or “video/mp4”. Defaults to “image/jpeg”
    pub thumbnail_mime_type: Option<String>,
    /// Title for the result
    pub title: Option<String>,
    /// Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass <em>True</em>, if the caption must be shown above the message media
    pub show_caption_above_media: Option<bool>,
    /// <a href="/bots/features#inline-keyboards">Inline keyboard</a> attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the video animation
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to a page containing an embedded video player or a video file. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the video.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultVideo {
    /// Type of the result, must be <em>video</em>
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the embedded video player or video file
    pub video_url: String,
    /// MIME type of the content of the video URL, “text/html” or “video/mp4”
    pub mime_type: String,
    /// URL of the thumbnail (JPEG only) for the video
    pub thumbnail_url: String,
    /// Title for the result
    pub title: String,
    /// Caption of the video to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the video caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass <em>True</em>, if the caption must be shown above the message media
    pub show_caption_above_media: Option<bool>,
    /// Video width
    pub video_width: Option<i64>,
    /// Video height
    pub video_height: Option<i64>,
    /// Video duration in seconds
    pub video_duration: Option<i64>,
    /// Short description of the result
    pub description: Option<String>,
    /// <a href="/bots/features#inline-keyboards">Inline keyboard</a> attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the video. This field is <strong>required</strong> if InlineQueryResultVideo is used to send an HTML-page as a result (e.g., a YouTube video).
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to an MP3 audio file. By default, this audio file will be sent by the user. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the audio.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultAudio {
    /// Type of the result, must be <em>audio</em>
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the audio file
    pub audio_url: String,
    /// Title
    pub title: String,
    /// Caption, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the audio caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Performer
    pub performer: Option<String>,
    /// Audio duration in seconds
    pub audio_duration: Option<i64>,
    /// <a href="/bots/features#inline-keyboards">Inline keyboard</a> attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the audio
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to a voice recording in an .OGG container encoded with OPUS. By default, this voice recording will be sent by the user. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the the voice message.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultVoice {
    /// Type of the result, must be <em>voice</em>
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the voice recording
    pub voice_url: String,
    /// Recording title
    pub title: String,
    /// Caption, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the voice message caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Recording duration in seconds
    pub voice_duration: Option<i64>,
    /// <a href="/bots/features#inline-keyboards">Inline keyboard</a> attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the voice recording
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to a file. By default, this file will be sent by the user with an optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the file. Currently, only <strong>.PDF</strong> and <strong>.ZIP</strong> files can be sent using this method.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultDocument {
    /// Type of the result, must be <em>document</em>
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Title for the result
    pub title: String,
    /// Caption of the document to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the document caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// A valid URL for the file
    pub document_url: String,
    /// MIME type of the content of the file, either “application/pdf” or “application/zip”
    pub mime_type: String,
    /// Short description of the result
    pub description: Option<String>,
    /// <a href="/bots/features#inline-keyboards">Inline keyboard</a> attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the file
    pub input_message_content: Option<InputMessageContent>,
    /// URL of the thumbnail (JPEG only) for the file
    pub thumbnail_url: Option<String>,
    /// Thumbnail width
    pub thumbnail_width: Option<i64>,
    /// Thumbnail height
    pub thumbnail_height: Option<i64>,
}
/// Represents a location on a map. By default, the location will be sent by the user. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the location.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultLocation {
    /// Type of the result, must be <em>location</em>
    pub r#type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Location latitude in degrees
    pub latitude: f64,
    /// Location longitude in degrees
    pub longitude: f64,
    /// Location title
    pub title: String,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,
    /// Period in seconds during which the location can be updated, should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely.
    pub live_period: Option<i64>,
    /// For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    pub heading: Option<i64>,
    /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    pub proximity_alert_radius: Option<i64>,
    /// <a href="/bots/features#inline-keyboards">Inline keyboard</a> attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the location
    pub input_message_content: Option<InputMessageContent>,
    /// Url of the thumbnail for the result
    pub thumbnail_url: Option<String>,
    /// Thumbnail width
    pub thumbnail_width: Option<i64>,
    /// Thumbnail height
    pub thumbnail_height: Option<i64>,
}
/// Represents a venue. By default, the venue will be sent by the user. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the venue.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultVenue {
    /// Type of the result, must be <em>venue</em>
    pub r#type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Latitude of the venue location in degrees
    pub latitude: f64,
    /// Longitude of the venue location in degrees
    pub longitude: f64,
    /// Title of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue if known
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue
    pub google_place_id: Option<String>,
    /// Google Places type of the venue. (See <a href="https://developers.google.com/places/web-service/supported_types">supported types</a>.)
    pub google_place_type: Option<String>,
    /// <a href="/bots/features#inline-keyboards">Inline keyboard</a> attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the venue
    pub input_message_content: Option<InputMessageContent>,
    /// Url of the thumbnail for the result
    pub thumbnail_url: Option<String>,
    /// Thumbnail width
    pub thumbnail_width: Option<i64>,
    /// Thumbnail height
    pub thumbnail_height: Option<i64>,
}
/// Represents a contact with a phone number. By default, this contact will be sent by the user. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the contact.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultContact {
    /// Type of the result, must be <em>contact</em>
    pub r#type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    pub last_name: Option<String>,
    /// Additional data about the contact in the form of a <a href="https://en.wikipedia.org/wiki/VCard">vCard</a>, 0-2048 bytes
    pub vcard: Option<String>,
    /// <a href="/bots/features#inline-keyboards">Inline keyboard</a> attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the contact
    pub input_message_content: Option<InputMessageContent>,
    /// Url of the thumbnail for the result
    pub thumbnail_url: Option<String>,
    /// Thumbnail width
    pub thumbnail_width: Option<i64>,
    /// Thumbnail height
    pub thumbnail_height: Option<i64>,
}
/// Represents a <a href="https://core.telegram.org/bots/api#games">Game</a>.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultGame {
    /// Type of the result, must be <em>game</em>
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Short name of the game
    pub game_short_name: String,
    /// <a href="/bots/features#inline-keyboards">Inline keyboard</a> attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// Represents a link to a photo stored on the Telegram servers. By default, this photo will be sent by the user with an optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the photo.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultCachedPhoto {
    /// Type of the result, must be <em>photo</em>
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier of the photo
    pub photo_file_id: String,
    /// Title for the result
    pub title: Option<String>,
    /// Short description of the result
    pub description: Option<String>,
    /// Caption of the photo to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass <em>True</em>, if the caption must be shown above the message media
    pub show_caption_above_media: Option<bool>,
    /// <a href="/bots/features#inline-keyboards">Inline keyboard</a> attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the photo
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to an animated GIF file stored on the Telegram servers. By default, this animated GIF file will be sent by the user with an optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with specified content instead of the animation.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultCachedGif {
    /// Type of the result, must be <em>gif</em>
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the GIF file
    pub gif_file_id: String,
    /// Title for the result
    pub title: Option<String>,
    /// Caption of the GIF file to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass <em>True</em>, if the caption must be shown above the message media
    pub show_caption_above_media: Option<bool>,
    /// <a href="/bots/features#inline-keyboards">Inline keyboard</a> attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the GIF animation
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers. By default, this animated MPEG-4 file will be sent by the user with an optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the animation.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultCachedMpeg4Gif {
    /// Type of the result, must be <em>mpeg4_gif</em>
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the MPEG4 file
    pub mpeg4_file_id: String,
    /// Title for the result
    pub title: Option<String>,
    /// Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass <em>True</em>, if the caption must be shown above the message media
    pub show_caption_above_media: Option<bool>,
    /// <a href="/bots/features#inline-keyboards">Inline keyboard</a> attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the video animation
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to a sticker stored on the Telegram servers. By default, this sticker will be sent by the user. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the sticker.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultCachedSticker {
    /// Type of the result, must be <em>sticker</em>
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier of the sticker
    pub sticker_file_id: String,
    /// <a href="/bots/features#inline-keyboards">Inline keyboard</a> attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the sticker
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to a file stored on the Telegram servers. By default, this file will be sent by the user with an optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the file.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultCachedDocument {
    /// Type of the result, must be <em>document</em>
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Title for the result
    pub title: String,
    /// A valid file identifier for the file
    pub document_file_id: String,
    /// Short description of the result
    pub description: Option<String>,
    /// Caption of the document to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the document caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// <a href="/bots/features#inline-keyboards">Inline keyboard</a> attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the file
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to a video file stored on the Telegram servers. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the video.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultCachedVideo {
    /// Type of the result, must be <em>video</em>
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the video file
    pub video_file_id: String,
    /// Title for the result
    pub title: String,
    /// Short description of the result
    pub description: Option<String>,
    /// Caption of the video to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the video caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass <em>True</em>, if the caption must be shown above the message media
    pub show_caption_above_media: Option<bool>,
    /// <a href="/bots/features#inline-keyboards">Inline keyboard</a> attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the video
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to a voice message stored on the Telegram servers. By default, this voice message will be sent by the user. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the voice message.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultCachedVoice {
    /// Type of the result, must be <em>voice</em>
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the voice message
    pub voice_file_id: String,
    /// Voice message title
    pub title: String,
    /// Caption, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the voice message caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// <a href="/bots/features#inline-keyboards">Inline keyboard</a> attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the voice message
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to an MP3 audio file stored on the Telegram servers. By default, this audio file will be sent by the user. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the audio.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultCachedAudio {
    /// Type of the result, must be <em>audio</em>
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the audio file
    pub audio_file_id: String,
    /// Caption, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the audio caption. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of <em>parse_mode</em>
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// <a href="/bots/features#inline-keyboards">Inline keyboard</a> attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the audio
    pub input_message_content: Option<InputMessageContent>,
}
/// This object represents the content of a message to be sent as a result of an inline query.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    InputTextMessageContent(InputTextMessageContent),
    InputLocationMessageContent(InputLocationMessageContent),
    InputVenueMessageContent(InputVenueMessageContent),
    InputContactMessageContent(InputContactMessageContent),
    InputInvoiceMessageContent(InputInvoiceMessageContent),
}
/// Represents the <a href="https://core.telegram.org/bots/api#inputmessagecontent">content</a> of a text message to be sent as the result of an inline query.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputTextMessageContent {
    /// Text of the message to be sent, 1-4096 characters
    pub message_text: String,
    /// Mode for parsing entities in the message text. See <a href="https://core.telegram.org/bots/api#formatting-options">formatting options</a> for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in message text, which can be specified instead of <em>parse_mode</em>
    pub entities: Option<Vec<MessageEntity>>,
    /// Link preview generation options for the message
    pub link_preview_options: Option<LinkPreviewOptions>,
}
/// Represents the <a href="https://core.telegram.org/bots/api#inputmessagecontent">content</a> of a location message to be sent as the result of an inline query.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputLocationMessageContent {
    /// Latitude of the location in degrees
    pub latitude: f64,
    /// Longitude of the location in degrees
    pub longitude: f64,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,
    /// Period in seconds during which the location can be updated, should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely.
    pub live_period: Option<i64>,
    /// For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    pub heading: Option<i64>,
    /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    pub proximity_alert_radius: Option<i64>,
}
/// Represents the <a href="https://core.telegram.org/bots/api#inputmessagecontent">content</a> of a venue message to be sent as the result of an inline query.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputVenueMessageContent {
    /// Latitude of the venue in degrees
    pub latitude: f64,
    /// Longitude of the venue in degrees
    pub longitude: f64,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue, if known
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue
    pub google_place_id: Option<String>,
    /// Google Places type of the venue. (See <a href="https://developers.google.com/places/web-service/supported_types">supported types</a>.)
    pub google_place_type: Option<String>,
}
/// Represents the <a href="https://core.telegram.org/bots/api#inputmessagecontent">content</a> of a contact message to be sent as the result of an inline query.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputContactMessageContent {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    pub last_name: Option<String>,
    /// Additional data about the contact in the form of a <a href="https://en.wikipedia.org/wiki/VCard">vCard</a>, 0-2048 bytes
    pub vcard: Option<String>,
}
/// Represents the <a href="https://core.telegram.org/bots/api#inputmessagecontent">content</a> of an invoice message to be sent as the result of an inline query.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputInvoiceMessageContent {
    /// Product name, 1-32 characters
    pub title: String,
    /// Product description, 1-255 characters
    pub description: String,
    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use it for your internal processes.
    pub payload: String,
    /// Payment provider token, obtained via <a href="https://t.me/botfather">@BotFather</a>. Pass an empty string for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub provider_token: Option<String>,
    /// Three-letter ISO 4217 currency code, see <a href="/bots/payments#supported-currencies">more on currencies</a>. Pass “XTR” for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub currency: String,
    /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.). Must contain exactly one item for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub prices: Vec<LabeledPrice>,
    /// The maximum accepted amount for tips in the <em>smallest units</em> of the currency (integer, <strong>not</strong> float/double). For example, for a maximum tip of <code>US$ 1.45</code> pass <code>max_tip_amount = 145</code>. See the <em>exp</em> parameter in <a href="/bots/payments/currencies.json">currencies.json</a>, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub max_tip_amount: Option<i64>,
    /// A JSON-serialized array of suggested amounts of tip in the <em>smallest units</em> of the currency (integer, <strong>not</strong> float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed <em>max_tip_amount</em>.
    pub suggested_tip_amounts: Option<Vec<i64>>,
    /// A JSON-serialized object for data about the invoice, which will be shared with the payment provider. A detailed description of the required fields should be provided by the payment provider.
    pub provider_data: Option<String>,
    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
    pub photo_url: Option<String>,
    /// Photo size in bytes
    pub photo_size: Option<i64>,
    /// Photo width
    pub photo_width: Option<i64>,
    /// Photo height
    pub photo_height: Option<i64>,
    /// Pass <em>True</em> if you require the user's full name to complete the order. Ignored for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub need_name: Option<bool>,
    /// Pass <em>True</em> if you require the user's phone number to complete the order. Ignored for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub need_phone_number: Option<bool>,
    /// Pass <em>True</em> if you require the user's email address to complete the order. Ignored for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub need_email: Option<bool>,
    /// Pass <em>True</em> if you require the user's shipping address to complete the order. Ignored for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub need_shipping_address: Option<bool>,
    /// Pass <em>True</em> if the user's phone number should be sent to the provider. Ignored for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub send_phone_number_to_provider: Option<bool>,
    /// Pass <em>True</em> if the user's email address should be sent to the provider. Ignored for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub send_email_to_provider: Option<bool>,
    /// Pass <em>True</em> if the final price depends on the shipping method. Ignored for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub is_flexible: Option<bool>,
}
/// Represents a <a href="https://core.telegram.org/bots/api#inlinequeryresult">result</a> of an inline query that was chosen by the user and sent to their chat partner.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChosenInlineResult {
    /// The unique identifier for the result that was chosen
    pub result_id: String,
    /// The user that chose the result
    pub from: User,
    /// Sender location, only for bots that require user location
    pub location: Option<Location>,
    /// Identifier of the sent inline message. Available only if there is an <a href="https://core.telegram.org/bots/api#inlinekeyboardmarkup">inline keyboard</a> attached to the message. Will be also received in <a href="https://core.telegram.org/bots/api#callbackquery">callback queries</a> and can be used to <a href="https://core.telegram.org/bots/api#updating-messages">edit</a> the message.
    pub inline_message_id: Option<String>,
    /// The query that was used to obtain the result
    pub query: String,
}
/// Use this method to set the result of an interaction with a <a href="/bots/webapps">Web App</a> and send a corresponding message on behalf of the user to the chat from which the query originated. On success, a <a href="https://core.telegram.org/bots/api#sentwebappmessage">SentWebAppMessage</a> object is returned.
#[derive(macros::Method)]
#[method(name = "answerWebAppQuery", response(SentWebAppMessage))]
pub struct AnswerWebAppQueryRequest {
    /// Unique identifier for the query to be answered
    pub web_app_query_id: String,
    /// A JSON-serialized object describing the message to be sent
    pub result: InlineQueryResult,
}
/// Describes an inline message sent by a <a href="/bots/webapps">Web App</a> on behalf of a user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SentWebAppMessage {
    /// Identifier of the sent inline message. Available only if there is an <a href="https://core.telegram.org/bots/api#inlinekeyboardmarkup">inline keyboard</a> attached to the message.
    pub inline_message_id: Option<String>,
}
/// Stores a message that can be sent by a user of a Mini App. Returns a <a href="https://core.telegram.org/bots/api#preparedinlinemessage">PreparedInlineMessage</a> object.
#[derive(macros::Method)]
#[method(name = "savePreparedInlineMessage", response(PreparedInlineMessage))]
pub struct SavePreparedInlineMessageRequest {
    /// Unique identifier of the target user that can use the prepared message
    pub user_id: i64,
    /// A JSON-serialized object describing the message to be sent
    pub result: InlineQueryResult,
    /// Pass <em>True</em> if the message can be sent to private chats with users
    pub allow_user_chats: Option<bool>,
    /// Pass <em>True</em> if the message can be sent to private chats with bots
    pub allow_bot_chats: Option<bool>,
    /// Pass <em>True</em> if the message can be sent to group and supergroup chats
    pub allow_group_chats: Option<bool>,
    /// Pass <em>True</em> if the message can be sent to channel chats
    pub allow_channel_chats: Option<bool>,
}
/// Describes an inline message to be sent by a user of a Mini App.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PreparedInlineMessage {
    /// Unique identifier of the prepared message
    pub id: String,
    /// Expiration date of the prepared message, in Unix time. Expired prepared messages can no longer be used
    pub expiration_date: i64,
}
/// Use this method to send invoices. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendInvoice", response(Message))]
pub struct SendInvoiceRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format <code>@channelusername</code>)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    pub direct_messages_topic_id: Option<i64>,
    /// Product name, 1-32 characters
    pub title: String,
    /// Product description, 1-255 characters
    pub description: String,
    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use it for your internal processes.
    pub payload: String,
    /// Payment provider token, obtained via <a href="https://t.me/botfather">@BotFather</a>. Pass an empty string for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub provider_token: Option<String>,
    /// Three-letter ISO 4217 currency code, see <a href="/bots/payments#supported-currencies">more on currencies</a>. Pass “XTR” for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub currency: String,
    /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.). Must contain exactly one item for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub prices: Vec<LabeledPrice>,
    /// The maximum accepted amount for tips in the <em>smallest units</em> of the currency (integer, <strong>not</strong> float/double). For example, for a maximum tip of <code>US$ 1.45</code> pass <code>max_tip_amount = 145</code>. See the <em>exp</em> parameter in <a href="/bots/payments/currencies.json">currencies.json</a>, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub max_tip_amount: Option<i64>,
    /// A JSON-serialized array of suggested amounts of tips in the <em>smallest units</em> of the currency (integer, <strong>not</strong> float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed <em>max_tip_amount</em>.
    pub suggested_tip_amounts: Option<Vec<i64>>,
    /// Unique deep-linking parameter. If left empty, <strong>forwarded copies</strong> of the sent message will have a <em>Pay</em> button, allowing multiple users to pay directly from the forwarded message, using the same invoice. If non-empty, forwarded copies of the sent message will have a <em>URL</em> button with a deep link to the bot (instead of a <em>Pay</em> button), with the value used as the start parameter
    pub start_parameter: Option<String>,
    /// JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
    pub provider_data: Option<String>,
    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service. People like it better when they see what they are paying for.
    pub photo_url: Option<String>,
    /// Photo size in bytes
    pub photo_size: Option<i64>,
    /// Photo width
    pub photo_width: Option<i64>,
    /// Photo height
    pub photo_height: Option<i64>,
    /// Pass <em>True</em> if you require the user's full name to complete the order. Ignored for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub need_name: Option<bool>,
    /// Pass <em>True</em> if you require the user's phone number to complete the order. Ignored for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub need_phone_number: Option<bool>,
    /// Pass <em>True</em> if you require the user's email address to complete the order. Ignored for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub need_email: Option<bool>,
    /// Pass <em>True</em> if you require the user's shipping address to complete the order. Ignored for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub need_shipping_address: Option<bool>,
    /// Pass <em>True</em> if the user's phone number should be sent to the provider. Ignored for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub send_phone_number_to_provider: Option<bool>,
    /// Pass <em>True</em> if the user's email address should be sent to the provider. Ignored for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub send_email_to_provider: Option<bool>,
    /// Pass <em>True</em> if the final price depends on the shipping method. Ignored for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub is_flexible: Option<bool>,
    /// Sends the message <a href="https://telegram.org/blog/channels-2-0#silent-messages">silently</a>. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Pass <em>True</em> to allow up to 1000 messages per second, ignoring <a href="https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once">broadcasting limits</a> for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
    /// A JSON-serialized object for an <a href="/bots/features#inline-keyboards">inline keyboard</a>. If empty, one 'Pay <code>total price</code>' button will be shown. If not empty, the first button must be a Pay button.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// Use this method to create a link for an invoice. Returns the created invoice link as <em>String</em> on success.
#[derive(macros::Method)]
#[method(name = "createInvoiceLink", response(String))]
pub struct CreateInvoiceLinkRequest {
    /// Unique identifier of the business connection on behalf of which the link will be created. For payments in <a href="https://t.me/BotNews/90">Telegram Stars</a> only.
    pub business_connection_id: Option<String>,
    /// Product name, 1-32 characters
    pub title: String,
    /// Product description, 1-255 characters
    pub description: String,
    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use it for your internal processes.
    pub payload: String,
    /// Payment provider token, obtained via <a href="https://t.me/botfather">@BotFather</a>. Pass an empty string for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub provider_token: Option<String>,
    /// Three-letter ISO 4217 currency code, see <a href="/bots/payments#supported-currencies">more on currencies</a>. Pass “XTR” for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub currency: String,
    /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.). Must contain exactly one item for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub prices: Vec<LabeledPrice>,
    /// The number of seconds the subscription will be active for before the next payment. The currency must be set to “XTR” (Telegram Stars) if the parameter is used. Currently, it must always be 2592000 (30 days) if specified. Any number of subscriptions can be active for a given bot at the same time, including multiple concurrent subscriptions from the same user. Subscription price must no exceed 10000 Telegram Stars.
    pub subscription_period: Option<i64>,
    /// The maximum accepted amount for tips in the <em>smallest units</em> of the currency (integer, <strong>not</strong> float/double). For example, for a maximum tip of <code>US$ 1.45</code> pass <code>max_tip_amount = 145</code>. See the <em>exp</em> parameter in <a href="/bots/payments/currencies.json">currencies.json</a>, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub max_tip_amount: Option<i64>,
    /// A JSON-serialized array of suggested amounts of tips in the <em>smallest units</em> of the currency (integer, <strong>not</strong> float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed <em>max_tip_amount</em>.
    pub suggested_tip_amounts: Option<Vec<i64>>,
    /// JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
    pub provider_data: Option<String>,
    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
    pub photo_url: Option<String>,
    /// Photo size in bytes
    pub photo_size: Option<i64>,
    /// Photo width
    pub photo_width: Option<i64>,
    /// Photo height
    pub photo_height: Option<i64>,
    /// Pass <em>True</em> if you require the user's full name to complete the order. Ignored for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub need_name: Option<bool>,
    /// Pass <em>True</em> if you require the user's phone number to complete the order. Ignored for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub need_phone_number: Option<bool>,
    /// Pass <em>True</em> if you require the user's email address to complete the order. Ignored for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub need_email: Option<bool>,
    /// Pass <em>True</em> if you require the user's shipping address to complete the order. Ignored for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub need_shipping_address: Option<bool>,
    /// Pass <em>True</em> if the user's phone number should be sent to the provider. Ignored for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub send_phone_number_to_provider: Option<bool>,
    /// Pass <em>True</em> if the user's email address should be sent to the provider. Ignored for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub send_email_to_provider: Option<bool>,
    /// Pass <em>True</em> if the final price depends on the shipping method. Ignored for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>.
    pub is_flexible: Option<bool>,
}
/// If you sent an invoice requesting a shipping address and the parameter <em>is_flexible</em> was specified, the Bot API will send an <a href="https://core.telegram.org/bots/api#update">Update</a> with a <em>shipping_query</em> field to the bot. Use this method to reply to shipping queries. On success, <em>True</em> is returned.
#[derive(macros::Method)]
#[method(name = "answerShippingQuery", response(crate::True))]
pub struct AnswerShippingQueryRequest {
    /// Unique identifier for the query to be answered
    pub shipping_query_id: String,
    /// Pass <em>True</em> if delivery to the specified address is possible and <em>False</em> if there are any problems (for example, if delivery to the specified address is not possible)
    pub ok: bool,
    /// Required if <em>ok</em> is <em>True</em>. A JSON-serialized array of available shipping options.
    pub shipping_options: Option<Vec<ShippingOption>>,
    /// Required if <em>ok</em> is <em>False</em>. Error message in human readable form that explains why it is impossible to complete the order (e.g. “Sorry, delivery to your desired address is unavailable”). Telegram will display this message to the user.
    pub error_message: Option<String>,
}
/// Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an <a href="https://core.telegram.org/bots/api#update">Update</a> with the field <em>pre_checkout_query</em>. Use this method to respond to such pre-checkout queries. On success, <em>True</em> is returned. <strong>Note:</strong> The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent.
#[derive(macros::Method)]
#[method(name = "answerPreCheckoutQuery", response(crate::True))]
pub struct AnswerPreCheckoutQueryRequest {
    /// Unique identifier for the query to be answered
    pub pre_checkout_query_id: String,
    /// Specify <em>True</em> if everything is alright (goods are available, etc.) and the bot is ready to proceed with the order. Use <em>False</em> if there are any problems.
    pub ok: bool,
    /// Required if <em>ok</em> is <em>False</em>. Error message in human readable form that explains the reason for failure to proceed with the checkout (e.g. "Sorry, somebody just bought the last of our amazing black T-shirts while you were busy filling out your payment details. Please choose a different color or garment!"). Telegram will display this message to the user.
    pub error_message: Option<String>,
}
/// A method to get the current Telegram Stars balance of the bot. Requires no parameters. On success, returns a <a href="https://core.telegram.org/bots/api#staramount">StarAmount</a> object.
#[derive(macros::Method)]
#[method(name = "getMyStarBalance", response(StarAmount))]
pub struct GetMyStarBalanceRequest;
/// Returns the bot's Telegram Star transactions in chronological order. On success, returns a <a href="https://core.telegram.org/bots/api#startransactions">StarTransactions</a> object.
#[derive(macros::Method)]
#[method(name = "getStarTransactions", response(StarTransactions))]
pub struct GetStarTransactionsRequest {
    /// Number of transactions to skip in the response
    pub offset: Option<i64>,
    /// The maximum number of transactions to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    pub limit: Option<i64>,
}
/// Refunds a successful payment in <a href="https://t.me/BotNews/90">Telegram Stars</a>. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "refundStarPayment", response(crate::True))]
pub struct RefundStarPaymentRequest {
    /// Identifier of the user whose payment will be refunded
    pub user_id: i64,
    /// Telegram payment identifier
    pub telegram_payment_charge_id: String,
}
/// Allows the bot to cancel or re-enable extension of a subscription paid in Telegram Stars. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "editUserStarSubscription", response(crate::True))]
pub struct EditUserStarSubscriptionRequest {
    /// Identifier of the user whose subscription will be edited
    pub user_id: i64,
    /// Telegram payment identifier for the subscription
    pub telegram_payment_charge_id: String,
    /// Pass <em>True</em> to cancel extension of the user subscription; the subscription must be active up to the end of the current subscription period. Pass <em>False</em> to allow the user to re-enable a subscription that was previously canceled by the bot.
    pub is_canceled: bool,
}
/// This object represents a portion of the price for goods or services.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LabeledPrice {
    /// Portion label
    pub label: String,
    /// Price of the product in the <em>smallest units</em> of the <a href="/bots/payments#supported-currencies">currency</a> (integer, <strong>not</strong> float/double). For example, for a price of <code>US$ 1.45</code> pass <code>amount = 145</code>. See the <em>exp</em> parameter in <a href="/bots/payments/currencies.json">currencies.json</a>, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub amount: i64,
}
/// This object contains basic information about an invoice.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Invoice {
    /// Product name
    pub title: String,
    /// Product description
    pub description: String,
    /// Unique bot deep-linking parameter that can be used to generate this invoice
    pub start_parameter: String,
    /// Three-letter ISO 4217 <a href="/bots/payments#supported-currencies">currency</a> code, or “XTR” for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>
    pub currency: String,
    /// Total price in the <em>smallest units</em> of the currency (integer, <strong>not</strong> float/double). For example, for a price of <code>US$ 1.45</code> pass <code>amount = 145</code>. See the <em>exp</em> parameter in <a href="/bots/payments/currencies.json">currencies.json</a>, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,
}
/// This object represents a shipping address.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ShippingAddress {
    /// Two-letter <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO 3166-1 alpha-2</a> country code
    pub country_code: String,
    /// State, if applicable
    pub state: String,
    /// City
    pub city: String,
    /// First line for the address
    pub street_line1: String,
    /// Second line for the address
    pub street_line2: String,
    /// Address post code
    pub post_code: String,
}
/// This object represents information about an order.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OrderInfo {
    /// User name
    pub name: Option<String>,
    /// User's phone number
    pub phone_number: Option<String>,
    /// User email
    pub email: Option<String>,
    /// User shipping address
    pub shipping_address: Option<ShippingAddress>,
}
/// This object represents one shipping option.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ShippingOption {
    /// Shipping option identifier
    pub id: String,
    /// Option title
    pub title: String,
    /// List of price portions
    pub prices: Vec<LabeledPrice>,
}
/// This object contains basic information about a successful payment. Note that if the buyer initiates a chargeback with the relevant payment provider following this transaction, the funds may be debited from your balance. This is outside of Telegram's control.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuccessfulPayment {
    /// Three-letter ISO 4217 <a href="/bots/payments#supported-currencies">currency</a> code, or “XTR” for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>
    pub currency: String,
    /// Total price in the <em>smallest units</em> of the currency (integer, <strong>not</strong> float/double). For example, for a price of <code>US$ 1.45</code> pass <code>amount = 145</code>. See the <em>exp</em> parameter in <a href="/bots/payments/currencies.json">currencies.json</a>, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,
    /// Bot-specified invoice payload
    pub invoice_payload: String,
    /// Expiration date of the subscription, in Unix time; for recurring payments only
    pub subscription_expiration_date: Option<i64>,
    /// <em>True</em>, if the payment is a recurring payment for a subscription
    pub is_recurring: Option<crate::True>,
    /// <em>True</em>, if the payment is the first payment for a subscription
    pub is_first_recurring: Option<crate::True>,
    /// Identifier of the shipping option chosen by the user
    pub shipping_option_id: Option<String>,
    /// Order information provided by the user
    pub order_info: Option<OrderInfo>,
    /// Telegram payment identifier
    pub telegram_payment_charge_id: String,
    /// Provider payment identifier
    pub provider_payment_charge_id: String,
}
/// This object contains basic information about a refunded payment.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RefundedPayment {
    /// Three-letter ISO 4217 <a href="/bots/payments#supported-currencies">currency</a> code, or “XTR” for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>. Currently, always “XTR”
    pub currency: String,
    /// Total refunded price in the <em>smallest units</em> of the currency (integer, <strong>not</strong> float/double). For example, for a price of <code>US$ 1.45</code>, <code>total_amount = 145</code>. See the <em>exp</em> parameter in <a href="/bots/payments/currencies.json">currencies.json</a>, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,
    /// Bot-specified invoice payload
    pub invoice_payload: String,
    /// Telegram payment identifier
    pub telegram_payment_charge_id: String,
    /// Provider payment identifier
    pub provider_payment_charge_id: Option<String>,
}
/// This object contains information about an incoming shipping query.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ShippingQuery {
    /// Unique query identifier
    pub id: String,
    /// User who sent the query
    pub from: User,
    /// Bot-specified invoice payload
    pub invoice_payload: String,
    /// User specified shipping address
    pub shipping_address: ShippingAddress,
}
/// This object contains information about an incoming pre-checkout query.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PreCheckoutQuery {
    /// Unique query identifier
    pub id: String,
    /// User who sent the query
    pub from: User,
    /// Three-letter ISO 4217 <a href="/bots/payments#supported-currencies">currency</a> code, or “XTR” for payments in <a href="https://t.me/BotNews/90">Telegram Stars</a>
    pub currency: String,
    /// Total price in the <em>smallest units</em> of the currency (integer, <strong>not</strong> float/double). For example, for a price of <code>US$ 1.45</code> pass <code>amount = 145</code>. See the <em>exp</em> parameter in <a href="/bots/payments/currencies.json">currencies.json</a>, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,
    /// Bot-specified invoice payload
    pub invoice_payload: String,
    /// Identifier of the shipping option chosen by the user
    pub shipping_option_id: Option<String>,
    /// Order information provided by the user
    pub order_info: Option<OrderInfo>,
}
/// This object contains information about a paid media purchase.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaidMediaPurchased {
    /// User who purchased the media
    pub from: User,
    /// Bot-specified paid media payload
    pub paid_media_payload: String,
}
/// This object describes the state of a revenue withdrawal operation.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum RevenueWithdrawalState {
    Pending(RevenueWithdrawalStatePending),
    Succeeded(RevenueWithdrawalStateSucceeded),
    Failed(RevenueWithdrawalStateFailed),
}
/// The withdrawal is in progress.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RevenueWithdrawalStatePending {
    /// Type of the state, always “pending”
    pub r#type: String,
}
/// The withdrawal succeeded.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RevenueWithdrawalStateSucceeded {
    /// Type of the state, always “succeeded”
    pub r#type: String,
    /// Date the withdrawal was completed in Unix time
    pub date: i64,
    /// An HTTPS URL that can be used to see transaction details
    pub url: String,
}
/// The withdrawal failed and the transaction was refunded.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RevenueWithdrawalStateFailed {
    /// Type of the state, always “failed”
    pub r#type: String,
}
/// Contains information about the affiliate that received a commission via this transaction.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AffiliateInfo {
    /// The bot or the user that received an affiliate commission if it was received by a bot or a user
    pub affiliate_user: Option<User>,
    /// The chat that received an affiliate commission if it was received by a chat
    pub affiliate_chat: Option<Chat>,
    /// The number of Telegram Stars received by the affiliate for each 1000 Telegram Stars received by the bot from referred users
    pub commission_per_mille: i64,
    /// Integer amount of Telegram Stars received by the affiliate from the transaction, rounded to 0; can be negative for refunds
    pub amount: i64,
    /// The number of 1/1000000000 shares of Telegram Stars received by the affiliate; from -999999999 to 999999999; can be negative for refunds
    pub nanostar_amount: Option<i64>,
}
/// This object describes the source of a transaction, or its recipient for outgoing transactions.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum TransactionPartner {
    User(TransactionPartnerUser),
    Chat(TransactionPartnerChat),
    AffiliateProgram(TransactionPartnerAffiliateProgram),
    Fragment(TransactionPartnerFragment),
    TelegramAds(TransactionPartnerTelegramAds),
    TelegramApi(TransactionPartnerTelegramApi),
    Other(TransactionPartnerOther),
}
/// Describes a transaction with a user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransactionPartnerUser {
    /// Type of the transaction partner, always “user”
    pub r#type: String,
    /// Type of the transaction, currently one of “invoice_payment” for payments via invoices, “paid_media_payment” for payments for paid media, “gift_purchase” for gifts sent by the bot, “premium_purchase” for Telegram Premium subscriptions gifted by the bot, “business_account_transfer” for direct transfers from managed business accounts
    pub transaction_type: String,
    /// Information about the user
    pub user: User,
    /// Information about the affiliate that received a commission via this transaction. Can be available only for “invoice_payment” and “paid_media_payment” transactions.
    pub affiliate: Option<AffiliateInfo>,
    /// Bot-specified invoice payload. Can be available only for “invoice_payment” transactions.
    pub invoice_payload: Option<String>,
    /// The duration of the paid subscription. Can be available only for “invoice_payment” transactions.
    pub subscription_period: Option<i64>,
    /// Information about the paid media bought by the user; for “paid_media_payment” transactions only
    pub paid_media: Option<Vec<PaidMedia>>,
    /// Bot-specified paid media payload. Can be available only for “paid_media_payment” transactions.
    pub paid_media_payload: Option<String>,
    /// The gift sent to the user by the bot; for “gift_purchase” transactions only
    pub gift: Option<Gift>,
    /// Number of months the gifted Telegram Premium subscription will be active for; for “premium_purchase” transactions only
    pub premium_subscription_duration: Option<i64>,
}
/// Describes a transaction with a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransactionPartnerChat {
    /// Type of the transaction partner, always “chat”
    pub r#type: String,
    /// Information about the chat
    pub chat: Chat,
    /// The gift sent to the chat by the bot
    pub gift: Option<Gift>,
}
/// Describes the affiliate program that issued the affiliate commission received via this transaction.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransactionPartnerAffiliateProgram {
    /// Type of the transaction partner, always “affiliate_program”
    pub r#type: String,
    /// Information about the bot that sponsored the affiliate program
    pub sponsor_user: Option<User>,
    /// The number of Telegram Stars received by the bot for each 1000 Telegram Stars received by the affiliate program sponsor from referred users
    pub commission_per_mille: i64,
}
/// Describes a withdrawal transaction with Fragment.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransactionPartnerFragment {
    /// Type of the transaction partner, always “fragment”
    pub r#type: String,
    /// State of the transaction if the transaction is outgoing
    pub withdrawal_state: Option<RevenueWithdrawalState>,
}
/// Describes a withdrawal transaction to the Telegram Ads platform.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransactionPartnerTelegramAds {
    /// Type of the transaction partner, always “telegram_ads”
    pub r#type: String,
}
/// Describes a transaction with payment for <a href="https://core.telegram.org/bots/api#paid-broadcasts">paid broadcasting</a>.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransactionPartnerTelegramApi {
    /// Type of the transaction partner, always “telegram_api”
    pub r#type: String,
    /// The number of successful requests that exceeded regular limits and were therefore billed
    pub request_count: i64,
}
/// Describes a transaction with an unknown source or recipient.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransactionPartnerOther {
    /// Type of the transaction partner, always “other”
    pub r#type: String,
}
/// Describes a Telegram Star transaction. Note that if the buyer initiates a chargeback with the payment provider from whom they acquired Stars (e.g., Apple, Google) following this transaction, the refunded Stars will be deducted from the bot's balance. This is outside of Telegram's control.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StarTransaction {
    /// Unique identifier of the transaction. Coincides with the identifier of the original transaction for refund transactions. Coincides with <em>SuccessfulPayment.telegram_payment_charge_id</em> for successful incoming payments from users.
    pub id: String,
    /// Integer amount of Telegram Stars transferred by the transaction
    pub amount: i64,
    /// The number of 1/1000000000 shares of Telegram Stars transferred by the transaction; from 0 to 999999999
    pub nanostar_amount: Option<i64>,
    /// Date the transaction was created in Unix time
    pub date: i64,
    /// Source of an incoming transaction (e.g., a user purchasing goods or services, Fragment refunding a failed withdrawal). Only for incoming transactions
    pub source: Option<TransactionPartner>,
    /// Receiver of an outgoing transaction (e.g., a user for a purchase refund, Fragment for a withdrawal). Only for outgoing transactions
    pub receiver: Option<TransactionPartner>,
}
/// Contains a list of Telegram Star transactions.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StarTransactions {
    /// The list of transactions
    pub transactions: Vec<StarTransaction>,
}
/// Describes Telegram Passport data shared with the bot by the user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportData {
    /// Array with information about documents and other Telegram Passport elements that was shared with the bot
    pub data: Vec<EncryptedPassportElement>,
    /// Encrypted credentials required to decrypt the data
    pub credentials: EncryptedCredentials,
}
/// This object represents a file uploaded to Telegram Passport. Currently all Telegram Passport files are in JPEG format when decrypted and don't exceed 10MB.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportFile {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// File size in bytes
    pub file_size: i64,
    /// Unix time when the file was uploaded
    pub file_date: i64,
}
/// Describes documents or other Telegram Passport elements shared with the bot by the user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EncryptedPassportElement {
    /// Element type. One of “personal_details”, “passport”, “driver_license”, “identity_card”, “internal_passport”, “address”, “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”, “phone_number”, “email”.
    pub r#type: String,
    /// Base64-encoded encrypted Telegram Passport element data provided by the user; available only for “personal_details”, “passport”, “driver_license”, “identity_card”, “internal_passport” and “address” types. Can be decrypted and verified using the accompanying <a href="https://core.telegram.org/bots/api#encryptedcredentials">EncryptedCredentials</a>.
    pub data: Option<String>,
    /// User's verified phone number; available only for “phone_number” type
    pub phone_number: Option<String>,
    /// User's verified email address; available only for “email” type
    pub email: Option<String>,
    /// Array of encrypted files with documents provided by the user; available only for “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration” and “temporary_registration” types. Files can be decrypted and verified using the accompanying <a href="https://core.telegram.org/bots/api#encryptedcredentials">EncryptedCredentials</a>.
    pub files: Option<Vec<PassportFile>>,
    /// Encrypted file with the front side of the document, provided by the user; available only for “passport”, “driver_license”, “identity_card” and “internal_passport”. The file can be decrypted and verified using the accompanying <a href="https://core.telegram.org/bots/api#encryptedcredentials">EncryptedCredentials</a>.
    pub front_side: Option<PassportFile>,
    /// Encrypted file with the reverse side of the document, provided by the user; available only for “driver_license” and “identity_card”. The file can be decrypted and verified using the accompanying <a href="https://core.telegram.org/bots/api#encryptedcredentials">EncryptedCredentials</a>.
    pub reverse_side: Option<PassportFile>,
    /// Encrypted file with the selfie of the user holding a document, provided by the user; available if requested for “passport”, “driver_license”, “identity_card” and “internal_passport”. The file can be decrypted and verified using the accompanying <a href="https://core.telegram.org/bots/api#encryptedcredentials">EncryptedCredentials</a>.
    pub selfie: Option<PassportFile>,
    /// Array of encrypted files with translated versions of documents provided by the user; available if requested for “passport”, “driver_license”, “identity_card”, “internal_passport”, “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration” and “temporary_registration” types. Files can be decrypted and verified using the accompanying <a href="https://core.telegram.org/bots/api#encryptedcredentials">EncryptedCredentials</a>.
    pub translation: Option<Vec<PassportFile>>,
    /// Base64-encoded element hash for using in <a href="https://core.telegram.org/bots/api#passportelementerrorunspecified">PassportElementErrorUnspecified</a>
    pub hash: String,
}
/// Describes data required for decrypting and authenticating <a href="https://core.telegram.org/bots/api#encryptedpassportelement">EncryptedPassportElement</a>. See the <a href="/passport#receiving-information">Telegram Passport Documentation</a> for a complete description of the data decryption and authentication processes.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EncryptedCredentials {
    /// Base64-encoded encrypted JSON-serialized data with unique user's payload, data hashes and secrets required for <a href="https://core.telegram.org/bots/api#encryptedpassportelement">EncryptedPassportElement</a> decryption and authentication
    pub data: String,
    /// Base64-encoded data hash for data authentication
    pub hash: String,
    /// Base64-encoded secret, encrypted with the bot's public RSA key, required for data decryption
    pub secret: String,
}
/// Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setPassportDataErrors", response(crate::True))]
pub struct SetPassportDataErrorsRequest {
    /// User identifier
    pub user_id: i64,
    /// A JSON-serialized array describing the errors
    pub errors: Vec<PassportElementError>,
}
/// This object represents an error in the Telegram Passport element which was submitted that should be resolved by the user.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum PassportElementError {
    DataField(PassportElementErrorDataField),
    FrontSide(PassportElementErrorFrontSide),
    ReverseSide(PassportElementErrorReverseSide),
    Selfie(PassportElementErrorSelfie),
    File(PassportElementErrorFile),
    Files(PassportElementErrorFiles),
    TranslationFile(PassportElementErrorTranslationFile),
    TranslationFiles(PassportElementErrorTranslationFiles),
    Unspecified(PassportElementErrorUnspecified),
}
/// Represents an issue in one of the data fields that was provided by the user. The error is considered resolved when the field's value changes.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorDataField {
    /// Error source, must be <em>data</em>
    pub source: String,
    /// The section of the user's Telegram Passport which has the error, one of “personal_details”, “passport”, “driver_license”, “identity_card”, “internal_passport”, “address”
    pub r#type: String,
    /// Name of the data field which has the error
    pub field_name: String,
    /// Base64-encoded data hash
    pub data_hash: String,
    /// Error message
    pub message: String,
}
/// Represents an issue with the front side of a document. The error is considered resolved when the file with the front side of the document changes.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorFrontSide {
    /// Error source, must be <em>front_side</em>
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of “passport”, “driver_license”, “identity_card”, “internal_passport”
    pub r#type: String,
    /// Base64-encoded hash of the file with the front side of the document
    pub file_hash: String,
    /// Error message
    pub message: String,
}
/// Represents an issue with the reverse side of a document. The error is considered resolved when the file with reverse side of the document changes.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorReverseSide {
    /// Error source, must be <em>reverse_side</em>
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of “driver_license”, “identity_card”
    pub r#type: String,
    /// Base64-encoded hash of the file with the reverse side of the document
    pub file_hash: String,
    /// Error message
    pub message: String,
}
/// Represents an issue with the selfie with a document. The error is considered resolved when the file with the selfie changes.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorSelfie {
    /// Error source, must be <em>selfie</em>
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of “passport”, “driver_license”, “identity_card”, “internal_passport”
    pub r#type: String,
    /// Base64-encoded hash of the file with the selfie
    pub file_hash: String,
    /// Error message
    pub message: String,
}
/// Represents an issue with a document scan. The error is considered resolved when the file with the document scan changes.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorFile {
    /// Error source, must be <em>file</em>
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
    pub r#type: String,
    /// Base64-encoded file hash
    pub file_hash: String,
    /// Error message
    pub message: String,
}
/// Represents an issue with a list of scans. The error is considered resolved when the list of files containing the scans changes.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorFiles {
    /// Error source, must be <em>files</em>
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
    pub r#type: String,
    /// List of base64-encoded file hashes
    pub file_hashes: Vec<String>,
    /// Error message
    pub message: String,
}
/// Represents an issue with one of the files that constitute the translation of a document. The error is considered resolved when the file changes.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorTranslationFile {
    /// Error source, must be <em>translation_file</em>
    pub source: String,
    /// Type of element of the user's Telegram Passport which has the issue, one of “passport”, “driver_license”, “identity_card”, “internal_passport”, “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
    pub r#type: String,
    /// Base64-encoded file hash
    pub file_hash: String,
    /// Error message
    pub message: String,
}
/// Represents an issue with the translated version of a document. The error is considered resolved when a file with the document translation change.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorTranslationFiles {
    /// Error source, must be <em>translation_files</em>
    pub source: String,
    /// Type of element of the user's Telegram Passport which has the issue, one of “passport”, “driver_license”, “identity_card”, “internal_passport”, “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
    pub r#type: String,
    /// List of base64-encoded file hashes
    pub file_hashes: Vec<String>,
    /// Error message
    pub message: String,
}
/// Represents an issue in an unspecified place. The error is considered resolved when new data is added.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorUnspecified {
    /// Error source, must be <em>unspecified</em>
    pub source: String,
    /// Type of element of the user's Telegram Passport which has the issue
    pub r#type: String,
    /// Base64-encoded element hash
    pub element_hash: String,
    /// Error message
    pub message: String,
}
/// Use this method to send a game. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendGame", response(Message))]
pub struct SendGameRequest {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat. Games can't be sent to channel direct messages chats and channel chats.
    pub chat_id: i64,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Short name of the game, serves as the unique identifier for the game. Set up your games via <a href="https://t.me/botfather">@BotFather</a>.
    pub game_short_name: String,
    /// Sends the message <a href="https://telegram.org/blog/channels-2-0#silent-messages">silently</a>. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Pass <em>True</em> to allow up to 1000 messages per second, ignoring <a href="https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once">broadcasting limits</a> for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    pub message_effect_id: Option<String>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
    /// A JSON-serialized object for an <a href="/bots/features#inline-keyboards">inline keyboard</a>. If empty, one 'Play game_title' button will be shown. If not empty, the first button must launch the game.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// This object represents a game. Use BotFather to create and edit games, their short names will act as unique identifiers.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Game {
    /// Title of the game
    pub title: String,
    /// Description of the game
    pub description: String,
    /// Photo that will be displayed in the game message in chats.
    pub photo: Vec<PhotoSize>,
    /// Brief description of the game or high scores included in the game message. Can be automatically edited to include current high scores for the game when the bot calls <a href="https://core.telegram.org/bots/api#setgamescore">setGameScore</a>, or manually edited using <a href="https://core.telegram.org/bots/api#editmessagetext">editMessageText</a>. 0-4096 characters.
    pub text: Option<String>,
    /// Special entities that appear in <em>text</em>, such as usernames, URLs, bot commands, etc.
    pub text_entities: Option<Vec<MessageEntity>>,
    /// Animation that will be displayed in the game message in chats. Upload via <a href="https://t.me/botfather">BotFather</a>
    pub animation: Option<Animation>,
}
/// A placeholder, currently holds no information. Use <a href="https://t.me/botfather">BotFather</a> to set up your game.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CallbackGame;
/// Use this method to set the score of the specified user in a game message. On success, if the message is not an inline message, the <a href="https://core.telegram.org/bots/api#message">Message</a> is returned, otherwise <em>True</em> is returned. Returns an error, if the new score is not greater than the user's current score in the chat and <em>force</em> is <em>False</em>.
#[derive(macros::Method)]
#[method(name = "setGameScore", response(MessageOrTrue))]
pub struct SetGameScoreRequest {
    /// User identifier
    pub user_id: i64,
    /// New score, must be non-negative
    pub score: i64,
    /// Pass <em>True</em> if the high score is allowed to decrease. This can be useful when fixing mistakes or banning cheaters
    pub force: Option<bool>,
    /// Pass <em>True</em> if the game message should not be automatically edited to include the current scoreboard
    pub disable_edit_message: Option<bool>,
    /// Required if <em>inline_message_id</em> is not specified. Unique identifier for the target chat
    pub chat_id: Option<i64>,
    /// Required if <em>inline_message_id</em> is not specified. Identifier of the sent message
    pub message_id: Option<i64>,
    /// Required if <em>chat_id</em> and <em>message_id</em> are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
}
/// Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. Returns an Array of <a href="https://core.telegram.org/bots/api#gamehighscore">GameHighScore</a> objects.
#[derive(macros::Method)]
#[method(name = "getGameHighScores", response(Vec<GameHighScore>))]
pub struct GetGameHighScoresRequest {
    /// Target user id
    pub user_id: i64,
    /// Required if <em>inline_message_id</em> is not specified. Unique identifier for the target chat
    pub chat_id: Option<i64>,
    /// Required if <em>inline_message_id</em> is not specified. Identifier of the sent message
    pub message_id: Option<i64>,
    /// Required if <em>chat_id</em> and <em>message_id</em> are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
}
/// This object represents one row of the high scores table for a game.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GameHighScore {
    /// Position in high score table for the game
    pub position: i64,
    /// User
    pub user: User,
    /// Score
    pub score: i64,
}
