// this file is auto-generated

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum InputFileOrString {
    InputFile(InputFile),
    String(String),
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum IntegerOrString {
    Integer(i64),
    String(String),
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum InlineKeyboardMarkupOrReplyKeyboardMarkupOrReplyKeyboardRemoveOrForceReply {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum MessageOrTrue {
    Message(Message),
    True(crate::True),
}
/// This <a href="https://core.telegram.org/bots/api#available-types">object</a> represents an incoming update.<br>At most <strong>one</strong> of the optional parameters can be present in any given update.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Update {}
/// Use this method to receive incoming updates using long polling (<a href="https://en.wikipedia.org/wiki/Push_technology#Long_polling">wiki</a>). Returns an Array of <a href="https://core.telegram.org/bots/api#update">Update</a> objects.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getUpdates", response(Vec<Update>))]
pub struct GetUpdatesRequest {
}
/// Use this method to specify a URL and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified URL, containing a JSON-serialized <a href="https://core.telegram.org/bots/api#update">Update</a>. In case of an unsuccessful request (a request with response <a href="https://en.wikipedia.org/wiki/List_of_HTTP_status_codes">HTTP status code</a> different from <code>2XY</code>), we will repeat the request and give up after a reasonable amount of attempts. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setWebhook", response(crate::True))]
pub struct SetWebhookRequest {
}
/// Use this method to remove webhook integration if you decide to switch back to <a href="https://core.telegram.org/bots/api#getupdates">getUpdates</a>. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "deleteWebhook", response(crate::True))]
pub struct DeleteWebhookRequest {
}
/// Use this method to get current webhook status. Requires no parameters. On success, returns a <a href="https://core.telegram.org/bots/api#webhookinfo">WebhookInfo</a> object. If the bot is using <a href="https://core.telegram.org/bots/api#getupdates">getUpdates</a>, will return an object with the <em>url</em> field empty.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getWebhookInfo", response(WebhookInfo))]
pub struct GetWebhookInfoRequest;
/// Describes the current status of a webhook.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WebhookInfo {}
/// This object represents a Telegram user or bot.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {}
/// This object represents a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Chat {}
/// This object contains full information about a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatFullInfo {}
/// This object represents a message.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Message {}
/// This object represents a unique message identifier.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageId {}
/// This object describes a message that was deleted or is otherwise inaccessible to the bot.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InaccessibleMessage {}
/// This object describes a message that can be inaccessible to the bot.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum MaybeInaccessibleMessage {
    Message(Message),
    InaccessibleMessage(InaccessibleMessage),
}
/// This object represents one special entity in a text message. For example, hashtags, usernames, URLs, etc.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageEntity {}
/// This object contains information about the quoted part of a message that is replied to by the given message.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TextQuote {}
/// This object contains information about a message that is being replied to, which may come from another chat or forum topic.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExternalReplyInfo {}
/// Describes reply parameters for the message that is being sent.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReplyParameters {}
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
pub struct MessageOriginUser {}
/// The message was originally sent by an unknown user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageOriginHiddenUser {}
/// The message was originally sent on behalf of a chat to a group chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageOriginChat {}
/// The message was originally sent to a channel chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageOriginChannel {}
/// This object represents one size of a photo or a <a href="https://core.telegram.org/bots/api#document">file</a> / <a href="https://core.telegram.org/bots/api#sticker">sticker</a> thumbnail.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PhotoSize {}
/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video without sound).
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Animation {}
/// This object represents an audio file to be treated as music by the Telegram clients.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Audio {}
/// This object represents a general file (as opposed to <a href="https://core.telegram.org/bots/api#photosize">photos</a>, <a href="https://core.telegram.org/bots/api#voice">voice messages</a> and <a href="https://core.telegram.org/bots/api#audio">audio files</a>).
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Document {}
/// This object represents a story.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Story {}
/// This object represents a video file.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Video {}
/// This object represents a <a href="https://telegram.org/blog/video-messages-and-telescope">video message</a> (available in Telegram apps as of <a href="https://telegram.org/blog/video-messages-and-telescope">v.4.0</a>).
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VideoNote {}
/// This object represents a voice note.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Voice {}
/// Describes the paid media added to a message.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaidMediaInfo {}
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
pub struct PaidMediaPreview {}
/// The paid media is a photo.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaidMediaPhoto {}
/// The paid media is a video.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaidMediaVideo {}
/// This object represents a phone contact.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Contact {}
/// This object represents an animated emoji that displays a random value.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Dice {}
/// This object contains information about one answer option in a poll.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PollOption {}
/// This object contains information about one answer option in a poll to be sent.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputPollOption {}
/// This object represents an answer of a user in a non-anonymous poll.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PollAnswer {}
/// This object contains information about a poll.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Poll {}
/// Describes a task in a checklist.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChecklistTask {}
/// Describes a checklist.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Checklist {}
/// Describes a task to add to a checklist.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputChecklistTask {}
/// Describes a checklist to create.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputChecklist {}
/// Describes a service message about checklist tasks marked as done or not done.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChecklistTasksDone {}
/// Describes a service message about tasks added to a checklist.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChecklistTasksAdded {}
/// This object represents a point on the map.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Location {}
/// This object represents a venue.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Venue {}
/// Describes data sent from a <a href="/bots/webapps">Web App</a> to the bot.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WebAppData {}
/// This object represents the content of a service message, sent whenever a user in the chat triggers a proximity alert set by another user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProximityAlertTriggered {}
/// This object represents a service message about a change in auto-delete timer settings.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageAutoDeleteTimerChanged {}
/// This object represents a service message about a user boosting a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatBoostAdded {}
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
pub struct BackgroundFillSolid {}
/// The background is a gradient fill.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BackgroundFillGradient {}
/// The background is a freeform gradient that rotates after every message in the chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BackgroundFillFreeformGradient {}
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
pub struct BackgroundTypeFill {}
/// The background is a wallpaper in the JPEG format.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BackgroundTypeWallpaper {}
/// The background is a .PNG or .TGV (gzipped subset of SVG with MIME type “application/x-tgwallpattern”) pattern to be combined with the background fill chosen by the user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BackgroundTypePattern {}
/// The background is taken directly from a built-in chat theme.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BackgroundTypeChatTheme {}
/// This object represents a chat background.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatBackground {}
/// This object represents a service message about a new forum topic created in the chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForumTopicCreated {}
/// This object represents a service message about a forum topic closed in the chat. Currently holds no information.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForumTopicClosed {}
/// This object represents a service message about an edited forum topic.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForumTopicEdited {}
/// This object represents a service message about a forum topic reopened in the chat. Currently holds no information.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForumTopicReopened {}
/// This object represents a service message about General forum topic hidden in the chat. Currently holds no information.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GeneralForumTopicHidden {}
/// This object represents a service message about General forum topic unhidden in the chat. Currently holds no information.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GeneralForumTopicUnhidden {}
/// This object contains information about a user that was shared with the bot using a <a href="https://core.telegram.org/bots/api#keyboardbuttonrequestusers">KeyboardButtonRequestUsers</a> button.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SharedUser {}
/// This object contains information about the users whose identifiers were shared with the bot using a <a href="https://core.telegram.org/bots/api#keyboardbuttonrequestusers">KeyboardButtonRequestUsers</a> button.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UsersShared {}
/// This object contains information about a chat that was shared with the bot using a <a href="https://core.telegram.org/bots/api#keyboardbuttonrequestchat">KeyboardButtonRequestChat</a> button.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatShared {}
/// This object represents a service message about a user allowing a bot to write messages after adding it to the attachment menu, launching a Web App from a link, or accepting an explicit request from a Web App sent by the method <a href="/bots/webapps#initializing-mini-apps">requestWriteAccess</a>.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WriteAccessAllowed {}
/// This object represents a service message about a video chat scheduled in the chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VideoChatScheduled {}
/// This object represents a service message about a video chat started in the chat. Currently holds no information.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VideoChatStarted {}
/// This object represents a service message about a video chat ended in the chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VideoChatEnded {}
/// This object represents a service message about new members invited to a video chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VideoChatParticipantsInvited {}
/// Describes a service message about a change in the price of paid messages within a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaidMessagePriceChanged {}
/// Describes a service message about a change in the price of direct messages sent to a channel chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DirectMessagePriceChanged {}
/// Describes a service message about the approval of a suggested post.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuggestedPostApproved {}
/// Describes a service message about the failed approval of a suggested post. Currently, only caused by insufficient user funds at the time of approval.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuggestedPostApprovalFailed {}
/// Describes a service message about the rejection of a suggested post.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuggestedPostDeclined {}
/// Describes a service message about a successful payment for a suggested post.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuggestedPostPaid {}
/// Describes a service message about a payment refund for a suggested post.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuggestedPostRefunded {}
/// This object represents a service message about the creation of a scheduled giveaway.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GiveawayCreated {}
/// This object represents a message about a scheduled giveaway.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Giveaway {}
/// This object represents a message about the completion of a giveaway with public winners.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GiveawayWinners {}
/// This object represents a service message about the completion of a giveaway without public winners.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GiveawayCompleted {}
/// Describes the options used for link preview generation.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LinkPreviewOptions {}
/// Describes the price of a suggested post.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuggestedPostPrice {}
/// Contains information about a suggested post.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuggestedPostInfo {}
/// Contains parameters of a post that is being suggested by the bot.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuggestedPostParameters {}
/// Describes a topic of a direct messages chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DirectMessagesTopic {}
/// This object represent a user's profile pictures.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserProfilePhotos {}
/// This object represents a file ready to be downloaded. The file can be downloaded via the link <code>https://api.telegram.org/file/bot<token>/<file_path></code>. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling <a href="https://core.telegram.org/bots/api#getfile">getFile</a>.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct File {}
/// Describes a <a href="/bots/webapps">Web App</a>.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WebAppInfo {}
/// This object represents a <a href="/bots/features#keyboards">custom keyboard</a> with reply options (see <a href="/bots/features#keyboards">Introduction to bots</a> for details and examples). Not supported in channels and for messages sent on behalf of a Telegram Business account.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReplyKeyboardMarkup {}
/// This object represents one button of the reply keyboard. At most one of the optional fields must be used to specify type of the button. For simple text buttons, <em>String</em> can be used instead of this object to specify the button text.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct KeyboardButton {}
/// This object defines the criteria used to request suitable users. Information about the selected users will be shared with the bot when the corresponding button is pressed. <a href="/bots/features#chat-and-user-selection">More about requesting users »</a>
#[derive(serde::Serialize, serde::Deserialize)]
pub struct KeyboardButtonRequestUsers {}
/// This object defines the criteria used to request a suitable chat. Information about the selected chat will be shared with the bot when the corresponding button is pressed. The bot will be granted requested rights in the chat if appropriate. <a href="/bots/features#chat-and-user-selection">More about requesting chats »</a>.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct KeyboardButtonRequestChat {}
/// This object represents type of a poll, which is allowed to be created and sent when the corresponding button is pressed.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct KeyboardButtonPollType {}
/// Upon receiving a message with this object, Telegram clients will remove the current custom keyboard and display the default letter-keyboard. By default, custom keyboards are displayed until a new keyboard is sent by a bot. An exception is made for one-time keyboards that are hidden immediately after the user presses a button (see <a href="https://core.telegram.org/bots/api#replykeyboardmarkup">ReplyKeyboardMarkup</a>). Not supported in channels and for messages sent on behalf of a Telegram Business account.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReplyKeyboardRemove {}
/// This object represents an <a href="/bots/features#inline-keyboards">inline keyboard</a> that appears right next to the message it belongs to.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineKeyboardMarkup {}
/// This object represents one button of an inline keyboard. Exactly one of the optional fields must be used to specify type of the button.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineKeyboardButton {}
/// This object represents a parameter of the inline keyboard button used to automatically authorize a user. Serves as a great replacement for the <a href="/widgets/login">Telegram Login Widget</a> when the user is coming from Telegram. All the user needs to do is tap/click a button and confirm that they want to log in:
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginUrl {}
/// This object represents an inline button that switches the current user to inline mode in a chosen chat, with an optional default inline query.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchInlineQueryChosenChat {}
/// This object represents an inline keyboard button that copies specified text to the clipboard.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CopyTextButton {}
/// This object represents an incoming callback query from a callback button in an <a href="/bots/features#inline-keyboards">inline keyboard</a>. If the button that originated the query was attached to a message sent by the bot, the field <em>message</em> will be present. If the button was attached to a message sent via the bot (in <a href="https://core.telegram.org/bots/api#inline-mode">inline mode</a>), the field <em>inline_message_id</em> will be present. Exactly one of the fields <em>data</em> or <em>game_short_name</em> will be present.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CallbackQuery {}
/// Upon receiving a message with this object, Telegram clients will display a reply interface to the user (act as if the user has selected the bot's message and tapped 'Reply'). This can be extremely useful if you want to create user-friendly step-by-step interfaces without having to sacrifice <a href="/bots/features#privacy-mode">privacy mode</a>. Not supported in channels and for messages sent on behalf of a Telegram Business account.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForceReply {}
/// This object represents a chat photo.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatPhoto {}
/// Represents an invite link for a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatInviteLink {}
/// Represents the rights of an administrator in a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatAdministratorRights {}
/// This object represents changes in the status of a chat member.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatMemberUpdated {}
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
pub struct ChatMemberOwner {}
/// Represents a <a href="https://core.telegram.org/bots/api#chatmember">chat member</a> that has some additional privileges.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatMemberAdministrator {}
/// Represents a <a href="https://core.telegram.org/bots/api#chatmember">chat member</a> that has no additional privileges or restrictions.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatMemberMember {}
/// Represents a <a href="https://core.telegram.org/bots/api#chatmember">chat member</a> that is under certain restrictions in the chat. Supergroups only.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatMemberRestricted {}
/// Represents a <a href="https://core.telegram.org/bots/api#chatmember">chat member</a> that isn't currently a member of the chat, but may join it themselves.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatMemberLeft {}
/// Represents a <a href="https://core.telegram.org/bots/api#chatmember">chat member</a> that was banned in the chat and can't return to the chat or view chat messages.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatMemberBanned {}
/// Represents a join request sent to a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatJoinRequest {}
/// Describes actions that a non-administrator user is allowed to take in a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatPermissions {}
/// Describes the birthdate of a user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Birthdate {}
/// Contains information about the start page settings of a Telegram Business account.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BusinessIntro {}
/// Contains information about the location of a Telegram Business account.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BusinessLocation {}
/// Describes an interval of time during which a business is open.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BusinessOpeningHoursInterval {}
/// Describes the opening hours of a business.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BusinessOpeningHours {}
/// This object describes the rating of a user based on their Telegram Star spendings.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserRating {}
/// Describes the position of a clickable area within a story.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StoryAreaPosition {}
/// Describes the physical address of a location.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LocationAddress {}
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
pub struct StoryAreaTypeLocation {}
/// Describes a story area pointing to a suggested reaction. Currently, a story can have up to 5 suggested reaction areas.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StoryAreaTypeSuggestedReaction {}
/// Describes a story area pointing to an HTTP or tg:// link. Currently, a story can have up to 3 link areas.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StoryAreaTypeLink {}
/// Describes a story area containing weather information. Currently, a story can have up to 3 weather areas.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StoryAreaTypeWeather {}
/// Describes a story area pointing to a unique gift. Currently, a story can have at most 1 unique gift area.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StoryAreaTypeUniqueGift {}
/// Describes a clickable area on a story media.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StoryArea {}
/// Represents a location to which a chat is connected.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatLocation {}
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
pub struct ReactionTypeEmoji {}
/// The reaction is based on a custom emoji.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReactionTypeCustomEmoji {}
/// The reaction is paid.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReactionTypePaid {}
/// Represents a reaction added to a message along with the number of times it was added.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReactionCount {}
/// This object represents a change of a reaction on a message performed by a user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageReactionUpdated {}
/// This object represents reaction changes on a message with anonymous reactions.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageReactionCountUpdated {}
/// This object represents a forum topic.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForumTopic {}
/// This object describes the background of a gift.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GiftBackground {}
/// This object represents a gift that can be sent by the bot.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Gift {}
/// This object represent a list of gifts.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Gifts {}
/// This object describes the model of a unique gift.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UniqueGiftModel {}
/// This object describes the symbol shown on the pattern of a unique gift.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UniqueGiftSymbol {}
/// This object describes the colors of the backdrop of a unique gift.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UniqueGiftBackdropColors {}
/// This object describes the backdrop of a unique gift.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UniqueGiftBackdrop {}
/// This object contains information about the color scheme for a user's name, message replies and link previews based on a unique gift.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UniqueGiftColors {}
/// This object describes a unique gift that was upgraded from a regular gift.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UniqueGift {}
/// Describes a service message about a regular gift that was sent or received.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GiftInfo {}
/// Describes a service message about a unique gift that was sent or received.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UniqueGiftInfo {}
/// This object describes a gift received and owned by a user or a chat.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum OwnedGift {
    Regular(OwnedGiftRegular),
    Unique(OwnedGiftUnique),
}
/// Describes a regular gift owned by a user or a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OwnedGiftRegular {}
/// Describes a unique gift received and owned by a user or a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OwnedGiftUnique {}
/// Contains the list of gifts received and owned by a user or a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OwnedGifts {}
/// This object describes the types of gifts that can be gifted to a user or a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AcceptedGiftTypes {}
/// Describes an amount of Telegram Stars.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StarAmount {}
/// This object represents a bot command.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotCommand {}
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
pub struct BotCommandScopeDefault {}
/// Represents the <a href="https://core.telegram.org/bots/api#botcommandscope">scope</a> of bot commands, covering all private chats.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotCommandScopeAllPrivateChats {}
/// Represents the <a href="https://core.telegram.org/bots/api#botcommandscope">scope</a> of bot commands, covering all group and supergroup chats.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotCommandScopeAllGroupChats {}
/// Represents the <a href="https://core.telegram.org/bots/api#botcommandscope">scope</a> of bot commands, covering all group and supergroup chat administrators.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotCommandScopeAllChatAdministrators {}
/// Represents the <a href="https://core.telegram.org/bots/api#botcommandscope">scope</a> of bot commands, covering a specific chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotCommandScopeChat {}
/// Represents the <a href="https://core.telegram.org/bots/api#botcommandscope">scope</a> of bot commands, covering all administrators of a specific group or supergroup chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotCommandScopeChatAdministrators {}
/// Represents the <a href="https://core.telegram.org/bots/api#botcommandscope">scope</a> of bot commands, covering a specific member of a group or supergroup chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotCommandScopeChatMember {}
/// This object represents the bot's name.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotName {}
/// This object represents the bot's description.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotDescription {}
/// This object represents the bot's short description.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BotShortDescription {}
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
pub struct MenuButtonCommands {}
/// Represents a menu button, which launches a <a href="/bots/webapps">Web App</a>.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MenuButtonWebApp {}
/// Describes that no specific value for the menu button was set.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MenuButtonDefault {}
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
pub struct ChatBoostSourcePremium {}
/// The boost was obtained by the creation of Telegram Premium gift codes to boost a chat. Each such code boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatBoostSourceGiftCode {}
/// The boost was obtained by the creation of a Telegram Premium or a Telegram Star giveaway. This boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription for Telegram Premium giveaways and <em>prize_star_count</em> / 500 times for one year for Telegram Star giveaways.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatBoostSourceGiveaway {}
/// This object contains information about a chat boost.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatBoost {}
/// This object represents a boost added to a chat or changed.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatBoostUpdated {}
/// This object represents a boost removed from a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatBoostRemoved {}
/// This object represents a list of boosts added to a chat by a user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserChatBoosts {}
/// Represents the rights of a business bot.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BusinessBotRights {}
/// Describes the connection of the bot with a business account.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BusinessConnection {}
/// This object is received when messages are deleted from a connected business account.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BusinessMessagesDeleted {}
/// Describes why a request was unsuccessful.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResponseParameters {}
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
pub struct InputMediaPhoto {}
/// Represents a video to be sent.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputMediaVideo {}
/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputMediaAnimation {}
/// Represents an audio file to be treated as music to be sent.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputMediaAudio {}
/// Represents a general file to be sent.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputMediaDocument {}
/// This object represents the contents of a file to be uploaded. Must be posted using multipart/form-data in the usual way that files are uploaded via the browser.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputFile {}
/// This object describes the paid media to be sent.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum InputPaidMedia {
    Photo(InputPaidMediaPhoto),
    Video(InputPaidMediaVideo),
}
/// The paid media to send is a photo.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputPaidMediaPhoto {}
/// The paid media to send is a video.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputPaidMediaVideo {}
/// This object describes a profile photo to set.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum InputProfilePhoto {
    Static(InputProfilePhotoStatic),
    Animated(InputProfilePhotoAnimated),
}
/// A static profile photo in the .JPG format.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputProfilePhotoStatic {}
/// An animated profile photo in the MPEG4 format.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputProfilePhotoAnimated {}
/// This object describes the content of a story to post.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum InputStoryContent {
    Photo(InputStoryContentPhoto),
    Video(InputStoryContentVideo),
}
/// Describes a photo to post as a story.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputStoryContentPhoto {}
/// Describes a video to post as a story.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputStoryContentVideo {}
/// A simple method for testing your bot's authentication token. Requires no parameters. Returns basic information about the bot in form of a <a href="https://core.telegram.org/bots/api#user">User</a> object.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getMe", response(User))]
pub struct GetMeRequest;
/// Use this method to log out from the cloud Bot API server before launching the bot locally. You <strong>must</strong> log out the bot before running it locally, otherwise there is no guarantee that the bot will receive updates. After a successful call, you can immediately log in on a local server, but will not be able to log in back to the cloud Bot API server for 10 minutes. Returns <em>True</em> on success. Requires no parameters.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "logOut", response(crate::True))]
pub struct LogOutRequest;
/// Use this method to close the bot instance before moving it from one local server to another. You need to delete the webhook before calling this method to ensure that the bot isn't launched again after server restart. The method will return error 429 in the first 10 minutes after the bot is launched. Returns <em>True</em> on success. Requires no parameters.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "close", response(crate::True))]
pub struct CloseRequest;
/// Use this method to send text messages. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "sendMessage", response(Message))]
pub struct SendMessageRequest {
}
/// Use this method to forward messages of any kind. Service messages and messages with protected content can't be forwarded. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "forwardMessage", response(Message))]
pub struct ForwardMessageRequest {
}
/// Use this method to forward multiple messages of any kind. If some of the specified messages can't be found or forwarded, they are skipped. Service messages and messages with protected content can't be forwarded. Album grouping is kept for forwarded messages. On success, an array of <a href="https://core.telegram.org/bots/api#messageid">MessageId</a> of the sent messages is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "forwardMessages", response(MessageId))]
pub struct ForwardMessagesRequest {
}
/// Use this method to copy messages of any kind. Service messages, paid media messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz <a href="https://core.telegram.org/bots/api#poll">poll</a> can be copied only if the value of the field <em>correct_option_id</em> is known to the bot. The method is analogous to the method <a href="https://core.telegram.org/bots/api#forwardmessage">forwardMessage</a>, but the copied message doesn't have a link to the original message. Returns the <a href="https://core.telegram.org/bots/api#messageid">MessageId</a> of the sent message on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "copyMessage", response(MessageId))]
pub struct CopyMessageRequest {
}
/// Use this method to copy messages of any kind. If some of the specified messages can't be found or copied, they are skipped. Service messages, paid media messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz <a href="https://core.telegram.org/bots/api#poll">poll</a> can be copied only if the value of the field <em>correct_option_id</em> is known to the bot. The method is analogous to the method <a href="https://core.telegram.org/bots/api#forwardmessages">forwardMessages</a>, but the copied messages don't have a link to the original message. Album grouping is kept for copied messages. On success, an array of <a href="https://core.telegram.org/bots/api#messageid">MessageId</a> of the sent messages is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "copyMessages", response(MessageId))]
pub struct CopyMessagesRequest {
}
/// Use this method to send photos. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "sendPhoto", response(Message))]
pub struct SendPhotoRequest {
}
/// Use this method to send audio files, if you want Telegram clients to display them in the music player. Your audio must be in the .MP3 or .M4A format. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned. Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "sendAudio", response(Message))]
pub struct SendAudioRequest {
}
/// Use this method to send general files. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned. Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "sendDocument", response(Message))]
pub struct SendDocumentRequest {
}
/// Use this method to send video files, Telegram clients support MPEG4 videos (other formats may be sent as <a href="https://core.telegram.org/bots/api#document">Document</a>). On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned. Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "sendVideo", response(Message))]
pub struct SendVideoRequest {
}
/// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "sendAnimation", response(Message))]
pub struct SendAnimationRequest {
}
/// Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message. For this to work, your audio must be in an .OGG file encoded with OPUS, or in .MP3 format, or in .M4A format (other formats may be sent as <a href="https://core.telegram.org/bots/api#audio">Audio</a> or <a href="https://core.telegram.org/bots/api#document">Document</a>). On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "sendVoice", response(Message))]
pub struct SendVoiceRequest {
}
/// As of <a href="https://telegram.org/blog/video-messages-and-telescope">v.4.0</a>, Telegram clients support rounded square MPEG4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "sendVideoNote", response(Message))]
pub struct SendVideoNoteRequest {
}
/// Use this method to send paid media. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "sendPaidMedia", response(Message))]
pub struct SendPaidMediaRequest {
}
/// Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of <a href="https://core.telegram.org/bots/api#message">Message</a> objects that were sent is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "sendMediaGroup", response(Message))]
pub struct SendMediaGroupRequest {
}
/// Use this method to send point on the map. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "sendLocation", response(Message))]
pub struct SendLocationRequest {
}
/// Use this method to send information about a venue. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "sendVenue", response(Message))]
pub struct SendVenueRequest {
}
/// Use this method to send phone contacts. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "sendContact", response(Message))]
pub struct SendContactRequest {
}
/// Use this method to send a native poll. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "sendPoll", response(Message))]
pub struct SendPollRequest {
}
/// Use this method to send a checklist on behalf of a connected business account. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "sendChecklist", response(Message))]
pub struct SendChecklistRequest {
}
/// Use this method to send an animated emoji that will display a random value. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "sendDice", response(Message))]
pub struct SendDiceRequest {
}
/// Use this method to stream a partial message to a user while the message is being generated; supported only for bots with forum topic mode enabled. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "sendMessageDraft", response(crate::True))]
pub struct SendMessageDraftRequest {
}
/// Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "sendChatAction", response(crate::True))]
pub struct SendChatActionRequest {
}
/// Use this method to change the chosen reactions on a message. Service messages of some types can't be reacted to. Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel. Bots can't use paid reactions. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setMessageReaction", response(crate::True))]
pub struct SetMessageReactionRequest {
}
/// Use this method to get a list of profile pictures for a user. Returns a <a href="https://core.telegram.org/bots/api#userprofilephotos">UserProfilePhotos</a> object.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getUserProfilePhotos", response(UserProfilePhotos))]
pub struct GetUserProfilePhotosRequest {
}
/// Changes the emoji status for a given user that previously allowed the bot to manage their emoji status via the Mini App method <a href="/bots/webapps#initializing-mini-apps">requestEmojiStatusAccess</a>. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setUserEmojiStatus", response(crate::True))]
pub struct SetUserEmojiStatusRequest {
}
/// Use this method to get basic information about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a <a href="https://core.telegram.org/bots/api#file">File</a> object is returned. The file can then be downloaded via the link <code>https://api.telegram.org/file/bot<token>/<file_path></code>, where <code><file_path></code> is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling <a href="https://core.telegram.org/bots/api#getfile">getFile</a> again.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getFile", response(File))]
pub struct GetFileRequest {
}
/// Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless <a href="https://core.telegram.org/bots/api#unbanchatmember">unbanned</a> first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "banChatMember", response(crate::True))]
pub struct BanChatMemberRequest {
}
/// Use this method to unban a previously banned user in a supergroup or channel. The user will <strong>not</strong> return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it. So if the user is a member of the chat they will also be <strong>removed</strong> from the chat. If you don't want this, use the parameter <em>only_if_banned</em>. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "unbanChatMember", response(crate::True))]
pub struct UnbanChatMemberRequest {
}
/// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass <em>True</em> for all permissions to lift restrictions from a user. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "restrictChatMember", response(crate::True))]
pub struct RestrictChatMemberRequest {
}
/// Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass <em>False</em> for all boolean parameters to demote a user. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "promoteChatMember", response(crate::True))]
pub struct PromoteChatMemberRequest {
}
/// Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setChatAdministratorCustomTitle", response(crate::True))]
pub struct SetChatAdministratorCustomTitleRequest {
}
/// Use this method to ban a channel chat in a supergroup or a channel. Until the chat is <a href="https://core.telegram.org/bots/api#unbanchatsenderchat">unbanned</a>, the owner of the banned chat won't be able to send messages on behalf of <strong>any of their channels</strong>. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "banChatSenderChat", response(crate::True))]
pub struct BanChatSenderChatRequest {
}
/// Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "unbanChatSenderChat", response(crate::True))]
pub struct UnbanChatSenderChatRequest {
}
/// Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the <em>can_restrict_members</em> administrator rights. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setChatPermissions", response(crate::True))]
pub struct SetChatPermissionsRequest {
}
/// Use this method to generate a new primary invite link for a chat; any previously generated primary link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the new invite link as <em>String</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "exportChatInviteLink", response(String))]
pub struct ExportChatInviteLinkRequest {
}
/// Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method <a href="https://core.telegram.org/bots/api#revokechatinvitelink">revokeChatInviteLink</a>. Returns the new invite link as <a href="https://core.telegram.org/bots/api#chatinvitelink">ChatInviteLink</a> object.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "createChatInviteLink", response(ChatInviteLink))]
pub struct CreateChatInviteLinkRequest {
}
/// Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a <a href="https://core.telegram.org/bots/api#chatinvitelink">ChatInviteLink</a> object.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "editChatInviteLink", response(ChatInviteLink))]
pub struct EditChatInviteLinkRequest {
}
/// Use this method to create a <a href="https://telegram.org/blog/superchannels-star-reactions-subscriptions#star-subscriptions">subscription invite link</a> for a channel chat. The bot must have the <em>can_invite_users</em> administrator rights. The link can be edited using the method <a href="https://core.telegram.org/bots/api#editchatsubscriptioninvitelink">editChatSubscriptionInviteLink</a> or revoked using the method <a href="https://core.telegram.org/bots/api#revokechatinvitelink">revokeChatInviteLink</a>. Returns the new invite link as a <a href="https://core.telegram.org/bots/api#chatinvitelink">ChatInviteLink</a> object.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "createChatSubscriptionInviteLink", response(ChatInviteLink))]
pub struct CreateChatSubscriptionInviteLinkRequest {
}
/// Use this method to edit a subscription invite link created by the bot. The bot must have the <em>can_invite_users</em> administrator rights. Returns the edited invite link as a <a href="https://core.telegram.org/bots/api#chatinvitelink">ChatInviteLink</a> object.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "editChatSubscriptionInviteLink", response(ChatInviteLink))]
pub struct EditChatSubscriptionInviteLinkRequest {
}
/// Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the revoked invite link as <a href="https://core.telegram.org/bots/api#chatinvitelink">ChatInviteLink</a> object.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "revokeChatInviteLink", response(ChatInviteLink))]
pub struct RevokeChatInviteLinkRequest {
}
/// Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the <em>can_invite_users</em> administrator right. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "approveChatJoinRequest", response(crate::True))]
pub struct ApproveChatJoinRequestRequest {
}
/// Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the <em>can_invite_users</em> administrator right. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "declineChatJoinRequest", response(crate::True))]
pub struct DeclineChatJoinRequestRequest {
}
/// Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setChatPhoto", response(crate::True))]
pub struct SetChatPhotoRequest {
}
/// Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "deleteChatPhoto", response(crate::True))]
pub struct DeleteChatPhotoRequest {
}
/// Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setChatTitle", response(crate::True))]
pub struct SetChatTitleRequest {
}
/// Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setChatDescription", response(crate::True))]
pub struct SetChatDescriptionRequest {
}
/// Use this method to add a message to the list of pinned messages in a chat. In private chats and channel direct messages chats, all non-service messages can be pinned. Conversely, the bot must be an administrator with the 'can_pin_messages' right or the 'can_edit_messages' right to pin messages in groups and channels respectively. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "pinChatMessage", response(crate::True))]
pub struct PinChatMessageRequest {
}
/// Use this method to remove a message from the list of pinned messages in a chat. In private chats and channel direct messages chats, all messages can be unpinned. Conversely, the bot must be an administrator with the 'can_pin_messages' right or the 'can_edit_messages' right to unpin messages in groups and channels respectively. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "unpinChatMessage", response(crate::True))]
pub struct UnpinChatMessageRequest {
}
/// Use this method to clear the list of pinned messages in a chat. In private chats and channel direct messages chats, no additional rights are required to unpin all pinned messages. Conversely, the bot must be an administrator with the 'can_pin_messages' right or the 'can_edit_messages' right to unpin all pinned messages in groups and channels respectively. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "unpinAllChatMessages", response(crate::True))]
pub struct UnpinAllChatMessagesRequest {
}
/// Use this method for your bot to leave a group, supergroup or channel. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "leaveChat", response(crate::True))]
pub struct LeaveChatRequest {
}
/// Use this method to get up-to-date information about the chat. Returns a <a href="https://core.telegram.org/bots/api#chatfullinfo">ChatFullInfo</a> object on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getChat", response(ChatFullInfo))]
pub struct GetChatRequest {
}
/// Use this method to get a list of administrators in a chat, which aren't bots. Returns an Array of <a href="https://core.telegram.org/bots/api#chatmember">ChatMember</a> objects.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getChatAdministrators", response(Vec<ChatMember>))]
pub struct GetChatAdministratorsRequest {
}
/// Use this method to get the number of members in a chat. Returns <em>Int</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getChatMemberCount", response(i64))]
pub struct GetChatMemberCountRequest {
}
/// Use this method to get information about a member of a chat. The method is only guaranteed to work for other users if the bot is an administrator in the chat. Returns a <a href="https://core.telegram.org/bots/api#chatmember">ChatMember</a> object on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getChatMember", response(ChatMember))]
pub struct GetChatMemberRequest {
}
/// Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field <em>can_set_sticker_set</em> optionally returned in <a href="https://core.telegram.org/bots/api#getchat">getChat</a> requests to check if the bot can use this method. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setChatStickerSet", response(crate::True))]
pub struct SetChatStickerSetRequest {
}
/// Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field <em>can_set_sticker_set</em> optionally returned in <a href="https://core.telegram.org/bots/api#getchat">getChat</a> requests to check if the bot can use this method. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "deleteChatStickerSet", response(crate::True))]
pub struct DeleteChatStickerSetRequest {
}
/// Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters. Returns an Array of <a href="https://core.telegram.org/bots/api#sticker">Sticker</a> objects.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getForumTopicIconStickers", response(Vec<Sticker>))]
pub struct GetForumTopicIconStickersRequest;
/// Use this method to create a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights. Returns information about the created topic as a <a href="https://core.telegram.org/bots/api#forumtopic">ForumTopic</a> object.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "createForumTopic", response(ForumTopic))]
pub struct CreateForumTopicRequest {
}
/// Use this method to edit name and icon of a topic in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights, unless it is the creator of the topic. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "editForumTopic", response(crate::True))]
pub struct EditForumTopicRequest {
}
/// Use this method to close an open topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights, unless it is the creator of the topic. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "closeForumTopic", response(crate::True))]
pub struct CloseForumTopicRequest {
}
/// Use this method to reopen a closed topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights, unless it is the creator of the topic. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "reopenForumTopic", response(crate::True))]
pub struct ReopenForumTopicRequest {
}
/// Use this method to delete a forum topic along with all its messages in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the <em>can_delete_messages</em> administrator rights. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "deleteForumTopic", response(crate::True))]
pub struct DeleteForumTopicRequest {
}
/// Use this method to clear the list of pinned messages in a forum topic in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the <em>can_pin_messages</em> administrator right in the supergroup. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "unpinAllForumTopicMessages", response(crate::True))]
pub struct UnpinAllForumTopicMessagesRequest {
}
/// Use this method to edit the name of the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "editGeneralForumTopic", response(crate::True))]
pub struct EditGeneralForumTopicRequest {
}
/// Use this method to close an open 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "closeGeneralForumTopic", response(crate::True))]
pub struct CloseGeneralForumTopicRequest {
}
/// Use this method to reopen a closed 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights. The topic will be automatically unhidden if it was hidden. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "reopenGeneralForumTopic", response(crate::True))]
pub struct ReopenGeneralForumTopicRequest {
}
/// Use this method to hide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights. The topic will be automatically closed if it was open. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "hideGeneralForumTopic", response(crate::True))]
pub struct HideGeneralForumTopicRequest {
}
/// Use this method to unhide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the <em>can_manage_topics</em> administrator rights. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "unhideGeneralForumTopic", response(crate::True))]
pub struct UnhideGeneralForumTopicRequest {
}
/// Use this method to clear the list of pinned messages in a General forum topic. The bot must be an administrator in the chat for this to work and must have the <em>can_pin_messages</em> administrator right in the supergroup. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "unpinAllGeneralForumTopicMessages", response(crate::True))]
pub struct UnpinAllGeneralForumTopicMessagesRequest {
}
/// Use this method to send answers to callback queries sent from <a href="/bots/features#inline-keyboards">inline keyboards</a>. The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, <em>True</em> is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "answerCallbackQuery", response(crate::True))]
pub struct AnswerCallbackQueryRequest {
}
/// Use this method to get the list of boosts added to a chat by a user. Requires administrator rights in the chat. Returns a <a href="https://core.telegram.org/bots/api#userchatboosts">UserChatBoosts</a> object.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getUserChatBoosts", response(UserChatBoosts))]
pub struct GetUserChatBoostsRequest {
}
/// Use this method to get information about the connection of the bot with a business account. Returns a <a href="https://core.telegram.org/bots/api#businessconnection">BusinessConnection</a> object on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getBusinessConnection", response(BusinessConnection))]
pub struct GetBusinessConnectionRequest {
}
/// Use this method to change the list of the bot's commands. See <a href="/bots/features#commands">this manual</a> for more details about bot commands. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setMyCommands", response(crate::True))]
pub struct SetMyCommandsRequest {
}
/// Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, <a href="https://core.telegram.org/bots/api#determining-list-of-commands">higher level commands</a> will be shown to affected users. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "deleteMyCommands", response(crate::True))]
pub struct DeleteMyCommandsRequest {
}
/// Use this method to get the current list of the bot's commands for the given scope and user language. Returns an Array of <a href="https://core.telegram.org/bots/api#botcommand">BotCommand</a> objects. If commands aren't set, an empty list is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getMyCommands", response(Vec<BotCommand>))]
pub struct GetMyCommandsRequest {
}
/// Use this method to change the bot's name. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setMyName", response(crate::True))]
pub struct SetMyNameRequest {
}
/// Use this method to get the current bot name for the given user language. Returns <a href="https://core.telegram.org/bots/api#botname">BotName</a> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getMyName", response(BotName))]
pub struct GetMyNameRequest {
}
/// Use this method to change the bot's description, which is shown in the chat with the bot if the chat is empty. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setMyDescription", response(crate::True))]
pub struct SetMyDescriptionRequest {
}
/// Use this method to get the current bot description for the given user language. Returns <a href="https://core.telegram.org/bots/api#botdescription">BotDescription</a> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getMyDescription", response(BotDescription))]
pub struct GetMyDescriptionRequest {
}
/// Use this method to change the bot's short description, which is shown on the bot's profile page and is sent together with the link when users share the bot. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setMyShortDescription", response(crate::True))]
pub struct SetMyShortDescriptionRequest {
}
/// Use this method to get the current bot short description for the given user language. Returns <a href="https://core.telegram.org/bots/api#botshortdescription">BotShortDescription</a> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getMyShortDescription", response(BotShortDescription))]
pub struct GetMyShortDescriptionRequest {
}
/// Use this method to change the bot's menu button in a private chat, or the default menu button. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setChatMenuButton", response(crate::True))]
pub struct SetChatMenuButtonRequest {
}
/// Use this method to get the current value of the bot's menu button in a private chat, or the default menu button. Returns <a href="https://core.telegram.org/bots/api#menubutton">MenuButton</a> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getChatMenuButton", response(MenuButton))]
pub struct GetChatMenuButtonRequest {
}
/// Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are free to modify the list before adding the bot. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setMyDefaultAdministratorRights", response(crate::True))]
pub struct SetMyDefaultAdministratorRightsRequest {
}
/// Use this method to get the current default administrator rights of the bot. Returns <a href="https://core.telegram.org/bots/api#chatadministratorrights">ChatAdministratorRights</a> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getMyDefaultAdministratorRights", response(ChatAdministratorRights))]
pub struct GetMyDefaultAdministratorRightsRequest {
}
/// Returns the list of gifts that can be sent by the bot to users and channel chats. Requires no parameters. Returns a <a href="https://core.telegram.org/bots/api#gifts">Gifts</a> object.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getAvailableGifts", response(Gifts))]
pub struct GetAvailableGiftsRequest;
/// Sends a gift to the given user or channel chat. The gift can't be converted to Telegram Stars by the receiver. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "sendGift", response(crate::True))]
pub struct SendGiftRequest {
}
/// Gifts a Telegram Premium subscription to the given user. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "giftPremiumSubscription", response(crate::True))]
pub struct GiftPremiumSubscriptionRequest {
}
/// Verifies a user <a href="https://telegram.org/verify#third-party-verification">on behalf of the organization</a> which is represented by the bot. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "verifyUser", response(crate::True))]
pub struct VerifyUserRequest {
}
/// Verifies a chat <a href="https://telegram.org/verify#third-party-verification">on behalf of the organization</a> which is represented by the bot. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "verifyChat", response(crate::True))]
pub struct VerifyChatRequest {
}
/// Removes verification from a user who is currently verified <a href="https://telegram.org/verify#third-party-verification">on behalf of the organization</a> represented by the bot. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "removeUserVerification", response(crate::True))]
pub struct RemoveUserVerificationRequest {
}
/// Removes verification from a chat that is currently verified <a href="https://telegram.org/verify#third-party-verification">on behalf of the organization</a> represented by the bot. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "removeChatVerification", response(crate::True))]
pub struct RemoveChatVerificationRequest {
}
/// Marks incoming message as read on behalf of a business account. Requires the <em>can_read_messages</em> business bot right. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "readBusinessMessage", response(crate::True))]
pub struct ReadBusinessMessageRequest {
}
/// Delete messages on behalf of a business account. Requires the <em>can_delete_sent_messages</em> business bot right to delete messages sent by the bot itself, or the <em>can_delete_all_messages</em> business bot right to delete any message. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "deleteBusinessMessages", response(crate::True))]
pub struct DeleteBusinessMessagesRequest {
}
/// Changes the first and last name of a managed business account. Requires the <em>can_change_name</em> business bot right. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setBusinessAccountName", response(crate::True))]
pub struct SetBusinessAccountNameRequest {
}
/// Changes the username of a managed business account. Requires the <em>can_change_username</em> business bot right. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setBusinessAccountUsername", response(crate::True))]
pub struct SetBusinessAccountUsernameRequest {
}
/// Changes the bio of a managed business account. Requires the <em>can_change_bio</em> business bot right. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setBusinessAccountBio", response(crate::True))]
pub struct SetBusinessAccountBioRequest {
}
/// Changes the profile photo of a managed business account. Requires the <em>can_edit_profile_photo</em> business bot right. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setBusinessAccountProfilePhoto", response(crate::True))]
pub struct SetBusinessAccountProfilePhotoRequest {
}
/// Removes the current profile photo of a managed business account. Requires the <em>can_edit_profile_photo</em> business bot right. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "removeBusinessAccountProfilePhoto", response(crate::True))]
pub struct RemoveBusinessAccountProfilePhotoRequest {
}
/// Changes the privacy settings pertaining to incoming gifts in a managed business account. Requires the <em>can_change_gift_settings</em> business bot right. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setBusinessAccountGiftSettings", response(crate::True))]
pub struct SetBusinessAccountGiftSettingsRequest {
}
/// Returns the amount of Telegram Stars owned by a managed business account. Requires the <em>can_view_gifts_and_stars</em> business bot right. Returns <a href="https://core.telegram.org/bots/api#staramount">StarAmount</a> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getBusinessAccountStarBalance", response(StarAmount))]
pub struct GetBusinessAccountStarBalanceRequest {
}
/// Transfers Telegram Stars from the business account balance to the bot's balance. Requires the <em>can_transfer_stars</em> business bot right. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "transferBusinessAccountStars", response(crate::True))]
pub struct TransferBusinessAccountStarsRequest {
}
/// Returns the gifts received and owned by a managed business account. Requires the <em>can_view_gifts_and_stars</em> business bot right. Returns <a href="https://core.telegram.org/bots/api#ownedgifts">OwnedGifts</a> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getBusinessAccountGifts", response(OwnedGifts))]
pub struct GetBusinessAccountGiftsRequest {
}
/// Returns the gifts owned and hosted by a user. Returns <a href="https://core.telegram.org/bots/api#ownedgifts">OwnedGifts</a> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getUserGifts", response(OwnedGifts))]
pub struct GetUserGiftsRequest {
}
/// Returns the gifts owned by a chat. Returns <a href="https://core.telegram.org/bots/api#ownedgifts">OwnedGifts</a> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getChatGifts", response(OwnedGifts))]
pub struct GetChatGiftsRequest {
}
/// Converts a given regular gift to Telegram Stars. Requires the <em>can_convert_gifts_to_stars</em> business bot right. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "convertGiftToStars", response(crate::True))]
pub struct ConvertGiftToStarsRequest {
}
/// Upgrades a given regular gift to a unique gift. Requires the <em>can_transfer_and_upgrade_gifts</em> business bot right. Additionally requires the <em>can_transfer_stars</em> business bot right if the upgrade is paid. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "upgradeGift", response(crate::True))]
pub struct UpgradeGiftRequest {
}
/// Transfers an owned unique gift to another user. Requires the <em>can_transfer_and_upgrade_gifts</em> business bot right. Requires <em>can_transfer_stars</em> business bot right if the transfer is paid. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "transferGift", response(crate::True))]
pub struct TransferGiftRequest {
}
/// Posts a story on behalf of a managed business account. Requires the <em>can_manage_stories</em> business bot right. Returns <a href="https://core.telegram.org/bots/api#story">Story</a> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "postStory", response(Story))]
pub struct PostStoryRequest {
}
/// Reposts a story on behalf of a business account from another business account. Both business accounts must be managed by the same bot, and the story on the source account must have been posted (or reposted) by the bot. Requires the <em>can_manage_stories</em> business bot right for both business accounts. Returns <a href="https://core.telegram.org/bots/api#story">Story</a> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "repostStory", response(Story))]
pub struct RepostStoryRequest {
}
/// Edits a story previously posted by the bot on behalf of a managed business account. Requires the <em>can_manage_stories</em> business bot right. Returns <a href="https://core.telegram.org/bots/api#story">Story</a> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "editStory", response(Story))]
pub struct EditStoryRequest {
}
/// Deletes a story previously posted by the bot on behalf of a managed business account. Requires the <em>can_manage_stories</em> business bot right. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "deleteStory", response(crate::True))]
pub struct DeleteStoryRequest {
}
/// Use this method to edit text and <a href="https://core.telegram.org/bots/api#games">game</a> messages. On success, if the edited message is not an inline message, the edited <a href="https://core.telegram.org/bots/api#message">Message</a> is returned, otherwise <em>True</em> is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within <strong>48 hours</strong> from the time they were sent.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "editMessageText", response(MessageOrTrue))]
pub struct EditMessageTextRequest {
}
/// Use this method to edit captions of messages. On success, if the edited message is not an inline message, the edited <a href="https://core.telegram.org/bots/api#message">Message</a> is returned, otherwise <em>True</em> is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within <strong>48 hours</strong> from the time they were sent.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "editMessageCaption", response(MessageOrTrue))]
pub struct EditMessageCaptionRequest {
}
/// Use this method to edit animation, audio, document, photo, or video messages, or to add media to text messages. If a message is part of a message album, then it can be edited only to an audio for audio albums, only to a document for document albums and to a photo or a video otherwise. When an inline message is edited, a new file can't be uploaded; use a previously uploaded file via its file_id or specify a URL. On success, if the edited message is not an inline message, the edited <a href="https://core.telegram.org/bots/api#message">Message</a> is returned, otherwise <em>True</em> is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within <strong>48 hours</strong> from the time they were sent.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "editMessageMedia", response(MessageOrTrue))]
pub struct EditMessageMediaRequest {
}
/// Use this method to edit live location messages. A location can be edited until its <em>live_period</em> expires or editing is explicitly disabled by a call to <a href="https://core.telegram.org/bots/api#stopmessagelivelocation">stopMessageLiveLocation</a>. On success, if the edited message is not an inline message, the edited <a href="https://core.telegram.org/bots/api#message">Message</a> is returned, otherwise <em>True</em> is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "editMessageLiveLocation", response(MessageOrTrue))]
pub struct EditMessageLiveLocationRequest {
}
/// Use this method to stop updating a live location message before <em>live_period</em> expires. On success, if the message is not an inline message, the edited <a href="https://core.telegram.org/bots/api#message">Message</a> is returned, otherwise <em>True</em> is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "stopMessageLiveLocation", response(MessageOrTrue))]
pub struct StopMessageLiveLocationRequest {
}
/// Use this method to edit a checklist on behalf of a connected business account. On success, the edited <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "editMessageChecklist", response(Message))]
pub struct EditMessageChecklistRequest {
}
/// Use this method to edit only the reply markup of messages. On success, if the edited message is not an inline message, the edited <a href="https://core.telegram.org/bots/api#message">Message</a> is returned, otherwise <em>True</em> is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within <strong>48 hours</strong> from the time they were sent.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "editMessageReplyMarkup", response(MessageOrTrue))]
pub struct EditMessageReplyMarkupRequest {
}
/// Use this method to stop a poll which was sent by the bot. On success, the stopped <a href="https://core.telegram.org/bots/api#poll">Poll</a> is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "stopPoll", response(Poll))]
pub struct StopPollRequest {
}
/// Use this method to approve a suggested post in a direct messages chat. The bot must have the 'can_post_messages' administrator right in the corresponding channel chat. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "approveSuggestedPost", response(crate::True))]
pub struct ApproveSuggestedPostRequest {
}
/// Use this method to decline a suggested post in a direct messages chat. The bot must have the 'can_manage_direct_messages' administrator right in the corresponding channel chat. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "declineSuggestedPost", response(crate::True))]
pub struct DeclineSuggestedPostRequest {
}
/// Use this method to delete a message, including service messages, with the following limitations:<br>- A message can only be deleted if it was sent less than 48 hours ago.<br>- Service messages about a supergroup, channel, or forum topic creation can't be deleted.<br>- A dice message in a private chat can only be deleted if it was sent more than 24 hours ago.<br>- Bots can delete outgoing messages in private chats, groups, and supergroups.<br>- Bots can delete incoming messages in private chats.<br>- Bots granted <em>can_post_messages</em> permissions can delete outgoing messages in channels.<br>- If the bot is an administrator of a group, it can delete any message there.<br>- If the bot has <em>can_delete_messages</em> administrator right in a supergroup or a channel, it can delete any message there.<br>- If the bot has <em>can_manage_direct_messages</em> administrator right in a channel, it can delete any message in the corresponding direct messages chat.<br>Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "deleteMessage", response(crate::True))]
pub struct DeleteMessageRequest {
}
/// Use this method to delete multiple messages simultaneously. If some of the specified messages can't be found, they are skipped. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "deleteMessages", response(crate::True))]
pub struct DeleteMessagesRequest {
}
/// This object represents a sticker.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Sticker {}
/// This object represents a sticker set.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StickerSet {}
/// This object describes the position on faces where a mask should be placed by default.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MaskPosition {}
/// This object describes a sticker to be added to a sticker set.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputSticker {}
/// Use this method to send static .WEBP, <a href="https://telegram.org/blog/animated-stickers">animated</a> .TGS, or <a href="https://telegram.org/blog/video-stickers-better-reactions">video</a> .WEBM stickers. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "sendSticker", response(Message))]
pub struct SendStickerRequest {
}
/// Use this method to get a sticker set. On success, a <a href="https://core.telegram.org/bots/api#stickerset">StickerSet</a> object is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getStickerSet", response(StickerSet))]
pub struct GetStickerSetRequest {
}
/// Use this method to get information about custom emoji stickers by their identifiers. Returns an Array of <a href="https://core.telegram.org/bots/api#sticker">Sticker</a> objects.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getCustomEmojiStickers", response(Vec<Sticker>))]
pub struct GetCustomEmojiStickersRequest {
}
/// Use this method to upload a file with a sticker for later use in the <a href="https://core.telegram.org/bots/api#createnewstickerset">createNewStickerSet</a>, <a href="https://core.telegram.org/bots/api#addstickertoset">addStickerToSet</a>, or <a href="https://core.telegram.org/bots/api#replacestickerinset">replaceStickerInSet</a> methods (the file can be used multiple times). Returns the uploaded <a href="https://core.telegram.org/bots/api#file">File</a> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "uploadStickerFile", response(File))]
pub struct UploadStickerFileRequest {
}
/// Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "createNewStickerSet", response(crate::True))]
pub struct CreateNewStickerSetRequest {
}
/// Use this method to add a new sticker to a set created by the bot. Emoji sticker sets can have up to 200 stickers. Other sticker sets can have up to 120 stickers. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "addStickerToSet", response(crate::True))]
pub struct AddStickerToSetRequest {
}
/// Use this method to move a sticker in a set created by the bot to a specific position. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setStickerPositionInSet", response(crate::True))]
pub struct SetStickerPositionInSetRequest {
}
/// Use this method to delete a sticker from a set created by the bot. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "deleteStickerFromSet", response(crate::True))]
pub struct DeleteStickerFromSetRequest {
}
/// Use this method to replace an existing sticker in a sticker set with a new one. The method is equivalent to calling <a href="https://core.telegram.org/bots/api#deletestickerfromset">deleteStickerFromSet</a>, then <a href="https://core.telegram.org/bots/api#addstickertoset">addStickerToSet</a>, then <a href="https://core.telegram.org/bots/api#setstickerpositioninset">setStickerPositionInSet</a>. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "replaceStickerInSet", response(crate::True))]
pub struct ReplaceStickerInSetRequest {
}
/// Use this method to change the list of emoji assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setStickerEmojiList", response(crate::True))]
pub struct SetStickerEmojiListRequest {
}
/// Use this method to change search keywords assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setStickerKeywords", response(crate::True))]
pub struct SetStickerKeywordsRequest {
}
/// Use this method to change the <a href="https://core.telegram.org/bots/api#maskposition">mask position</a> of a mask sticker. The sticker must belong to a sticker set that was created by the bot. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setStickerMaskPosition", response(crate::True))]
pub struct SetStickerMaskPositionRequest {
}
/// Use this method to set the title of a created sticker set. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setStickerSetTitle", response(crate::True))]
pub struct SetStickerSetTitleRequest {
}
/// Use this method to set the thumbnail of a regular or mask sticker set. The format of the thumbnail file must match the format of the stickers in the set. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setStickerSetThumbnail", response(crate::True))]
pub struct SetStickerSetThumbnailRequest {
}
/// Use this method to set the thumbnail of a custom emoji sticker set. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setCustomEmojiStickerSetThumbnail", response(crate::True))]
pub struct SetCustomEmojiStickerSetThumbnailRequest {
}
/// Use this method to delete a sticker set that was created by the bot. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "deleteStickerSet", response(crate::True))]
pub struct DeleteStickerSetRequest {
}
/// This object represents an incoming inline query. When the user sends an empty query, your bot could return some default or trending results.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQuery {}
/// Use this method to send answers to an inline query. On success, <em>True</em> is returned.<br>No more than <strong>50</strong> results per query are allowed.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "answerInlineQuery", response(crate::True))]
pub struct AnswerInlineQueryRequest {
}
/// This object represents a button to be shown above inline query results. You <strong>must</strong> use exactly one of the optional fields.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultsButton {}
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
pub struct InlineQueryResultArticle {}
/// Represents a link to a photo. By default, this photo will be sent by the user with optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the photo.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultPhoto {}
/// Represents a link to an animated GIF file. By default, this animated GIF file will be sent by the user with optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the animation.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultGif {}
/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default, this animated MPEG-4 file will be sent by the user with optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the animation.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultMpeg4Gif {}
/// Represents a link to a page containing an embedded video player or a video file. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the video.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultVideo {}
/// Represents a link to an MP3 audio file. By default, this audio file will be sent by the user. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the audio.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultAudio {}
/// Represents a link to a voice recording in an .OGG container encoded with OPUS. By default, this voice recording will be sent by the user. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the the voice message.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultVoice {}
/// Represents a link to a file. By default, this file will be sent by the user with an optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the file. Currently, only <strong>.PDF</strong> and <strong>.ZIP</strong> files can be sent using this method.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultDocument {}
/// Represents a location on a map. By default, the location will be sent by the user. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the location.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultLocation {}
/// Represents a venue. By default, the venue will be sent by the user. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the venue.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultVenue {}
/// Represents a contact with a phone number. By default, this contact will be sent by the user. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the contact.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultContact {}
/// Represents a <a href="https://core.telegram.org/bots/api#games">Game</a>.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultGame {}
/// Represents a link to a photo stored on the Telegram servers. By default, this photo will be sent by the user with an optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the photo.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultCachedPhoto {}
/// Represents a link to an animated GIF file stored on the Telegram servers. By default, this animated GIF file will be sent by the user with an optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with specified content instead of the animation.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultCachedGif {}
/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers. By default, this animated MPEG-4 file will be sent by the user with an optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the animation.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultCachedMpeg4Gif {}
/// Represents a link to a sticker stored on the Telegram servers. By default, this sticker will be sent by the user. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the sticker.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultCachedSticker {}
/// Represents a link to a file stored on the Telegram servers. By default, this file will be sent by the user with an optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the file.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultCachedDocument {}
/// Represents a link to a video file stored on the Telegram servers. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the video.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultCachedVideo {}
/// Represents a link to a voice message stored on the Telegram servers. By default, this voice message will be sent by the user. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the voice message.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultCachedVoice {}
/// Represents a link to an MP3 audio file stored on the Telegram servers. By default, this audio file will be sent by the user. Alternatively, you can use <em>input_message_content</em> to send a message with the specified content instead of the audio.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InlineQueryResultCachedAudio {}
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
pub struct InputTextMessageContent {}
/// Represents the <a href="https://core.telegram.org/bots/api#inputmessagecontent">content</a> of a location message to be sent as the result of an inline query.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputLocationMessageContent {}
/// Represents the <a href="https://core.telegram.org/bots/api#inputmessagecontent">content</a> of a venue message to be sent as the result of an inline query.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputVenueMessageContent {}
/// Represents the <a href="https://core.telegram.org/bots/api#inputmessagecontent">content</a> of a contact message to be sent as the result of an inline query.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputContactMessageContent {}
/// Represents the <a href="https://core.telegram.org/bots/api#inputmessagecontent">content</a> of an invoice message to be sent as the result of an inline query.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputInvoiceMessageContent {}
/// Represents a <a href="https://core.telegram.org/bots/api#inlinequeryresult">result</a> of an inline query that was chosen by the user and sent to their chat partner.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChosenInlineResult {}
/// Use this method to set the result of an interaction with a <a href="/bots/webapps">Web App</a> and send a corresponding message on behalf of the user to the chat from which the query originated. On success, a <a href="https://core.telegram.org/bots/api#sentwebappmessage">SentWebAppMessage</a> object is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "answerWebAppQuery", response(SentWebAppMessage))]
pub struct AnswerWebAppQueryRequest {
}
/// Describes an inline message sent by a <a href="/bots/webapps">Web App</a> on behalf of a user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SentWebAppMessage {}
/// Stores a message that can be sent by a user of a Mini App. Returns a <a href="https://core.telegram.org/bots/api#preparedinlinemessage">PreparedInlineMessage</a> object.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "savePreparedInlineMessage", response(PreparedInlineMessage))]
pub struct SavePreparedInlineMessageRequest {
}
/// Describes an inline message to be sent by a user of a Mini App.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PreparedInlineMessage {}
/// Use this method to send invoices. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "sendInvoice", response(Message))]
pub struct SendInvoiceRequest {
}
/// Use this method to create a link for an invoice. Returns the created invoice link as <em>String</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "createInvoiceLink", response(String))]
pub struct CreateInvoiceLinkRequest {
}
/// If you sent an invoice requesting a shipping address and the parameter <em>is_flexible</em> was specified, the Bot API will send an <a href="https://core.telegram.org/bots/api#update">Update</a> with a <em>shipping_query</em> field to the bot. Use this method to reply to shipping queries. On success, <em>True</em> is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "answerShippingQuery", response(crate::True))]
pub struct AnswerShippingQueryRequest {
}
/// Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an <a href="https://core.telegram.org/bots/api#update">Update</a> with the field <em>pre_checkout_query</em>. Use this method to respond to such pre-checkout queries. On success, <em>True</em> is returned. <strong>Note:</strong> The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "answerPreCheckoutQuery", response(crate::True))]
pub struct AnswerPreCheckoutQueryRequest {
}
/// A method to get the current Telegram Stars balance of the bot. Requires no parameters. On success, returns a <a href="https://core.telegram.org/bots/api#staramount">StarAmount</a> object.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getMyStarBalance", response(StarAmount))]
pub struct GetMyStarBalanceRequest;
/// Returns the bot's Telegram Star transactions in chronological order. On success, returns a <a href="https://core.telegram.org/bots/api#startransactions">StarTransactions</a> object.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getStarTransactions", response(StarTransactions))]
pub struct GetStarTransactionsRequest {
}
/// Refunds a successful payment in <a href="https://t.me/BotNews/90">Telegram Stars</a>. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "refundStarPayment", response(crate::True))]
pub struct RefundStarPaymentRequest {
}
/// Allows the bot to cancel or re-enable extension of a subscription paid in Telegram Stars. Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "editUserStarSubscription", response(crate::True))]
pub struct EditUserStarSubscriptionRequest {
}
/// This object represents a portion of the price for goods or services.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LabeledPrice {}
/// This object contains basic information about an invoice.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Invoice {}
/// This object represents a shipping address.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ShippingAddress {}
/// This object represents information about an order.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OrderInfo {}
/// This object represents one shipping option.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ShippingOption {}
/// This object contains basic information about a successful payment. Note that if the buyer initiates a chargeback with the relevant payment provider following this transaction, the funds may be debited from your balance. This is outside of Telegram's control.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuccessfulPayment {}
/// This object contains basic information about a refunded payment.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RefundedPayment {}
/// This object contains information about an incoming shipping query.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ShippingQuery {}
/// This object contains information about an incoming pre-checkout query.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PreCheckoutQuery {}
/// This object contains information about a paid media purchase.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaidMediaPurchased {}
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
pub struct RevenueWithdrawalStatePending {}
/// The withdrawal succeeded.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RevenueWithdrawalStateSucceeded {}
/// The withdrawal failed and the transaction was refunded.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RevenueWithdrawalStateFailed {}
/// Contains information about the affiliate that received a commission via this transaction.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AffiliateInfo {}
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
pub struct TransactionPartnerUser {}
/// Describes a transaction with a chat.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransactionPartnerChat {}
/// Describes the affiliate program that issued the affiliate commission received via this transaction.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransactionPartnerAffiliateProgram {}
/// Describes a withdrawal transaction with Fragment.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransactionPartnerFragment {}
/// Describes a withdrawal transaction to the Telegram Ads platform.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransactionPartnerTelegramAds {}
/// Describes a transaction with payment for <a href="https://core.telegram.org/bots/api#paid-broadcasts">paid broadcasting</a>.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransactionPartnerTelegramApi {}
/// Describes a transaction with an unknown source or recipient.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransactionPartnerOther {}
/// Describes a Telegram Star transaction. Note that if the buyer initiates a chargeback with the payment provider from whom they acquired Stars (e.g., Apple, Google) following this transaction, the refunded Stars will be deducted from the bot's balance. This is outside of Telegram's control.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StarTransaction {}
/// Contains a list of Telegram Star transactions.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StarTransactions {}
/// Describes Telegram Passport data shared with the bot by the user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportData {}
/// This object represents a file uploaded to Telegram Passport. Currently all Telegram Passport files are in JPEG format when decrypted and don't exceed 10MB.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportFile {}
/// Describes documents or other Telegram Passport elements shared with the bot by the user.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EncryptedPassportElement {}
/// Describes data required for decrypting and authenticating <a href="https://core.telegram.org/bots/api#encryptedpassportelement">EncryptedPassportElement</a>. See the <a href="/passport#receiving-information">Telegram Passport Documentation</a> for a complete description of the data decryption and authentication processes.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EncryptedCredentials {}
/// Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns <em>True</em> on success.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setPassportDataErrors", response(crate::True))]
pub struct SetPassportDataErrorsRequest {
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
pub struct PassportElementErrorDataField {}
/// Represents an issue with the front side of a document. The error is considered resolved when the file with the front side of the document changes.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorFrontSide {}
/// Represents an issue with the reverse side of a document. The error is considered resolved when the file with reverse side of the document changes.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorReverseSide {}
/// Represents an issue with the selfie with a document. The error is considered resolved when the file with the selfie changes.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorSelfie {}
/// Represents an issue with a document scan. The error is considered resolved when the file with the document scan changes.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorFile {}
/// Represents an issue with a list of scans. The error is considered resolved when the list of files containing the scans changes.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorFiles {}
/// Represents an issue with one of the files that constitute the translation of a document. The error is considered resolved when the file changes.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorTranslationFile {}
/// Represents an issue with the translated version of a document. The error is considered resolved when a file with the document translation change.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorTranslationFiles {}
/// Represents an issue in an unspecified place. The error is considered resolved when new data is added.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PassportElementErrorUnspecified {}
/// Use this method to send a game. On success, the sent <a href="https://core.telegram.org/bots/api#message">Message</a> is returned.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "sendGame", response(Message))]
pub struct SendGameRequest {
}
/// This object represents a game. Use BotFather to create and edit games, their short names will act as unique identifiers.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Game {}
/// A placeholder, currently holds no information. Use <a href="https://t.me/botfather">BotFather</a> to set up your game.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CallbackGame {}
/// Use this method to set the score of the specified user in a game message. On success, if the message is not an inline message, the <a href="https://core.telegram.org/bots/api#message">Message</a> is returned, otherwise <em>True</em> is returned. Returns an error, if the new score is not greater than the user's current score in the chat and <em>force</em> is <em>False</em>.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "setGameScore", response(MessageOrTrue))]
pub struct SetGameScoreRequest {
}
/// Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. Returns an Array of <a href="https://core.telegram.org/bots/api#gamehighscore">GameHighScore</a> objects.
#[derive(serde::Serialize, macros::Method)]
#[method(name = "getGameHighScores", response(Vec<GameHighScore>))]
pub struct GetGameHighScoresRequest {
}
/// This object represents one row of the high scores table for a game.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GameHighScore {}
