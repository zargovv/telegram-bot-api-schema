// this file is auto-generated

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum MessageOrTrue {
    Message(Message),
    True(crate::True),
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
pub enum ChatId {
    Integer(i64),
    String(String),
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Attachment {
    InputFile(InputFile),
    String(String),
}
/// This <a href="https://core.telegram.org/bots/api#available-types">object</a> represents an incoming update.<br>At most <strong>one</strong> of the optional parameters can be present in any given update.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
    pub edited_message: Option<Message>,
    pub channel_post: Option<Message>,
    pub edited_channel_post: Option<Message>,
    pub business_connection: Option<BusinessConnection>,
    pub business_message: Option<Message>,
    pub edited_business_message: Option<Message>,
    pub deleted_business_messages: Option<BusinessMessagesDeleted>,
    pub message_reaction: Option<MessageReactionUpdated>,
    pub message_reaction_count: Option<MessageReactionCountUpdated>,
    pub inline_query: Option<InlineQuery>,
    pub chosen_inline_result: Option<ChosenInlineResult>,
    pub callback_query: Option<CallbackQuery>,
    pub shipping_query: Option<ShippingQuery>,
    pub pre_checkout_query: Option<PreCheckoutQuery>,
    pub purchased_paid_media: Option<PaidMediaPurchased>,
    pub poll: Option<Poll>,
    pub poll_answer: Option<PollAnswer>,
    pub my_chat_member: Option<ChatMemberUpdated>,
    pub chat_member: Option<ChatMemberUpdated>,
    pub chat_join_request: Option<ChatJoinRequest>,
    pub chat_boost: Option<ChatBoostUpdated>,
    pub removed_chat_boost: Option<ChatBoostRemoved>,
}
/// Use this method to receive incoming updates using long polling (<a href="https://en.wikipedia.org/wiki/Push_technology#Long_polling">wiki</a>). Returns an Array of <a href="https://core.telegram.org/bots/api#update">Update</a> objects.
#[derive(macros::Method)]
#[method(name = "getUpdates", response(Vec<Update>))]
pub struct GetUpdatesRequest {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub timeout: Option<i64>,
    pub allowed_updates: Option<Vec<String>>,
}
/// Use this method to specify a URL and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified URL, containing a JSON-serialized <a href="https://core.telegram.org/bots/api#update">Update</a>. In case of an unsuccessful request (a request with response <a href="https://en.wikipedia.org/wiki/List_of_HTTP_status_codes">HTTP status code</a> different from <code>2XY</code>), we will repeat the request and give up after a reasonable amount of attempts. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setWebhook", response(crate::True))]
pub struct SetWebhookRequest {
    pub url: String,
    pub certificate: Option<InputFile>,
    pub ip_address: Option<String>,
    pub max_connections: Option<i64>,
    pub allowed_updates: Option<Vec<String>>,
    pub drop_pending_updates: Option<bool>,
    pub secret_token: Option<String>,
}
/// Use this method to remove webhook integration if you decide to switch back to <a href="https://core.telegram.org/bots/api#getupdates">getUpdates</a>. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "deleteWebhook", response(crate::True))]
pub struct DeleteWebhookRequest {
    pub drop_pending_updates: Option<bool>,
}
/// Use this method to get current webhook status. Requires no parameters. On success, returns a <a href="https://core.telegram.org/bots/api#webhookinfo">WebhookInfo</a> object. If the bot is using <a href="https://core.telegram.org/bots/api#getupdates">getUpdates</a>, will return an object with the <em>url</em> field empty.
#[derive(macros::Method)]
#[method(name = "getWebhookInfo", response(WebhookInfo))]
pub struct GetWebhookInfoRequest;
/// Describes the current status of a webhook.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WebhookInfo {
    pub url: String,
    pub has_custom_certificate: bool,
    pub pending_update_count: i64,
    pub ip_address: Option<String>,
    pub last_error_date: Option<i64>,
    pub last_error_message: Option<String>,
    pub last_synchronization_error_date: Option<i64>,
    pub max_connections: Option<i64>,
    pub allowed_updates: Option<Vec<String>>,
}
/// This object represents a Telegram user or bot.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub is_premium: Option<crate::True>,
    pub added_to_attachment_menu: Option<crate::True>,
    pub can_join_groups: Option<bool>,
    pub can_read_all_group_messages: Option<bool>,
    pub supports_inline_queries: Option<bool>,
    pub can_connect_to_business: Option<bool>,
    pub has_main_web_app: Option<bool>,
    pub has_topics_enabled: Option<bool>,
}
/// This object represents a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Chat {
    pub id: i64,
    pub r#type: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_forum: Option<crate::True>,
    pub is_direct_messages: Option<crate::True>,
}
/// This object contains full information about a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatFullInfo {
    pub id: i64,
    pub r#type: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_forum: Option<crate::True>,
    pub is_direct_messages: Option<crate::True>,
    pub accent_color_id: i64,
    pub max_reaction_count: i64,
    pub photo: Option<ChatPhoto>,
    pub active_usernames: Option<Vec<String>>,
    pub birthdate: Option<Birthdate>,
    pub business_intro: Option<BusinessIntro>,
    pub business_location: Option<BusinessLocation>,
    pub business_opening_hours: Option<BusinessOpeningHours>,
    pub personal_chat: Option<Chat>,
    pub parent_chat: Option<Chat>,
    pub available_reactions: Option<Vec<ReactionType>>,
    pub background_custom_emoji_id: Option<String>,
    pub profile_accent_color_id: Option<i64>,
    pub profile_background_custom_emoji_id: Option<String>,
    pub emoji_status_custom_emoji_id: Option<String>,
    pub emoji_status_expiration_date: Option<i64>,
    pub bio: Option<String>,
    pub has_private_forwards: Option<crate::True>,
    pub has_restricted_voice_and_video_messages: Option<crate::True>,
    pub join_to_send_messages: Option<crate::True>,
    pub join_by_request: Option<crate::True>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub pinned_message: Option<Message>,
    pub permissions: Option<ChatPermissions>,
    pub accepted_gift_types: AcceptedGiftTypes,
    pub can_send_paid_media: Option<crate::True>,
    pub slow_mode_delay: Option<i64>,
    pub unrestrict_boost_count: Option<i64>,
    pub message_auto_delete_time: Option<i64>,
    pub has_aggressive_anti_spam_enabled: Option<crate::True>,
    pub has_hidden_members: Option<crate::True>,
    pub has_protected_content: Option<crate::True>,
    pub has_visible_history: Option<crate::True>,
    pub sticker_set_name: Option<String>,
    pub can_set_sticker_set: Option<crate::True>,
    pub custom_emoji_sticker_set_name: Option<String>,
    pub linked_chat_id: Option<i64>,
    pub location: Option<ChatLocation>,
    pub rating: Option<UserRating>,
    pub unique_gift_colors: Option<UniqueGiftColors>,
    pub paid_message_star_count: Option<i64>,
}
/// This object represents a message.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub message_id: i64,
    pub message_thread_id: Option<i64>,
    pub direct_messages_topic: Option<DirectMessagesTopic>,
    pub from: Option<User>,
    pub sender_chat: Option<Chat>,
    pub sender_boost_count: Option<i64>,
    pub sender_business_bot: Option<User>,
    pub date: i64,
    pub business_connection_id: Option<String>,
    pub chat: Chat,
    pub forward_origin: Option<MessageOrigin>,
    pub is_topic_message: Option<crate::True>,
    pub is_automatic_forward: Option<crate::True>,
    pub reply_to_message: Option<Box<Message>>,
    pub external_reply: Option<ExternalReplyInfo>,
    pub quote: Option<TextQuote>,
    pub reply_to_story: Option<Story>,
    pub reply_to_checklist_task_id: Option<i64>,
    pub via_bot: Option<User>,
    pub edit_date: Option<i64>,
    pub has_protected_content: Option<crate::True>,
    pub is_from_offline: Option<crate::True>,
    pub is_paid_post: Option<crate::True>,
    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    pub paid_star_count: Option<i64>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub link_preview_options: Option<LinkPreviewOptions>,
    pub suggested_post_info: Option<SuggestedPostInfo>,
    pub effect_id: Option<String>,
    pub animation: Option<Animation>,
    pub audio: Option<Audio>,
    pub document: Option<Document>,
    pub paid_media: Option<PaidMediaInfo>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub story: Option<Story>,
    pub video: Option<Video>,
    pub video_note: Option<VideoNote>,
    pub voice: Option<Voice>,
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<crate::True>,
    pub has_media_spoiler: Option<crate::True>,
    pub checklist: Option<Checklist>,
    pub contact: Option<Contact>,
    pub dice: Option<Dice>,
    pub game: Option<Game>,
    pub poll: Option<Poll>,
    pub venue: Option<Venue>,
    pub location: Option<Location>,
    pub new_chat_members: Option<Vec<User>>,
    pub left_chat_member: Option<User>,
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    pub delete_chat_photo: Option<crate::True>,
    pub group_chat_created: Option<crate::True>,
    pub supergroup_chat_created: Option<crate::True>,
    pub channel_chat_created: Option<crate::True>,
    pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
    pub migrate_to_chat_id: Option<i64>,
    pub migrate_from_chat_id: Option<i64>,
    pub pinned_message: Option<Box<MaybeInaccessibleMessage>>,
    pub invoice: Option<Invoice>,
    pub successful_payment: Option<SuccessfulPayment>,
    pub refunded_payment: Option<RefundedPayment>,
    pub users_shared: Option<UsersShared>,
    pub chat_shared: Option<ChatShared>,
    pub gift: Option<GiftInfo>,
    pub unique_gift: Option<UniqueGiftInfo>,
    pub gift_upgrade_sent: Option<GiftInfo>,
    pub connected_website: Option<String>,
    pub write_access_allowed: Option<WriteAccessAllowed>,
    pub passport_data: Option<PassportData>,
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,
    pub boost_added: Option<ChatBoostAdded>,
    pub chat_background_set: Option<ChatBackground>,
    pub checklist_tasks_done: Option<Box<ChecklistTasksDone>>,
    pub checklist_tasks_added: Option<Box<ChecklistTasksAdded>>,
    pub direct_message_price_changed: Option<DirectMessagePriceChanged>,
    pub forum_topic_created: Option<ForumTopicCreated>,
    pub forum_topic_edited: Option<ForumTopicEdited>,
    pub forum_topic_closed: Option<ForumTopicClosed>,
    pub forum_topic_reopened: Option<ForumTopicReopened>,
    pub general_forum_topic_hidden: Option<GeneralForumTopicHidden>,
    pub general_forum_topic_unhidden: Option<GeneralForumTopicUnhidden>,
    pub giveaway_created: Option<GiveawayCreated>,
    pub giveaway: Option<Giveaway>,
    pub giveaway_winners: Option<GiveawayWinners>,
    pub giveaway_completed: Option<Box<GiveawayCompleted>>,
    pub paid_message_price_changed: Option<PaidMessagePriceChanged>,
    pub suggested_post_approved: Option<Box<SuggestedPostApproved>>,
    pub suggested_post_approval_failed: Option<Box<SuggestedPostApprovalFailed>>,
    pub suggested_post_declined: Option<Box<SuggestedPostDeclined>>,
    pub suggested_post_paid: Option<Box<SuggestedPostPaid>>,
    pub suggested_post_refunded: Option<Box<SuggestedPostRefunded>>,
    pub video_chat_scheduled: Option<VideoChatScheduled>,
    pub video_chat_started: Option<VideoChatStarted>,
    pub video_chat_ended: Option<VideoChatEnded>,
    pub video_chat_participants_invited: Option<VideoChatParticipantsInvited>,
    pub web_app_data: Option<WebAppData>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// This object represents a unique message identifier.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageId {
    pub message_id: i64,
}
/// This object describes a message that was deleted or is otherwise inaccessible to the bot.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InaccessibleMessage {
    pub chat: Chat,
    pub message_id: i64,
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
    pub r#type: String,
    pub offset: i64,
    pub length: i64,
    pub url: Option<String>,
    pub user: Option<User>,
    pub language: Option<String>,
    pub custom_emoji_id: Option<String>,
}
/// This object contains information about the quoted part of a message that is replied to by the given message.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TextQuote {
    pub text: String,
    pub entities: Option<Vec<MessageEntity>>,
    pub position: i64,
    pub is_manual: Option<crate::True>,
}
/// This object contains information about a message that is being replied to, which may come from another chat or forum topic.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExternalReplyInfo {
    pub origin: MessageOrigin,
    pub chat: Option<Chat>,
    pub message_id: Option<i64>,
    pub link_preview_options: Option<LinkPreviewOptions>,
    pub animation: Option<Animation>,
    pub audio: Option<Audio>,
    pub document: Option<Document>,
    pub paid_media: Option<PaidMediaInfo>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub story: Option<Story>,
    pub video: Option<Video>,
    pub video_note: Option<VideoNote>,
    pub voice: Option<Voice>,
    pub has_media_spoiler: Option<crate::True>,
    pub checklist: Option<Checklist>,
    pub contact: Option<Contact>,
    pub dice: Option<Dice>,
    pub game: Option<Game>,
    pub giveaway: Option<Giveaway>,
    pub giveaway_winners: Option<GiveawayWinners>,
    pub invoice: Option<Invoice>,
    pub location: Option<Location>,
    pub poll: Option<Poll>,
    pub venue: Option<Venue>,
}
/// Describes reply parameters for the message that is being sent.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReplyParameters {
    pub message_id: i64,
    pub chat_id: Option<ChatId>,
    pub allow_sending_without_reply: Option<bool>,
    pub quote: Option<String>,
    pub quote_parse_mode: Option<String>,
    pub quote_entities: Option<Vec<MessageEntity>>,
    pub quote_position: Option<i64>,
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
    pub r#type: String,
    pub date: i64,
    pub sender_user: User,
}
/// The message was originally sent by an unknown user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageOriginHiddenUser {
    pub r#type: String,
    pub date: i64,
    pub sender_user_name: String,
}
/// The message was originally sent on behalf of a chat to a group chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageOriginChat {
    pub r#type: String,
    pub date: i64,
    pub sender_chat: Chat,
    pub author_signature: Option<String>,
}
/// The message was originally sent to a channel chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageOriginChannel {
    pub r#type: String,
    pub date: i64,
    pub chat: Chat,
    pub message_id: i64,
    pub author_signature: Option<String>,
}
/// This object represents one size of a photo or a <a href="https://core.telegram.org/bots/api#document">file</a> / <a href="https://core.telegram.org/bots/api#sticker">sticker</a> thumbnail.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PhotoSize {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i64,
    pub height: i64,
    pub file_size: Option<i64>,
}
/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video without sound).
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Animation {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i64,
    pub height: i64,
    pub duration: i64,
    pub thumbnail: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}
/// This object represents an audio file to be treated as music by the Telegram clients.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Audio {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: i64,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
    pub thumbnail: Option<PhotoSize>,
}
/// This object represents a general file (as opposed to <a href="https://core.telegram.org/bots/api#photosize">photos</a>, <a href="https://core.telegram.org/bots/api#voice">voice messages</a> and <a href="https://core.telegram.org/bots/api#audio">audio files</a>).
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Document {
    pub file_id: String,
    pub file_unique_id: String,
    pub thumbnail: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}
/// This object represents a story.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Story {
    pub chat: Chat,
    pub id: i64,
}
/// This object represents a video file.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Video {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i64,
    pub height: i64,
    pub duration: i64,
    pub thumbnail: Option<PhotoSize>,
    pub cover: Option<Vec<PhotoSize>>,
    pub start_timestamp: Option<i64>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}
/// This object represents a <a href="https://telegram.org/blog/video-messages-and-telescope">video message</a> (available in Telegram apps as of <a href="https://telegram.org/blog/video-messages-and-telescope">v.4.0</a>).
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VideoNote {
    pub file_id: String,
    pub file_unique_id: String,
    pub length: i64,
    pub duration: i64,
    pub thumbnail: Option<PhotoSize>,
    pub file_size: Option<i64>,
}
/// This object represents a voice note.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Voice {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: i64,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}
/// Describes the paid media added to a message.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaidMediaInfo {
    pub star_count: i64,
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
    pub r#type: String,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub duration: Option<i64>,
}
/// The paid media is a photo.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaidMediaPhoto {
    pub r#type: String,
    pub photo: Vec<PhotoSize>,
}
/// The paid media is a video.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaidMediaVideo {
    pub r#type: String,
    pub video: Video,
}
/// This object represents a phone contact.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<i64>,
    pub vcard: Option<String>,
}
/// This object represents an animated emoji that displays a random value.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Dice {
    pub emoji: String,
    pub value: i64,
}
/// This object contains information about one answer option in a poll.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PollOption {
    pub text: String,
    pub text_entities: Option<Vec<MessageEntity>>,
    pub voter_count: i64,
}
/// This object contains information about one answer option in a poll to be sent.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputPollOption {
    pub text: String,
    pub text_parse_mode: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
}
/// This object represents an answer of a user in a non-anonymous poll.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PollAnswer {
    pub poll_id: String,
    pub voter_chat: Option<Chat>,
    pub user: Option<User>,
    pub option_ids: Vec<i64>,
}
/// This object contains information about a poll.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Poll {
    pub id: String,
    pub question: String,
    pub question_entities: Option<Vec<MessageEntity>>,
    pub options: Vec<PollOption>,
    pub total_voter_count: i64,
    pub is_closed: bool,
    pub is_anonymous: bool,
    pub r#type: String,
    pub allows_multiple_answers: bool,
    pub correct_option_id: Option<i64>,
    pub explanation: Option<String>,
    pub explanation_entities: Option<Vec<MessageEntity>>,
    pub open_period: Option<i64>,
    pub close_date: Option<i64>,
}
/// Describes a task in a checklist.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChecklistTask {
    pub id: i64,
    pub text: String,
    pub text_entities: Option<Vec<MessageEntity>>,
    pub completed_by_user: Option<User>,
    pub completed_by_chat: Option<Chat>,
    pub completion_date: Option<i64>,
}
/// Describes a checklist.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Checklist {
    pub title: String,
    pub title_entities: Option<Vec<MessageEntity>>,
    pub tasks: Vec<ChecklistTask>,
    pub others_can_add_tasks: Option<crate::True>,
    pub others_can_mark_tasks_as_done: Option<crate::True>,
}
/// Describes a task to add to a checklist.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputChecklistTask {
    pub id: i64,
    pub text: String,
    pub parse_mode: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
}
/// Describes a checklist to create.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputChecklist {
    pub title: String,
    pub parse_mode: Option<String>,
    pub title_entities: Option<Vec<MessageEntity>>,
    pub tasks: Vec<InputChecklistTask>,
    pub others_can_add_tasks: Option<bool>,
    pub others_can_mark_tasks_as_done: Option<bool>,
}
/// Describes a service message about checklist tasks marked as done or not done.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChecklistTasksDone {
    pub checklist_message: Option<Box<Message>>,
    pub marked_as_done_task_ids: Option<Vec<i64>>,
    pub marked_as_not_done_task_ids: Option<Vec<i64>>,
}
/// Describes a service message about tasks added to a checklist.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChecklistTasksAdded {
    pub checklist_message: Option<Box<Message>>,
    pub tasks: Vec<ChecklistTask>,
}
/// This object represents a point on the map.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<i64>,
    pub heading: Option<i64>,
    pub proximity_alert_radius: Option<i64>,
}
/// This object represents a venue.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Venue {
    pub location: Location,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub google_place_id: Option<String>,
    pub google_place_type: Option<String>,
}
/// Describes data sent from a <a href="/bots/webapps">Web App</a> to the bot.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WebAppData {
    pub data: String,
    pub button_text: String,
}
/// This object represents the content of a service message, sent whenever a user in the chat triggers a proximity alert set by another user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProximityAlertTriggered {
    pub traveler: User,
    pub watcher: User,
    pub distance: i64,
}
/// This object represents a service message about a change in auto-delete timer settings.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageAutoDeleteTimerChanged {
    pub message_auto_delete_time: i64,
}
/// This object represents a service message about a user boosting a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatBoostAdded {
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
    pub r#type: String,
    pub color: i64,
}
/// The background is a gradient fill.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BackgroundFillGradient {
    pub r#type: String,
    pub top_color: i64,
    pub bottom_color: i64,
    pub rotation_angle: i64,
}
/// The background is a freeform gradient that rotates after every message in the chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BackgroundFillFreeformGradient {
    pub r#type: String,
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
    pub r#type: String,
    pub fill: BackgroundFill,
    pub dark_theme_dimming: i64,
}
/// The background is a wallpaper in the JPEG format.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BackgroundTypeWallpaper {
    pub r#type: String,
    pub document: Document,
    pub dark_theme_dimming: i64,
    pub is_blurred: Option<crate::True>,
    pub is_moving: Option<crate::True>,
}
/// The background is a .PNG or .TGV (gzipped subset of SVG with MIME type “application/x-tgwallpattern”) pattern to be combined with the background fill chosen by the user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BackgroundTypePattern {
    pub r#type: String,
    pub document: Document,
    pub fill: BackgroundFill,
    pub intensity: i64,
    pub is_inverted: Option<crate::True>,
    pub is_moving: Option<crate::True>,
}
/// The background is taken directly from a built-in chat theme.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BackgroundTypeChatTheme {
    pub r#type: String,
    pub theme_name: String,
}
/// This object represents a chat background.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatBackground {
    pub r#type: BackgroundType,
}
/// This object represents a service message about a new forum topic created in the chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForumTopicCreated {
    pub name: String,
    pub icon_color: i64,
    pub icon_custom_emoji_id: Option<String>,
    pub is_name_implicit: Option<crate::True>,
}
/// This object represents a service message about a forum topic closed in the chat. Currently holds no information.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForumTopicClosed;
/// This object represents a service message about an edited forum topic.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForumTopicEdited {
    pub name: Option<String>,
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
    pub user_id: i64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub photo: Option<Vec<PhotoSize>>,
}
/// This object contains information about the users whose identifiers were shared with the bot using a <a href="https://core.telegram.org/bots/api#keyboardbuttonrequestusers">KeyboardButtonRequestUsers</a> button.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UsersShared {
    pub request_id: i64,
    pub users: Vec<SharedUser>,
}
/// This object contains information about a chat that was shared with the bot using a <a href="https://core.telegram.org/bots/api#keyboardbuttonrequestchat">KeyboardButtonRequestChat</a> button.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatShared {
    pub request_id: i64,
    pub chat_id: i64,
    pub title: Option<String>,
    pub username: Option<String>,
    pub photo: Option<Vec<PhotoSize>>,
}
/// This object represents a service message about a user allowing a bot to write messages after adding it to the attachment menu, launching a Web App from a link, or accepting an explicit request from a Web App sent by the method <a href="/bots/webapps#initializing-mini-apps">requestWriteAccess</a>.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WriteAccessAllowed {
    pub from_request: Option<bool>,
    pub web_app_name: Option<String>,
    pub from_attachment_menu: Option<bool>,
}
/// This object represents a service message about a video chat scheduled in the chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VideoChatScheduled {
    pub start_date: i64,
}
/// This object represents a service message about a video chat started in the chat. Currently holds no information.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VideoChatStarted;
/// This object represents a service message about a video chat ended in the chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VideoChatEnded {
    pub duration: i64,
}
/// This object represents a service message about new members invited to a video chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VideoChatParticipantsInvited {
    pub users: Vec<User>,
}
/// Describes a service message about a change in the price of paid messages within a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaidMessagePriceChanged {
    pub paid_message_star_count: i64,
}
/// Describes a service message about a change in the price of direct messages sent to a channel chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DirectMessagePriceChanged {
    pub are_direct_messages_enabled: bool,
    pub direct_message_star_count: Option<i64>,
}
/// Describes a service message about the approval of a suggested post.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuggestedPostApproved {
    pub suggested_post_message: Option<Box<Message>>,
    pub price: Option<SuggestedPostPrice>,
    pub send_date: i64,
}
/// Describes a service message about the failed approval of a suggested post. Currently, only caused by insufficient user funds at the time of approval.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuggestedPostApprovalFailed {
    pub suggested_post_message: Option<Box<Message>>,
    pub price: SuggestedPostPrice,
}
/// Describes a service message about the rejection of a suggested post.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuggestedPostDeclined {
    pub suggested_post_message: Option<Box<Message>>,
    pub comment: Option<String>,
}
/// Describes a service message about a successful payment for a suggested post.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuggestedPostPaid {
    pub suggested_post_message: Option<Box<Message>>,
    pub currency: String,
    pub amount: Option<i64>,
    pub star_amount: Option<StarAmount>,
}
/// Describes a service message about a payment refund for a suggested post.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuggestedPostRefunded {
    pub suggested_post_message: Option<Box<Message>>,
    pub reason: String,
}
/// This object represents a service message about the creation of a scheduled giveaway.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GiveawayCreated {
    pub prize_star_count: Option<i64>,
}
/// This object represents a message about a scheduled giveaway.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Giveaway {
    pub chats: Vec<Chat>,
    pub winners_selection_date: i64,
    pub winner_count: i64,
    pub only_new_members: Option<crate::True>,
    pub has_public_winners: Option<crate::True>,
    pub prize_description: Option<String>,
    pub country_codes: Option<Vec<String>>,
    pub prize_star_count: Option<i64>,
    pub premium_subscription_month_count: Option<i64>,
}
/// This object represents a message about the completion of a giveaway with public winners.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GiveawayWinners {
    pub chat: Chat,
    pub giveaway_message_id: i64,
    pub winners_selection_date: i64,
    pub winner_count: i64,
    pub winners: Vec<User>,
    pub additional_chat_count: Option<i64>,
    pub prize_star_count: Option<i64>,
    pub premium_subscription_month_count: Option<i64>,
    pub unclaimed_prize_count: Option<i64>,
    pub only_new_members: Option<crate::True>,
    pub was_refunded: Option<crate::True>,
    pub prize_description: Option<String>,
}
/// This object represents a service message about the completion of a giveaway without public winners.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GiveawayCompleted {
    pub winner_count: i64,
    pub unclaimed_prize_count: Option<i64>,
    pub giveaway_message: Option<Box<Message>>,
    pub is_star_giveaway: Option<crate::True>,
}
/// Describes the options used for link preview generation.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LinkPreviewOptions {
    pub is_disabled: Option<bool>,
    pub url: Option<String>,
    pub prefer_small_media: Option<bool>,
    pub prefer_large_media: Option<bool>,
    pub show_above_text: Option<bool>,
}
/// Describes the price of a suggested post.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuggestedPostPrice {
    pub currency: String,
    pub amount: i64,
}
/// Contains information about a suggested post.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuggestedPostInfo {
    pub state: String,
    pub price: Option<SuggestedPostPrice>,
    pub send_date: Option<i64>,
}
/// Contains parameters of a post that is being suggested by the bot.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuggestedPostParameters {
    pub price: Option<SuggestedPostPrice>,
    pub send_date: Option<i64>,
}
/// Describes a topic of a direct messages chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DirectMessagesTopic {
    pub topic_id: i64,
    pub user: Option<User>,
}
/// This object represent a user's profile pictures.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserProfilePhotos {
    pub total_count: i64,
    pub photos: Vec<Vec<PhotoSize>>,
}
/// This object represents a file ready to be downloaded. The file can be downloaded via the link <code>https://api.telegram.org/file/bot<token>/<file_path></code>. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling <a href="https://core.telegram.org/bots/api#getfile">getFile</a>.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct File {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: Option<i64>,
    pub file_path: Option<String>,
}
/// Describes a <a href="/bots/webapps">Web App</a>.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WebAppInfo {
    pub url: String,
}
/// This object represents a <a href="/bots/features#keyboards">custom keyboard</a> with reply options (see <a href="/bots/features#keyboards">Introduction to bots</a> for details and examples). Not supported in channels and for messages sent on behalf of a Telegram Business account.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,
    pub is_persistent: Option<bool>,
    pub resize_keyboard: Option<bool>,
    pub one_time_keyboard: Option<bool>,
    pub input_field_placeholder: Option<String>,
    pub selective: Option<bool>,
}
/// This object represents one button of the reply keyboard. At most one of the optional fields must be used to specify type of the button. For simple text buttons, <em>String</em> can be used instead of this object to specify the button text.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct KeyboardButton {
    pub text: String,
    pub request_users: Option<KeyboardButtonRequestUsers>,
    pub request_chat: Option<KeyboardButtonRequestChat>,
    pub request_contact: Option<bool>,
    pub request_location: Option<bool>,
    pub request_poll: Option<KeyboardButtonPollType>,
    pub web_app: Option<WebAppInfo>,
}
/// This object defines the criteria used to request suitable users. Information about the selected users will be shared with the bot when the corresponding button is pressed. <a href="/bots/features#chat-and-user-selection">More about requesting users »</a>
#[derive(serde::Serialize, serde::Deserialize)]
pub struct KeyboardButtonRequestUsers {
    pub request_id: i64,
    pub user_is_bot: Option<bool>,
    pub user_is_premium: Option<bool>,
    pub max_quantity: Option<i64>,
    pub request_name: Option<bool>,
    pub request_username: Option<bool>,
    pub request_photo: Option<bool>,
}
/// This object defines the criteria used to request a suitable chat. Information about the selected chat will be shared with the bot when the corresponding button is pressed. The bot will be granted requested rights in the chat if appropriate. <a href="/bots/features#chat-and-user-selection">More about requesting chats »</a>.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct KeyboardButtonRequestChat {
    pub request_id: i64,
    pub chat_is_channel: bool,
    pub chat_is_forum: Option<bool>,
    pub chat_has_username: Option<bool>,
    pub chat_is_created: Option<bool>,
    pub user_administrator_rights: Option<ChatAdministratorRights>,
    pub bot_administrator_rights: Option<ChatAdministratorRights>,
    pub bot_is_member: Option<bool>,
    pub request_title: Option<bool>,
    pub request_username: Option<bool>,
    pub request_photo: Option<bool>,
}
/// This object represents type of a poll, which is allowed to be created and sent when the corresponding button is pressed.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct KeyboardButtonPollType {
    pub r#type: Option<String>,
}
/// Upon receiving a message with this object, Telegram clients will remove the current custom keyboard and display the default letter-keyboard. By default, custom keyboards are displayed until a new keyboard is sent by a bot. An exception is made for one-time keyboards that are hidden immediately after the user presses a button (see <a href="https://core.telegram.org/bots/api#replykeyboardmarkup">ReplyKeyboardMarkup</a>). Not supported in channels and for messages sent on behalf of a Telegram Business account.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReplyKeyboardRemove {
    pub remove_keyboard: crate::True,
    pub selective: Option<bool>,
}
/// This object represents an <a href="/bots/features#inline-keyboards">inline keyboard</a> that appears right next to the message it belongs to.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}
/// This object represents one button of an inline keyboard. Exactly one of the optional fields must be used to specify type of the button.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub url: Option<String>,
    pub callback_data: Option<String>,
    pub web_app: Option<WebAppInfo>,
    pub login_url: Option<LoginUrl>,
    pub switch_inline_query: Option<String>,
    pub switch_inline_query_current_chat: Option<String>,
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,
    pub copy_text: Option<CopyTextButton>,
    pub callback_game: Option<CallbackGame>,
    pub pay: Option<bool>,
}
/// This object represents a parameter of the inline keyboard button used to automatically authorize a user. Serves as a great replacement for the <a href="/widgets/login">Telegram Login Widget</a> when the user is coming from Telegram. All the user needs to do is tap/click a button and confirm that they want to log in:
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginUrl {
    pub url: String,
    pub forward_text: Option<String>,
    pub bot_username: Option<String>,
    pub request_write_access: Option<bool>,
}
/// This object represents an inline button that switches the current user to inline mode in a chosen chat, with an optional default inline query.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchInlineQueryChosenChat {
    pub query: Option<String>,
    pub allow_user_chats: Option<bool>,
    pub allow_bot_chats: Option<bool>,
    pub allow_group_chats: Option<bool>,
    pub allow_channel_chats: Option<bool>,
}
/// This object represents an inline keyboard button that copies specified text to the clipboard.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CopyTextButton {
    pub text: String,
}
/// This object represents an incoming callback query from a callback button in an <a href="/bots/features#inline-keyboards">inline keyboard</a>. If the button that originated the query was attached to a message sent by the bot, the field <em>message</em> will be present. If the button was attached to a message sent via the bot (in <a href="https://core.telegram.org/bots/api#inline-mode">inline mode</a>), the field <em>inline_message_id</em> will be present. Exactly one of the fields <em>data</em> or <em>game_short_name</em> will be present.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<MaybeInaccessibleMessage>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>,
}
/// Upon receiving a message with this object, Telegram clients will display a reply interface to the user (act as if the user has selected the bot's message and tapped 'Reply'). This can be extremely useful if you want to create user-friendly step-by-step interfaces without having to sacrifice <a href="/bots/features#privacy-mode">privacy mode</a>. Not supported in channels and for messages sent on behalf of a Telegram Business account.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForceReply {
    pub force_reply: crate::True,
    pub input_field_placeholder: Option<String>,
    pub selective: Option<bool>,
}
/// This object represents a chat photo.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatPhoto {
    pub small_file_id: String,
    pub small_file_unique_id: String,
    pub big_file_id: String,
    pub big_file_unique_id: String,
}
/// Represents an invite link for a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatInviteLink {
    pub invite_link: String,
    pub creator: User,
    pub creates_join_request: bool,
    pub is_primary: bool,
    pub is_revoked: bool,
    pub name: Option<String>,
    pub expire_date: Option<i64>,
    pub member_limit: Option<i64>,
    pub pending_join_request_count: Option<i64>,
    pub subscription_period: Option<i64>,
    pub subscription_price: Option<i64>,
}
/// Represents the rights of an administrator in a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatAdministratorRights {
    pub is_anonymous: bool,
    pub can_manage_chat: bool,
    pub can_delete_messages: bool,
    pub can_manage_video_chats: bool,
    pub can_restrict_members: bool,
    pub can_promote_members: bool,
    pub can_change_info: bool,
    pub can_invite_users: bool,
    pub can_post_stories: bool,
    pub can_edit_stories: bool,
    pub can_delete_stories: bool,
    pub can_post_messages: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_manage_topics: Option<bool>,
    pub can_manage_direct_messages: Option<bool>,
}
/// This object represents changes in the status of a chat member.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatMemberUpdated {
    pub chat: Chat,
    pub from: User,
    pub date: i64,
    pub old_chat_member: ChatMember,
    pub new_chat_member: ChatMember,
    pub invite_link: Option<ChatInviteLink>,
    pub via_join_request: Option<bool>,
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
    pub status: String,
    pub user: User,
    pub is_anonymous: bool,
    pub custom_title: Option<String>,
}
/// Represents a <a href="https://core.telegram.org/bots/api#chatmember">chat member</a> that has some additional privileges.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatMemberAdministrator {
    pub status: String,
    pub user: User,
    pub can_be_edited: bool,
    pub is_anonymous: bool,
    pub can_manage_chat: bool,
    pub can_delete_messages: bool,
    pub can_manage_video_chats: bool,
    pub can_restrict_members: bool,
    pub can_promote_members: bool,
    pub can_change_info: bool,
    pub can_invite_users: bool,
    pub can_post_stories: bool,
    pub can_edit_stories: bool,
    pub can_delete_stories: bool,
    pub can_post_messages: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_manage_topics: Option<bool>,
    pub can_manage_direct_messages: Option<bool>,
    pub custom_title: Option<String>,
}
/// Represents a <a href="https://core.telegram.org/bots/api#chatmember">chat member</a> that has no additional privileges or restrictions.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatMemberMember {
    pub status: String,
    pub user: User,
    pub until_date: Option<i64>,
}
/// Represents a <a href="https://core.telegram.org/bots/api#chatmember">chat member</a> that is under certain restrictions in the chat. Supergroups only.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatMemberRestricted {
    pub status: String,
    pub user: User,
    pub is_member: bool,
    pub can_send_messages: bool,
    pub can_send_audios: bool,
    pub can_send_documents: bool,
    pub can_send_photos: bool,
    pub can_send_videos: bool,
    pub can_send_video_notes: bool,
    pub can_send_voice_notes: bool,
    pub can_send_polls: bool,
    pub can_send_other_messages: bool,
    pub can_add_web_page_previews: bool,
    pub can_change_info: bool,
    pub can_invite_users: bool,
    pub can_pin_messages: bool,
    pub can_manage_topics: bool,
    pub until_date: i64,
}
/// Represents a <a href="https://core.telegram.org/bots/api#chatmember">chat member</a> that isn't currently a member of the chat, but may join it themselves.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatMemberLeft {
    pub status: String,
    pub user: User,
}
/// Represents a <a href="https://core.telegram.org/bots/api#chatmember">chat member</a> that was banned in the chat and can't return to the chat or view chat messages.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatMemberBanned {
    pub status: String,
    pub user: User,
    pub until_date: i64,
}
/// Represents a join request sent to a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatJoinRequest {
    pub chat: Chat,
    pub from: User,
    pub user_chat_id: i64,
    pub date: i64,
    pub bio: Option<String>,
    pub invite_link: Option<ChatInviteLink>,
}
/// Describes actions that a non-administrator user is allowed to take in a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatPermissions {
    pub can_send_messages: Option<bool>,
    pub can_send_audios: Option<bool>,
    pub can_send_documents: Option<bool>,
    pub can_send_photos: Option<bool>,
    pub can_send_videos: Option<bool>,
    pub can_send_video_notes: Option<bool>,
    pub can_send_voice_notes: Option<bool>,
    pub can_send_polls: Option<bool>,
    pub can_send_other_messages: Option<bool>,
    pub can_add_web_page_previews: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_manage_topics: Option<bool>,
}
/// Describes the birthdate of a user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Birthdate {
    pub day: i64,
    pub month: i64,
    pub year: Option<i64>,
}
/// Contains information about the start page settings of a Telegram Business account.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BusinessIntro {
    pub title: Option<String>,
    pub message: Option<String>,
    pub sticker: Option<Sticker>,
}
/// Contains information about the location of a Telegram Business account.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BusinessLocation {
    pub address: String,
    pub location: Option<Location>,
}
/// Describes an interval of time during which a business is open.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BusinessOpeningHoursInterval {
    pub opening_minute: i64,
    pub closing_minute: i64,
}
/// Describes the opening hours of a business.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BusinessOpeningHours {
    pub time_zone_name: String,
    pub opening_hours: Vec<BusinessOpeningHoursInterval>,
}
/// This object describes the rating of a user based on their Telegram Star spendings.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserRating {
    pub level: i64,
    pub rating: i64,
    pub current_level_rating: i64,
    pub next_level_rating: Option<i64>,
}
/// Describes the position of a clickable area within a story.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StoryAreaPosition {
    pub x_percentage: f64,
    pub y_percentage: f64,
    pub width_percentage: f64,
    pub height_percentage: f64,
    pub rotation_angle: f64,
    pub corner_radius_percentage: f64,
}
/// Describes the physical address of a location.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LocationAddress {
    pub country_code: String,
    pub state: Option<String>,
    pub city: Option<String>,
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
    pub r#type: String,
    pub latitude: f64,
    pub longitude: f64,
    pub address: Option<LocationAddress>,
}
/// Describes a story area pointing to a suggested reaction. Currently, a story can have up to 5 suggested reaction areas.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StoryAreaTypeSuggestedReaction {
    pub r#type: String,
    pub reaction_type: ReactionType,
    pub is_dark: Option<bool>,
    pub is_flipped: Option<bool>,
}
/// Describes a story area pointing to an HTTP or tg:// link. Currently, a story can have up to 3 link areas.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StoryAreaTypeLink {
    pub r#type: String,
    pub url: String,
}
/// Describes a story area containing weather information. Currently, a story can have up to 3 weather areas.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StoryAreaTypeWeather {
    pub r#type: String,
    pub temperature: f64,
    pub emoji: String,
    pub background_color: i64,
}
/// Describes a story area pointing to a unique gift. Currently, a story can have at most 1 unique gift area.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StoryAreaTypeUniqueGift {
    pub r#type: String,
    pub name: String,
}
/// Describes a clickable area on a story media.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StoryArea {
    pub position: StoryAreaPosition,
    pub r#type: StoryAreaType,
}
/// Represents a location to which a chat is connected.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatLocation {
    pub location: Location,
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
    pub r#type: String,
    pub emoji: String,
}
/// The reaction is based on a custom emoji.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReactionTypeCustomEmoji {
    pub r#type: String,
    pub custom_emoji_id: String,
}
/// The reaction is paid.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReactionTypePaid {
    pub r#type: String,
}
/// Represents a reaction added to a message along with the number of times it was added.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReactionCount {
    pub r#type: ReactionType,
    pub total_count: i64,
}
/// This object represents a change of a reaction on a message performed by a user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageReactionUpdated {
    pub chat: Chat,
    pub message_id: i64,
    pub user: Option<User>,
    pub actor_chat: Option<Chat>,
    pub date: i64,
    pub old_reaction: Vec<ReactionType>,
    pub new_reaction: Vec<ReactionType>,
}
/// This object represents reaction changes on a message with anonymous reactions.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageReactionCountUpdated {
    pub chat: Chat,
    pub message_id: i64,
    pub date: i64,
    pub reactions: Vec<ReactionCount>,
}
/// This object represents a forum topic.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForumTopic {
    pub message_thread_id: i64,
    pub name: String,
    pub icon_color: i64,
    pub icon_custom_emoji_id: Option<String>,
    pub is_name_implicit: Option<crate::True>,
}
/// This object describes the background of a gift.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GiftBackground {
    pub center_color: i64,
    pub edge_color: i64,
    pub text_color: i64,
}
/// This object represents a gift that can be sent by the bot.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Gift {
    pub id: String,
    pub sticker: Sticker,
    pub star_count: i64,
    pub upgrade_star_count: Option<i64>,
    pub is_premium: Option<crate::True>,
    pub has_colors: Option<crate::True>,
    pub total_count: Option<i64>,
    pub remaining_count: Option<i64>,
    pub personal_total_count: Option<i64>,
    pub personal_remaining_count: Option<i64>,
    pub background: Option<GiftBackground>,
    pub unique_gift_variant_count: Option<i64>,
    pub publisher_chat: Option<Chat>,
}
/// This object represent a list of gifts.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Gifts {
    pub gifts: Vec<Gift>,
}
/// This object describes the model of a unique gift.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UniqueGiftModel {
    pub name: String,
    pub sticker: Sticker,
    pub rarity_per_mille: i64,
}
/// This object describes the symbol shown on the pattern of a unique gift.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UniqueGiftSymbol {
    pub name: String,
    pub sticker: Sticker,
    pub rarity_per_mille: i64,
}
/// This object describes the colors of the backdrop of a unique gift.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UniqueGiftBackdropColors {
    pub center_color: i64,
    pub edge_color: i64,
    pub symbol_color: i64,
    pub text_color: i64,
}
/// This object describes the backdrop of a unique gift.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UniqueGiftBackdrop {
    pub name: String,
    pub colors: UniqueGiftBackdropColors,
    pub rarity_per_mille: i64,
}
/// This object contains information about the color scheme for a user's name, message replies and link previews based on a unique gift.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UniqueGiftColors {
    pub model_custom_emoji_id: String,
    pub symbol_custom_emoji_id: String,
    pub light_theme_main_color: i64,
    pub light_theme_other_colors: Vec<i64>,
    pub dark_theme_main_color: i64,
    pub dark_theme_other_colors: Vec<i64>,
}
/// This object describes a unique gift that was upgraded from a regular gift.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UniqueGift {
    pub gift_id: String,
    pub base_name: String,
    pub name: String,
    pub number: i64,
    pub model: UniqueGiftModel,
    pub symbol: UniqueGiftSymbol,
    pub backdrop: UniqueGiftBackdrop,
    pub is_premium: Option<crate::True>,
    pub is_from_blockchain: Option<crate::True>,
    pub colors: Option<UniqueGiftColors>,
    pub publisher_chat: Option<Chat>,
}
/// Describes a service message about a regular gift that was sent or received.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GiftInfo {
    pub gift: Gift,
    pub owned_gift_id: Option<String>,
    pub convert_star_count: Option<i64>,
    pub prepaid_upgrade_star_count: Option<i64>,
    pub is_upgrade_separate: Option<crate::True>,
    pub can_be_upgraded: Option<crate::True>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub is_private: Option<crate::True>,
    pub unique_gift_number: Option<i64>,
}
/// Describes a service message about a unique gift that was sent or received.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UniqueGiftInfo {
    pub gift: UniqueGift,
    pub origin: String,
    pub last_resale_currency: Option<String>,
    pub last_resale_amount: Option<i64>,
    pub owned_gift_id: Option<String>,
    pub transfer_star_count: Option<i64>,
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
    pub r#type: String,
    pub gift: Gift,
    pub owned_gift_id: Option<String>,
    pub sender_user: Option<User>,
    pub send_date: i64,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub is_private: Option<crate::True>,
    pub is_saved: Option<crate::True>,
    pub can_be_upgraded: Option<crate::True>,
    pub was_refunded: Option<crate::True>,
    pub convert_star_count: Option<i64>,
    pub prepaid_upgrade_star_count: Option<i64>,
    pub is_upgrade_separate: Option<crate::True>,
    pub unique_gift_number: Option<i64>,
}
/// Describes a unique gift received and owned by a user or a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OwnedGiftUnique {
    pub r#type: String,
    pub gift: UniqueGift,
    pub owned_gift_id: Option<String>,
    pub sender_user: Option<User>,
    pub send_date: i64,
    pub is_saved: Option<crate::True>,
    pub can_be_transferred: Option<crate::True>,
    pub transfer_star_count: Option<i64>,
    pub next_transfer_date: Option<i64>,
}
/// Contains the list of gifts received and owned by a user or a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OwnedGifts {
    pub total_count: i64,
    pub gifts: Vec<OwnedGift>,
    pub next_offset: Option<String>,
}
/// This object describes the types of gifts that can be gifted to a user or a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AcceptedGiftTypes {
    pub unlimited_gifts: bool,
    pub limited_gifts: bool,
    pub unique_gifts: bool,
    pub premium_subscription: bool,
    pub gifts_from_channels: bool,
}
/// Describes an amount of Telegram Stars.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StarAmount {
    pub amount: i64,
    pub nanostar_amount: Option<i64>,
}
/// This object represents a bot command.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotCommand {
    pub command: String,
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
    pub r#type: String,
}
/// Represents the <a href="https://core.telegram.org/bots/api#botcommandscope">scope</a> of bot commands, covering all private chats.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotCommandScopeAllPrivateChats {
    pub r#type: String,
}
/// Represents the <a href="https://core.telegram.org/bots/api#botcommandscope">scope</a> of bot commands, covering all group and supergroup chats.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotCommandScopeAllGroupChats {
    pub r#type: String,
}
/// Represents the <a href="https://core.telegram.org/bots/api#botcommandscope">scope</a> of bot commands, covering all group and supergroup chat administrators.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotCommandScopeAllChatAdministrators {
    pub r#type: String,
}
/// Represents the <a href="https://core.telegram.org/bots/api#botcommandscope">scope</a> of bot commands, covering a specific chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotCommandScopeChat {
    pub r#type: String,
    pub chat_id: ChatId,
}
/// Represents the <a href="https://core.telegram.org/bots/api#botcommandscope">scope</a> of bot commands, covering all administrators of a specific group or supergroup chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotCommandScopeChatAdministrators {
    pub r#type: String,
    pub chat_id: ChatId,
}
/// Represents the <a href="https://core.telegram.org/bots/api#botcommandscope">scope</a> of bot commands, covering a specific member of a group or supergroup chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotCommandScopeChatMember {
    pub r#type: String,
    pub chat_id: ChatId,
    pub user_id: i64,
}
/// This object represents the bot's name.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotName {
    pub name: String,
}
/// This object represents the bot's description.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotDescription {
    pub description: String,
}
/// This object represents the bot's short description.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotShortDescription {
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
    pub r#type: String,
}
/// Represents a menu button, which launches a <a href="/bots/webapps">Web App</a>.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MenuButtonWebApp {
    pub r#type: String,
    pub text: String,
    pub web_app: WebAppInfo,
}
/// Describes that no specific value for the menu button was set.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MenuButtonDefault {
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
    pub source: String,
    pub user: User,
}
/// The boost was obtained by the creation of Telegram Premium gift codes to boost a chat. Each such code boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatBoostSourceGiftCode {
    pub source: String,
    pub user: User,
}
/// The boost was obtained by the creation of a Telegram Premium or a Telegram Star giveaway. This boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription for Telegram Premium giveaways and <em>prize_star_count</em> / 500 times for one year for Telegram Star giveaways.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatBoostSourceGiveaway {
    pub source: String,
    pub giveaway_message_id: i64,
    pub user: Option<User>,
    pub prize_star_count: Option<i64>,
    pub is_unclaimed: Option<crate::True>,
}
/// This object contains information about a chat boost.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatBoost {
    pub boost_id: String,
    pub add_date: i64,
    pub expiration_date: i64,
    pub source: ChatBoostSource,
}
/// This object represents a boost added to a chat or changed.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatBoostUpdated {
    pub chat: Chat,
    pub boost: ChatBoost,
}
/// This object represents a boost removed from a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatBoostRemoved {
    pub chat: Chat,
    pub boost_id: String,
    pub remove_date: i64,
    pub source: ChatBoostSource,
}
/// This object represents a list of boosts added to a chat by a user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserChatBoosts {
    pub boosts: Vec<ChatBoost>,
}
/// Represents the rights of a business bot.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BusinessBotRights {
    pub can_reply: Option<crate::True>,
    pub can_read_messages: Option<crate::True>,
    pub can_delete_sent_messages: Option<crate::True>,
    pub can_delete_all_messages: Option<crate::True>,
    pub can_edit_name: Option<crate::True>,
    pub can_edit_bio: Option<crate::True>,
    pub can_edit_profile_photo: Option<crate::True>,
    pub can_edit_username: Option<crate::True>,
    pub can_change_gift_settings: Option<crate::True>,
    pub can_view_gifts_and_stars: Option<crate::True>,
    pub can_convert_gifts_to_stars: Option<crate::True>,
    pub can_transfer_and_upgrade_gifts: Option<crate::True>,
    pub can_transfer_stars: Option<crate::True>,
    pub can_manage_stories: Option<crate::True>,
}
/// Describes the connection of the bot with a business account.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BusinessConnection {
    pub id: String,
    pub user: User,
    pub user_chat_id: i64,
    pub date: i64,
    pub rights: Option<BusinessBotRights>,
    pub is_enabled: bool,
}
/// This object is received when messages are deleted from a connected business account.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BusinessMessagesDeleted {
    pub business_connection_id: String,
    pub chat: Chat,
    pub message_ids: Vec<i64>,
}
/// Describes why a request was unsuccessful.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResponseParameters {
    pub migrate_to_chat_id: Option<i64>,
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
    pub r#type: String,
    pub media: String,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub has_spoiler: Option<bool>,
}
/// Represents a video to be sent.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputMediaVideo {
    pub r#type: String,
    pub media: String,
    pub thumbnail: Option<String>,
    pub cover: Option<String>,
    pub start_timestamp: Option<i64>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub duration: Option<i64>,
    pub supports_streaming: Option<bool>,
    pub has_spoiler: Option<bool>,
}
/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputMediaAnimation {
    pub r#type: String,
    pub media: String,
    pub thumbnail: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub duration: Option<i64>,
    pub has_spoiler: Option<bool>,
}
/// Represents an audio file to be treated as music to be sent.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputMediaAudio {
    pub r#type: String,
    pub media: String,
    pub thumbnail: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub duration: Option<i64>,
    pub performer: Option<String>,
    pub title: Option<String>,
}
/// Represents a general file to be sent.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputMediaDocument {
    pub r#type: String,
    pub media: String,
    pub thumbnail: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
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
    pub r#type: String,
    pub media: String,
}
/// The paid media to send is a video.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputPaidMediaVideo {
    pub r#type: String,
    pub media: String,
    pub thumbnail: Option<String>,
    pub cover: Option<String>,
    pub start_timestamp: Option<i64>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub duration: Option<i64>,
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
    pub r#type: String,
    pub photo: String,
}
/// An animated profile photo in the MPEG4 format.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputProfilePhotoAnimated {
    pub r#type: String,
    pub animation: String,
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
    pub r#type: String,
    pub photo: String,
}
/// Describes a video to post as a story.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputStoryContentVideo {
    pub r#type: String,
    pub video: String,
    pub duration: Option<f64>,
    pub cover_frame_timestamp: Option<f64>,
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
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i64>,
    pub direct_messages_topic_id: Option<i64>,
    pub text: String,
    pub parse_mode: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub link_preview_options: Option<LinkPreviewOptions>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to forward messages of any kind. Service messages and messages with protected content can't be forwarded. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "forwardMessage", response(Message))]
pub struct ForwardMessageRequest {
    pub chat_id: ChatId,
    pub message_thread_id: Option<i64>,
    pub direct_messages_topic_id: Option<i64>,
    pub from_chat_id: ChatId,
    pub video_start_timestamp: Option<i64>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub message_effect_id: Option<String>,
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    pub message_id: i64,
}
/// Use this method to forward multiple messages of any kind. If some of the specified messages can't be found or forwarded, they are skipped. Service messages and messages with protected content can't be forwarded. Album grouping is kept for forwarded messages. On success, an array of <a href="https://core.telegram.org/bots/api#messageid">MessageId</a> of the sent messages is returned.
#[derive(macros::Method)]
#[method(name = "forwardMessages", response(MessageId))]
pub struct ForwardMessagesRequest {
    pub chat_id: ChatId,
    pub message_thread_id: Option<i64>,
    pub direct_messages_topic_id: Option<i64>,
    pub from_chat_id: ChatId,
    pub message_ids: Vec<i64>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
}
/// Use this method to copy messages of any kind. Service messages, paid media messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz <a href="https://core.telegram.org/bots/api#poll">poll</a> can be copied only if the value of the field <em>correct_option_id</em> is known to the bot. The method is analogous to the method <a href="https://core.telegram.org/bots/api#forwardmessage">forwardMessage</a>, but the copied message doesn't have a link to the original message. Returns the <a href="https://core.telegram.org/bots/api#messageid">MessageId</a> of the sent message on success.
#[derive(macros::Method)]
#[method(name = "copyMessage", response(MessageId))]
pub struct CopyMessageRequest {
    pub chat_id: ChatId,
    pub message_thread_id: Option<i64>,
    pub direct_messages_topic_id: Option<i64>,
    pub from_chat_id: ChatId,
    pub message_id: i64,
    pub video_start_timestamp: Option<i64>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to copy messages of any kind. If some of the specified messages can't be found or copied, they are skipped. Service messages, paid media messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz <a href="https://core.telegram.org/bots/api#poll">poll</a> can be copied only if the value of the field <em>correct_option_id</em> is known to the bot. The method is analogous to the method <a href="https://core.telegram.org/bots/api#forwardmessages">forwardMessages</a>, but the copied messages don't have a link to the original message. Album grouping is kept for copied messages. On success, an array of <a href="https://core.telegram.org/bots/api#messageid">MessageId</a> of the sent messages is returned.
#[derive(macros::Method)]
#[method(name = "copyMessages", response(MessageId))]
pub struct CopyMessagesRequest {
    pub chat_id: ChatId,
    pub message_thread_id: Option<i64>,
    pub direct_messages_topic_id: Option<i64>,
    pub from_chat_id: ChatId,
    pub message_ids: Vec<i64>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub remove_caption: Option<bool>,
}
/// Use this method to send photos. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendPhoto", response(Message))]
pub struct SendPhotoRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i64>,
    pub direct_messages_topic_id: Option<i64>,
    pub photo: Attachment,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub has_spoiler: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to send audio files, if you want Telegram clients to display them in the music player. Your audio must be in the .MP3 or .M4A format. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned. Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.
#[derive(macros::Method)]
#[method(name = "sendAudio", response(Message))]
pub struct SendAudioRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i64>,
    pub direct_messages_topic_id: Option<i64>,
    pub audio: Attachment,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub duration: Option<i64>,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub thumbnail: Option<Attachment>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to send general files. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned. Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.
#[derive(macros::Method)]
#[method(name = "sendDocument", response(Message))]
pub struct SendDocumentRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i64>,
    pub direct_messages_topic_id: Option<i64>,
    pub document: Attachment,
    pub thumbnail: Option<Attachment>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub disable_content_type_detection: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to send video files, Telegram clients support MPEG4 videos (other formats may be sent as <a href="https://core.telegram.org/bots/api#document">Document</a>). On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned. Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
#[derive(macros::Method)]
#[method(name = "sendVideo", response(Message))]
pub struct SendVideoRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i64>,
    pub direct_messages_topic_id: Option<i64>,
    pub video: Attachment,
    pub duration: Option<i64>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub thumbnail: Option<Attachment>,
    pub cover: Option<Attachment>,
    pub start_timestamp: Option<i64>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub has_spoiler: Option<bool>,
    pub supports_streaming: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
#[derive(macros::Method)]
#[method(name = "sendAnimation", response(Message))]
pub struct SendAnimationRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i64>,
    pub direct_messages_topic_id: Option<i64>,
    pub animation: Attachment,
    pub duration: Option<i64>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub thumbnail: Option<Attachment>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub has_spoiler: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message. For this to work, your audio must be in an .OGG file encoded with OPUS, or in .MP3 format, or in .M4A format (other formats may be sent as <a href="https://core.telegram.org/bots/api#audio">Audio</a> or <a href="https://core.telegram.org/bots/api#document">Document</a>). On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future.
#[derive(macros::Method)]
#[method(name = "sendVoice", response(Message))]
pub struct SendVoiceRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i64>,
    pub direct_messages_topic_id: Option<i64>,
    pub voice: Attachment,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub duration: Option<i64>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
/// As of <a href="https://telegram.org/blog/video-messages-and-telescope">v.4.0</a>, Telegram clients support rounded square MPEG4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendVideoNote", response(Message))]
pub struct SendVideoNoteRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i64>,
    pub direct_messages_topic_id: Option<i64>,
    pub video_note: Attachment,
    pub duration: Option<i64>,
    pub length: Option<i64>,
    pub thumbnail: Option<Attachment>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to send paid media. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendPaidMedia", response(Message))]
pub struct SendPaidMediaRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i64>,
    pub direct_messages_topic_id: Option<i64>,
    pub star_count: i64,
    pub media: Vec<InputPaidMedia>,
    pub payload: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of <a href="https://core.telegram.org/bots/api#message">Message</a> objects that were sent is returned.
#[derive(macros::Method)]
#[method(name = "sendMediaGroup", response(Message))]
pub struct SendMediaGroupRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i64>,
    pub direct_messages_topic_id: Option<i64>,
    pub media: Vec<InputMediaAudio>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_parameters: Option<ReplyParameters>,
}
/// Use this method to send point on the map. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendLocation", response(Message))]
pub struct SendLocationRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i64>,
    pub direct_messages_topic_id: Option<i64>,
    pub latitude: f64,
    pub longitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<i64>,
    pub heading: Option<i64>,
    pub proximity_alert_radius: Option<i64>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to send information about a venue. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendVenue", response(Message))]
pub struct SendVenueRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i64>,
    pub direct_messages_topic_id: Option<i64>,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub google_place_id: Option<String>,
    pub google_place_type: Option<String>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to send phone contacts. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendContact", response(Message))]
pub struct SendContactRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i64>,
    pub direct_messages_topic_id: Option<i64>,
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub vcard: Option<String>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to send a native poll. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendPoll", response(Message))]
pub struct SendPollRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i64>,
    pub question: String,
    pub question_parse_mode: Option<String>,
    pub question_entities: Option<Vec<MessageEntity>>,
    pub options: Vec<InputPollOption>,
    pub is_anonymous: Option<bool>,
    pub r#type: Option<String>,
    pub allows_multiple_answers: Option<bool>,
    pub correct_option_id: Option<i64>,
    pub explanation: Option<String>,
    pub explanation_parse_mode: Option<String>,
    pub explanation_entities: Option<Vec<MessageEntity>>,
    pub open_period: Option<i64>,
    pub close_date: Option<i64>,
    pub is_closed: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to send a checklist on behalf of a connected business account. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendChecklist", response(Message))]
pub struct SendChecklistRequest {
    pub business_connection_id: String,
    pub chat_id: i64,
    pub checklist: InputChecklist,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// Use this method to send an animated emoji that will display a random value. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendDice", response(Message))]
pub struct SendDiceRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i64>,
    pub direct_messages_topic_id: Option<i64>,
    pub emoji: Option<String>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to stream a partial message to a user while the message is being generated; supported only for bots with forum topic mode enabled. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "sendMessageDraft", response(crate::True))]
pub struct SendMessageDraftRequest {
    pub chat_id: i64,
    pub message_thread_id: Option<i64>,
    pub draft_id: i64,
    pub text: String,
    pub parse_mode: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
}
/// Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "sendChatAction", response(crate::True))]
pub struct SendChatActionRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i64>,
    pub action: String,
}
/// Use this method to change the chosen reactions on a message. Service messages of some types can't be reacted to. Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel. Bots can't use paid reactions. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setMessageReaction", response(crate::True))]
pub struct SetMessageReactionRequest {
    pub chat_id: ChatId,
    pub message_id: i64,
    pub reaction: Option<Vec<ReactionType>>,
    pub is_big: Option<bool>,
}
/// Use this method to get a list of profile pictures for a user. Returns a <a href="https://core.telegram.org/bots/api#userprofilephotos">UserProfilePhotos</a> object.
#[derive(macros::Method)]
#[method(name = "getUserProfilePhotos", response(UserProfilePhotos))]
pub struct GetUserProfilePhotosRequest {
    pub user_id: i64,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}
/// Changes the emoji status for a given user that previously allowed the bot to manage their emoji status via the Mini App method <a href="/bots/webapps#initializing-mini-apps">requestEmojiStatusAccess</a>. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setUserEmojiStatus", response(crate::True))]
pub struct SetUserEmojiStatusRequest {
    pub user_id: i64,
    pub emoji_status_custom_emoji_id: Option<String>,
    pub emoji_status_expiration_date: Option<i64>,
}
/// Use this method to get basic information about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a <a href="https://core.telegram.org/bots/api#file">File</a> object is returned. The file can then be downloaded via the link <code>https://api.telegram.org/file/bot<token>/<file_path></code>, where <code><file_path></code> is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling <a href="https://core.telegram.org/bots/api#getfile">getFile</a> again.
#[derive(macros::Method)]
#[method(name = "getFile", response(File))]
pub struct GetFileRequest {
    pub file_id: String,
}
/// Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless <a href="https://core.telegram.org/bots/api#unbanchatmember">unbanned</a> first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "banChatMember", response(crate::True))]
pub struct BanChatMemberRequest {
    pub chat_id: ChatId,
    pub user_id: i64,
    pub until_date: Option<i64>,
    pub revoke_messages: Option<bool>,
}
/// Use this method to unban a previously banned user in a supergroup or channel. The user will <strong>not</strong> return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it. So if the user is a member of the chat they will also be <strong>removed</strong> from the chat. If you don't want this, use the parameter <em>only_if_banned</em>. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "unbanChatMember", response(crate::True))]
pub struct UnbanChatMemberRequest {
    pub chat_id: ChatId,
    pub user_id: i64,
    pub only_if_banned: Option<bool>,
}
/// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass <em>True</em> for all permissions to lift restrictions from a user. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "restrictChatMember", response(crate::True))]
pub struct RestrictChatMemberRequest {
    pub chat_id: ChatId,
    pub user_id: i64,
    pub permissions: ChatPermissions,
    pub use_independent_chat_permissions: Option<bool>,
    pub until_date: Option<i64>,
}
/// Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass <em>False</em> for all boolean parameters to demote a user. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "promoteChatMember", response(crate::True))]
pub struct PromoteChatMemberRequest {
    pub chat_id: ChatId,
    pub user_id: i64,
    pub is_anonymous: Option<bool>,
    pub can_manage_chat: Option<bool>,
    pub can_delete_messages: Option<bool>,
    pub can_manage_video_chats: Option<bool>,
    pub can_restrict_members: Option<bool>,
    pub can_promote_members: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_post_stories: Option<bool>,
    pub can_edit_stories: Option<bool>,
    pub can_delete_stories: Option<bool>,
    pub can_post_messages: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_manage_topics: Option<bool>,
    pub can_manage_direct_messages: Option<bool>,
}
/// Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setChatAdministratorCustomTitle", response(crate::True))]
pub struct SetChatAdministratorCustomTitleRequest {
    pub chat_id: ChatId,
    pub user_id: i64,
    pub custom_title: String,
}
/// Use this method to ban a channel chat in a supergroup or a channel. Until the chat is <a href="https://core.telegram.org/bots/api#unbanchatsenderchat">unbanned</a>, the owner of the banned chat won't be able to send messages on behalf of <strong>any of their channels</strong>. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "banChatSenderChat", response(crate::True))]
pub struct BanChatSenderChatRequest {
    pub chat_id: ChatId,
    pub sender_chat_id: i64,
}
/// Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "unbanChatSenderChat", response(crate::True))]
pub struct UnbanChatSenderChatRequest {
    pub chat_id: ChatId,
    pub sender_chat_id: i64,
}
/// Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the <em>can_restrict_members</em> administrator rights. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setChatPermissions", response(crate::True))]
pub struct SetChatPermissionsRequest {
    pub chat_id: ChatId,
    pub permissions: ChatPermissions,
    pub use_independent_chat_permissions: Option<bool>,
}
/// Use this method to generate a new primary invite link for a chat; any previously generated primary link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the new invite link as <em>String</em> on success.
#[derive(macros::Method)]
#[method(name = "exportChatInviteLink", response(String))]
pub struct ExportChatInviteLinkRequest {
    pub chat_id: ChatId,
}
/// Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method <a href="https://core.telegram.org/bots/api#revokechatinvitelink">revokeChatInviteLink</a>. Returns the new invite link as <a href="https://core.telegram.org/bots/api#chatinvitelink">ChatInviteLink</a> object.
#[derive(macros::Method)]
#[method(name = "createChatInviteLink", response(ChatInviteLink))]
pub struct CreateChatInviteLinkRequest {
    pub chat_id: ChatId,
    pub name: Option<String>,
    pub expire_date: Option<i64>,
    pub member_limit: Option<i64>,
    pub creates_join_request: Option<bool>,
}
/// Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a <a href="https://core.telegram.org/bots/api#chatinvitelink">ChatInviteLink</a> object.
#[derive(macros::Method)]
#[method(name = "editChatInviteLink", response(ChatInviteLink))]
pub struct EditChatInviteLinkRequest {
    pub chat_id: ChatId,
    pub invite_link: String,
    pub name: Option<String>,
    pub expire_date: Option<i64>,
    pub member_limit: Option<i64>,
    pub creates_join_request: Option<bool>,
}
/// Use this method to create a <a href="https://telegram.org/blog/superchannels-star-reactions-subscriptions#star-subscriptions">subscription invite link</a> for a channel chat. The bot must have the <em>can_invite_users</em> administrator rights. The link can be edited using the method <a href="https://core.telegram.org/bots/api#editchatsubscriptioninvitelink">editChatSubscriptionInviteLink</a> or revoked using the method <a href="https://core.telegram.org/bots/api#revokechatinvitelink">revokeChatInviteLink</a>. Returns the new invite link as a <a href="https://core.telegram.org/bots/api#chatinvitelink">ChatInviteLink</a> object.
#[derive(macros::Method)]
#[method(name = "createChatSubscriptionInviteLink", response(ChatInviteLink))]
pub struct CreateChatSubscriptionInviteLinkRequest {
    pub chat_id: ChatId,
    pub name: Option<String>,
    pub subscription_period: i64,
    pub subscription_price: i64,
}
/// Use this method to edit a subscription invite link created by the bot. The bot must have the <em>can_invite_users</em> administrator rights. Returns the edited invite link as a <a href="https://core.telegram.org/bots/api#chatinvitelink">ChatInviteLink</a> object.
#[derive(macros::Method)]
#[method(name = "editChatSubscriptionInviteLink", response(ChatInviteLink))]
pub struct EditChatSubscriptionInviteLinkRequest {
    pub chat_id: ChatId,
    pub invite_link: String,
    pub name: Option<String>,
}
/// Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the revoked invite link as <a href="https://core.telegram.org/bots/api#chatinvitelink">ChatInviteLink</a> object.
#[derive(macros::Method)]
#[method(name = "revokeChatInviteLink", response(ChatInviteLink))]
pub struct RevokeChatInviteLinkRequest {
    pub chat_id: ChatId,
    pub invite_link: String,
}
/// Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the <em>can_invite_users</em> administrator right. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "approveChatJoinRequest", response(crate::True))]
pub struct ApproveChatJoinRequestRequest {
    pub chat_id: ChatId,
    pub user_id: i64,
}
/// Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the <em>can_invite_users</em> administrator right. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "declineChatJoinRequest", response(crate::True))]
pub struct DeclineChatJoinRequestRequest {
    pub chat_id: ChatId,
    pub user_id: i64,
}
/// Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setChatPhoto", response(crate::True))]
pub struct SetChatPhotoRequest {
    pub chat_id: ChatId,
    pub photo: InputFile,
}
/// Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "deleteChatPhoto", response(crate::True))]
pub struct DeleteChatPhotoRequest {
    pub chat_id: ChatId,
}
/// Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setChatTitle", response(crate::True))]
pub struct SetChatTitleRequest {
    pub chat_id: ChatId,
    pub title: String,
}
/// Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setChatDescription", response(crate::True))]
pub struct SetChatDescriptionRequest {
    pub chat_id: ChatId,
    pub description: Option<String>,
}
/// Use this method to add a message to the list of pinned messages in a chat. In private chats and channel direct messages chats, all non-service messages can be pinned. Conversely, the bot must be an administrator with the 'can_pin_messages' right or the 'can_edit_messages' right to pin messages in groups and channels respectively. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "pinChatMessage", response(crate::True))]
pub struct PinChatMessageRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_id: i64,
    pub disable_notification: Option<bool>,
}
/// Use this method to remove a message from the list of pinned messages in a chat. In private chats and channel direct messages chats, all messages can be unpinned. Conversely, the bot must be an administrator with the 'can_pin_messages' right or the 'can_edit_messages' right to unpin messages in groups and channels respectively. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "unpinChatMessage", response(crate::True))]
pub struct UnpinChatMessageRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_id: Option<i64>,
}
/// Use this method to clear the list of pinned messages in a chat. In private chats and channel direct messages chats, no additional rights are required to unpin all pinned messages. Conversely, the bot must be an administrator with the 'can_pin_messages' right or the 'can_edit_messages' right to unpin all pinned messages in groups and channels respectively. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "unpinAllChatMessages", response(crate::True))]
pub struct UnpinAllChatMessagesRequest {
    pub chat_id: ChatId,
}
/// Use this method for your bot to leave a group, supergroup or channel. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "leaveChat", response(crate::True))]
pub struct LeaveChatRequest {
    pub chat_id: ChatId,
}
/// Use this method to get up-to-date information about the chat. Returns a <a href="https://core.telegram.org/bots/api#chatfullinfo">ChatFullInfo</a> object on success.
#[derive(macros::Method)]
#[method(name = "getChat", response(ChatFullInfo))]
pub struct GetChatRequest {
    pub chat_id: ChatId,
}
/// Use this method to get a list of administrators in a chat, which aren't bots. Returns an Array of <a href="https://core.telegram.org/bots/api#chatmember">ChatMember</a> objects.
#[derive(macros::Method)]
#[method(name = "getChatAdministrators", response(Vec<ChatMember>))]
pub struct GetChatAdministratorsRequest {
    pub chat_id: ChatId,
}
/// Use this method to get the number of members in a chat. Returns <em>Int</em> on success.
#[derive(macros::Method)]
#[method(name = "getChatMemberCount", response(i64))]
pub struct GetChatMemberCountRequest {
    pub chat_id: ChatId,
}
/// Use this method to get information about a member of a chat. The method is only guaranteed to work for other users if the bot is an administrator in the chat. Returns a <a href="https://core.telegram.org/bots/api#chatmember">ChatMember</a> object on success.
#[derive(macros::Method)]
#[method(name = "getChatMember", response(ChatMember))]
pub struct GetChatMemberRequest {
    pub chat_id: ChatId,
    pub user_id: i64,
}
/// Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field <em>can_set_sticker_set</em> optionally returned in <a href="https://core.telegram.org/bots/api#getchat">getChat</a> requests to check if the bot can use this method. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setChatStickerSet", response(crate::True))]
pub struct SetChatStickerSetRequest {
    pub chat_id: ChatId,
    pub sticker_set_name: String,
}
/// Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field <em>can_set_sticker_set</em> optionally returned in <a href="https://core.telegram.org/bots/api#getchat">getChat</a> requests to check if the bot can use this method. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "deleteChatStickerSet", response(crate::True))]
pub struct DeleteChatStickerSetRequest {
    pub chat_id: ChatId,
}
/// Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters. Returns an Array of <a href="https://core.telegram.org/bots/api#sticker">Sticker</a> objects.
#[derive(macros::Method)]
#[method(name = "getForumTopicIconStickers", response(Vec<Sticker>))]
pub struct GetForumTopicIconStickersRequest;
/// Use this method to create a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights. Returns information about the created topic as a <a href="https://core.telegram.org/bots/api#forumtopic">ForumTopic</a> object.
#[derive(macros::Method)]
#[method(name = "createForumTopic", response(ForumTopic))]
pub struct CreateForumTopicRequest {
    pub chat_id: ChatId,
    pub name: String,
    pub icon_color: Option<i64>,
    pub icon_custom_emoji_id: Option<String>,
}
/// Use this method to edit name and icon of a topic in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights, unless it is the creator of the topic. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "editForumTopic", response(crate::True))]
pub struct EditForumTopicRequest {
    pub chat_id: ChatId,
    pub message_thread_id: i64,
    pub name: Option<String>,
    pub icon_custom_emoji_id: Option<String>,
}
/// Use this method to close an open topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights, unless it is the creator of the topic. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "closeForumTopic", response(crate::True))]
pub struct CloseForumTopicRequest {
    pub chat_id: ChatId,
    pub message_thread_id: i64,
}
/// Use this method to reopen a closed topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights, unless it is the creator of the topic. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "reopenForumTopic", response(crate::True))]
pub struct ReopenForumTopicRequest {
    pub chat_id: ChatId,
    pub message_thread_id: i64,
}
/// Use this method to delete a forum topic along with all its messages in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the <em>can_delete_messages</em> administrator rights. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "deleteForumTopic", response(crate::True))]
pub struct DeleteForumTopicRequest {
    pub chat_id: ChatId,
    pub message_thread_id: i64,
}
/// Use this method to clear the list of pinned messages in a forum topic in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the <em>can_pin_messages</em> administrator right in the supergroup. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "unpinAllForumTopicMessages", response(crate::True))]
pub struct UnpinAllForumTopicMessagesRequest {
    pub chat_id: ChatId,
    pub message_thread_id: i64,
}
/// Use this method to edit the name of the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "editGeneralForumTopic", response(crate::True))]
pub struct EditGeneralForumTopicRequest {
    pub chat_id: ChatId,
    pub name: String,
}
/// Use this method to close an open 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "closeGeneralForumTopic", response(crate::True))]
pub struct CloseGeneralForumTopicRequest {
    pub chat_id: ChatId,
}
/// Use this method to reopen a closed 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights. The topic will be automatically unhidden if it was hidden. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "reopenGeneralForumTopic", response(crate::True))]
pub struct ReopenGeneralForumTopicRequest {
    pub chat_id: ChatId,
}
/// Use this method to hide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights. The topic will be automatically closed if it was open. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "hideGeneralForumTopic", response(crate::True))]
pub struct HideGeneralForumTopicRequest {
    pub chat_id: ChatId,
}
/// Use this method to unhide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "unhideGeneralForumTopic", response(crate::True))]
pub struct UnhideGeneralForumTopicRequest {
    pub chat_id: ChatId,
}
/// Use this method to clear the list of pinned messages in a General forum topic. The bot must be an administrator in the chat for this to work and must have the <em>can_pin_messages</em> administrator right in the supergroup. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "unpinAllGeneralForumTopicMessages", response(crate::True))]
pub struct UnpinAllGeneralForumTopicMessagesRequest {
    pub chat_id: ChatId,
}
/// Use this method to send answers to callback queries sent from <a href="/bots/features#inline-keyboards">inline keyboards</a>. The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, <em>True</em> is returned.
#[derive(macros::Method)]
#[method(name = "answerCallbackQuery", response(crate::True))]
pub struct AnswerCallbackQueryRequest {
    pub callback_query_id: String,
    pub text: Option<String>,
    pub show_alert: Option<bool>,
    pub url: Option<String>,
    pub cache_time: Option<i64>,
}
/// Use this method to get the list of boosts added to a chat by a user. Requires administrator rights in the chat. Returns a <a href="https://core.telegram.org/bots/api#userchatboosts">UserChatBoosts</a> object.
#[derive(macros::Method)]
#[method(name = "getUserChatBoosts", response(UserChatBoosts))]
pub struct GetUserChatBoostsRequest {
    pub chat_id: ChatId,
    pub user_id: i64,
}
/// Use this method to get information about the connection of the bot with a business account. Returns a <a href="https://core.telegram.org/bots/api#businessconnection">BusinessConnection</a> object on success.
#[derive(macros::Method)]
#[method(name = "getBusinessConnection", response(BusinessConnection))]
pub struct GetBusinessConnectionRequest {
    pub business_connection_id: String,
}
/// Use this method to change the list of the bot's commands. See <a href="/bots/features#commands">this manual</a> for more details about bot commands. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setMyCommands", response(crate::True))]
pub struct SetMyCommandsRequest {
    pub commands: Vec<BotCommand>,
    pub scope: Option<BotCommandScope>,
    pub language_code: Option<String>,
}
/// Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, <a href="https://core.telegram.org/bots/api#determining-list-of-commands">higher level commands</a> will be shown to affected users. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "deleteMyCommands", response(crate::True))]
pub struct DeleteMyCommandsRequest {
    pub scope: Option<BotCommandScope>,
    pub language_code: Option<String>,
}
/// Use this method to get the current list of the bot's commands for the given scope and user language. Returns an Array of <a href="https://core.telegram.org/bots/api#botcommand">BotCommand</a> objects. If commands aren't set, an empty list is returned.
#[derive(macros::Method)]
#[method(name = "getMyCommands", response(Vec<BotCommand>))]
pub struct GetMyCommandsRequest {
    pub scope: Option<BotCommandScope>,
    pub language_code: Option<String>,
}
/// Use this method to change the bot's name. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setMyName", response(crate::True))]
pub struct SetMyNameRequest {
    pub name: Option<String>,
    pub language_code: Option<String>,
}
/// Use this method to get the current bot name for the given user language. Returns <a href="https://core.telegram.org/bots/api#botname">BotName</a> on success.
#[derive(macros::Method)]
#[method(name = "getMyName", response(BotName))]
pub struct GetMyNameRequest {
    pub language_code: Option<String>,
}
/// Use this method to change the bot's description, which is shown in the chat with the bot if the chat is empty. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setMyDescription", response(crate::True))]
pub struct SetMyDescriptionRequest {
    pub description: Option<String>,
    pub language_code: Option<String>,
}
/// Use this method to get the current bot description for the given user language. Returns <a href="https://core.telegram.org/bots/api#botdescription">BotDescription</a> on success.
#[derive(macros::Method)]
#[method(name = "getMyDescription", response(BotDescription))]
pub struct GetMyDescriptionRequest {
    pub language_code: Option<String>,
}
/// Use this method to change the bot's short description, which is shown on the bot's profile page and is sent together with the link when users share the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setMyShortDescription", response(crate::True))]
pub struct SetMyShortDescriptionRequest {
    pub short_description: Option<String>,
    pub language_code: Option<String>,
}
/// Use this method to get the current bot short description for the given user language. Returns <a href="https://core.telegram.org/bots/api#botshortdescription">BotShortDescription</a> on success.
#[derive(macros::Method)]
#[method(name = "getMyShortDescription", response(BotShortDescription))]
pub struct GetMyShortDescriptionRequest {
    pub language_code: Option<String>,
}
/// Use this method to change the bot's menu button in a private chat, or the default menu button. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setChatMenuButton", response(crate::True))]
pub struct SetChatMenuButtonRequest {
    pub chat_id: Option<i64>,
    pub menu_button: Option<MenuButton>,
}
/// Use this method to get the current value of the bot's menu button in a private chat, or the default menu button. Returns <a href="https://core.telegram.org/bots/api#menubutton">MenuButton</a> on success.
#[derive(macros::Method)]
#[method(name = "getChatMenuButton", response(MenuButton))]
pub struct GetChatMenuButtonRequest {
    pub chat_id: Option<i64>,
}
/// Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are free to modify the list before adding the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setMyDefaultAdministratorRights", response(crate::True))]
pub struct SetMyDefaultAdministratorRightsRequest {
    pub rights: Option<ChatAdministratorRights>,
    pub for_channels: Option<bool>,
}
/// Use this method to get the current default administrator rights of the bot. Returns <a href="https://core.telegram.org/bots/api#chatadministratorrights">ChatAdministratorRights</a> on success.
#[derive(macros::Method)]
#[method(name = "getMyDefaultAdministratorRights", response(ChatAdministratorRights))]
pub struct GetMyDefaultAdministratorRightsRequest {
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
    pub user_id: Option<i64>,
    pub chat_id: Option<ChatId>,
    pub gift_id: String,
    pub pay_for_upgrade: Option<bool>,
    pub text: Option<String>,
    pub text_parse_mode: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
}
/// Gifts a Telegram Premium subscription to the given user. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "giftPremiumSubscription", response(crate::True))]
pub struct GiftPremiumSubscriptionRequest {
    pub user_id: i64,
    pub month_count: i64,
    pub star_count: i64,
    pub text: Option<String>,
    pub text_parse_mode: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
}
/// Verifies a user <a href="https://telegram.org/verify#third-party-verification">on behalf of the organization</a> which is represented by the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "verifyUser", response(crate::True))]
pub struct VerifyUserRequest {
    pub user_id: i64,
    pub custom_description: Option<String>,
}
/// Verifies a chat <a href="https://telegram.org/verify#third-party-verification">on behalf of the organization</a> which is represented by the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "verifyChat", response(crate::True))]
pub struct VerifyChatRequest {
    pub chat_id: ChatId,
    pub custom_description: Option<String>,
}
/// Removes verification from a user who is currently verified <a href="https://telegram.org/verify#third-party-verification">on behalf of the organization</a> represented by the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "removeUserVerification", response(crate::True))]
pub struct RemoveUserVerificationRequest {
    pub user_id: i64,
}
/// Removes verification from a chat that is currently verified <a href="https://telegram.org/verify#third-party-verification">on behalf of the organization</a> represented by the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "removeChatVerification", response(crate::True))]
pub struct RemoveChatVerificationRequest {
    pub chat_id: ChatId,
}
/// Marks incoming message as read on behalf of a business account. Requires the <em>can_read_messages</em> business bot right. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "readBusinessMessage", response(crate::True))]
pub struct ReadBusinessMessageRequest {
    pub business_connection_id: String,
    pub chat_id: i64,
    pub message_id: i64,
}
/// Delete messages on behalf of a business account. Requires the <em>can_delete_sent_messages</em> business bot right to delete messages sent by the bot itself, or the <em>can_delete_all_messages</em> business bot right to delete any message. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "deleteBusinessMessages", response(crate::True))]
pub struct DeleteBusinessMessagesRequest {
    pub business_connection_id: String,
    pub message_ids: Vec<i64>,
}
/// Changes the first and last name of a managed business account. Requires the <em>can_change_name</em> business bot right. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setBusinessAccountName", response(crate::True))]
pub struct SetBusinessAccountNameRequest {
    pub business_connection_id: String,
    pub first_name: String,
    pub last_name: Option<String>,
}
/// Changes the username of a managed business account. Requires the <em>can_change_username</em> business bot right. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setBusinessAccountUsername", response(crate::True))]
pub struct SetBusinessAccountUsernameRequest {
    pub business_connection_id: String,
    pub username: Option<String>,
}
/// Changes the bio of a managed business account. Requires the <em>can_change_bio</em> business bot right. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setBusinessAccountBio", response(crate::True))]
pub struct SetBusinessAccountBioRequest {
    pub business_connection_id: String,
    pub bio: Option<String>,
}
/// Changes the profile photo of a managed business account. Requires the <em>can_edit_profile_photo</em> business bot right. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setBusinessAccountProfilePhoto", response(crate::True))]
pub struct SetBusinessAccountProfilePhotoRequest {
    pub business_connection_id: String,
    pub photo: InputProfilePhoto,
    pub is_public: Option<bool>,
}
/// Removes the current profile photo of a managed business account. Requires the <em>can_edit_profile_photo</em> business bot right. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "removeBusinessAccountProfilePhoto", response(crate::True))]
pub struct RemoveBusinessAccountProfilePhotoRequest {
    pub business_connection_id: String,
    pub is_public: Option<bool>,
}
/// Changes the privacy settings pertaining to incoming gifts in a managed business account. Requires the <em>can_change_gift_settings</em> business bot right. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setBusinessAccountGiftSettings", response(crate::True))]
pub struct SetBusinessAccountGiftSettingsRequest {
    pub business_connection_id: String,
    pub show_gift_button: bool,
    pub accepted_gift_types: AcceptedGiftTypes,
}
/// Returns the amount of Telegram Stars owned by a managed business account. Requires the <em>can_view_gifts_and_stars</em> business bot right. Returns <a href="https://core.telegram.org/bots/api#staramount">StarAmount</a> on success.
#[derive(macros::Method)]
#[method(name = "getBusinessAccountStarBalance", response(StarAmount))]
pub struct GetBusinessAccountStarBalanceRequest {
    pub business_connection_id: String,
}
/// Transfers Telegram Stars from the business account balance to the bot's balance. Requires the <em>can_transfer_stars</em> business bot right. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "transferBusinessAccountStars", response(crate::True))]
pub struct TransferBusinessAccountStarsRequest {
    pub business_connection_id: String,
    pub star_count: i64,
}
/// Returns the gifts received and owned by a managed business account. Requires the <em>can_view_gifts_and_stars</em> business bot right. Returns <a href="https://core.telegram.org/bots/api#ownedgifts">OwnedGifts</a> on success.
#[derive(macros::Method)]
#[method(name = "getBusinessAccountGifts", response(OwnedGifts))]
pub struct GetBusinessAccountGiftsRequest {
    pub business_connection_id: String,
    pub exclude_unsaved: Option<bool>,
    pub exclude_saved: Option<bool>,
    pub exclude_unlimited: Option<bool>,
    pub exclude_limited_upgradable: Option<bool>,
    pub exclude_limited_non_upgradable: Option<bool>,
    pub exclude_unique: Option<bool>,
    pub exclude_from_blockchain: Option<bool>,
    pub sort_by_price: Option<bool>,
    pub offset: Option<String>,
    pub limit: Option<i64>,
}
/// Returns the gifts owned and hosted by a user. Returns <a href="https://core.telegram.org/bots/api#ownedgifts">OwnedGifts</a> on success.
#[derive(macros::Method)]
#[method(name = "getUserGifts", response(OwnedGifts))]
pub struct GetUserGiftsRequest {
    pub user_id: i64,
    pub exclude_unlimited: Option<bool>,
    pub exclude_limited_upgradable: Option<bool>,
    pub exclude_limited_non_upgradable: Option<bool>,
    pub exclude_from_blockchain: Option<bool>,
    pub exclude_unique: Option<bool>,
    pub sort_by_price: Option<bool>,
    pub offset: Option<String>,
    pub limit: Option<i64>,
}
/// Returns the gifts owned by a chat. Returns <a href="https://core.telegram.org/bots/api#ownedgifts">OwnedGifts</a> on success.
#[derive(macros::Method)]
#[method(name = "getChatGifts", response(OwnedGifts))]
pub struct GetChatGiftsRequest {
    pub chat_id: ChatId,
    pub exclude_unsaved: Option<bool>,
    pub exclude_saved: Option<bool>,
    pub exclude_unlimited: Option<bool>,
    pub exclude_limited_upgradable: Option<bool>,
    pub exclude_limited_non_upgradable: Option<bool>,
    pub exclude_from_blockchain: Option<bool>,
    pub exclude_unique: Option<bool>,
    pub sort_by_price: Option<bool>,
    pub offset: Option<String>,
    pub limit: Option<i64>,
}
/// Converts a given regular gift to Telegram Stars. Requires the <em>can_convert_gifts_to_stars</em> business bot right. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "convertGiftToStars", response(crate::True))]
pub struct ConvertGiftToStarsRequest {
    pub business_connection_id: String,
    pub owned_gift_id: String,
}
/// Upgrades a given regular gift to a unique gift. Requires the <em>can_transfer_and_upgrade_gifts</em> business bot right. Additionally requires the <em>can_transfer_stars</em> business bot right if the upgrade is paid. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "upgradeGift", response(crate::True))]
pub struct UpgradeGiftRequest {
    pub business_connection_id: String,
    pub owned_gift_id: String,
    pub keep_original_details: Option<bool>,
    pub star_count: Option<i64>,
}
/// Transfers an owned unique gift to another user. Requires the <em>can_transfer_and_upgrade_gifts</em> business bot right. Requires <em>can_transfer_stars</em> business bot right if the transfer is paid. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "transferGift", response(crate::True))]
pub struct TransferGiftRequest {
    pub business_connection_id: String,
    pub owned_gift_id: String,
    pub new_owner_chat_id: i64,
    pub star_count: Option<i64>,
}
/// Posts a story on behalf of a managed business account. Requires the <em>can_manage_stories</em> business bot right. Returns <a href="https://core.telegram.org/bots/api#story">Story</a> on success.
#[derive(macros::Method)]
#[method(name = "postStory", response(Story))]
pub struct PostStoryRequest {
    pub business_connection_id: String,
    pub content: InputStoryContent,
    pub active_period: i64,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub areas: Option<Vec<StoryArea>>,
    pub post_to_chat_page: Option<bool>,
    pub protect_content: Option<bool>,
}
/// Reposts a story on behalf of a business account from another business account. Both business accounts must be managed by the same bot, and the story on the source account must have been posted (or reposted) by the bot. Requires the <em>can_manage_stories</em> business bot right for both business accounts. Returns <a href="https://core.telegram.org/bots/api#story">Story</a> on success.
#[derive(macros::Method)]
#[method(name = "repostStory", response(Story))]
pub struct RepostStoryRequest {
    pub business_connection_id: String,
    pub from_chat_id: i64,
    pub from_story_id: i64,
    pub active_period: i64,
    pub post_to_chat_page: Option<bool>,
    pub protect_content: Option<bool>,
}
/// Edits a story previously posted by the bot on behalf of a managed business account. Requires the <em>can_manage_stories</em> business bot right. Returns <a href="https://core.telegram.org/bots/api#story">Story</a> on success.
#[derive(macros::Method)]
#[method(name = "editStory", response(Story))]
pub struct EditStoryRequest {
    pub business_connection_id: String,
    pub story_id: i64,
    pub content: InputStoryContent,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub areas: Option<Vec<StoryArea>>,
}
/// Deletes a story previously posted by the bot on behalf of a managed business account. Requires the <em>can_manage_stories</em> business bot right. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "deleteStory", response(crate::True))]
pub struct DeleteStoryRequest {
    pub business_connection_id: String,
    pub story_id: i64,
}
/// Use this method to edit text and <a href="https://core.telegram.org/bots/api#games">game</a> messages. On success, if the edited message is not an inline message, the edited <a href="https://core.telegram.org/bots/api#message">Message</a> is returned, otherwise <em>True</em> is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within <strong>48 hours</strong> from the time they were sent.
#[derive(macros::Method)]
#[method(name = "editMessageText", response(MessageOrTrue))]
pub struct EditMessageTextRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: Option<ChatId>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
    pub text: String,
    pub parse_mode: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub link_preview_options: Option<LinkPreviewOptions>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// Use this method to edit captions of messages. On success, if the edited message is not an inline message, the edited <a href="https://core.telegram.org/bots/api#message">Message</a> is returned, otherwise <em>True</em> is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within <strong>48 hours</strong> from the time they were sent.
#[derive(macros::Method)]
#[method(name = "editMessageCaption", response(MessageOrTrue))]
pub struct EditMessageCaptionRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: Option<ChatId>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// Use this method to edit animation, audio, document, photo, or video messages, or to add media to text messages. If a message is part of a message album, then it can be edited only to an audio for audio albums, only to a document for document albums and to a photo or a video otherwise. When an inline message is edited, a new file can't be uploaded; use a previously uploaded file via its file_id or specify a URL. On success, if the edited message is not an inline message, the edited <a href="https://core.telegram.org/bots/api#message">Message</a> is returned, otherwise <em>True</em> is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within <strong>48 hours</strong> from the time they were sent.
#[derive(macros::Method)]
#[method(name = "editMessageMedia", response(MessageOrTrue))]
pub struct EditMessageMediaRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: Option<ChatId>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
    pub media: InputMedia,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// Use this method to edit live location messages. A location can be edited until its <em>live_period</em> expires or editing is explicitly disabled by a call to <a href="https://core.telegram.org/bots/api#stopmessagelivelocation">stopMessageLiveLocation</a>. On success, if the edited message is not an inline message, the edited <a href="https://core.telegram.org/bots/api#message">Message</a> is returned, otherwise <em>True</em> is returned.
#[derive(macros::Method)]
#[method(name = "editMessageLiveLocation", response(MessageOrTrue))]
pub struct EditMessageLiveLocationRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: Option<ChatId>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub live_period: Option<i64>,
    pub horizontal_accuracy: Option<f64>,
    pub heading: Option<i64>,
    pub proximity_alert_radius: Option<i64>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// Use this method to stop updating a live location message before <em>live_period</em> expires. On success, if the message is not an inline message, the edited <a href="https://core.telegram.org/bots/api#message">Message</a> is returned, otherwise <em>True</em> is returned.
#[derive(macros::Method)]
#[method(name = "stopMessageLiveLocation", response(MessageOrTrue))]
pub struct StopMessageLiveLocationRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: Option<ChatId>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// Use this method to edit a checklist on behalf of a connected business account. On success, the edited <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "editMessageChecklist", response(Message))]
pub struct EditMessageChecklistRequest {
    pub business_connection_id: String,
    pub chat_id: i64,
    pub message_id: i64,
    pub checklist: InputChecklist,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// Use this method to edit only the reply markup of messages. On success, if the edited message is not an inline message, the edited <a href="https://core.telegram.org/bots/api#message">Message</a> is returned, otherwise <em>True</em> is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within <strong>48 hours</strong> from the time they were sent.
#[derive(macros::Method)]
#[method(name = "editMessageReplyMarkup", response(MessageOrTrue))]
pub struct EditMessageReplyMarkupRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: Option<ChatId>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// Use this method to stop a poll which was sent by the bot. On success, the stopped <a href="https://core.telegram.org/bots/api#poll">Poll</a> is returned.
#[derive(macros::Method)]
#[method(name = "stopPoll", response(Poll))]
pub struct StopPollRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_id: i64,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// Use this method to approve a suggested post in a direct messages chat. The bot must have the 'can_post_messages' administrator right in the corresponding channel chat. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "approveSuggestedPost", response(crate::True))]
pub struct ApproveSuggestedPostRequest {
    pub chat_id: i64,
    pub message_id: i64,
    pub send_date: Option<i64>,
}
/// Use this method to decline a suggested post in a direct messages chat. The bot must have the 'can_manage_direct_messages' administrator right in the corresponding channel chat. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "declineSuggestedPost", response(crate::True))]
pub struct DeclineSuggestedPostRequest {
    pub chat_id: i64,
    pub message_id: i64,
    pub comment: Option<String>,
}
/// Use this method to delete a message, including service messages, with the following limitations:<br>- A message can only be deleted if it was sent less than 48 hours ago.<br>- Service messages about a supergroup, channel, or forum topic creation can't be deleted.<br>- A dice message in a private chat can only be deleted if it was sent more than 24 hours ago.<br>- Bots can delete outgoing messages in private chats, groups, and supergroups.<br>- Bots can delete incoming messages in private chats.<br>- Bots granted <em>can_post_messages</em> permissions can delete outgoing messages in channels.<br>- If the bot is an administrator of a group, it can delete any message there.<br>- If the bot has <em>can_delete_messages</em> administrator right in a supergroup or a channel, it can delete any message there.<br>- If the bot has <em>can_manage_direct_messages</em> administrator right in a channel, it can delete any message in the corresponding direct messages chat.<br>Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "deleteMessage", response(crate::True))]
pub struct DeleteMessageRequest {
    pub chat_id: ChatId,
    pub message_id: i64,
}
/// Use this method to delete multiple messages simultaneously. If some of the specified messages can't be found, they are skipped. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "deleteMessages", response(crate::True))]
pub struct DeleteMessagesRequest {
    pub chat_id: ChatId,
    pub message_ids: Vec<i64>,
}
/// This object represents a sticker.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Sticker {
    pub file_id: String,
    pub file_unique_id: String,
    pub r#type: String,
    pub width: i64,
    pub height: i64,
    pub is_animated: bool,
    pub is_video: bool,
    pub thumbnail: Option<PhotoSize>,
    pub emoji: Option<String>,
    pub set_name: Option<String>,
    pub premium_animation: Option<File>,
    pub mask_position: Option<MaskPosition>,
    pub custom_emoji_id: Option<String>,
    pub needs_repainting: Option<crate::True>,
    pub file_size: Option<i64>,
}
/// This object represents a sticker set.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StickerSet {
    pub name: String,
    pub title: String,
    pub sticker_type: String,
    pub stickers: Vec<Sticker>,
    pub thumbnail: Option<PhotoSize>,
}
/// This object describes the position on faces where a mask should be placed by default.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MaskPosition {
    pub point: String,
    pub x_shift: f64,
    pub y_shift: f64,
    pub scale: f64,
}
/// This object describes a sticker to be added to a sticker set.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputSticker {
    pub sticker: String,
    pub format: String,
    pub emoji_list: Vec<String>,
    pub mask_position: Option<MaskPosition>,
    pub keywords: Option<Vec<String>>,
}
/// Use this method to send static .WEBP, <a href="https://telegram.org/blog/animated-stickers">animated</a> .TGS, or <a href="https://telegram.org/blog/video-stickers-better-reactions">video</a> .WEBM stickers. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendSticker", response(Message))]
pub struct SendStickerRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_thread_id: Option<i64>,
    pub direct_messages_topic_id: Option<i64>,
    pub sticker: Attachment,
    pub emoji: Option<String>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
/// Use this method to get a sticker set. On success, a <a href="https://core.telegram.org/bots/api#stickerset">StickerSet</a> object is returned.
#[derive(macros::Method)]
#[method(name = "getStickerSet", response(StickerSet))]
pub struct GetStickerSetRequest {
    pub name: String,
}
/// Use this method to get information about custom emoji stickers by their identifiers. Returns an Array of <a href="https://core.telegram.org/bots/api#sticker">Sticker</a> objects.
#[derive(macros::Method)]
#[method(name = "getCustomEmojiStickers", response(Vec<Sticker>))]
pub struct GetCustomEmojiStickersRequest {
    pub custom_emoji_ids: Vec<String>,
}
/// Use this method to upload a file with a sticker for later use in the <a href="https://core.telegram.org/bots/api#createnewstickerset">createNewStickerSet</a>, <a href="https://core.telegram.org/bots/api#addstickertoset">addStickerToSet</a>, or <a href="https://core.telegram.org/bots/api#replacestickerinset">replaceStickerInSet</a> methods (the file can be used multiple times). Returns the uploaded <a href="https://core.telegram.org/bots/api#file">File</a> on success.
#[derive(macros::Method)]
#[method(name = "uploadStickerFile", response(File))]
pub struct UploadStickerFileRequest {
    pub user_id: i64,
    pub sticker: InputFile,
    pub sticker_format: String,
}
/// Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "createNewStickerSet", response(crate::True))]
pub struct CreateNewStickerSetRequest {
    pub user_id: i64,
    pub name: String,
    pub title: String,
    pub stickers: Vec<InputSticker>,
    pub sticker_type: Option<String>,
    pub needs_repainting: Option<bool>,
}
/// Use this method to add a new sticker to a set created by the bot. Emoji sticker sets can have up to 200 stickers. Other sticker sets can have up to 120 stickers. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "addStickerToSet", response(crate::True))]
pub struct AddStickerToSetRequest {
    pub user_id: i64,
    pub name: String,
    pub sticker: InputSticker,
}
/// Use this method to move a sticker in a set created by the bot to a specific position. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setStickerPositionInSet", response(crate::True))]
pub struct SetStickerPositionInSetRequest {
    pub sticker: String,
    pub position: i64,
}
/// Use this method to delete a sticker from a set created by the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "deleteStickerFromSet", response(crate::True))]
pub struct DeleteStickerFromSetRequest {
    pub sticker: String,
}
/// Use this method to replace an existing sticker in a sticker set with a new one. The method is equivalent to calling <a href="https://core.telegram.org/bots/api#deletestickerfromset">deleteStickerFromSet</a>, then <a href="https://core.telegram.org/bots/api#addstickertoset">addStickerToSet</a>, then <a href="https://core.telegram.org/bots/api#setstickerpositioninset">setStickerPositionInSet</a>. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "replaceStickerInSet", response(crate::True))]
pub struct ReplaceStickerInSetRequest {
    pub user_id: i64,
    pub name: String,
    pub old_sticker: String,
    pub sticker: InputSticker,
}
/// Use this method to change the list of emoji assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setStickerEmojiList", response(crate::True))]
pub struct SetStickerEmojiListRequest {
    pub sticker: String,
    pub emoji_list: Vec<String>,
}
/// Use this method to change search keywords assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setStickerKeywords", response(crate::True))]
pub struct SetStickerKeywordsRequest {
    pub sticker: String,
    pub keywords: Option<Vec<String>>,
}
/// Use this method to change the <a href="https://core.telegram.org/bots/api#maskposition">mask position</a> of a mask sticker. The sticker must belong to a sticker set that was created by the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setStickerMaskPosition", response(crate::True))]
pub struct SetStickerMaskPositionRequest {
    pub sticker: String,
    pub mask_position: Option<MaskPosition>,
}
/// Use this method to set the title of a created sticker set. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setStickerSetTitle", response(crate::True))]
pub struct SetStickerSetTitleRequest {
    pub name: String,
    pub title: String,
}
/// Use this method to set the thumbnail of a regular or mask sticker set. The format of the thumbnail file must match the format of the stickers in the set. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setStickerSetThumbnail", response(crate::True))]
pub struct SetStickerSetThumbnailRequest {
    pub name: String,
    pub user_id: i64,
    pub thumbnail: Option<Attachment>,
    pub format: String,
}
/// Use this method to set the thumbnail of a custom emoji sticker set. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setCustomEmojiStickerSetThumbnail", response(crate::True))]
pub struct SetCustomEmojiStickerSetThumbnailRequest {
    pub name: String,
    pub custom_emoji_id: Option<String>,
}
/// Use this method to delete a sticker set that was created by the bot. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "deleteStickerSet", response(crate::True))]
pub struct DeleteStickerSetRequest {
    pub name: String,
}
/// This object represents an incoming inline query. When the user sends an empty query, your bot could return some default or trending results.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,
    pub query: String,
    pub offset: String,
    pub chat_type: Option<String>,
    pub location: Option<Location>,
}
/// Use this method to send answers to an inline query. On success, <em>True</em> is returned.<br>No more than <strong>50</strong> results per query are allowed.
#[derive(macros::Method)]
#[method(name = "answerInlineQuery", response(crate::True))]
pub struct AnswerInlineQueryRequest {
    pub inline_query_id: String,
    pub results: Vec<InlineQueryResult>,
    pub cache_time: Option<i64>,
    pub is_personal: Option<bool>,
    pub next_offset: Option<String>,
    pub button: Option<InlineQueryResultsButton>,
}
/// This object represents a button to be shown above inline query results. You <strong>must</strong> use exactly one of the optional fields.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultsButton {
    pub text: String,
    pub web_app: Option<WebAppInfo>,
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
    pub r#type: String,
    pub id: String,
    pub title: String,
    pub input_message_content: InputMessageContent,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_width: Option<i64>,
    pub thumbnail_height: Option<i64>,
}
/// Represents a link to a photo. By default, this photo will be sent by the user with optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the photo.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultPhoto {
    pub r#type: String,
    pub id: String,
    pub photo_url: String,
    pub thumbnail_url: String,
    pub photo_width: Option<i64>,
    pub photo_height: Option<i64>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to an animated GIF file. By default, this animated GIF file will be sent by the user with optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the animation.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultGif {
    pub r#type: String,
    pub id: String,
    pub gif_url: String,
    pub gif_width: Option<i64>,
    pub gif_height: Option<i64>,
    pub gif_duration: Option<i64>,
    pub thumbnail_url: String,
    pub thumbnail_mime_type: Option<String>,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default, this animated MPEG-4 file will be sent by the user with optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the animation.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultMpeg4Gif {
    pub r#type: String,
    pub id: String,
    pub mpeg4_url: String,
    pub mpeg4_width: Option<i64>,
    pub mpeg4_height: Option<i64>,
    pub mpeg4_duration: Option<i64>,
    pub thumbnail_url: String,
    pub thumbnail_mime_type: Option<String>,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to a page containing an embedded video player or a video file. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the video.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultVideo {
    pub r#type: String,
    pub id: String,
    pub video_url: String,
    pub mime_type: String,
    pub thumbnail_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub video_width: Option<i64>,
    pub video_height: Option<i64>,
    pub video_duration: Option<i64>,
    pub description: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to an MP3 audio file. By default, this audio file will be sent by the user. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the audio.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultAudio {
    pub r#type: String,
    pub id: String,
    pub audio_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub performer: Option<String>,
    pub audio_duration: Option<i64>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to a voice recording in an .OGG container encoded with OPUS. By default, this voice recording will be sent by the user. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the the voice message.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultVoice {
    pub r#type: String,
    pub id: String,
    pub voice_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub voice_duration: Option<i64>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to a file. By default, this file will be sent by the user with an optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the file. Currently, only <strong>.PDF</strong> and <strong>.ZIP</strong> files can be sent using this method.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultDocument {
    pub r#type: String,
    pub id: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub document_url: String,
    pub mime_type: String,
    pub description: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_width: Option<i64>,
    pub thumbnail_height: Option<i64>,
}
/// Represents a location on a map. By default, the location will be sent by the user. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the location.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultLocation {
    pub r#type: String,
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<i64>,
    pub heading: Option<i64>,
    pub proximity_alert_radius: Option<i64>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_width: Option<i64>,
    pub thumbnail_height: Option<i64>,
}
/// Represents a venue. By default, the venue will be sent by the user. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the venue.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultVenue {
    pub r#type: String,
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub google_place_id: Option<String>,
    pub google_place_type: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_width: Option<i64>,
    pub thumbnail_height: Option<i64>,
}
/// Represents a contact with a phone number. By default, this contact will be sent by the user. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the contact.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultContact {
    pub r#type: String,
    pub id: String,
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub vcard: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_width: Option<i64>,
    pub thumbnail_height: Option<i64>,
}
/// Represents a <a href="https://core.telegram.org/bots/api#games">Game</a>.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultGame {
    pub r#type: String,
    pub id: String,
    pub game_short_name: String,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// Represents a link to a photo stored on the Telegram servers. By default, this photo will be sent by the user with an optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the photo.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultCachedPhoto {
    pub r#type: String,
    pub id: String,
    pub photo_file_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to an animated GIF file stored on the Telegram servers. By default, this animated GIF file will be sent by the user with an optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with specified content instead of the animation.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultCachedGif {
    pub r#type: String,
    pub id: String,
    pub gif_file_id: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers. By default, this animated MPEG-4 file will be sent by the user with an optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the animation.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultCachedMpeg4Gif {
    pub r#type: String,
    pub id: String,
    pub mpeg4_file_id: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to a sticker stored on the Telegram servers. By default, this sticker will be sent by the user. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the sticker.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultCachedSticker {
    pub r#type: String,
    pub id: String,
    pub sticker_file_id: String,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to a file stored on the Telegram servers. By default, this file will be sent by the user with an optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the file.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultCachedDocument {
    pub r#type: String,
    pub id: String,
    pub title: String,
    pub document_file_id: String,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to a video file stored on the Telegram servers. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the video.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultCachedVideo {
    pub r#type: String,
    pub id: String,
    pub video_file_id: String,
    pub title: String,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to a voice message stored on the Telegram servers. By default, this voice message will be sent by the user. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the voice message.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultCachedVoice {
    pub r#type: String,
    pub id: String,
    pub voice_file_id: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}
/// Represents a link to an MP3 audio file stored on the Telegram servers. By default, this audio file will be sent by the user. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the audio.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultCachedAudio {
    pub r#type: String,
    pub id: String,
    pub audio_file_id: String,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
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
    pub message_text: String,
    pub parse_mode: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub link_preview_options: Option<LinkPreviewOptions>,
}
/// Represents the <a href="https://core.telegram.org/bots/api#inputmessagecontent">content</a> of a location message to be sent as the result of an inline query.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputLocationMessageContent {
    pub latitude: f64,
    pub longitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<i64>,
    pub heading: Option<i64>,
    pub proximity_alert_radius: Option<i64>,
}
/// Represents the <a href="https://core.telegram.org/bots/api#inputmessagecontent">content</a> of a venue message to be sent as the result of an inline query.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputVenueMessageContent {
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub google_place_id: Option<String>,
    pub google_place_type: Option<String>,
}
/// Represents the <a href="https://core.telegram.org/bots/api#inputmessagecontent">content</a> of a contact message to be sent as the result of an inline query.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputContactMessageContent {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub vcard: Option<String>,
}
/// Represents the <a href="https://core.telegram.org/bots/api#inputmessagecontent">content</a> of an invoice message to be sent as the result of an inline query.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputInvoiceMessageContent {
    pub title: String,
    pub description: String,
    pub payload: String,
    pub provider_token: Option<String>,
    pub currency: String,
    pub prices: Vec<LabeledPrice>,
    pub max_tip_amount: Option<i64>,
    pub suggested_tip_amounts: Option<Vec<i64>>,
    pub provider_data: Option<String>,
    pub photo_url: Option<String>,
    pub photo_size: Option<i64>,
    pub photo_width: Option<i64>,
    pub photo_height: Option<i64>,
    pub need_name: Option<bool>,
    pub need_phone_number: Option<bool>,
    pub need_email: Option<bool>,
    pub need_shipping_address: Option<bool>,
    pub send_phone_number_to_provider: Option<bool>,
    pub send_email_to_provider: Option<bool>,
    pub is_flexible: Option<bool>,
}
/// Represents a <a href="https://core.telegram.org/bots/api#inlinequeryresult">result</a> of an inline query that was chosen by the user and sent to their chat partner.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,
    pub location: Option<Location>,
    pub inline_message_id: Option<String>,
    pub query: String,
}
/// Use this method to set the result of an interaction with a <a href="/bots/webapps">Web App</a> and send a corresponding message on behalf of the user to the chat from which the query originated. On success, a <a href="https://core.telegram.org/bots/api#sentwebappmessage">SentWebAppMessage</a> object is returned.
#[derive(macros::Method)]
#[method(name = "answerWebAppQuery", response(SentWebAppMessage))]
pub struct AnswerWebAppQueryRequest {
    pub web_app_query_id: String,
    pub result: InlineQueryResult,
}
/// Describes an inline message sent by a <a href="/bots/webapps">Web App</a> on behalf of a user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SentWebAppMessage {
    pub inline_message_id: Option<String>,
}
/// Stores a message that can be sent by a user of a Mini App. Returns a <a href="https://core.telegram.org/bots/api#preparedinlinemessage">PreparedInlineMessage</a> object.
#[derive(macros::Method)]
#[method(name = "savePreparedInlineMessage", response(PreparedInlineMessage))]
pub struct SavePreparedInlineMessageRequest {
    pub user_id: i64,
    pub result: InlineQueryResult,
    pub allow_user_chats: Option<bool>,
    pub allow_bot_chats: Option<bool>,
    pub allow_group_chats: Option<bool>,
    pub allow_channel_chats: Option<bool>,
}
/// Describes an inline message to be sent by a user of a Mini App.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PreparedInlineMessage {
    pub id: String,
    pub expiration_date: i64,
}
/// Use this method to send invoices. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendInvoice", response(Message))]
pub struct SendInvoiceRequest {
    pub chat_id: ChatId,
    pub message_thread_id: Option<i64>,
    pub direct_messages_topic_id: Option<i64>,
    pub title: String,
    pub description: String,
    pub payload: String,
    pub provider_token: Option<String>,
    pub currency: String,
    pub prices: Vec<LabeledPrice>,
    pub max_tip_amount: Option<i64>,
    pub suggested_tip_amounts: Option<Vec<i64>>,
    pub start_parameter: Option<String>,
    pub provider_data: Option<String>,
    pub photo_url: Option<String>,
    pub photo_size: Option<i64>,
    pub photo_width: Option<i64>,
    pub photo_height: Option<i64>,
    pub need_name: Option<bool>,
    pub need_phone_number: Option<bool>,
    pub need_email: Option<bool>,
    pub need_shipping_address: Option<bool>,
    pub send_phone_number_to_provider: Option<bool>,
    pub send_email_to_provider: Option<bool>,
    pub is_flexible: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// Use this method to create a link for an invoice. Returns the created invoice link as <em>String</em> on success.
#[derive(macros::Method)]
#[method(name = "createInvoiceLink", response(String))]
pub struct CreateInvoiceLinkRequest {
    pub business_connection_id: Option<String>,
    pub title: String,
    pub description: String,
    pub payload: String,
    pub provider_token: Option<String>,
    pub currency: String,
    pub prices: Vec<LabeledPrice>,
    pub subscription_period: Option<i64>,
    pub max_tip_amount: Option<i64>,
    pub suggested_tip_amounts: Option<Vec<i64>>,
    pub provider_data: Option<String>,
    pub photo_url: Option<String>,
    pub photo_size: Option<i64>,
    pub photo_width: Option<i64>,
    pub photo_height: Option<i64>,
    pub need_name: Option<bool>,
    pub need_phone_number: Option<bool>,
    pub need_email: Option<bool>,
    pub need_shipping_address: Option<bool>,
    pub send_phone_number_to_provider: Option<bool>,
    pub send_email_to_provider: Option<bool>,
    pub is_flexible: Option<bool>,
}
/// If you sent an invoice requesting a shipping address and the parameter <em>is_flexible</em> was specified, the Bot API will send an <a href="https://core.telegram.org/bots/api#update">Update</a> with a <em>shipping_query</em> field to the bot. Use this method to reply to shipping queries. On success, <em>True</em> is returned.
#[derive(macros::Method)]
#[method(name = "answerShippingQuery", response(crate::True))]
pub struct AnswerShippingQueryRequest {
    pub shipping_query_id: String,
    pub ok: bool,
    pub shipping_options: Option<Vec<ShippingOption>>,
    pub error_message: Option<String>,
}
/// Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an <a href="https://core.telegram.org/bots/api#update">Update</a> with the field <em>pre_checkout_query</em>. Use this method to respond to such pre-checkout queries. On success, <em>True</em> is returned. <strong>Note:</strong> The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent.
#[derive(macros::Method)]
#[method(name = "answerPreCheckoutQuery", response(crate::True))]
pub struct AnswerPreCheckoutQueryRequest {
    pub pre_checkout_query_id: String,
    pub ok: bool,
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
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}
/// Refunds a successful payment in <a href="https://t.me/BotNews/90">Telegram Stars</a>. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "refundStarPayment", response(crate::True))]
pub struct RefundStarPaymentRequest {
    pub user_id: i64,
    pub telegram_payment_charge_id: String,
}
/// Allows the bot to cancel or re-enable extension of a subscription paid in Telegram Stars. Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "editUserStarSubscription", response(crate::True))]
pub struct EditUserStarSubscriptionRequest {
    pub user_id: i64,
    pub telegram_payment_charge_id: String,
    pub is_canceled: bool,
}
/// This object represents a portion of the price for goods or services.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: i64,
}
/// This object contains basic information about an invoice.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Invoice {
    pub title: String,
    pub description: String,
    pub start_parameter: String,
    pub currency: String,
    pub total_amount: i64,
}
/// This object represents a shipping address.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ShippingAddress {
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String,
}
/// This object represents information about an order.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OrderInfo {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<ShippingAddress>,
}
/// This object represents one shipping option.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ShippingOption {
    pub id: String,
    pub title: String,
    pub prices: Vec<LabeledPrice>,
}
/// This object contains basic information about a successful payment. Note that if the buyer initiates a chargeback with the relevant payment provider following this transaction, the funds may be debited from your balance. This is outside of Telegram's control.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuccessfulPayment {
    pub currency: String,
    pub total_amount: i64,
    pub invoice_payload: String,
    pub subscription_expiration_date: Option<i64>,
    pub is_recurring: Option<crate::True>,
    pub is_first_recurring: Option<crate::True>,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String,
}
/// This object contains basic information about a refunded payment.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RefundedPayment {
    pub currency: String,
    pub total_amount: i64,
    pub invoice_payload: String,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: Option<String>,
}
/// This object contains information about an incoming shipping query.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ShippingQuery {
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress,
}
/// This object contains information about an incoming pre-checkout query.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PreCheckoutQuery {
    pub id: String,
    pub from: User,
    pub currency: String,
    pub total_amount: i64,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
}
/// This object contains information about a paid media purchase.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaidMediaPurchased {
    pub from: User,
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
    pub r#type: String,
}
/// The withdrawal succeeded.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RevenueWithdrawalStateSucceeded {
    pub r#type: String,
    pub date: i64,
    pub url: String,
}
/// The withdrawal failed and the transaction was refunded.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RevenueWithdrawalStateFailed {
    pub r#type: String,
}
/// Contains information about the affiliate that received a commission via this transaction.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AffiliateInfo {
    pub affiliate_user: Option<User>,
    pub affiliate_chat: Option<Chat>,
    pub commission_per_mille: i64,
    pub amount: i64,
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
    pub r#type: String,
    pub transaction_type: String,
    pub user: User,
    pub affiliate: Option<AffiliateInfo>,
    pub invoice_payload: Option<String>,
    pub subscription_period: Option<i64>,
    pub paid_media: Option<Vec<PaidMedia>>,
    pub paid_media_payload: Option<String>,
    pub gift: Option<Gift>,
    pub premium_subscription_duration: Option<i64>,
}
/// Describes a transaction with a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransactionPartnerChat {
    pub r#type: String,
    pub chat: Chat,
    pub gift: Option<Gift>,
}
/// Describes the affiliate program that issued the affiliate commission received via this transaction.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransactionPartnerAffiliateProgram {
    pub r#type: String,
    pub sponsor_user: Option<User>,
    pub commission_per_mille: i64,
}
/// Describes a withdrawal transaction with Fragment.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransactionPartnerFragment {
    pub r#type: String,
    pub withdrawal_state: Option<RevenueWithdrawalState>,
}
/// Describes a withdrawal transaction to the Telegram Ads platform.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransactionPartnerTelegramAds {
    pub r#type: String,
}
/// Describes a transaction with payment for <a href="https://core.telegram.org/bots/api#paid-broadcasts">paid broadcasting</a>.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransactionPartnerTelegramApi {
    pub r#type: String,
    pub request_count: i64,
}
/// Describes a transaction with an unknown source or recipient.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransactionPartnerOther {
    pub r#type: String,
}
/// Describes a Telegram Star transaction. Note that if the buyer initiates a chargeback with the payment provider from whom they acquired Stars (e.g., Apple, Google) following this transaction, the refunded Stars will be deducted from the bot's balance. This is outside of Telegram's control.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StarTransaction {
    pub id: String,
    pub amount: i64,
    pub nanostar_amount: Option<i64>,
    pub date: i64,
    pub source: Option<TransactionPartner>,
    pub receiver: Option<TransactionPartner>,
}
/// Contains a list of Telegram Star transactions.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StarTransactions {
    pub transactions: Vec<StarTransaction>,
}
/// Describes Telegram Passport data shared with the bot by the user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials,
}
/// This object represents a file uploaded to Telegram Passport. Currently all Telegram Passport files are in JPEG format when decrypted and don't exceed 10MB.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportFile {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: i64,
    pub file_date: i64,
}
/// Describes documents or other Telegram Passport elements shared with the bot by the user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EncryptedPassportElement {
    pub r#type: String,
    pub data: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub files: Option<Vec<PassportFile>>,
    pub front_side: Option<PassportFile>,
    pub reverse_side: Option<PassportFile>,
    pub selfie: Option<PassportFile>,
    pub translation: Option<Vec<PassportFile>>,
    pub hash: String,
}
/// Describes data required for decrypting and authenticating <a href="https://core.telegram.org/bots/api#encryptedpassportelement">EncryptedPassportElement</a>. See the <a href="/passport#receiving-information">Telegram Passport Documentation</a> for a complete description of the data decryption and authentication processes.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EncryptedCredentials {
    pub data: String,
    pub hash: String,
    pub secret: String,
}
/// Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns <em>True</em> on success.
#[derive(macros::Method)]
#[method(name = "setPassportDataErrors", response(crate::True))]
pub struct SetPassportDataErrorsRequest {
    pub user_id: i64,
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
    pub source: String,
    pub r#type: String,
    pub field_name: String,
    pub data_hash: String,
    pub message: String,
}
/// Represents an issue with the front side of a document. The error is considered resolved when the file with the front side of the document changes.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorFrontSide {
    pub source: String,
    pub r#type: String,
    pub file_hash: String,
    pub message: String,
}
/// Represents an issue with the reverse side of a document. The error is considered resolved when the file with reverse side of the document changes.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorReverseSide {
    pub source: String,
    pub r#type: String,
    pub file_hash: String,
    pub message: String,
}
/// Represents an issue with the selfie with a document. The error is considered resolved when the file with the selfie changes.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorSelfie {
    pub source: String,
    pub r#type: String,
    pub file_hash: String,
    pub message: String,
}
/// Represents an issue with a document scan. The error is considered resolved when the file with the document scan changes.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorFile {
    pub source: String,
    pub r#type: String,
    pub file_hash: String,
    pub message: String,
}
/// Represents an issue with a list of scans. The error is considered resolved when the list of files containing the scans changes.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorFiles {
    pub source: String,
    pub r#type: String,
    pub file_hashes: Vec<String>,
    pub message: String,
}
/// Represents an issue with one of the files that constitute the translation of a document. The error is considered resolved when the file changes.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorTranslationFile {
    pub source: String,
    pub r#type: String,
    pub file_hash: String,
    pub message: String,
}
/// Represents an issue with the translated version of a document. The error is considered resolved when a file with the document translation change.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorTranslationFiles {
    pub source: String,
    pub r#type: String,
    pub file_hashes: Vec<String>,
    pub message: String,
}
/// Represents an issue in an unspecified place. The error is considered resolved when new data is added.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorUnspecified {
    pub source: String,
    pub r#type: String,
    pub element_hash: String,
    pub message: String,
}
/// Use this method to send a game. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(macros::Method)]
#[method(name = "sendGame", response(Message))]
pub struct SendGameRequest {
    pub business_connection_id: Option<String>,
    pub chat_id: i64,
    pub message_thread_id: Option<i64>,
    pub game_short_name: String,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
/// This object represents a game. Use BotFather to create and edit games, their short names will act as unique identifiers.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    pub text: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
}
/// A placeholder, currently holds no information. Use <a href="https://t.me/botfather">BotFather</a> to set up your game.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CallbackGame;
/// Use this method to set the score of the specified user in a game message. On success, if the message is not an inline message, the <a href="https://core.telegram.org/bots/api#message">Message</a> is returned, otherwise <em>True</em> is returned. Returns an error, if the new score is not greater than the user's current score in the chat and <em>force</em> is <em>False</em>.
#[derive(macros::Method)]
#[method(name = "setGameScore", response(MessageOrTrue))]
pub struct SetGameScoreRequest {
    pub user_id: i64,
    pub score: i64,
    pub force: Option<bool>,
    pub disable_edit_message: Option<bool>,
    pub chat_id: Option<i64>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
}
/// Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. Returns an Array of <a href="https://core.telegram.org/bots/api#gamehighscore">GameHighScore</a> objects.
#[derive(macros::Method)]
#[method(name = "getGameHighScores", response(Vec<GameHighScore>))]
pub struct GetGameHighScoresRequest {
    pub user_id: i64,
    pub chat_id: Option<i64>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
}
/// This object represents one row of the high scores table for a game.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GameHighScore {
    pub position: i64,
    pub user: User,
    pub score: i64,
}
