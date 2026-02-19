// this file is auto-generated

/**
 * This [object](https://core.telegram.org/bots/api#available-types) represents an incoming update.
 * At most **one** of the optional parameters can be present in any given update.
 */
export interface Update {
    /**
     * The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using [webhooks](https://core.telegram.org/bots/api#setwebhook), since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
     */
    update_id: number;
    /**
     * New incoming message of any kind - text, photo, sticker, etc.
     */
    message: Message | null;
    /**
     * New version of a message that is known to the bot and was edited. This update may at times be triggered by changes to message fields that are either unavailable or not actively used by your bot.
     */
    edited_message: Message | null;
    /**
     * New incoming channel post of any kind - text, photo, sticker, etc.
     */
    channel_post: Message | null;
    /**
     * New version of a channel post that is known to the bot and was edited. This update may at times be triggered by changes to message fields that are either unavailable or not actively used by your bot.
     */
    edited_channel_post: Message | null;
    /**
     * The bot was connected to or disconnected from a business account, or a user edited an existing connection with the bot
     */
    business_connection: BusinessConnection | null;
    /**
     * New message from a connected business account
     */
    business_message: Message | null;
    /**
     * New version of a message from a connected business account
     */
    edited_business_message: Message | null;
    /**
     * Messages were deleted from a connected business account
     */
    deleted_business_messages: BusinessMessagesDeleted | null;
    /**
     * A reaction to a message was changed by a user. The bot must be an administrator in the chat and must explicitly specify `"message_reaction"` in the list of *allowed_updates* to receive these updates. The update isn't received for reactions set by bots.
     */
    message_reaction: MessageReactionUpdated | null;
    /**
     * Reactions to a message with anonymous reactions were changed. The bot must be an administrator in the chat and must explicitly specify `"message_reaction_count"` in the list of *allowed_updates* to receive these updates. The updates are grouped and can be sent with delay up to a few minutes.
     */
    message_reaction_count: MessageReactionCountUpdated | null;
    /**
     * New incoming [inline](https://core.telegram.org/bots/api#inline-mode) query
     */
    inline_query: InlineQuery | null;
    /**
     * The result of an [inline](https://core.telegram.org/bots/api#inline-mode) query that was chosen by a user and sent to their chat partner. Please see our documentation on the [feedback collecting](https://core.telegram.org/bots/api/bots/inline#collecting-feedback) for details on how to enable these updates for your bot.
     */
    chosen_inline_result: ChosenInlineResult | null;
    /**
     * New incoming callback query
     */
    callback_query: CallbackQuery | null;
    /**
     * New incoming shipping query. Only for invoices with flexible price
     */
    shipping_query: ShippingQuery | null;
    /**
     * New incoming pre-checkout query. Contains full information about checkout
     */
    pre_checkout_query: PreCheckoutQuery | null;
    /**
     * A user purchased paid media with a non-empty payload sent by the bot in a non-channel chat
     */
    purchased_paid_media: PaidMediaPurchased | null;
    /**
     * New poll state. Bots receive only updates about manually stopped polls and polls, which are sent by the bot
     */
    poll: Poll | null;
    /**
     * A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself.
     */
    poll_answer: PollAnswer | null;
    /**
     * The bot's chat member status was updated in a chat. For private chats, this update is received only when the bot is blocked or unblocked by the user.
     */
    my_chat_member: ChatMemberUpdated | null;
    /**
     * A chat member's status was updated in a chat. The bot must be an administrator in the chat and must explicitly specify `"chat_member"` in the list of *allowed_updates* to receive these updates.
     */
    chat_member: ChatMemberUpdated | null;
    /**
     * A request to join the chat has been sent. The bot must have the *can_invite_users* administrator right in the chat to receive these updates.
     */
    chat_join_request: ChatJoinRequest | null;
    /**
     * A chat boost was added or changed. The bot must be an administrator in the chat to receive these updates.
     */
    chat_boost: ChatBoostUpdated | null;
    /**
     * A boost was removed from a chat. The bot must be an administrator in the chat to receive these updates.
     */
    removed_chat_boost: ChatBoostRemoved | null;
}
/**
 * Use this method to receive incoming updates using long polling ([wiki](https://en.wikipedia.org/wiki/Push_technology#Long_polling)). Returns an Array of [Update](https://core.telegram.org/bots/api#update) objects.
 */
export interface GetUpdatesRequest {
    /**
     * Identifier of the first update to be returned. Must be greater by one than the highest among the identifiers of previously received updates. By default, updates starting with the earliest unconfirmed update are returned. An update is considered confirmed as soon as [getUpdates](https://core.telegram.org/bots/api#getupdates) is called with an *offset* higher than its *update_id*. The negative offset can be specified to retrieve updates starting from *-offset* update from the end of the updates queue. All previous updates will be forgotten.
     */
    offset: number | null;
    /**
     * Limits the number of updates to be retrieved. Values between 1-100 are accepted. Defaults to 100.
     */
    limit: number | null;
    /**
     * Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling. Should be positive, short polling should be used for testing purposes only.
     */
    timeout: number | null;
    /**
     * A JSON-serialized list of the update types you want your bot to receive. For example, specify `["message", "edited_channel_post", "callback_query"]` to only receive updates of these types. See [Update](https://core.telegram.org/bots/api#update) for a complete list of available update types. Specify an empty list to receive all update types except *chat_member*, *message_reaction*, and *message_reaction_count* (default). If not specified, the previous setting will be used.
     *
     * Please note that this parameter doesn't affect updates created before the call to getUpdates, so unwanted updates may be received for a short period of time.
     */
    allowed_updates: string[] | null;
}
export type GetUpdatesResponse = Update[];
/**
 * Use this method to specify a URL and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified URL, containing a JSON-serialized [Update](https://core.telegram.org/bots/api#update). In case of an unsuccessful request (a request with response [HTTP status code](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes) different from `2XY`), we will repeat the request and give up after a reasonable amount of attempts. Returns *True* on success.
 */
export interface SetWebhookRequest {
    /**
     * HTTPS URL to send updates to. Use an empty string to remove webhook integration
     */
    url: string;
    /**
     * Upload your public key certificate so that the root certificate in use can be checked. See our [self-signed guide](https://core.telegram.org/bots/api/bots/self-signed) for details.
     */
    certificate: InputFile | null;
    /**
     * The fixed IP address which will be used to send webhook requests instead of the IP address resolved through DNS
     */
    ip_address: string | null;
    /**
     * The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100. Defaults to *40*. Use lower values to limit the load on your bot's server, and higher values to increase your bot's throughput.
     */
    max_connections: number | null;
    /**
     * A JSON-serialized list of the update types you want your bot to receive. For example, specify `["message", "edited_channel_post", "callback_query"]` to only receive updates of these types. See [Update](https://core.telegram.org/bots/api#update) for a complete list of available update types. Specify an empty list to receive all update types except *chat_member*, *message_reaction*, and *message_reaction_count* (default). If not specified, the previous setting will be used.
     * Please note that this parameter doesn't affect updates created before the call to the setWebhook, so unwanted updates may be received for a short period of time.
     */
    allowed_updates: string[] | null;
    /**
     * Pass *True* to drop all pending updates
     */
    drop_pending_updates: boolean | null;
    /**
     * A secret token to be sent in a header “X-Telegram-Bot-Api-Secret-Token” in every webhook request, 1-256 characters. Only characters `A-Z`, `a-z`, `0-9`, `_` and `-` are allowed. The header is useful to ensure that the request comes from a webhook set by you.
     */
    secret_token: string | null;
}
export type SetWebhookResponse = true;
/**
 * Use this method to remove webhook integration if you decide to switch back to [getUpdates](https://core.telegram.org/bots/api#getupdates). Returns *True* on success.
 */
export interface DeleteWebhookRequest {
    /**
     * Pass *True* to drop all pending updates
     */
    drop_pending_updates: boolean | null;
}
export type DeleteWebhookResponse = true;
/**
 * Use this method to get current webhook status. Requires no parameters. On success, returns a [WebhookInfo](https://core.telegram.org/bots/api#webhookinfo) object. If the bot is using [getUpdates](https://core.telegram.org/bots/api#getupdates), will return an object with the *url* field empty.
 */
// deno-lint-ignore no-empty-interface
export interface GetWebhookInfoRequest {}
export type GetWebhookInfoResponse = WebhookInfo;
/**
 * Describes the current status of a webhook.
 */
export interface WebhookInfo {
    /**
     * Webhook URL, may be empty if webhook is not set up
     */
    url: string;
    /**
     * *True*, if a custom certificate was provided for webhook certificate checks
     */
    has_custom_certificate: boolean;
    /**
     * Number of updates awaiting delivery
     */
    pending_update_count: number;
    /**
     * Currently used webhook IP address
     */
    ip_address: string | null;
    /**
     * Unix time for the most recent error that happened when trying to deliver an update via webhook
     */
    last_error_date: number | null;
    /**
     * Error message in human-readable format for the most recent error that happened when trying to deliver an update via webhook
     */
    last_error_message: string | null;
    /**
     * Unix time of the most recent error that happened when trying to synchronize available updates with Telegram datacenters
     */
    last_synchronization_error_date: number | null;
    /**
     * The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery
     */
    max_connections: number | null;
    /**
     * A list of update types the bot is subscribed to. Defaults to all update types except *chat_member*
     */
    allowed_updates: string[] | null;
}
/**
 * This object represents a Telegram user or bot.
 */
export interface User {
    /**
     * Unique identifier for this user or bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
     */
    id: number;
    /**
     * *True*, if this user is a bot
     */
    is_bot: boolean;
    /**
     * User's or bot's first name
     */
    first_name: string;
    /**
     * User's or bot's last name
     */
    last_name: string | null;
    /**
     * User's or bot's username
     */
    username: string | null;
    /**
     * [IETF language tag](https://en.wikipedia.org/wiki/IETF_language_tag) of the user's language
     */
    language_code: string | null;
    /**
     * *True*, if this user is a Telegram Premium user
     */
    is_premium: true | null;
    /**
     * *True*, if this user added the bot to the attachment menu
     */
    added_to_attachment_menu: true | null;
    /**
     * *True*, if the bot can be invited to groups. Returned only in [getMe](https://core.telegram.org/bots/api#getme).
     */
    can_join_groups: boolean | null;
    /**
     * *True*, if [privacy mode](https://core.telegram.org/bots/api/bots/features#privacy-mode) is disabled for the bot. Returned only in [getMe](https://core.telegram.org/bots/api#getme).
     */
    can_read_all_group_messages: boolean | null;
    /**
     * *True*, if the bot supports inline queries. Returned only in [getMe](https://core.telegram.org/bots/api#getme).
     */
    supports_inline_queries: boolean | null;
    /**
     * *True*, if the bot can be connected to a Telegram Business account to receive its messages. Returned only in [getMe](https://core.telegram.org/bots/api#getme).
     */
    can_connect_to_business: boolean | null;
    /**
     * *True*, if the bot has a main Web App. Returned only in [getMe](https://core.telegram.org/bots/api#getme).
     */
    has_main_web_app: boolean | null;
    /**
     * *True*, if the bot has forum topic mode enabled in private chats. Returned only in [getMe](https://core.telegram.org/bots/api#getme).
     */
    has_topics_enabled: boolean | null;
    /**
     * *True*, if the bot allows users to create and delete topics in private chats. Returned only in [getMe](https://core.telegram.org/bots/api#getme).
     */
    allows_users_to_create_topics: boolean | null;
}
/**
 * This object represents a chat.
 */
export interface Chat {
    /**
     * Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
     */
    id: number;
    /**
     * Type of the chat, can be either “private”, “group”, “supergroup” or “channel”
     */
    type: string;
    /**
     * Title, for supergroups, channels and group chats
     */
    title: string | null;
    /**
     * Username, for private chats, supergroups and channels if available
     */
    username: string | null;
    /**
     * First name of the other party in a private chat
     */
    first_name: string | null;
    /**
     * Last name of the other party in a private chat
     */
    last_name: string | null;
    /**
     * *True*, if the supergroup chat is a forum (has [topics](https://telegram.org/blog/topics-in-groups-collectible-usernames#topics-in-groups) enabled)
     */
    is_forum: true | null;
    /**
     * *True*, if the chat is the direct messages chat of a channel
     */
    is_direct_messages: true | null;
}
/**
 * This object contains full information about a chat.
 */
export interface ChatFullInfo {
    /**
     * Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
     */
    id: number;
    /**
     * Type of the chat, can be either “private”, “group”, “supergroup” or “channel”
     */
    type: string;
    /**
     * Title, for supergroups, channels and group chats
     */
    title: string | null;
    /**
     * Username, for private chats, supergroups and channels if available
     */
    username: string | null;
    /**
     * First name of the other party in a private chat
     */
    first_name: string | null;
    /**
     * Last name of the other party in a private chat
     */
    last_name: string | null;
    /**
     * *True*, if the supergroup chat is a forum (has [topics](https://telegram.org/blog/topics-in-groups-collectible-usernames#topics-in-groups) enabled)
     */
    is_forum: true | null;
    /**
     * *True*, if the chat is the direct messages chat of a channel
     */
    is_direct_messages: true | null;
    /**
     * Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See [accent colors](https://core.telegram.org/bots/api#accent-colors) for more details.
     */
    accent_color_id: number;
    /**
     * The maximum number of reactions that can be set on a message in the chat
     */
    max_reaction_count: number;
    /**
     * Chat photo
     */
    photo: ChatPhoto | null;
    /**
     * If non-empty, the list of all [active chat usernames](https://telegram.org/blog/topics-in-groups-collectible-usernames#collectible-usernames); for private chats, supergroups and channels
     */
    active_usernames: string[] | null;
    /**
     * For private chats, the date of birth of the user
     */
    birthdate: Birthdate | null;
    /**
     * For private chats with business accounts, the intro of the business
     */
    business_intro: BusinessIntro | null;
    /**
     * For private chats with business accounts, the location of the business
     */
    business_location: BusinessLocation | null;
    /**
     * For private chats with business accounts, the opening hours of the business
     */
    business_opening_hours: BusinessOpeningHours | null;
    /**
     * For private chats, the personal channel of the user
     */
    personal_chat: Chat | null;
    /**
     * Information about the corresponding channel chat; for direct messages chats only
     */
    parent_chat: Chat | null;
    /**
     * List of available reactions allowed in the chat. If omitted, then all [emoji reactions](https://core.telegram.org/bots/api#reactiontypeemoji) are allowed.
     */
    available_reactions: ReactionType[] | null;
    /**
     * Custom emoji identifier of the emoji chosen by the chat for the reply header and link preview background
     */
    background_custom_emoji_id: string | null;
    /**
     * Identifier of the accent color for the chat's profile background. See [profile accent colors](https://core.telegram.org/bots/api#profile-accent-colors) for more details.
     */
    profile_accent_color_id: number | null;
    /**
     * Custom emoji identifier of the emoji chosen by the chat for its profile background
     */
    profile_background_custom_emoji_id: string | null;
    /**
     * Custom emoji identifier of the emoji status of the chat or the other party in a private chat
     */
    emoji_status_custom_emoji_id: string | null;
    /**
     * Expiration date of the emoji status of the chat or the other party in a private chat, in Unix time, if any
     */
    emoji_status_expiration_date: number | null;
    /**
     * Bio of the other party in a private chat
     */
    bio: string | null;
    /**
     * *True*, if privacy settings of the other party in the private chat allows to use `tg://user?id=<user_id>` links only in chats with the user
     */
    has_private_forwards: true | null;
    /**
     * *True*, if the privacy settings of the other party restrict sending voice and video note messages in the private chat
     */
    has_restricted_voice_and_video_messages: true | null;
    /**
     * *True*, if users need to join the supergroup before they can send messages
     */
    join_to_send_messages: true | null;
    /**
     * *True*, if all users directly joining the supergroup without using an invite link need to be approved by supergroup administrators
     */
    join_by_request: true | null;
    /**
     * Description, for groups, supergroups and channel chats
     */
    description: string | null;
    /**
     * Primary invite link, for groups, supergroups and channel chats
     */
    invite_link: string | null;
    /**
     * The most recent pinned message (by sending date)
     */
    pinned_message: Message | null;
    /**
     * Default chat member permissions, for groups and supergroups
     */
    permissions: ChatPermissions | null;
    /**
     * Information about types of gifts that are accepted by the chat or by the corresponding user for private chats
     */
    accepted_gift_types: AcceptedGiftTypes;
    /**
     * *True*, if paid media messages can be sent or forwarded to the channel chat. The field is available only for channel chats.
     */
    can_send_paid_media: true | null;
    /**
     * For supergroups, the minimum allowed delay between consecutive messages sent by each unprivileged user; in seconds
     */
    slow_mode_delay: number | null;
    /**
     * For supergroups, the minimum number of boosts that a non-administrator user needs to add in order to ignore slow mode and chat permissions
     */
    unrestrict_boost_count: number | null;
    /**
     * The time after which all messages sent to the chat will be automatically deleted; in seconds
     */
    message_auto_delete_time: number | null;
    /**
     * *True*, if aggressive anti-spam checks are enabled in the supergroup. The field is only available to chat administrators.
     */
    has_aggressive_anti_spam_enabled: true | null;
    /**
     * *True*, if non-administrators can only get the list of bots and administrators in the chat
     */
    has_hidden_members: true | null;
    /**
     * *True*, if messages from the chat can't be forwarded to other chats
     */
    has_protected_content: true | null;
    /**
     * *True*, if new chat members will have access to old messages; available only to chat administrators
     */
    has_visible_history: true | null;
    /**
     * For supergroups, name of the group sticker set
     */
    sticker_set_name: string | null;
    /**
     * *True*, if the bot can change the group sticker set
     */
    can_set_sticker_set: true | null;
    /**
     * For supergroups, the name of the group's custom emoji sticker set. Custom emoji from this set can be used by all users and bots in the group.
     */
    custom_emoji_sticker_set_name: string | null;
    /**
     * Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
     */
    linked_chat_id: number | null;
    /**
     * For supergroups, the location to which the supergroup is connected
     */
    location: ChatLocation | null;
    /**
     * For private chats, the rating of the user if any
     */
    rating: UserRating | null;
    /**
     * For private chats, the first audio added to the profile of the user
     */
    first_profile_audio: Audio | null;
    /**
     * The color scheme based on a unique gift that must be used for the chat's name, message replies and link previews
     */
    unique_gift_colors: UniqueGiftColors | null;
    /**
     * The number of Telegram Stars a general user have to pay to send a message to the chat
     */
    paid_message_star_count: number | null;
}
/**
 * This object represents a message.
 */
export interface Message {
    /**
     * Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
     */
    message_id: number;
    /**
     * Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
     */
    message_thread_id: number | null;
    /**
     * Information about the direct messages chat topic that contains the message
     */
    direct_messages_topic: DirectMessagesTopic | null;
    /**
     * Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
     */
    from: User | null;
    /**
     * Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field *from* contains a fake sender user in non-channel chats.
     */
    sender_chat: Chat | null;
    /**
     * If the sender of the message boosted the chat, the number of boosts added by the user
     */
    sender_boost_count: number | null;
    /**
     * The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
     */
    sender_business_bot: User | null;
    /**
     * Date the message was sent in Unix time. It is always a positive number, representing a valid date.
     */
    date: number;
    /**
     * Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
     */
    business_connection_id: string | null;
    /**
     * Chat the message belongs to
     */
    chat: Chat;
    /**
     * Information about the original message for forwarded messages
     */
    forward_origin: MessageOrigin | null;
    /**
     * *True*, if the message is sent to a topic in a forum supergroup or a private chat with the bot
     */
    is_topic_message: true | null;
    /**
     * *True*, if the message is a channel post that was automatically forwarded to the connected discussion group
     */
    is_automatic_forward: true | null;
    /**
     * For replies in the same chat and message thread, the original message. Note that the [Message](https://core.telegram.org/bots/api#message) object in this field will not contain further *reply_to_message* fields even if it itself is a reply.
     */
    reply_to_message: Message | null;
    /**
     * Information about the message that is being replied to, which may come from another chat or forum topic
     */
    external_reply: ExternalReplyInfo | null;
    /**
     * For replies that quote part of the original message, the quoted part of the message
     */
    quote: TextQuote | null;
    /**
     * For replies to a story, the original story
     */
    reply_to_story: Story | null;
    /**
     * Identifier of the specific checklist task that is being replied to
     */
    reply_to_checklist_task_id: number | null;
    /**
     * Bot through which the message was sent
     */
    via_bot: User | null;
    /**
     * Date the message was last edited in Unix time
     */
    edit_date: number | null;
    /**
     * *True*, if the message can't be forwarded
     */
    has_protected_content: true | null;
    /**
     * *True*, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
     */
    is_from_offline: true | null;
    /**
     * *True*, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
     */
    is_paid_post: true | null;
    /**
     * The unique identifier of a media message group this message belongs to
     */
    media_group_id: string | null;
    /**
     * Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
     */
    author_signature: string | null;
    /**
     * The number of Telegram Stars that were paid by the sender of the message to send it
     */
    paid_star_count: number | null;
    /**
     * For text messages, the actual UTF-8 text of the message
     */
    text: string | null;
    /**
     * For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
     */
    entities: MessageEntity[] | null;
    /**
     * Options used for link preview generation for the message, if it is a text message and link preview options were changed
     */
    link_preview_options: LinkPreviewOptions | null;
    /**
     * Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
     */
    suggested_post_info: SuggestedPostInfo | null;
    /**
     * Unique identifier of the message effect added to the message
     */
    effect_id: string | null;
    /**
     * Message is an animation, information about the animation. For backward compatibility, when this field is set, the *document* field will also be set
     */
    animation: Animation | null;
    /**
     * Message is an audio file, information about the file
     */
    audio: Audio | null;
    /**
     * Message is a general file, information about the file
     */
    document: Document | null;
    /**
     * Message contains paid media; information about the paid media
     */
    paid_media: PaidMediaInfo | null;
    /**
     * Message is a photo, available sizes of the photo
     */
    photo: PhotoSize[] | null;
    /**
     * Message is a sticker, information about the sticker
     */
    sticker: Sticker | null;
    /**
     * Message is a forwarded story
     */
    story: Story | null;
    /**
     * Message is a video, information about the video
     */
    video: Video | null;
    /**
     * Message is a [video note](https://telegram.org/blog/video-messages-and-telescope), information about the video message
     */
    video_note: VideoNote | null;
    /**
     * Message is a voice message, information about the file
     */
    voice: Voice | null;
    /**
     * Caption for the animation, audio, document, paid media, photo, video or voice
     */
    caption: string | null;
    /**
     * For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
     */
    caption_entities: MessageEntity[] | null;
    /**
     * *True*, if the caption must be shown above the message media
     */
    show_caption_above_media: true | null;
    /**
     * *True*, if the message media is covered by a spoiler animation
     */
    has_media_spoiler: true | null;
    /**
     * Message is a checklist
     */
    checklist: Checklist | null;
    /**
     * Message is a shared contact, information about the contact
     */
    contact: Contact | null;
    /**
     * Message is a dice with random value
     */
    dice: Dice | null;
    /**
     * Message is a game, information about the game. [More about games »](https://core.telegram.org/bots/api#games)
     */
    game: Game | null;
    /**
     * Message is a native poll, information about the poll
     */
    poll: Poll | null;
    /**
     * Message is a venue, information about the venue. For backward compatibility, when this field is set, the *location* field will also be set
     */
    venue: Venue | null;
    /**
     * Message is a shared location, information about the location
     */
    location: Location | null;
    /**
     * New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)
     */
    new_chat_members: User[] | null;
    /**
     * A member was removed from the group, information about them (this member may be the bot itself)
     */
    left_chat_member: User | null;
    /**
     * Service message: chat owner has left
     */
    chat_owner_left: ChatOwnerLeft | null;
    /**
     * Service message: chat owner has changed
     */
    chat_owner_changed: ChatOwnerChanged | null;
    /**
     * A chat title was changed to this value
     */
    new_chat_title: string | null;
    /**
     * A chat photo was change to this value
     */
    new_chat_photo: PhotoSize[] | null;
    /**
     * Service message: the chat photo was deleted
     */
    delete_chat_photo: true | null;
    /**
     * Service message: the group has been created
     */
    group_chat_created: true | null;
    /**
     * Service message: the supergroup has been created. This field can't be received in a message coming through updates, because bot can't be a member of a supergroup when it is created. It can only be found in reply_to_message if someone replies to a very first message in a directly created supergroup.
     */
    supergroup_chat_created: true | null;
    /**
     * Service message: the channel has been created. This field can't be received in a message coming through updates, because bot can't be a member of a channel when it is created. It can only be found in reply_to_message if someone replies to a very first message in a channel.
     */
    channel_chat_created: true | null;
    /**
     * Service message: auto-delete timer settings changed in the chat
     */
    message_auto_delete_timer_changed: MessageAutoDeleteTimerChanged | null;
    /**
     * The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
     */
    migrate_to_chat_id: number | null;
    /**
     * The supergroup has been migrated from a group with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
     */
    migrate_from_chat_id: number | null;
    /**
     * Specified message was pinned. Note that the [Message](https://core.telegram.org/bots/api#message) object in this field will not contain further *reply_to_message* fields even if it itself is a reply.
     */
    pinned_message: MaybeInaccessibleMessage | null;
    /**
     * Message is an invoice for a [payment](https://core.telegram.org/bots/api#payments), information about the invoice. [More about payments »](https://core.telegram.org/bots/api#payments)
     */
    invoice: Invoice | null;
    /**
     * Message is a service message about a successful payment, information about the payment. [More about payments »](https://core.telegram.org/bots/api#payments)
     */
    successful_payment: SuccessfulPayment | null;
    /**
     * Message is a service message about a refunded payment, information about the payment. [More about payments »](https://core.telegram.org/bots/api#payments)
     */
    refunded_payment: RefundedPayment | null;
    /**
     * Service message: users were shared with the bot
     */
    users_shared: UsersShared | null;
    /**
     * Service message: a chat was shared with the bot
     */
    chat_shared: ChatShared | null;
    /**
     * Service message: a regular gift was sent or received
     */
    gift: GiftInfo | null;
    /**
     * Service message: a unique gift was sent or received
     */
    unique_gift: UniqueGiftInfo | null;
    /**
     * Service message: upgrade of a gift was purchased after the gift was sent
     */
    gift_upgrade_sent: GiftInfo | null;
    /**
     * The domain name of the website on which the user has logged in. [More about Telegram Login »](https://core.telegram.org/bots/api/widgets/login)
     */
    connected_website: string | null;
    /**
     * Service message: the user allowed the bot to write messages after adding it to the attachment or side menu, launching a Web App from a link, or accepting an explicit request from a Web App sent by the method [requestWriteAccess](https://core.telegram.org/bots/api/bots/webapps#initializing-mini-apps)
     */
    write_access_allowed: WriteAccessAllowed | null;
    /**
     * Telegram Passport data
     */
    passport_data: PassportData | null;
    /**
     * Service message. A user in the chat triggered another user's proximity alert while sharing Live Location.
     */
    proximity_alert_triggered: ProximityAlertTriggered | null;
    /**
     * Service message: user boosted the chat
     */
    boost_added: ChatBoostAdded | null;
    /**
     * Service message: chat background set
     */
    chat_background_set: ChatBackground | null;
    /**
     * Service message: some tasks in a checklist were marked as done or not done
     */
    checklist_tasks_done: ChecklistTasksDone | null;
    /**
     * Service message: tasks were added to a checklist
     */
    checklist_tasks_added: ChecklistTasksAdded | null;
    /**
     * Service message: the price for paid messages in the corresponding direct messages chat of a channel has changed
     */
    direct_message_price_changed: DirectMessagePriceChanged | null;
    /**
     * Service message: forum topic created
     */
    forum_topic_created: ForumTopicCreated | null;
    /**
     * Service message: forum topic edited
     */
    forum_topic_edited: ForumTopicEdited | null;
    /**
     * Service message: forum topic closed
     */
    forum_topic_closed: ForumTopicClosed | null;
    /**
     * Service message: forum topic reopened
     */
    forum_topic_reopened: ForumTopicReopened | null;
    /**
     * Service message: the 'General' forum topic hidden
     */
    general_forum_topic_hidden: GeneralForumTopicHidden | null;
    /**
     * Service message: the 'General' forum topic unhidden
     */
    general_forum_topic_unhidden: GeneralForumTopicUnhidden | null;
    /**
     * Service message: a scheduled giveaway was created
     */
    giveaway_created: GiveawayCreated | null;
    /**
     * The message is a scheduled giveaway message
     */
    giveaway: Giveaway | null;
    /**
     * A giveaway with public winners was completed
     */
    giveaway_winners: GiveawayWinners | null;
    /**
     * Service message: a giveaway without public winners was completed
     */
    giveaway_completed: GiveawayCompleted | null;
    /**
     * Service message: the price for paid messages has changed in the chat
     */
    paid_message_price_changed: PaidMessagePriceChanged | null;
    /**
     * Service message: a suggested post was approved
     */
    suggested_post_approved: SuggestedPostApproved | null;
    /**
     * Service message: approval of a suggested post has failed
     */
    suggested_post_approval_failed: SuggestedPostApprovalFailed | null;
    /**
     * Service message: a suggested post was declined
     */
    suggested_post_declined: SuggestedPostDeclined | null;
    /**
     * Service message: payment for a suggested post was received
     */
    suggested_post_paid: SuggestedPostPaid | null;
    /**
     * Service message: payment for a suggested post was refunded
     */
    suggested_post_refunded: SuggestedPostRefunded | null;
    /**
     * Service message: video chat scheduled
     */
    video_chat_scheduled: VideoChatScheduled | null;
    /**
     * Service message: video chat started
     */
    video_chat_started: VideoChatStarted | null;
    /**
     * Service message: video chat ended
     */
    video_chat_ended: VideoChatEnded | null;
    /**
     * Service message: new participants invited to a video chat
     */
    video_chat_participants_invited: VideoChatParticipantsInvited | null;
    /**
     * Service message: data sent by a Web App
     */
    web_app_data: WebAppData | null;
    /**
     * [Inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards) attached to the message. `login_url` buttons are represented as ordinary `url` buttons.
     */
    reply_markup: InlineKeyboardMarkup | null;
}
/**
 * This object represents a unique message identifier.
 */
export interface MessageId {
    /**
     * Unique message identifier. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
     */
    message_id: number;
}
/**
 * This object describes a message that was deleted or is otherwise inaccessible to the bot.
 */
export interface InaccessibleMessage {
    /**
     * Chat the message belonged to
     */
    chat: Chat;
    /**
     * Unique message identifier inside the chat
     */
    message_id: number;
    /**
     * Always 0. The field can be used to differentiate regular and inaccessible messages.
     */
    date: number;
}
/**
 * This object describes a message that can be inaccessible to the bot.
 */
export type MaybeInaccessibleMessage = Message | InaccessibleMessage;
/**
 * This object represents one special entity in a text message. For example, hashtags, usernames, URLs, etc.
 */
export interface MessageEntity {
    /**
     * Type of the entity. Currently, can be “mention” (`@username`), “hashtag” (`#hashtag` or `#hashtag@chatusername`), “cashtag” (`$USD` or `$USD@chatusername`), “bot_command” (`/start@jobs_bot`), “url” (`https://telegram.org`), “email” (`do-not-reply@telegram.org`), “phone_number” (`+1-212-555-0123`), “bold” (**bold text**), “italic” (*italic text*), “underline” (underlined text), “strikethrough” (strikethrough text), “spoiler” (spoiler message), “blockquote” (block quotation), “expandable_blockquote” (collapsed-by-default block quotation), “code” (monowidth string), “pre” (monowidth block), “text_link” (for clickable text URLs), “text_mention” (for users [without usernames](https://telegram.org/blog/edit#new-mentions)), “custom_emoji” (for inline custom emoji stickers)
     */
    type: string;
    /**
     * Offset in [UTF-16 code units](https://core.telegram.org/bots/api/api/entities#entity-length) to the start of the entity
     */
    offset: number;
    /**
     * Length of the entity in [UTF-16 code units](https://core.telegram.org/bots/api/api/entities#entity-length)
     */
    length: number;
    /**
     * For “text_link” only, URL that will be opened after user taps on the text
     */
    url: string | null;
    /**
     * For “text_mention” only, the mentioned user
     */
    user: User | null;
    /**
     * For “pre” only, the programming language of the entity text
     */
    language: string | null;
    /**
     * For “custom_emoji” only, unique identifier of the custom emoji. Use [getCustomEmojiStickers](https://core.telegram.org/bots/api#getcustomemojistickers) to get full information about the sticker
     */
    custom_emoji_id: string | null;
}
/**
 * This object contains information about the quoted part of a message that is replied to by the given message.
 */
export interface TextQuote {
    /**
     * Text of the quoted part of a message that is replied to by the given message
     */
    text: string;
    /**
     * Special entities that appear in the quote. Currently, only *bold*, *italic*, *underline*, *strikethrough*, *spoiler*, and *custom_emoji* entities are kept in quotes.
     */
    entities: MessageEntity[] | null;
    /**
     * Approximate quote position in the original message in UTF-16 code units as specified by the sender
     */
    position: number;
    /**
     * *True*, if the quote was chosen manually by the message sender. Otherwise, the quote was added automatically by the server.
     */
    is_manual: true | null;
}
/**
 * This object contains information about a message that is being replied to, which may come from another chat or forum topic.
 */
export interface ExternalReplyInfo {
    /**
     * Origin of the message replied to by the given message
     */
    origin: MessageOrigin;
    /**
     * Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
     */
    chat: Chat | null;
    /**
     * Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
     */
    message_id: number | null;
    /**
     * Options used for link preview generation for the original message, if it is a text message
     */
    link_preview_options: LinkPreviewOptions | null;
    /**
     * Message is an animation, information about the animation
     */
    animation: Animation | null;
    /**
     * Message is an audio file, information about the file
     */
    audio: Audio | null;
    /**
     * Message is a general file, information about the file
     */
    document: Document | null;
    /**
     * Message contains paid media; information about the paid media
     */
    paid_media: PaidMediaInfo | null;
    /**
     * Message is a photo, available sizes of the photo
     */
    photo: PhotoSize[] | null;
    /**
     * Message is a sticker, information about the sticker
     */
    sticker: Sticker | null;
    /**
     * Message is a forwarded story
     */
    story: Story | null;
    /**
     * Message is a video, information about the video
     */
    video: Video | null;
    /**
     * Message is a [video note](https://telegram.org/blog/video-messages-and-telescope), information about the video message
     */
    video_note: VideoNote | null;
    /**
     * Message is a voice message, information about the file
     */
    voice: Voice | null;
    /**
     * *True*, if the message media is covered by a spoiler animation
     */
    has_media_spoiler: true | null;
    /**
     * Message is a checklist
     */
    checklist: Checklist | null;
    /**
     * Message is a shared contact, information about the contact
     */
    contact: Contact | null;
    /**
     * Message is a dice with random value
     */
    dice: Dice | null;
    /**
     * Message is a game, information about the game. [More about games »](https://core.telegram.org/bots/api#games)
     */
    game: Game | null;
    /**
     * Message is a scheduled giveaway, information about the giveaway
     */
    giveaway: Giveaway | null;
    /**
     * A giveaway with public winners was completed
     */
    giveaway_winners: GiveawayWinners | null;
    /**
     * Message is an invoice for a [payment](https://core.telegram.org/bots/api#payments), information about the invoice. [More about payments »](https://core.telegram.org/bots/api#payments)
     */
    invoice: Invoice | null;
    /**
     * Message is a shared location, information about the location
     */
    location: Location | null;
    /**
     * Message is a native poll, information about the poll
     */
    poll: Poll | null;
    /**
     * Message is a venue, information about the venue
     */
    venue: Venue | null;
}
/**
 * Describes reply parameters for the message that is being sent.
 */
export interface ReplyParameters {
    /**
     * Identifier of the message that will be replied to in the current chat, or in the chat *chat_id* if it is specified
     */
    message_id: number;
    /**
     * If the message to be replied to is from a different chat, unique identifier for the chat or username of the channel (in the format `@channelusername`). Not supported for messages sent on behalf of a business account and messages from channel direct messages chats.
     */
    chat_id: number | string | null;
    /**
     * Pass *True* if the message should be sent even if the specified message to be replied to is not found. Always *False* for replies in another chat or forum topic. Always *True* for messages sent on behalf of a business account.
     */
    allow_sending_without_reply: boolean | null;
    /**
     * Quoted part of the message to be replied to; 0-1024 characters after entities parsing. The quote must be an exact substring of the message to be replied to, including *bold*, *italic*, *underline*, *strikethrough*, *spoiler*, and *custom_emoji* entities. The message will fail to send if the quote isn't found in the original message.
     */
    quote: string | null;
    /**
     * Mode for parsing entities in the quote. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    quote_parse_mode: string | null;
    /**
     * A JSON-serialized list of special entities that appear in the quote. It can be specified instead of *quote_parse_mode*.
     */
    quote_entities: MessageEntity[] | null;
    /**
     * Position of the quote in the original message in UTF-16 code units
     */
    quote_position: number | null;
    /**
     * Identifier of the specific checklist task to be replied to
     */
    checklist_task_id: number | null;
}
/**
 * This object describes the origin of a message.
 */
export type MessageOrigin = MessageOriginUser | MessageOriginHiddenUser | MessageOriginChat | MessageOriginChannel;
/**
 * The message was originally sent by a known user.
 */
export interface MessageOriginUser {
    /**
     * Type of the message origin, always “user”
     */
    type: string;
    /**
     * Date the message was sent originally in Unix time
     */
    date: number;
    /**
     * User that sent the message originally
     */
    sender_user: User;
}
/**
 * The message was originally sent by an unknown user.
 */
export interface MessageOriginHiddenUser {
    /**
     * Type of the message origin, always “hidden_user”
     */
    type: string;
    /**
     * Date the message was sent originally in Unix time
     */
    date: number;
    /**
     * Name of the user that sent the message originally
     */
    sender_user_name: string;
}
/**
 * The message was originally sent on behalf of a chat to a group chat.
 */
export interface MessageOriginChat {
    /**
     * Type of the message origin, always “chat”
     */
    type: string;
    /**
     * Date the message was sent originally in Unix time
     */
    date: number;
    /**
     * Chat that sent the message originally
     */
    sender_chat: Chat;
    /**
     * For messages originally sent by an anonymous chat administrator, original message author signature
     */
    author_signature: string | null;
}
/**
 * The message was originally sent to a channel chat.
 */
export interface MessageOriginChannel {
    /**
     * Type of the message origin, always “channel”
     */
    type: string;
    /**
     * Date the message was sent originally in Unix time
     */
    date: number;
    /**
     * Channel chat to which the message was originally sent
     */
    chat: Chat;
    /**
     * Unique message identifier inside the chat
     */
    message_id: number;
    /**
     * Signature of the original post author
     */
    author_signature: string | null;
}
/**
 * This object represents one size of a photo or a [file](https://core.telegram.org/bots/api#document) / [sticker](https://core.telegram.org/bots/api#sticker) thumbnail.
 */
export interface PhotoSize {
    /**
     * Identifier for this file, which can be used to download or reuse the file
     */
    file_id: string;
    /**
     * Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
     */
    file_unique_id: string;
    /**
     * Photo width
     */
    width: number;
    /**
     * Photo height
     */
    height: number;
    /**
     * File size in bytes
     */
    file_size: number | null;
}
/**
 * This object represents an animation file (GIF or H.264/MPEG-4 AVC video without sound).
 */
export interface Animation {
    /**
     * Identifier for this file, which can be used to download or reuse the file
     */
    file_id: string;
    /**
     * Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
     */
    file_unique_id: string;
    /**
     * Video width as defined by the sender
     */
    width: number;
    /**
     * Video height as defined by the sender
     */
    height: number;
    /**
     * Duration of the video in seconds as defined by the sender
     */
    duration: number;
    /**
     * Animation thumbnail as defined by the sender
     */
    thumbnail: PhotoSize | null;
    /**
     * Original animation filename as defined by the sender
     */
    file_name: string | null;
    /**
     * MIME type of the file as defined by the sender
     */
    mime_type: string | null;
    /**
     * File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
     */
    file_size: number | null;
}
/**
 * This object represents an audio file to be treated as music by the Telegram clients.
 */
export interface Audio {
    /**
     * Identifier for this file, which can be used to download or reuse the file
     */
    file_id: string;
    /**
     * Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
     */
    file_unique_id: string;
    /**
     * Duration of the audio in seconds as defined by the sender
     */
    duration: number;
    /**
     * Performer of the audio as defined by the sender or by audio tags
     */
    performer: string | null;
    /**
     * Title of the audio as defined by the sender or by audio tags
     */
    title: string | null;
    /**
     * Original filename as defined by the sender
     */
    file_name: string | null;
    /**
     * MIME type of the file as defined by the sender
     */
    mime_type: string | null;
    /**
     * File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
     */
    file_size: number | null;
    /**
     * Thumbnail of the album cover to which the music file belongs
     */
    thumbnail: PhotoSize | null;
}
/**
 * This object represents a general file (as opposed to [photos](https://core.telegram.org/bots/api#photosize), [voice messages](https://core.telegram.org/bots/api#voice) and [audio files](https://core.telegram.org/bots/api#audio)).
 */
export interface Document {
    /**
     * Identifier for this file, which can be used to download or reuse the file
     */
    file_id: string;
    /**
     * Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
     */
    file_unique_id: string;
    /**
     * Document thumbnail as defined by the sender
     */
    thumbnail: PhotoSize | null;
    /**
     * Original filename as defined by the sender
     */
    file_name: string | null;
    /**
     * MIME type of the file as defined by the sender
     */
    mime_type: string | null;
    /**
     * File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
     */
    file_size: number | null;
}
/**
 * This object represents a story.
 */
export interface Story {
    /**
     * Chat that posted the story
     */
    chat: Chat;
    /**
     * Unique identifier for the story in the chat
     */
    id: number;
}
/**
 * This object represents a video file of a specific quality.
 */
export interface VideoQuality {
    /**
     * Identifier for this file, which can be used to download or reuse the file
     */
    file_id: string;
    /**
     * Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
     */
    file_unique_id: string;
    /**
     * Video width
     */
    width: number;
    /**
     * Video height
     */
    height: number;
    /**
     * Codec that was used to encode the video, for example, “h264”, “h265”, or “av01”
     */
    codec: string;
    /**
     * File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
     */
    file_size: number | null;
}
/**
 * This object represents a video file.
 */
export interface Video {
    /**
     * Identifier for this file, which can be used to download or reuse the file
     */
    file_id: string;
    /**
     * Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
     */
    file_unique_id: string;
    /**
     * Video width as defined by the sender
     */
    width: number;
    /**
     * Video height as defined by the sender
     */
    height: number;
    /**
     * Duration of the video in seconds as defined by the sender
     */
    duration: number;
    /**
     * Video thumbnail
     */
    thumbnail: PhotoSize | null;
    /**
     * Available sizes of the cover of the video in the message
     */
    cover: PhotoSize[] | null;
    /**
     * Timestamp in seconds from which the video will play in the message
     */
    start_timestamp: number | null;
    /**
     * List of available qualities of the video
     */
    qualities: VideoQuality[] | null;
    /**
     * Original filename as defined by the sender
     */
    file_name: string | null;
    /**
     * MIME type of the file as defined by the sender
     */
    mime_type: string | null;
    /**
     * File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
     */
    file_size: number | null;
}
/**
 * This object represents a [video message](https://telegram.org/blog/video-messages-and-telescope) (available in Telegram apps as of [v.4.0](https://telegram.org/blog/video-messages-and-telescope)).
 */
export interface VideoNote {
    /**
     * Identifier for this file, which can be used to download or reuse the file
     */
    file_id: string;
    /**
     * Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
     */
    file_unique_id: string;
    /**
     * Video width and height (diameter of the video message) as defined by the sender
     */
    length: number;
    /**
     * Duration of the video in seconds as defined by the sender
     */
    duration: number;
    /**
     * Video thumbnail
     */
    thumbnail: PhotoSize | null;
    /**
     * File size in bytes
     */
    file_size: number | null;
}
/**
 * This object represents a voice note.
 */
export interface Voice {
    /**
     * Identifier for this file, which can be used to download or reuse the file
     */
    file_id: string;
    /**
     * Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
     */
    file_unique_id: string;
    /**
     * Duration of the audio in seconds as defined by the sender
     */
    duration: number;
    /**
     * MIME type of the file as defined by the sender
     */
    mime_type: string | null;
    /**
     * File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
     */
    file_size: number | null;
}
/**
 * Describes the paid media added to a message.
 */
export interface PaidMediaInfo {
    /**
     * The number of Telegram Stars that must be paid to buy access to the media
     */
    star_count: number;
    /**
     * Information about the paid media
     */
    paid_media: PaidMedia[];
}
/**
 * This object describes paid media.
 */
export type PaidMedia = PaidMediaPreview | PaidMediaPhoto | PaidMediaVideo;
/**
 * The paid media isn't available before the payment.
 */
export interface PaidMediaPreview {
    /**
     * Type of the paid media, always “preview”
     */
    type: string;
    /**
     * Media width as defined by the sender
     */
    width: number | null;
    /**
     * Media height as defined by the sender
     */
    height: number | null;
    /**
     * Duration of the media in seconds as defined by the sender
     */
    duration: number | null;
}
/**
 * The paid media is a photo.
 */
export interface PaidMediaPhoto {
    /**
     * Type of the paid media, always “photo”
     */
    type: string;
    /**
     * The photo
     */
    photo: PhotoSize[];
}
/**
 * The paid media is a video.
 */
export interface PaidMediaVideo {
    /**
     * Type of the paid media, always “video”
     */
    type: string;
    /**
     * The video
     */
    video: Video;
}
/**
 * This object represents a phone contact.
 */
export interface Contact {
    /**
     * Contact's phone number
     */
    phone_number: string;
    /**
     * Contact's first name
     */
    first_name: string;
    /**
     * Contact's last name
     */
    last_name: string | null;
    /**
     * Contact's user identifier in Telegram. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
     */
    user_id: number | null;
    /**
     * Additional data about the contact in the form of a [vCard](https://en.wikipedia.org/wiki/VCard)
     */
    vcard: string | null;
}
/**
 * This object represents an animated emoji that displays a random value.
 */
export interface Dice {
    /**
     * Emoji on which the dice throw animation is based
     */
    emoji: string;
    /**
     * Value of the dice, 1-6 for “🎲”, “🎯” and “🎳” base emoji, 1-5 for “🏀” and “⚽” base emoji, 1-64 for “🎰” base emoji
     */
    value: number;
}
/**
 * This object contains information about one answer option in a poll.
 */
export interface PollOption {
    /**
     * Option text, 1-100 characters
     */
    text: string;
    /**
     * Special entities that appear in the option *text*. Currently, only custom emoji entities are allowed in poll option texts
     */
    text_entities: MessageEntity[] | null;
    /**
     * Number of users that voted for this option
     */
    voter_count: number;
}
/**
 * This object contains information about one answer option in a poll to be sent.
 */
export interface InputPollOption {
    /**
     * Option text, 1-100 characters
     */
    text: string;
    /**
     * Mode for parsing entities in the text. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details. Currently, only custom emoji entities are allowed
     */
    text_parse_mode: string | null;
    /**
     * A JSON-serialized list of special entities that appear in the poll option text. It can be specified instead of *text_parse_mode*
     */
    text_entities: MessageEntity[] | null;
}
/**
 * This object represents an answer of a user in a non-anonymous poll.
 */
export interface PollAnswer {
    /**
     * Unique poll identifier
     */
    poll_id: string;
    /**
     * The chat that changed the answer to the poll, if the voter is anonymous
     */
    voter_chat: Chat | null;
    /**
     * The user that changed the answer to the poll, if the voter isn't anonymous
     */
    user: User | null;
    /**
     * 0-based identifiers of chosen answer options. May be empty if the vote was retracted.
     */
    option_ids: number[];
}
/**
 * This object contains information about a poll.
 */
export interface Poll {
    /**
     * Unique poll identifier
     */
    id: string;
    /**
     * Poll question, 1-300 characters
     */
    question: string;
    /**
     * Special entities that appear in the *question*. Currently, only custom emoji entities are allowed in poll questions
     */
    question_entities: MessageEntity[] | null;
    /**
     * List of poll options
     */
    options: PollOption[];
    /**
     * Total number of users that voted in the poll
     */
    total_voter_count: number;
    /**
     * *True*, if the poll is closed
     */
    is_closed: boolean;
    /**
     * *True*, if the poll is anonymous
     */
    is_anonymous: boolean;
    /**
     * Poll type, currently can be “regular” or “quiz”
     */
    type: string;
    /**
     * *True*, if the poll allows multiple answers
     */
    allows_multiple_answers: boolean;
    /**
     * 0-based identifier of the correct answer option. Available only for polls in the quiz mode, which are closed, or was sent (not forwarded) by the bot or to the private chat with the bot.
     */
    correct_option_id: number | null;
    /**
     * Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters
     */
    explanation: string | null;
    /**
     * Special entities like usernames, URLs, bot commands, etc. that appear in the *explanation*
     */
    explanation_entities: MessageEntity[] | null;
    /**
     * Amount of time in seconds the poll will be active after creation
     */
    open_period: number | null;
    /**
     * Point in time (Unix timestamp) when the poll will be automatically closed
     */
    close_date: number | null;
}
/**
 * Describes a task in a checklist.
 */
export interface ChecklistTask {
    /**
     * Unique identifier of the task
     */
    id: number;
    /**
     * Text of the task
     */
    text: string;
    /**
     * Special entities that appear in the task text
     */
    text_entities: MessageEntity[] | null;
    /**
     * User that completed the task; omitted if the task wasn't completed by a user
     */
    completed_by_user: User | null;
    /**
     * Chat that completed the task; omitted if the task wasn't completed by a chat
     */
    completed_by_chat: Chat | null;
    /**
     * Point in time (Unix timestamp) when the task was completed; 0 if the task wasn't completed
     */
    completion_date: number | null;
}
/**
 * Describes a checklist.
 */
export interface Checklist {
    /**
     * Title of the checklist
     */
    title: string;
    /**
     * Special entities that appear in the checklist title
     */
    title_entities: MessageEntity[] | null;
    /**
     * List of tasks in the checklist
     */
    tasks: ChecklistTask[];
    /**
     * *True*, if users other than the creator of the list can add tasks to the list
     */
    others_can_add_tasks: true | null;
    /**
     * *True*, if users other than the creator of the list can mark tasks as done or not done
     */
    others_can_mark_tasks_as_done: true | null;
}
/**
 * Describes a task to add to a checklist.
 */
export interface InputChecklistTask {
    /**
     * Unique identifier of the task; must be positive and unique among all task identifiers currently present in the checklist
     */
    id: number;
    /**
     * Text of the task; 1-100 characters after entities parsing
     */
    text: string;
    /**
     * Mode for parsing entities in the text. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * List of special entities that appear in the text, which can be specified instead of parse_mode. Currently, only *bold*, *italic*, *underline*, *strikethrough*, *spoiler*, and *custom_emoji* entities are allowed.
     */
    text_entities: MessageEntity[] | null;
}
/**
 * Describes a checklist to create.
 */
export interface InputChecklist {
    /**
     * Title of the checklist; 1-255 characters after entities parsing
     */
    title: string;
    /**
     * Mode for parsing entities in the title. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * List of special entities that appear in the title, which can be specified instead of parse_mode. Currently, only *bold*, *italic*, *underline*, *strikethrough*, *spoiler*, and *custom_emoji* entities are allowed.
     */
    title_entities: MessageEntity[] | null;
    /**
     * List of 1-30 tasks in the checklist
     */
    tasks: InputChecklistTask[];
    /**
     * Pass *True* if other users can add tasks to the checklist
     */
    others_can_add_tasks: boolean | null;
    /**
     * Pass *True* if other users can mark tasks as done or not done in the checklist
     */
    others_can_mark_tasks_as_done: boolean | null;
}
/**
 * Describes a service message about checklist tasks marked as done or not done.
 */
export interface ChecklistTasksDone {
    /**
     * Message containing the checklist whose tasks were marked as done or not done. Note that the [Message](https://core.telegram.org/bots/api#message) object in this field will not contain the *reply_to_message* field even if it itself is a reply.
     */
    checklist_message: Message | null;
    /**
     * Identifiers of the tasks that were marked as done
     */
    marked_as_done_task_ids: number[] | null;
    /**
     * Identifiers of the tasks that were marked as not done
     */
    marked_as_not_done_task_ids: number[] | null;
}
/**
 * Describes a service message about tasks added to a checklist.
 */
export interface ChecklistTasksAdded {
    /**
     * Message containing the checklist to which the tasks were added. Note that the [Message](https://core.telegram.org/bots/api#message) object in this field will not contain the *reply_to_message* field even if it itself is a reply.
     */
    checklist_message: Message | null;
    /**
     * List of tasks added to the checklist
     */
    tasks: ChecklistTask[];
}
/**
 * This object represents a point on the map.
 */
export interface Location {
    /**
     * Latitude as defined by the sender
     */
    latitude: number;
    /**
     * Longitude as defined by the sender
     */
    longitude: number;
    /**
     * The radius of uncertainty for the location, measured in meters; 0-1500
     */
    horizontal_accuracy: number | null;
    /**
     * Time relative to the message sending date, during which the location can be updated; in seconds. For active live locations only.
     */
    live_period: number | null;
    /**
     * The direction in which user is moving, in degrees; 1-360. For active live locations only.
     */
    heading: number | null;
    /**
     * The maximum distance for proximity alerts about approaching another chat member, in meters. For sent live locations only.
     */
    proximity_alert_radius: number | null;
}
/**
 * This object represents a venue.
 */
export interface Venue {
    /**
     * Venue location. Can't be a live location
     */
    location: Location;
    /**
     * Name of the venue
     */
    title: string;
    /**
     * Address of the venue
     */
    address: string;
    /**
     * Foursquare identifier of the venue
     */
    foursquare_id: string | null;
    /**
     * Foursquare type of the venue. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
     */
    foursquare_type: string | null;
    /**
     * Google Places identifier of the venue
     */
    google_place_id: string | null;
    /**
     * Google Places type of the venue. (See [supported types](https://developers.google.com/places/web-service/supported_types).)
     */
    google_place_type: string | null;
}
/**
 * Describes data sent from a [Web App](https://core.telegram.org/bots/api/bots/webapps) to the bot.
 */
export interface WebAppData {
    /**
     * The data. Be aware that a bad client can send arbitrary data in this field.
     */
    data: string;
    /**
     * Text of the *web_app* keyboard button from which the Web App was opened. Be aware that a bad client can send arbitrary data in this field.
     */
    button_text: string;
}
/**
 * This object represents the content of a service message, sent whenever a user in the chat triggers a proximity alert set by another user.
 */
export interface ProximityAlertTriggered {
    /**
     * User that triggered the alert
     */
    traveler: User;
    /**
     * User that set the alert
     */
    watcher: User;
    /**
     * The distance between the users
     */
    distance: number;
}
/**
 * This object represents a service message about a change in auto-delete timer settings.
 */
export interface MessageAutoDeleteTimerChanged {
    /**
     * New auto-delete time for messages in the chat; in seconds
     */
    message_auto_delete_time: number;
}
/**
 * This object represents a service message about a user boosting a chat.
 */
export interface ChatBoostAdded {
    /**
     * Number of boosts added by the user
     */
    boost_count: number;
}
/**
 * This object describes the way a background is filled based on the selected colors.
 */
export type BackgroundFill = BackgroundFillSolid | BackgroundFillGradient | BackgroundFillFreeformGradient;
/**
 * The background is filled using the selected color.
 */
export interface BackgroundFillSolid {
    /**
     * Type of the background fill, always “solid”
     */
    type: string;
    /**
     * The color of the background fill in the RGB24 format
     */
    color: number;
}
/**
 * The background is a gradient fill.
 */
export interface BackgroundFillGradient {
    /**
     * Type of the background fill, always “gradient”
     */
    type: string;
    /**
     * Top color of the gradient in the RGB24 format
     */
    top_color: number;
    /**
     * Bottom color of the gradient in the RGB24 format
     */
    bottom_color: number;
    /**
     * Clockwise rotation angle of the background fill in degrees; 0-359
     */
    rotation_angle: number;
}
/**
 * The background is a freeform gradient that rotates after every message in the chat.
 */
export interface BackgroundFillFreeformGradient {
    /**
     * Type of the background fill, always “freeform_gradient”
     */
    type: string;
    /**
     * A list of the 3 or 4 base colors that are used to generate the freeform gradient in the RGB24 format
     */
    colors: number[];
}
/**
 * This object describes the type of a background.
 */
export type BackgroundType = BackgroundTypeFill | BackgroundTypeWallpaper | BackgroundTypePattern | BackgroundTypeChatTheme;
/**
 * The background is automatically filled based on the selected colors.
 */
export interface BackgroundTypeFill {
    /**
     * Type of the background, always “fill”
     */
    type: string;
    /**
     * The background fill
     */
    fill: BackgroundFill;
    /**
     * Dimming of the background in dark themes, as a percentage; 0-100
     */
    dark_theme_dimming: number;
}
/**
 * The background is a wallpaper in the JPEG format.
 */
export interface BackgroundTypeWallpaper {
    /**
     * Type of the background, always “wallpaper”
     */
    type: string;
    /**
     * Document with the wallpaper
     */
    document: Document;
    /**
     * Dimming of the background in dark themes, as a percentage; 0-100
     */
    dark_theme_dimming: number;
    /**
     * *True*, if the wallpaper is downscaled to fit in a 450x450 square and then box-blurred with radius 12
     */
    is_blurred: true | null;
    /**
     * *True*, if the background moves slightly when the device is tilted
     */
    is_moving: true | null;
}
/**
 * The background is a .PNG or .TGV (gzipped subset of SVG with MIME type “application/x-tgwallpattern”) pattern to be combined with the background fill chosen by the user.
 */
export interface BackgroundTypePattern {
    /**
     * Type of the background, always “pattern”
     */
    type: string;
    /**
     * Document with the pattern
     */
    document: Document;
    /**
     * The background fill that is combined with the pattern
     */
    fill: BackgroundFill;
    /**
     * Intensity of the pattern when it is shown above the filled background; 0-100
     */
    intensity: number;
    /**
     * *True*, if the background fill must be applied only to the pattern itself. All other pixels are black in this case. For dark themes only
     */
    is_inverted: true | null;
    /**
     * *True*, if the background moves slightly when the device is tilted
     */
    is_moving: true | null;
}
/**
 * The background is taken directly from a built-in chat theme.
 */
export interface BackgroundTypeChatTheme {
    /**
     * Type of the background, always “chat_theme”
     */
    type: string;
    /**
     * Name of the chat theme, which is usually an emoji
     */
    theme_name: string;
}
/**
 * This object represents a chat background.
 */
export interface ChatBackground {
    /**
     * Type of the background
     */
    type: BackgroundType;
}
/**
 * This object represents a service message about a new forum topic created in the chat.
 */
export interface ForumTopicCreated {
    /**
     * Name of the topic
     */
    name: string;
    /**
     * Color of the topic icon in RGB format
     */
    icon_color: number;
    /**
     * Unique identifier of the custom emoji shown as the topic icon
     */
    icon_custom_emoji_id: string | null;
    /**
     * *True*, if the name of the topic wasn't specified explicitly by its creator and likely needs to be changed by the bot
     */
    is_name_implicit: true | null;
}
/**
 * This object represents a service message about a forum topic closed in the chat. Currently holds no information.
 */
// deno-lint-ignore no-empty-interface
export interface ForumTopicClosed {}
/**
 * This object represents a service message about an edited forum topic.
 */
export interface ForumTopicEdited {
    /**
     * New name of the topic, if it was edited
     */
    name: string | null;
    /**
     * New identifier of the custom emoji shown as the topic icon, if it was edited; an empty string if the icon was removed
     */
    icon_custom_emoji_id: string | null;
}
/**
 * This object represents a service message about a forum topic reopened in the chat. Currently holds no information.
 */
// deno-lint-ignore no-empty-interface
export interface ForumTopicReopened {}
/**
 * This object represents a service message about General forum topic hidden in the chat. Currently holds no information.
 */
// deno-lint-ignore no-empty-interface
export interface GeneralForumTopicHidden {}
/**
 * This object represents a service message about General forum topic unhidden in the chat. Currently holds no information.
 */
// deno-lint-ignore no-empty-interface
export interface GeneralForumTopicUnhidden {}
/**
 * This object contains information about a user that was shared with the bot using a [KeyboardButtonRequestUsers](https://core.telegram.org/bots/api#keyboardbuttonrequestusers) button.
 */
export interface SharedUser {
    /**
     * Identifier of the shared user. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so 64-bit integers or double-precision float types are safe for storing these identifiers. The bot may not have access to the user and could be unable to use this identifier, unless the user is already known to the bot by some other means.
     */
    user_id: number;
    /**
     * First name of the user, if the name was requested by the bot
     */
    first_name: string | null;
    /**
     * Last name of the user, if the name was requested by the bot
     */
    last_name: string | null;
    /**
     * Username of the user, if the username was requested by the bot
     */
    username: string | null;
    /**
     * Available sizes of the chat photo, if the photo was requested by the bot
     */
    photo: PhotoSize[] | null;
}
/**
 * This object contains information about the users whose identifiers were shared with the bot using a [KeyboardButtonRequestUsers](https://core.telegram.org/bots/api#keyboardbuttonrequestusers) button.
 */
export interface UsersShared {
    /**
     * Identifier of the request
     */
    request_id: number;
    /**
     * Information about users shared with the bot.
     */
    users: SharedUser[];
}
/**
 * This object contains information about a chat that was shared with the bot using a [KeyboardButtonRequestChat](https://core.telegram.org/bots/api#keyboardbuttonrequestchat) button.
 */
export interface ChatShared {
    /**
     * Identifier of the request
     */
    request_id: number;
    /**
     * Identifier of the shared chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. The bot may not have access to the chat and could be unable to use this identifier, unless the chat is already known to the bot by some other means.
     */
    chat_id: number;
    /**
     * Title of the chat, if the title was requested by the bot.
     */
    title: string | null;
    /**
     * Username of the chat, if the username was requested by the bot and available.
     */
    username: string | null;
    /**
     * Available sizes of the chat photo, if the photo was requested by the bot
     */
    photo: PhotoSize[] | null;
}
/**
 * This object represents a service message about a user allowing a bot to write messages after adding it to the attachment menu, launching a Web App from a link, or accepting an explicit request from a Web App sent by the method [requestWriteAccess](https://core.telegram.org/bots/api/bots/webapps#initializing-mini-apps).
 */
export interface WriteAccessAllowed {
    /**
     * *True*, if the access was granted after the user accepted an explicit request from a Web App sent by the method [requestWriteAccess](https://core.telegram.org/bots/api/bots/webapps#initializing-mini-apps)
     */
    from_request: boolean | null;
    /**
     * Name of the Web App, if the access was granted when the Web App was launched from a link
     */
    web_app_name: string | null;
    /**
     * *True*, if the access was granted when the bot was added to the attachment or side menu
     */
    from_attachment_menu: boolean | null;
}
/**
 * This object represents a service message about a video chat scheduled in the chat.
 */
export interface VideoChatScheduled {
    /**
     * Point in time (Unix timestamp) when the video chat is supposed to be started by a chat administrator
     */
    start_date: number;
}
/**
 * This object represents a service message about a video chat started in the chat. Currently holds no information.
 */
// deno-lint-ignore no-empty-interface
export interface VideoChatStarted {}
/**
 * This object represents a service message about a video chat ended in the chat.
 */
export interface VideoChatEnded {
    /**
     * Video chat duration in seconds
     */
    duration: number;
}
/**
 * This object represents a service message about new members invited to a video chat.
 */
export interface VideoChatParticipantsInvited {
    /**
     * New members that were invited to the video chat
     */
    users: User[];
}
/**
 * Describes a service message about a change in the price of paid messages within a chat.
 */
export interface PaidMessagePriceChanged {
    /**
     * The new number of Telegram Stars that must be paid by non-administrator users of the supergroup chat for each sent message
     */
    paid_message_star_count: number;
}
/**
 * Describes a service message about a change in the price of direct messages sent to a channel chat.
 */
export interface DirectMessagePriceChanged {
    /**
     * *True*, if direct messages are enabled for the channel chat; false otherwise
     */
    are_direct_messages_enabled: boolean;
    /**
     * The new number of Telegram Stars that must be paid by users for each direct message sent to the channel. Does not apply to users who have been exempted by administrators. Defaults to 0.
     */
    direct_message_star_count: number | null;
}
/**
 * Describes a service message about the approval of a suggested post.
 */
export interface SuggestedPostApproved {
    /**
     * Message containing the suggested post. Note that the [Message](https://core.telegram.org/bots/api#message) object in this field will not contain the *reply_to_message* field even if it itself is a reply.
     */
    suggested_post_message: Message | null;
    /**
     * Amount paid for the post
     */
    price: SuggestedPostPrice | null;
    /**
     * Date when the post will be published
     */
    send_date: number;
}
/**
 * Describes a service message about the failed approval of a suggested post. Currently, only caused by insufficient user funds at the time of approval.
 */
export interface SuggestedPostApprovalFailed {
    /**
     * Message containing the suggested post whose approval has failed. Note that the [Message](https://core.telegram.org/bots/api#message) object in this field will not contain the *reply_to_message* field even if it itself is a reply.
     */
    suggested_post_message: Message | null;
    /**
     * Expected price of the post
     */
    price: SuggestedPostPrice;
}
/**
 * Describes a service message about the rejection of a suggested post.
 */
export interface SuggestedPostDeclined {
    /**
     * Message containing the suggested post. Note that the [Message](https://core.telegram.org/bots/api#message) object in this field will not contain the *reply_to_message* field even if it itself is a reply.
     */
    suggested_post_message: Message | null;
    /**
     * Comment with which the post was declined
     */
    comment: string | null;
}
/**
 * Describes a service message about a successful payment for a suggested post.
 */
export interface SuggestedPostPaid {
    /**
     * Message containing the suggested post. Note that the [Message](https://core.telegram.org/bots/api#message) object in this field will not contain the *reply_to_message* field even if it itself is a reply.
     */
    suggested_post_message: Message | null;
    /**
     * Currency in which the payment was made. Currently, one of “XTR” for Telegram Stars or “TON” for toncoins
     */
    currency: string;
    /**
     * The amount of the currency that was received by the channel in nanotoncoins; for payments in toncoins only
     */
    amount: number | null;
    /**
     * The amount of Telegram Stars that was received by the channel; for payments in Telegram Stars only
     */
    star_amount: StarAmount | null;
}
/**
 * Describes a service message about a payment refund for a suggested post.
 */
export interface SuggestedPostRefunded {
    /**
     * Message containing the suggested post. Note that the [Message](https://core.telegram.org/bots/api#message) object in this field will not contain the *reply_to_message* field even if it itself is a reply.
     */
    suggested_post_message: Message | null;
    /**
     * Reason for the refund. Currently, one of “post_deleted” if the post was deleted within 24 hours of being posted or removed from scheduled messages without being posted, or “payment_refunded” if the payer refunded their payment.
     */
    reason: string;
}
/**
 * This object represents a service message about the creation of a scheduled giveaway.
 */
export interface GiveawayCreated {
    /**
     * The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only
     */
    prize_star_count: number | null;
}
/**
 * This object represents a message about a scheduled giveaway.
 */
export interface Giveaway {
    /**
     * The list of chats which the user must join to participate in the giveaway
     */
    chats: Chat[];
    /**
     * Point in time (Unix timestamp) when winners of the giveaway will be selected
     */
    winners_selection_date: number;
    /**
     * The number of users which are supposed to be selected as winners of the giveaway
     */
    winner_count: number;
    /**
     * *True*, if only users who join the chats after the giveaway started should be eligible to win
     */
    only_new_members: true | null;
    /**
     * *True*, if the list of giveaway winners will be visible to everyone
     */
    has_public_winners: true | null;
    /**
     * Description of additional giveaway prize
     */
    prize_description: string | null;
    /**
     * A list of two-letter [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country codes indicating the countries from which eligible users for the giveaway must come. If empty, then all users can participate in the giveaway. Users with a phone number that was bought on Fragment can always participate in giveaways.
     */
    country_codes: string[] | null;
    /**
     * The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only
     */
    prize_star_count: number | null;
    /**
     * The number of months the Telegram Premium subscription won from the giveaway will be active for; for Telegram Premium giveaways only
     */
    premium_subscription_month_count: number | null;
}
/**
 * This object represents a message about the completion of a giveaway with public winners.
 */
export interface GiveawayWinners {
    /**
     * The chat that created the giveaway
     */
    chat: Chat;
    /**
     * Identifier of the message with the giveaway in the chat
     */
    giveaway_message_id: number;
    /**
     * Point in time (Unix timestamp) when winners of the giveaway were selected
     */
    winners_selection_date: number;
    /**
     * Total number of winners in the giveaway
     */
    winner_count: number;
    /**
     * List of up to 100 winners of the giveaway
     */
    winners: User[];
    /**
     * The number of other chats the user had to join in order to be eligible for the giveaway
     */
    additional_chat_count: number | null;
    /**
     * The number of Telegram Stars that were split between giveaway winners; for Telegram Star giveaways only
     */
    prize_star_count: number | null;
    /**
     * The number of months the Telegram Premium subscription won from the giveaway will be active for; for Telegram Premium giveaways only
     */
    premium_subscription_month_count: number | null;
    /**
     * Number of undistributed prizes
     */
    unclaimed_prize_count: number | null;
    /**
     * *True*, if only users who had joined the chats after the giveaway started were eligible to win
     */
    only_new_members: true | null;
    /**
     * *True*, if the giveaway was canceled because the payment for it was refunded
     */
    was_refunded: true | null;
    /**
     * Description of additional giveaway prize
     */
    prize_description: string | null;
}
/**
 * This object represents a service message about the completion of a giveaway without public winners.
 */
export interface GiveawayCompleted {
    /**
     * Number of winners in the giveaway
     */
    winner_count: number;
    /**
     * Number of undistributed prizes
     */
    unclaimed_prize_count: number | null;
    /**
     * Message with the giveaway that was completed, if it wasn't deleted
     */
    giveaway_message: Message | null;
    /**
     * *True*, if the giveaway is a Telegram Star giveaway. Otherwise, currently, the giveaway is a Telegram Premium giveaway.
     */
    is_star_giveaway: true | null;
}
/**
 * Describes the options used for link preview generation.
 */
export interface LinkPreviewOptions {
    /**
     * *True*, if the link preview is disabled
     */
    is_disabled: boolean | null;
    /**
     * URL to use for the link preview. If empty, then the first URL found in the message text will be used
     */
    url: string | null;
    /**
     * *True*, if the media in the link preview is supposed to be shrunk; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview
     */
    prefer_small_media: boolean | null;
    /**
     * *True*, if the media in the link preview is supposed to be enlarged; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview
     */
    prefer_large_media: boolean | null;
    /**
     * *True*, if the link preview must be shown above the message text; otherwise, the link preview will be shown below the message text
     */
    show_above_text: boolean | null;
}
/**
 * Describes the price of a suggested post.
 */
export interface SuggestedPostPrice {
    /**
     * Currency in which the post will be paid. Currently, must be one of “XTR” for Telegram Stars or “TON” for toncoins
     */
    currency: string;
    /**
     * The amount of the currency that will be paid for the post in the *smallest units* of the currency, i.e. Telegram Stars or nanotoncoins. Currently, price in Telegram Stars must be between 5 and 100000, and price in nanotoncoins must be between 10000000 and 10000000000000.
     */
    amount: number;
}
/**
 * Contains information about a suggested post.
 */
export interface SuggestedPostInfo {
    /**
     * State of the suggested post. Currently, it can be one of “pending”, “approved”, “declined”.
     */
    state: string;
    /**
     * Proposed price of the post. If the field is omitted, then the post is unpaid.
     */
    price: SuggestedPostPrice | null;
    /**
     * Proposed send date of the post. If the field is omitted, then the post can be published at any time within 30 days at the sole discretion of the user or administrator who approves it.
     */
    send_date: number | null;
}
/**
 * Contains parameters of a post that is being suggested by the bot.
 */
export interface SuggestedPostParameters {
    /**
     * Proposed price for the post. If the field is omitted, then the post is unpaid.
     */
    price: SuggestedPostPrice | null;
    /**
     * Proposed send date of the post. If specified, then the date must be between 300 second and 2678400 seconds (30 days) in the future. If the field is omitted, then the post can be published at any time within 30 days at the sole discretion of the user who approves it.
     */
    send_date: number | null;
}
/**
 * Describes a topic of a direct messages chat.
 */
export interface DirectMessagesTopic {
    /**
     * Unique identifier of the topic. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
     */
    topic_id: number;
    /**
     * Information about the user that created the topic. Currently, it is always present
     */
    user: User | null;
}
/**
 * This object represent a user's profile pictures.
 */
export interface UserProfilePhotos {
    /**
     * Total number of profile pictures the target user has
     */
    total_count: number;
    /**
     * Requested profile pictures (in up to 4 sizes each)
     */
    photos: PhotoSize[][];
}
/**
 * This object represents the audios displayed on a user's profile.
 */
export interface UserProfileAudios {
    /**
     * Total number of profile audios for the target user
     */
    total_count: number;
    /**
     * Requested profile audios
     */
    audios: Audio[];
}
/**
 * This object represents a file ready to be downloaded. The file can be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling [getFile](https://core.telegram.org/bots/api#getfile).
 */
export interface File {
    /**
     * Identifier for this file, which can be used to download or reuse the file
     */
    file_id: string;
    /**
     * Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
     */
    file_unique_id: string;
    /**
     * File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
     */
    file_size: number | null;
    /**
     * File path. Use `https://api.telegram.org/file/bot<token>/<file_path>` to get the file.
     */
    file_path: string | null;
}
/**
 * Describes a [Web App](https://core.telegram.org/bots/api/bots/webapps).
 */
export interface WebAppInfo {
    /**
     * An HTTPS URL of a Web App to be opened with additional data as specified in [Initializing Web Apps](https://core.telegram.org/bots/api/bots/webapps#initializing-mini-apps)
     */
    url: string;
}
/**
 * This object represents a [custom keyboard](https://core.telegram.org/bots/api/bots/features#keyboards) with reply options (see [Introduction to bots](https://core.telegram.org/bots/api/bots/features#keyboards) for details and examples). Not supported in channels and for messages sent on behalf of a Telegram Business account.
 */
export interface ReplyKeyboardMarkup {
    /**
     * Array of button rows, each represented by an Array of [KeyboardButton](https://core.telegram.org/bots/api#keyboardbutton) objects
     */
    keyboard: KeyboardButton[][];
    /**
     * Requests clients to always show the keyboard when the regular keyboard is hidden. Defaults to *false*, in which case the custom keyboard can be hidden and opened with a keyboard icon.
     */
    is_persistent: boolean | null;
    /**
     * Requests clients to resize the keyboard vertically for optimal fit (e.g., make the keyboard smaller if there are just two rows of buttons). Defaults to *false*, in which case the custom keyboard is always of the same height as the app's standard keyboard.
     */
    resize_keyboard: boolean | null;
    /**
     * Requests clients to hide the keyboard as soon as it's been used. The keyboard will still be available, but clients will automatically display the usual letter-keyboard in the chat - the user can press a special button in the input field to see the custom keyboard again. Defaults to *false*.
     */
    one_time_keyboard: boolean | null;
    /**
     * The placeholder to be shown in the input field when the keyboard is active; 1-64 characters
     */
    input_field_placeholder: string | null;
    /**
     * Use this parameter if you want to show the keyboard to specific users only. Targets: 1) users that are @mentioned in the *text* of the [Message](https://core.telegram.org/bots/api#message) object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message.
     *
     * *Example:* A user requests to change the bot's language, bot replies to the request with a keyboard to select the new language. Other users in the group don't see the keyboard.
     */
    selective: boolean | null;
}
/**
 * This object represents one button of the reply keyboard. At most one of the fields other than *text*, *icon_custom_emoji_id*, and *style* must be used to specify the type of the button. For simple text buttons, *String* can be used instead of this object to specify the button text.
 */
export interface KeyboardButton {
    /**
     * Text of the button. If none of the fields other than *text*, *icon_custom_emoji_id*, and *style* are used, it will be sent as a message when the button is pressed
     */
    text: string;
    /**
     * Unique identifier of the custom emoji shown before the text of the button. Can only be used by bots that purchased additional usernames on [Fragment](https://fragment.com) or in the messages directly sent by the bot to private, group and supergroup chats if the owner of the bot has a Telegram Premium subscription.
     */
    icon_custom_emoji_id: string | null;
    /**
     * Style of the button. Must be one of “danger” (red), “success” (green) or “primary” (blue). If omitted, then an app-specific style is used.
     */
    style: string | null;
    /**
     * If specified, pressing the button will open a list of suitable users. Identifiers of selected users will be sent to the bot in a “users_shared” service message. Available in private chats only.
     */
    request_users: KeyboardButtonRequestUsers | null;
    /**
     * If specified, pressing the button will open a list of suitable chats. Tapping on a chat will send its identifier to the bot in a “chat_shared” service message. Available in private chats only.
     */
    request_chat: KeyboardButtonRequestChat | null;
    /**
     * If *True*, the user's phone number will be sent as a contact when the button is pressed. Available in private chats only.
     */
    request_contact: boolean | null;
    /**
     * If *True*, the user's current location will be sent when the button is pressed. Available in private chats only.
     */
    request_location: boolean | null;
    /**
     * If specified, the user will be asked to create a poll and send it to the bot when the button is pressed. Available in private chats only.
     */
    request_poll: KeyboardButtonPollType | null;
    /**
     * If specified, the described [Web App](https://core.telegram.org/bots/api/bots/webapps) will be launched when the button is pressed. The Web App will be able to send a “web_app_data” service message. Available in private chats only.
     */
    web_app: WebAppInfo | null;
}
/**
 * This object defines the criteria used to request suitable users. Information about the selected users will be shared with the bot when the corresponding button is pressed. [More about requesting users »](https://core.telegram.org/bots/api/bots/features#chat-and-user-selection)
 */
export interface KeyboardButtonRequestUsers {
    /**
     * Signed 32-bit identifier of the request that will be received back in the [UsersShared](https://core.telegram.org/bots/api#usersshared) object. Must be unique within the message
     */
    request_id: number;
    /**
     * Pass *True* to request bots, pass *False* to request regular users. If not specified, no additional restrictions are applied.
     */
    user_is_bot: boolean | null;
    /**
     * Pass *True* to request premium users, pass *False* to request non-premium users. If not specified, no additional restrictions are applied.
     */
    user_is_premium: boolean | null;
    /**
     * The maximum number of users to be selected; 1-10. Defaults to 1.
     */
    max_quantity: number | null;
    /**
     * Pass *True* to request the users' first and last names
     */
    request_name: boolean | null;
    /**
     * Pass *True* to request the users' usernames
     */
    request_username: boolean | null;
    /**
     * Pass *True* to request the users' photos
     */
    request_photo: boolean | null;
}
/**
 * This object defines the criteria used to request a suitable chat. Information about the selected chat will be shared with the bot when the corresponding button is pressed. The bot will be granted requested rights in the chat if appropriate. [More about requesting chats »](https://core.telegram.org/bots/api/bots/features#chat-and-user-selection).
 */
export interface KeyboardButtonRequestChat {
    /**
     * Signed 32-bit identifier of the request, which will be received back in the [ChatShared](https://core.telegram.org/bots/api#chatshared) object. Must be unique within the message
     */
    request_id: number;
    /**
     * Pass *True* to request a channel chat, pass *False* to request a group or a supergroup chat.
     */
    chat_is_channel: boolean;
    /**
     * Pass *True* to request a forum supergroup, pass *False* to request a non-forum chat. If not specified, no additional restrictions are applied.
     */
    chat_is_forum: boolean | null;
    /**
     * Pass *True* to request a supergroup or a channel with a username, pass *False* to request a chat without a username. If not specified, no additional restrictions are applied.
     */
    chat_has_username: boolean | null;
    /**
     * Pass *True* to request a chat owned by the user. Otherwise, no additional restrictions are applied.
     */
    chat_is_created: boolean | null;
    /**
     * A JSON-serialized object listing the required administrator rights of the user in the chat. The rights must be a superset of *bot_administrator_rights*. If not specified, no additional restrictions are applied.
     */
    user_administrator_rights: ChatAdministratorRights | null;
    /**
     * A JSON-serialized object listing the required administrator rights of the bot in the chat. The rights must be a subset of *user_administrator_rights*. If not specified, no additional restrictions are applied.
     */
    bot_administrator_rights: ChatAdministratorRights | null;
    /**
     * Pass *True* to request a chat with the bot as a member. Otherwise, no additional restrictions are applied.
     */
    bot_is_member: boolean | null;
    /**
     * Pass *True* to request the chat's title
     */
    request_title: boolean | null;
    /**
     * Pass *True* to request the chat's username
     */
    request_username: boolean | null;
    /**
     * Pass *True* to request the chat's photo
     */
    request_photo: boolean | null;
}
/**
 * This object represents type of a poll, which is allowed to be created and sent when the corresponding button is pressed.
 */
export interface KeyboardButtonPollType {
    /**
     * If *quiz* is passed, the user will be allowed to create only polls in the quiz mode. If *regular* is passed, only regular polls will be allowed. Otherwise, the user will be allowed to create a poll of any type.
     */
    type: string | null;
}
/**
 * Upon receiving a message with this object, Telegram clients will remove the current custom keyboard and display the default letter-keyboard. By default, custom keyboards are displayed until a new keyboard is sent by a bot. An exception is made for one-time keyboards that are hidden immediately after the user presses a button (see [ReplyKeyboardMarkup](https://core.telegram.org/bots/api#replykeyboardmarkup)). Not supported in channels and for messages sent on behalf of a Telegram Business account.
 */
export interface ReplyKeyboardRemove {
    /**
     * Requests clients to remove the custom keyboard (user will not be able to summon this keyboard; if you want to hide the keyboard from sight but keep it accessible, use *one_time_keyboard* in [ReplyKeyboardMarkup](https://core.telegram.org/bots/api#replykeyboardmarkup))
     */
    remove_keyboard: true;
    /**
     * Use this parameter if you want to remove the keyboard for specific users only. Targets: 1) users that are @mentioned in the *text* of the [Message](https://core.telegram.org/bots/api#message) object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message.
     *
     * *Example:* A user votes in a poll, bot returns confirmation message in reply to the vote and removes the keyboard for that user, while still showing the keyboard with poll options to users who haven't voted yet.
     */
    selective: boolean | null;
}
/**
 * This object represents an [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards) that appears right next to the message it belongs to.
 */
export interface InlineKeyboardMarkup {
    /**
     * Array of button rows, each represented by an Array of [InlineKeyboardButton](https://core.telegram.org/bots/api#inlinekeyboardbutton) objects
     */
    inline_keyboard: InlineKeyboardButton[][];
}
/**
 * This object represents one button of an inline keyboard. Exactly one of the fields other than *text*, *icon_custom_emoji_id*, and *style* must be used to specify the type of the button.
 */
export interface InlineKeyboardButton {
    /**
     * Label text on the button
     */
    text: string;
    /**
     * Unique identifier of the custom emoji shown before the text of the button. Can only be used by bots that purchased additional usernames on [Fragment](https://fragment.com) or in the messages directly sent by the bot to private, group and supergroup chats if the owner of the bot has a Telegram Premium subscription.
     */
    icon_custom_emoji_id: string | null;
    /**
     * Style of the button. Must be one of “danger” (red), “success” (green) or “primary” (blue). If omitted, then an app-specific style is used.
     */
    style: string | null;
    /**
     * HTTP or tg:// URL to be opened when the button is pressed. Links `tg://user?id=<user_id>` can be used to mention a user by their identifier without using a username, if this is allowed by their privacy settings.
     */
    url: string | null;
    /**
     * Data to be sent in a [callback query](https://core.telegram.org/bots/api#callbackquery) to the bot when the button is pressed, 1-64 bytes
     */
    callback_data: string | null;
    /**
     * Description of the [Web App](https://core.telegram.org/bots/api/bots/webapps) that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method [answerWebAppQuery](https://core.telegram.org/bots/api#answerwebappquery). Available only in private chats between a user and the bot. Not supported for messages sent on behalf of a Telegram Business account.
     */
    web_app: WebAppInfo | null;
    /**
     * An HTTPS URL used to automatically authorize the user. Can be used as a replacement for the [Telegram Login Widget](https://core.telegram.org/bots/api/widgets/login).
     */
    login_url: LoginUrl | null;
    /**
     * If set, pressing the button will prompt the user to select one of their chats, open that chat and insert the bot's username and the specified inline query in the input field. May be empty, in which case just the bot's username will be inserted. Not supported for messages sent in channel direct messages chats and on behalf of a Telegram Business account.
     */
    switch_inline_query: string | null;
    /**
     * If set, pressing the button will insert the bot's username and the specified inline query in the current chat's input field. May be empty, in which case only the bot's username will be inserted.
     *
     * This offers a quick way for the user to open your bot in inline mode in the same chat - good for selecting something from multiple options. Not supported in channels and for messages sent in channel direct messages chats and on behalf of a Telegram Business account.
     */
    switch_inline_query_current_chat: string | null;
    /**
     * If set, pressing the button will prompt the user to select one of their chats of the specified type, open that chat and insert the bot's username and the specified inline query in the input field. Not supported for messages sent in channel direct messages chats and on behalf of a Telegram Business account.
     */
    switch_inline_query_chosen_chat: SwitchInlineQueryChosenChat | null;
    /**
     * Description of the button that copies the specified text to the clipboard.
     */
    copy_text: CopyTextButton | null;
    /**
     * Description of the game that will be launched when the user presses the button.
     *
     * **NOTE:** This type of button **must** always be the first button in the first row.
     */
    callback_game: CallbackGame | null;
    /**
     * Specify *True*, to send a [Pay button](https://core.telegram.org/bots/api#payments). Substrings “⭐” and “XTR” in the buttons's text will be replaced with a Telegram Star icon.
     *
     * **NOTE:** This type of button **must** always be the first button in the first row and can only be used in invoice messages.
     */
    pay: boolean | null;
}
/**
 * This object represents a parameter of the inline keyboard button used to automatically authorize a user. Serves as a great replacement for the [Telegram Login Widget](https://core.telegram.org/bots/api/widgets/login) when the user is coming from Telegram. All the user needs to do is tap/click a button and confirm that they want to log in:
 */
export interface LoginUrl {
    /**
     * An HTTPS URL to be opened with user authorization data added to the query string when the button is pressed. If the user refuses to provide authorization data, the original URL without information about the user will be opened. The data added is the same as described in [Receiving authorization data](https://core.telegram.org/bots/api/widgets/login#receiving-authorization-data).
     *
     * **NOTE:** You **must** always check the hash of the received data to verify the authentication and the integrity of the data as described in [Checking authorization](https://core.telegram.org/bots/api/widgets/login#checking-authorization).
     */
    url: string;
    /**
     * New text of the button in forwarded messages.
     */
    forward_text: string | null;
    /**
     * Username of a bot, which will be used for user authorization. See [Setting up a bot](https://core.telegram.org/bots/api/widgets/login#setting-up-a-bot) for more details. If not specified, the current bot's username will be assumed. The *url*'s domain must be the same as the domain linked with the bot. See [Linking your domain to the bot](https://core.telegram.org/bots/api/widgets/login#linking-your-domain-to-the-bot) for more details.
     */
    bot_username: string | null;
    /**
     * Pass *True* to request the permission for your bot to send messages to the user.
     */
    request_write_access: boolean | null;
}
/**
 * This object represents an inline button that switches the current user to inline mode in a chosen chat, with an optional default inline query.
 */
export interface SwitchInlineQueryChosenChat {
    /**
     * The default inline query to be inserted in the input field. If left empty, only the bot's username will be inserted
     */
    query: string | null;
    /**
     * *True*, if private chats with users can be chosen
     */
    allow_user_chats: boolean | null;
    /**
     * *True*, if private chats with bots can be chosen
     */
    allow_bot_chats: boolean | null;
    /**
     * *True*, if group and supergroup chats can be chosen
     */
    allow_group_chats: boolean | null;
    /**
     * *True*, if channel chats can be chosen
     */
    allow_channel_chats: boolean | null;
}
/**
 * This object represents an inline keyboard button that copies specified text to the clipboard.
 */
export interface CopyTextButton {
    /**
     * The text to be copied to the clipboard; 1-256 characters
     */
    text: string;
}
/**
 * This object represents an incoming callback query from a callback button in an [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards). If the button that originated the query was attached to a message sent by the bot, the field *message* will be present. If the button was attached to a message sent via the bot (in [inline mode](https://core.telegram.org/bots/api#inline-mode)), the field *inline_message_id* will be present. Exactly one of the fields *data* or *game_short_name* will be present.
 */
export interface CallbackQuery {
    /**
     * Unique identifier for this query
     */
    id: string;
    /**
     * Sender
     */
    from: User;
    /**
     * Message sent by the bot with the callback button that originated the query
     */
    message: MaybeInaccessibleMessage | null;
    /**
     * Identifier of the message sent via the bot in inline mode, that originated the query.
     */
    inline_message_id: string | null;
    /**
     * Global identifier, uniquely corresponding to the chat to which the message with the callback button was sent. Useful for high scores in [games](https://core.telegram.org/bots/api#games).
     */
    chat_instance: string;
    /**
     * Data associated with the callback button. Be aware that the message originated the query can contain no callback buttons with this data.
     */
    data: string | null;
    /**
     * Short name of a [Game](https://core.telegram.org/bots/api#games) to be returned, serves as the unique identifier for the game
     */
    game_short_name: string | null;
}
/**
 * Upon receiving a message with this object, Telegram clients will display a reply interface to the user (act as if the user has selected the bot's message and tapped 'Reply'). This can be extremely useful if you want to create user-friendly step-by-step interfaces without having to sacrifice [privacy mode](https://core.telegram.org/bots/api/bots/features#privacy-mode). Not supported in channels and for messages sent on behalf of a Telegram Business account.
 */
export interface ForceReply {
    /**
     * Shows reply interface to the user, as if they manually selected the bot's message and tapped 'Reply'
     */
    force_reply: true;
    /**
     * The placeholder to be shown in the input field when the reply is active; 1-64 characters
     */
    input_field_placeholder: string | null;
    /**
     * Use this parameter if you want to force reply from specific users only. Targets: 1) users that are @mentioned in the *text* of the [Message](https://core.telegram.org/bots/api#message) object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message.
     */
    selective: boolean | null;
}
/**
 * This object represents a chat photo.
 */
export interface ChatPhoto {
    /**
     * File identifier of small (160x160) chat photo. This file_id can be used only for photo download and only for as long as the photo is not changed.
     */
    small_file_id: string;
    /**
     * Unique file identifier of small (160x160) chat photo, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
     */
    small_file_unique_id: string;
    /**
     * File identifier of big (640x640) chat photo. This file_id can be used only for photo download and only for as long as the photo is not changed.
     */
    big_file_id: string;
    /**
     * Unique file identifier of big (640x640) chat photo, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
     */
    big_file_unique_id: string;
}
/**
 * Represents an invite link for a chat.
 */
export interface ChatInviteLink {
    /**
     * The invite link. If the link was created by another chat administrator, then the second part of the link will be replaced with “…”.
     */
    invite_link: string;
    /**
     * Creator of the link
     */
    creator: User;
    /**
     * *True*, if users joining the chat via the link need to be approved by chat administrators
     */
    creates_join_request: boolean;
    /**
     * *True*, if the link is primary
     */
    is_primary: boolean;
    /**
     * *True*, if the link is revoked
     */
    is_revoked: boolean;
    /**
     * Invite link name
     */
    name: string | null;
    /**
     * Point in time (Unix timestamp) when the link will expire or has been expired
     */
    expire_date: number | null;
    /**
     * The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
     */
    member_limit: number | null;
    /**
     * Number of pending join requests created using this link
     */
    pending_join_request_count: number | null;
    /**
     * The number of seconds the subscription will be active for before the next payment
     */
    subscription_period: number | null;
    /**
     * The amount of Telegram Stars a user must pay initially and after each subsequent subscription period to be a member of the chat using the link
     */
    subscription_price: number | null;
}
/**
 * Represents the rights of an administrator in a chat.
 */
export interface ChatAdministratorRights {
    /**
     * *True*, if the user's presence in the chat is hidden
     */
    is_anonymous: boolean;
    /**
     * *True*, if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages, ignore slow mode, and send messages to the chat without paying Telegram Stars. Implied by any other administrator privilege.
     */
    can_manage_chat: boolean;
    /**
     * *True*, if the administrator can delete messages of other users
     */
    can_delete_messages: boolean;
    /**
     * *True*, if the administrator can manage video chats
     */
    can_manage_video_chats: boolean;
    /**
     * *True*, if the administrator can restrict, ban or unban chat members, or access supergroup statistics
     */
    can_restrict_members: boolean;
    /**
     * *True*, if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by the user)
     */
    can_promote_members: boolean;
    /**
     * *True*, if the user is allowed to change the chat title, photo and other settings
     */
    can_change_info: boolean;
    /**
     * *True*, if the user is allowed to invite new users to the chat
     */
    can_invite_users: boolean;
    /**
     * *True*, if the administrator can post stories to the chat
     */
    can_post_stories: boolean;
    /**
     * *True*, if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive
     */
    can_edit_stories: boolean;
    /**
     * *True*, if the administrator can delete stories posted by other users
     */
    can_delete_stories: boolean;
    /**
     * *True*, if the administrator can post messages in the channel, approve suggested posts, or access channel statistics; for channels only
     */
    can_post_messages: boolean | null;
    /**
     * *True*, if the administrator can edit messages of other users and can pin messages; for channels only
     */
    can_edit_messages: boolean | null;
    /**
     * *True*, if the user is allowed to pin messages; for groups and supergroups only
     */
    can_pin_messages: boolean | null;
    /**
     * *True*, if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only
     */
    can_manage_topics: boolean | null;
    /**
     * *True*, if the administrator can manage direct messages of the channel and decline suggested posts; for channels only
     */
    can_manage_direct_messages: boolean | null;
}
/**
 * This object represents changes in the status of a chat member.
 */
export interface ChatMemberUpdated {
    /**
     * Chat the user belongs to
     */
    chat: Chat;
    /**
     * Performer of the action, which resulted in the change
     */
    from: User;
    /**
     * Date the change was done in Unix time
     */
    date: number;
    /**
     * Previous information about the chat member
     */
    old_chat_member: ChatMember;
    /**
     * New information about the chat member
     */
    new_chat_member: ChatMember;
    /**
     * Chat invite link, which was used by the user to join the chat; for joining by invite link events only.
     */
    invite_link: ChatInviteLink | null;
    /**
     * *True*, if the user joined the chat after sending a direct join request without using an invite link and being approved by an administrator
     */
    via_join_request: boolean | null;
    /**
     * *True*, if the user joined the chat via a chat folder invite link
     */
    via_chat_folder_invite_link: boolean | null;
}
/**
 * This object contains information about one member of a chat.
 */
export type ChatMember = ChatMemberOwner | ChatMemberAdministrator | ChatMemberMember | ChatMemberRestricted | ChatMemberLeft | ChatMemberBanned;
/**
 * Represents a [chat member](https://core.telegram.org/bots/api#chatmember) that owns the chat and has all administrator privileges.
 */
export interface ChatMemberOwner {
    /**
     * The member's status in the chat, always “creator”
     */
    status: string;
    /**
     * Information about the user
     */
    user: User;
    /**
     * *True*, if the user's presence in the chat is hidden
     */
    is_anonymous: boolean;
    /**
     * Custom title for this user
     */
    custom_title: string | null;
}
/**
 * Represents a [chat member](https://core.telegram.org/bots/api#chatmember) that has some additional privileges.
 */
export interface ChatMemberAdministrator {
    /**
     * The member's status in the chat, always “administrator”
     */
    status: string;
    /**
     * Information about the user
     */
    user: User;
    /**
     * *True*, if the bot is allowed to edit administrator privileges of that user
     */
    can_be_edited: boolean;
    /**
     * *True*, if the user's presence in the chat is hidden
     */
    is_anonymous: boolean;
    /**
     * *True*, if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages, ignore slow mode, and send messages to the chat without paying Telegram Stars. Implied by any other administrator privilege.
     */
    can_manage_chat: boolean;
    /**
     * *True*, if the administrator can delete messages of other users
     */
    can_delete_messages: boolean;
    /**
     * *True*, if the administrator can manage video chats
     */
    can_manage_video_chats: boolean;
    /**
     * *True*, if the administrator can restrict, ban or unban chat members, or access supergroup statistics
     */
    can_restrict_members: boolean;
    /**
     * *True*, if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by the user)
     */
    can_promote_members: boolean;
    /**
     * *True*, if the user is allowed to change the chat title, photo and other settings
     */
    can_change_info: boolean;
    /**
     * *True*, if the user is allowed to invite new users to the chat
     */
    can_invite_users: boolean;
    /**
     * *True*, if the administrator can post stories to the chat
     */
    can_post_stories: boolean;
    /**
     * *True*, if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive
     */
    can_edit_stories: boolean;
    /**
     * *True*, if the administrator can delete stories posted by other users
     */
    can_delete_stories: boolean;
    /**
     * *True*, if the administrator can post messages in the channel, approve suggested posts, or access channel statistics; for channels only
     */
    can_post_messages: boolean | null;
    /**
     * *True*, if the administrator can edit messages of other users and can pin messages; for channels only
     */
    can_edit_messages: boolean | null;
    /**
     * *True*, if the user is allowed to pin messages; for groups and supergroups only
     */
    can_pin_messages: boolean | null;
    /**
     * *True*, if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only
     */
    can_manage_topics: boolean | null;
    /**
     * *True*, if the administrator can manage direct messages of the channel and decline suggested posts; for channels only
     */
    can_manage_direct_messages: boolean | null;
    /**
     * Custom title for this user
     */
    custom_title: string | null;
}
/**
 * Represents a [chat member](https://core.telegram.org/bots/api#chatmember) that has no additional privileges or restrictions.
 */
export interface ChatMemberMember {
    /**
     * The member's status in the chat, always “member”
     */
    status: string;
    /**
     * Information about the user
     */
    user: User;
    /**
     * Date when the user's subscription will expire; Unix time
     */
    until_date: number | null;
}
/**
 * Represents a [chat member](https://core.telegram.org/bots/api#chatmember) that is under certain restrictions in the chat. Supergroups only.
 */
export interface ChatMemberRestricted {
    /**
     * The member's status in the chat, always “restricted”
     */
    status: string;
    /**
     * Information about the user
     */
    user: User;
    /**
     * *True*, if the user is a member of the chat at the moment of the request
     */
    is_member: boolean;
    /**
     * *True*, if the user is allowed to send text messages, contacts, giveaways, giveaway winners, invoices, locations and venues
     */
    can_send_messages: boolean;
    /**
     * *True*, if the user is allowed to send audios
     */
    can_send_audios: boolean;
    /**
     * *True*, if the user is allowed to send documents
     */
    can_send_documents: boolean;
    /**
     * *True*, if the user is allowed to send photos
     */
    can_send_photos: boolean;
    /**
     * *True*, if the user is allowed to send videos
     */
    can_send_videos: boolean;
    /**
     * *True*, if the user is allowed to send video notes
     */
    can_send_video_notes: boolean;
    /**
     * *True*, if the user is allowed to send voice notes
     */
    can_send_voice_notes: boolean;
    /**
     * *True*, if the user is allowed to send polls and checklists
     */
    can_send_polls: boolean;
    /**
     * *True*, if the user is allowed to send animations, games, stickers and use inline bots
     */
    can_send_other_messages: boolean;
    /**
     * *True*, if the user is allowed to add web page previews to their messages
     */
    can_add_web_page_previews: boolean;
    /**
     * *True*, if the user is allowed to change the chat title, photo and other settings
     */
    can_change_info: boolean;
    /**
     * *True*, if the user is allowed to invite new users to the chat
     */
    can_invite_users: boolean;
    /**
     * *True*, if the user is allowed to pin messages
     */
    can_pin_messages: boolean;
    /**
     * *True*, if the user is allowed to create forum topics
     */
    can_manage_topics: boolean;
    /**
     * Date when restrictions will be lifted for this user; Unix time. If 0, then the user is restricted forever
     */
    until_date: number;
}
/**
 * Represents a [chat member](https://core.telegram.org/bots/api#chatmember) that isn't currently a member of the chat, but may join it themselves.
 */
export interface ChatMemberLeft {
    /**
     * The member's status in the chat, always “left”
     */
    status: string;
    /**
     * Information about the user
     */
    user: User;
}
/**
 * Represents a [chat member](https://core.telegram.org/bots/api#chatmember) that was banned in the chat and can't return to the chat or view chat messages.
 */
export interface ChatMemberBanned {
    /**
     * The member's status in the chat, always “kicked”
     */
    status: string;
    /**
     * Information about the user
     */
    user: User;
    /**
     * Date when restrictions will be lifted for this user; Unix time. If 0, then the user is banned forever
     */
    until_date: number;
}
/**
 * Represents a join request sent to a chat.
 */
export interface ChatJoinRequest {
    /**
     * Chat to which the request was sent
     */
    chat: Chat;
    /**
     * User that sent the join request
     */
    from: User;
    /**
     * Identifier of a private chat with the user who sent the join request. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. The bot can use this identifier for 5 minutes to send messages until the join request is processed, assuming no other administrator contacted the user.
     */
    user_chat_id: number;
    /**
     * Date the request was sent in Unix time
     */
    date: number;
    /**
     * Bio of the user.
     */
    bio: string | null;
    /**
     * Chat invite link that was used by the user to send the join request
     */
    invite_link: ChatInviteLink | null;
}
/**
 * Describes actions that a non-administrator user is allowed to take in a chat.
 */
export interface ChatPermissions {
    /**
     * *True*, if the user is allowed to send text messages, contacts, giveaways, giveaway winners, invoices, locations and venues
     */
    can_send_messages: boolean | null;
    /**
     * *True*, if the user is allowed to send audios
     */
    can_send_audios: boolean | null;
    /**
     * *True*, if the user is allowed to send documents
     */
    can_send_documents: boolean | null;
    /**
     * *True*, if the user is allowed to send photos
     */
    can_send_photos: boolean | null;
    /**
     * *True*, if the user is allowed to send videos
     */
    can_send_videos: boolean | null;
    /**
     * *True*, if the user is allowed to send video notes
     */
    can_send_video_notes: boolean | null;
    /**
     * *True*, if the user is allowed to send voice notes
     */
    can_send_voice_notes: boolean | null;
    /**
     * *True*, if the user is allowed to send polls and checklists
     */
    can_send_polls: boolean | null;
    /**
     * *True*, if the user is allowed to send animations, games, stickers and use inline bots
     */
    can_send_other_messages: boolean | null;
    /**
     * *True*, if the user is allowed to add web page previews to their messages
     */
    can_add_web_page_previews: boolean | null;
    /**
     * *True*, if the user is allowed to change the chat title, photo and other settings. Ignored in public supergroups
     */
    can_change_info: boolean | null;
    /**
     * *True*, if the user is allowed to invite new users to the chat
     */
    can_invite_users: boolean | null;
    /**
     * *True*, if the user is allowed to pin messages. Ignored in public supergroups
     */
    can_pin_messages: boolean | null;
    /**
     * *True*, if the user is allowed to create forum topics. If omitted defaults to the value of can_pin_messages
     */
    can_manage_topics: boolean | null;
}
/**
 * Describes the birthdate of a user.
 */
export interface Birthdate {
    /**
     * Day of the user's birth; 1-31
     */
    day: number;
    /**
     * Month of the user's birth; 1-12
     */
    month: number;
    /**
     * Year of the user's birth
     */
    year: number | null;
}
/**
 * Contains information about the start page settings of a Telegram Business account.
 */
export interface BusinessIntro {
    /**
     * Title text of the business intro
     */
    title: string | null;
    /**
     * Message text of the business intro
     */
    message: string | null;
    /**
     * Sticker of the business intro
     */
    sticker: Sticker | null;
}
/**
 * Contains information about the location of a Telegram Business account.
 */
export interface BusinessLocation {
    /**
     * Address of the business
     */
    address: string;
    /**
     * Location of the business
     */
    location: Location | null;
}
/**
 * Describes an interval of time during which a business is open.
 */
export interface BusinessOpeningHoursInterval {
    /**
     * The minute's sequence number in a week, starting on Monday, marking the start of the time interval during which the business is open; 0 - 7 * 24 * 60
     */
    opening_minute: number;
    /**
     * The minute's sequence number in a week, starting on Monday, marking the end of the time interval during which the business is open; 0 - 8 * 24 * 60
     */
    closing_minute: number;
}
/**
 * Describes the opening hours of a business.
 */
export interface BusinessOpeningHours {
    /**
     * Unique name of the time zone for which the opening hours are defined
     */
    time_zone_name: string;
    /**
     * List of time intervals describing business opening hours
     */
    opening_hours: BusinessOpeningHoursInterval[];
}
/**
 * This object describes the rating of a user based on their Telegram Star spendings.
 */
export interface UserRating {
    /**
     * Current level of the user, indicating their reliability when purchasing digital goods and services. A higher level suggests a more trustworthy customer; a negative level is likely reason for concern.
     */
    level: number;
    /**
     * Numerical value of the user's rating; the higher the rating, the better
     */
    rating: number;
    /**
     * The rating value required to get the current level
     */
    current_level_rating: number;
    /**
     * The rating value required to get to the next level; omitted if the maximum level was reached
     */
    next_level_rating: number | null;
}
/**
 * Describes the position of a clickable area within a story.
 */
export interface StoryAreaPosition {
    /**
     * The abscissa of the area's center, as a percentage of the media width
     */
    x_percentage: number;
    /**
     * The ordinate of the area's center, as a percentage of the media height
     */
    y_percentage: number;
    /**
     * The width of the area's rectangle, as a percentage of the media width
     */
    width_percentage: number;
    /**
     * The height of the area's rectangle, as a percentage of the media height
     */
    height_percentage: number;
    /**
     * The clockwise rotation angle of the rectangle, in degrees; 0-360
     */
    rotation_angle: number;
    /**
     * The radius of the rectangle corner rounding, as a percentage of the media width
     */
    corner_radius_percentage: number;
}
/**
 * Describes the physical address of a location.
 */
export interface LocationAddress {
    /**
     * The two-letter ISO 3166-1 alpha-2 country code of the country where the location is located
     */
    country_code: string;
    /**
     * State of the location
     */
    state: string | null;
    /**
     * City of the location
     */
    city: string | null;
    /**
     * Street address of the location
     */
    street: string | null;
}
/**
 * Describes the type of a clickable area on a story.
 */
export type StoryAreaType = StoryAreaTypeLocation | StoryAreaTypeSuggestedReaction | StoryAreaTypeLink | StoryAreaTypeWeather | StoryAreaTypeUniqueGift;
/**
 * Describes a story area pointing to a location. Currently, a story can have up to 10 location areas.
 */
export interface StoryAreaTypeLocation {
    /**
     * Type of the area, always “location”
     */
    type: string;
    /**
     * Location latitude in degrees
     */
    latitude: number;
    /**
     * Location longitude in degrees
     */
    longitude: number;
    /**
     * Address of the location
     */
    address: LocationAddress | null;
}
/**
 * Describes a story area pointing to a suggested reaction. Currently, a story can have up to 5 suggested reaction areas.
 */
export interface StoryAreaTypeSuggestedReaction {
    /**
     * Type of the area, always “suggested_reaction”
     */
    type: string;
    /**
     * Type of the reaction
     */
    reaction_type: ReactionType;
    /**
     * Pass *True* if the reaction area has a dark background
     */
    is_dark: boolean | null;
    /**
     * Pass *True* if reaction area corner is flipped
     */
    is_flipped: boolean | null;
}
/**
 * Describes a story area pointing to an HTTP or tg:// link. Currently, a story can have up to 3 link areas.
 */
export interface StoryAreaTypeLink {
    /**
     * Type of the area, always “link”
     */
    type: string;
    /**
     * HTTP or tg:// URL to be opened when the area is clicked
     */
    url: string;
}
/**
 * Describes a story area containing weather information. Currently, a story can have up to 3 weather areas.
 */
export interface StoryAreaTypeWeather {
    /**
     * Type of the area, always “weather”
     */
    type: string;
    /**
     * Temperature, in degree Celsius
     */
    temperature: number;
    /**
     * Emoji representing the weather
     */
    emoji: string;
    /**
     * A color of the area background in the ARGB format
     */
    background_color: number;
}
/**
 * Describes a story area pointing to a unique gift. Currently, a story can have at most 1 unique gift area.
 */
export interface StoryAreaTypeUniqueGift {
    /**
     * Type of the area, always “unique_gift”
     */
    type: string;
    /**
     * Unique name of the gift
     */
    name: string;
}
/**
 * Describes a clickable area on a story media.
 */
export interface StoryArea {
    /**
     * Position of the area
     */
    position: StoryAreaPosition;
    /**
     * Type of the area
     */
    type: StoryAreaType;
}
/**
 * Represents a location to which a chat is connected.
 */
export interface ChatLocation {
    /**
     * The location to which the supergroup is connected. Can't be a live location.
     */
    location: Location;
    /**
     * Location address; 1-64 characters, as defined by the chat owner
     */
    address: string;
}
/**
 * This object describes the type of a reaction.
 */
export type ReactionType = ReactionTypeEmoji | ReactionTypeCustomEmoji | ReactionTypePaid;
/**
 * The reaction is based on an emoji.
 */
export interface ReactionTypeEmoji {
    /**
     * Type of the reaction, always “emoji”
     */
    type: string;
    /**
     * Reaction emoji. Currently, it can be one of "❤", "👍", "👎", "🔥", "🥰", "👏", "😁", "🤔", "🤯", "😱", "🤬", "😢", "🎉", "🤩", "🤮", "💩", "🙏", "👌", "🕊", "🤡", "🥱", "🥴", "😍", "🐳", "❤‍🔥", "🌚", "🌭", "💯", "🤣", "⚡", "🍌", "🏆", "💔", "🤨", "😐", "🍓", "🍾", "💋", "🖕", "😈", "😴", "😭", "🤓", "👻", "👨‍💻", "👀", "🎃", "🙈", "😇", "😨", "🤝", "✍", "🤗", "🫡", "🎅", "🎄", "☃", "💅", "🤪", "🗿", "🆒", "💘", "🙉", "🦄", "😘", "💊", "🙊", "😎", "👾", "🤷‍♂", "🤷", "🤷‍♀", "😡"
     */
    emoji: string;
}
/**
 * The reaction is based on a custom emoji.
 */
export interface ReactionTypeCustomEmoji {
    /**
     * Type of the reaction, always “custom_emoji”
     */
    type: string;
    /**
     * Custom emoji identifier
     */
    custom_emoji_id: string;
}
/**
 * The reaction is paid.
 */
export interface ReactionTypePaid {
    /**
     * Type of the reaction, always “paid”
     */
    type: string;
}
/**
 * Represents a reaction added to a message along with the number of times it was added.
 */
export interface ReactionCount {
    /**
     * Type of the reaction
     */
    type: ReactionType;
    /**
     * Number of times the reaction was added
     */
    total_count: number;
}
/**
 * This object represents a change of a reaction on a message performed by a user.
 */
export interface MessageReactionUpdated {
    /**
     * The chat containing the message the user reacted to
     */
    chat: Chat;
    /**
     * Unique identifier of the message inside the chat
     */
    message_id: number;
    /**
     * The user that changed the reaction, if the user isn't anonymous
     */
    user: User | null;
    /**
     * The chat on behalf of which the reaction was changed, if the user is anonymous
     */
    actor_chat: Chat | null;
    /**
     * Date of the change in Unix time
     */
    date: number;
    /**
     * Previous list of reaction types that were set by the user
     */
    old_reaction: ReactionType[];
    /**
     * New list of reaction types that have been set by the user
     */
    new_reaction: ReactionType[];
}
/**
 * This object represents reaction changes on a message with anonymous reactions.
 */
export interface MessageReactionCountUpdated {
    /**
     * The chat containing the message
     */
    chat: Chat;
    /**
     * Unique message identifier inside the chat
     */
    message_id: number;
    /**
     * Date of the change in Unix time
     */
    date: number;
    /**
     * List of reactions that are present on the message
     */
    reactions: ReactionCount[];
}
/**
 * This object represents a forum topic.
 */
export interface ForumTopic {
    /**
     * Unique identifier of the forum topic
     */
    message_thread_id: number;
    /**
     * Name of the topic
     */
    name: string;
    /**
     * Color of the topic icon in RGB format
     */
    icon_color: number;
    /**
     * Unique identifier of the custom emoji shown as the topic icon
     */
    icon_custom_emoji_id: string | null;
    /**
     * *True*, if the name of the topic wasn't specified explicitly by its creator and likely needs to be changed by the bot
     */
    is_name_implicit: true | null;
}
/**
 * This object describes the background of a gift.
 */
export interface GiftBackground {
    /**
     * Center color of the background in RGB format
     */
    center_color: number;
    /**
     * Edge color of the background in RGB format
     */
    edge_color: number;
    /**
     * Text color of the background in RGB format
     */
    text_color: number;
}
/**
 * This object represents a gift that can be sent by the bot.
 */
export interface Gift {
    /**
     * Unique identifier of the gift
     */
    id: string;
    /**
     * The sticker that represents the gift
     */
    sticker: Sticker;
    /**
     * The number of Telegram Stars that must be paid to send the sticker
     */
    star_count: number;
    /**
     * The number of Telegram Stars that must be paid to upgrade the gift to a unique one
     */
    upgrade_star_count: number | null;
    /**
     * *True*, if the gift can only be purchased by Telegram Premium subscribers
     */
    is_premium: true | null;
    /**
     * *True*, if the gift can be used (after being upgraded) to customize a user's appearance
     */
    has_colors: true | null;
    /**
     * The total number of gifts of this type that can be sent by all users; for limited gifts only
     */
    total_count: number | null;
    /**
     * The number of remaining gifts of this type that can be sent by all users; for limited gifts only
     */
    remaining_count: number | null;
    /**
     * The total number of gifts of this type that can be sent by the bot; for limited gifts only
     */
    personal_total_count: number | null;
    /**
     * The number of remaining gifts of this type that can be sent by the bot; for limited gifts only
     */
    personal_remaining_count: number | null;
    /**
     * Background of the gift
     */
    background: GiftBackground | null;
    /**
     * The total number of different unique gifts that can be obtained by upgrading the gift
     */
    unique_gift_variant_count: number | null;
    /**
     * Information about the chat that published the gift
     */
    publisher_chat: Chat | null;
}
/**
 * This object represent a list of gifts.
 */
export interface Gifts {
    /**
     * The list of gifts
     */
    gifts: Gift[];
}
/**
 * This object describes the model of a unique gift.
 */
export interface UniqueGiftModel {
    /**
     * Name of the model
     */
    name: string;
    /**
     * The sticker that represents the unique gift
     */
    sticker: Sticker;
    /**
     * The number of unique gifts that receive this model for every 1000 gift upgrades. Always 0 for crafted gifts.
     */
    rarity_per_mille: number;
    /**
     * Rarity of the model if it is a crafted model. Currently, can be “uncommon”, “rare”, “epic”, or “legendary”.
     */
    rarity: string | null;
}
/**
 * This object describes the symbol shown on the pattern of a unique gift.
 */
export interface UniqueGiftSymbol {
    /**
     * Name of the symbol
     */
    name: string;
    /**
     * The sticker that represents the unique gift
     */
    sticker: Sticker;
    /**
     * The number of unique gifts that receive this model for every 1000 gifts upgraded
     */
    rarity_per_mille: number;
}
/**
 * This object describes the colors of the backdrop of a unique gift.
 */
export interface UniqueGiftBackdropColors {
    /**
     * The color in the center of the backdrop in RGB format
     */
    center_color: number;
    /**
     * The color on the edges of the backdrop in RGB format
     */
    edge_color: number;
    /**
     * The color to be applied to the symbol in RGB format
     */
    symbol_color: number;
    /**
     * The color for the text on the backdrop in RGB format
     */
    text_color: number;
}
/**
 * This object describes the backdrop of a unique gift.
 */
export interface UniqueGiftBackdrop {
    /**
     * Name of the backdrop
     */
    name: string;
    /**
     * Colors of the backdrop
     */
    colors: UniqueGiftBackdropColors;
    /**
     * The number of unique gifts that receive this backdrop for every 1000 gifts upgraded
     */
    rarity_per_mille: number;
}
/**
 * This object contains information about the color scheme for a user's name, message replies and link previews based on a unique gift.
 */
export interface UniqueGiftColors {
    /**
     * Custom emoji identifier of the unique gift's model
     */
    model_custom_emoji_id: string;
    /**
     * Custom emoji identifier of the unique gift's symbol
     */
    symbol_custom_emoji_id: string;
    /**
     * Main color used in light themes; RGB format
     */
    light_theme_main_color: number;
    /**
     * List of 1-3 additional colors used in light themes; RGB format
     */
    light_theme_other_colors: number[];
    /**
     * Main color used in dark themes; RGB format
     */
    dark_theme_main_color: number;
    /**
     * List of 1-3 additional colors used in dark themes; RGB format
     */
    dark_theme_other_colors: number[];
}
/**
 * This object describes a unique gift that was upgraded from a regular gift.
 */
export interface UniqueGift {
    /**
     * Identifier of the regular gift from which the gift was upgraded
     */
    gift_id: string;
    /**
     * Human-readable name of the regular gift from which this unique gift was upgraded
     */
    base_name: string;
    /**
     * Unique name of the gift. This name can be used in `https://t.me/nft/...` links and story areas
     */
    name: string;
    /**
     * Unique number of the upgraded gift among gifts upgraded from the same regular gift
     */
    number: number;
    /**
     * Model of the gift
     */
    model: UniqueGiftModel;
    /**
     * Symbol of the gift
     */
    symbol: UniqueGiftSymbol;
    /**
     * Backdrop of the gift
     */
    backdrop: UniqueGiftBackdrop;
    /**
     * *True*, if the original regular gift was exclusively purchaseable by Telegram Premium subscribers
     */
    is_premium: true | null;
    /**
     * *True*, if the gift was used to craft another gift and isn't available anymore
     */
    is_burned: true | null;
    /**
     * *True*, if the gift is assigned from the TON blockchain and can't be resold or transferred in Telegram
     */
    is_from_blockchain: true | null;
    /**
     * The color scheme that can be used by the gift's owner for the chat's name, replies to messages and link previews; for business account gifts and gifts that are currently on sale only
     */
    colors: UniqueGiftColors | null;
    /**
     * Information about the chat that published the gift
     */
    publisher_chat: Chat | null;
}
/**
 * Describes a service message about a regular gift that was sent or received.
 */
export interface GiftInfo {
    /**
     * Information about the gift
     */
    gift: Gift;
    /**
     * Unique identifier of the received gift for the bot; only present for gifts received on behalf of business accounts
     */
    owned_gift_id: string | null;
    /**
     * Number of Telegram Stars that can be claimed by the receiver by converting the gift; omitted if conversion to Telegram Stars is impossible
     */
    convert_star_count: number | null;
    /**
     * Number of Telegram Stars that were prepaid for the ability to upgrade the gift
     */
    prepaid_upgrade_star_count: number | null;
    /**
     * *True*, if the gift's upgrade was purchased after the gift was sent
     */
    is_upgrade_separate: true | null;
    /**
     * *True*, if the gift can be upgraded to a unique gift
     */
    can_be_upgraded: true | null;
    /**
     * Text of the message that was added to the gift
     */
    text: string | null;
    /**
     * Special entities that appear in the text
     */
    entities: MessageEntity[] | null;
    /**
     * *True*, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
     */
    is_private: true | null;
    /**
     * Unique number reserved for this gift when upgraded. See the *number* field in [UniqueGift](https://core.telegram.org/bots/api#uniquegift)
     */
    unique_gift_number: number | null;
}
/**
 * Describes a service message about a unique gift that was sent or received.
 */
export interface UniqueGiftInfo {
    /**
     * Information about the gift
     */
    gift: UniqueGift;
    /**
     * Origin of the gift. Currently, either “upgrade” for gifts upgraded from regular gifts, “transfer” for gifts transferred from other users or channels, “resale” for gifts bought from other users, “gifted_upgrade” for upgrades purchased after the gift was sent, or “offer” for gifts bought or sold through gift purchase offers
     */
    origin: string;
    /**
     * For gifts bought from other users, the currency in which the payment for the gift was done. Currently, one of “XTR” for Telegram Stars or “TON” for toncoins.
     */
    last_resale_currency: string | null;
    /**
     * For gifts bought from other users, the price paid for the gift in either Telegram Stars or nanotoncoins
     */
    last_resale_amount: number | null;
    /**
     * Unique identifier of the received gift for the bot; only present for gifts received on behalf of business accounts
     */
    owned_gift_id: string | null;
    /**
     * Number of Telegram Stars that must be paid to transfer the gift; omitted if the bot cannot transfer the gift
     */
    transfer_star_count: number | null;
    /**
     * Point in time (Unix timestamp) when the gift can be transferred. If it is in the past, then the gift can be transferred now
     */
    next_transfer_date: number | null;
}
/**
 * This object describes a gift received and owned by a user or a chat.
 */
export type OwnedGift = OwnedGiftRegular | OwnedGiftUnique;
/**
 * Describes a regular gift owned by a user or a chat.
 */
export interface OwnedGiftRegular {
    /**
     * Type of the gift, always “regular”
     */
    type: string;
    /**
     * Information about the regular gift
     */
    gift: Gift;
    /**
     * Unique identifier of the gift for the bot; for gifts received on behalf of business accounts only
     */
    owned_gift_id: string | null;
    /**
     * Sender of the gift if it is a known user
     */
    sender_user: User | null;
    /**
     * Date the gift was sent in Unix time
     */
    send_date: number;
    /**
     * Text of the message that was added to the gift
     */
    text: string | null;
    /**
     * Special entities that appear in the text
     */
    entities: MessageEntity[] | null;
    /**
     * *True*, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
     */
    is_private: true | null;
    /**
     * *True*, if the gift is displayed on the account's profile page; for gifts received on behalf of business accounts only
     */
    is_saved: true | null;
    /**
     * *True*, if the gift can be upgraded to a unique gift; for gifts received on behalf of business accounts only
     */
    can_be_upgraded: true | null;
    /**
     * *True*, if the gift was refunded and isn't available anymore
     */
    was_refunded: true | null;
    /**
     * Number of Telegram Stars that can be claimed by the receiver instead of the gift; omitted if the gift cannot be converted to Telegram Stars; for gifts received on behalf of business accounts only
     */
    convert_star_count: number | null;
    /**
     * Number of Telegram Stars that were paid for the ability to upgrade the gift
     */
    prepaid_upgrade_star_count: number | null;
    /**
     * *True*, if the gift's upgrade was purchased after the gift was sent; for gifts received on behalf of business accounts only
     */
    is_upgrade_separate: true | null;
    /**
     * Unique number reserved for this gift when upgraded. See the *number* field in [UniqueGift](https://core.telegram.org/bots/api#uniquegift)
     */
    unique_gift_number: number | null;
}
/**
 * Describes a unique gift received and owned by a user or a chat.
 */
export interface OwnedGiftUnique {
    /**
     * Type of the gift, always “unique”
     */
    type: string;
    /**
     * Information about the unique gift
     */
    gift: UniqueGift;
    /**
     * Unique identifier of the received gift for the bot; for gifts received on behalf of business accounts only
     */
    owned_gift_id: string | null;
    /**
     * Sender of the gift if it is a known user
     */
    sender_user: User | null;
    /**
     * Date the gift was sent in Unix time
     */
    send_date: number;
    /**
     * *True*, if the gift is displayed on the account's profile page; for gifts received on behalf of business accounts only
     */
    is_saved: true | null;
    /**
     * *True*, if the gift can be transferred to another owner; for gifts received on behalf of business accounts only
     */
    can_be_transferred: true | null;
    /**
     * Number of Telegram Stars that must be paid to transfer the gift; omitted if the bot cannot transfer the gift
     */
    transfer_star_count: number | null;
    /**
     * Point in time (Unix timestamp) when the gift can be transferred. If it is in the past, then the gift can be transferred now
     */
    next_transfer_date: number | null;
}
/**
 * Contains the list of gifts received and owned by a user or a chat.
 */
export interface OwnedGifts {
    /**
     * The total number of gifts owned by the user or the chat
     */
    total_count: number;
    /**
     * The list of gifts
     */
    gifts: OwnedGift[];
    /**
     * Offset for the next request. If empty, then there are no more results
     */
    next_offset: string | null;
}
/**
 * This object describes the types of gifts that can be gifted to a user or a chat.
 */
export interface AcceptedGiftTypes {
    /**
     * *True*, if unlimited regular gifts are accepted
     */
    unlimited_gifts: boolean;
    /**
     * *True*, if limited regular gifts are accepted
     */
    limited_gifts: boolean;
    /**
     * *True*, if unique gifts or gifts that can be upgraded to unique for free are accepted
     */
    unique_gifts: boolean;
    /**
     * *True*, if a Telegram Premium subscription is accepted
     */
    premium_subscription: boolean;
    /**
     * *True*, if transfers of unique gifts from channels are accepted
     */
    gifts_from_channels: boolean;
}
/**
 * Describes an amount of Telegram Stars.
 */
export interface StarAmount {
    /**
     * Integer amount of Telegram Stars, rounded to 0; can be negative
     */
    amount: number;
    /**
     * The number of 1/1000000000 shares of Telegram Stars; from -999999999 to 999999999; can be negative if and only if *amount* is non-positive
     */
    nanostar_amount: number | null;
}
/**
 * This object represents a bot command.
 */
export interface BotCommand {
    /**
     * Text of the command; 1-32 characters. Can contain only lowercase English letters, digits and underscores.
     */
    command: string;
    /**
     * Description of the command; 1-256 characters.
     */
    description: string;
}
/**
 * This object represents the scope to which bot commands are applied.
 */
export type BotCommandScope = BotCommandScopeDefault | BotCommandScopeAllPrivateChats | BotCommandScopeAllGroupChats | BotCommandScopeAllChatAdministrators | BotCommandScopeChat | BotCommandScopeChatAdministrators | BotCommandScopeChatMember;
/**
 * Represents the default [scope](https://core.telegram.org/bots/api#botcommandscope) of bot commands. Default commands are used if no commands with a [narrower scope](https://core.telegram.org/bots/api#determining-list-of-commands) are specified for the user.
 */
export interface BotCommandScopeDefault {
    /**
     * Scope type, must be *default*
     */
    type: string;
}
/**
 * Represents the [scope](https://core.telegram.org/bots/api#botcommandscope) of bot commands, covering all private chats.
 */
export interface BotCommandScopeAllPrivateChats {
    /**
     * Scope type, must be *all_private_chats*
     */
    type: string;
}
/**
 * Represents the [scope](https://core.telegram.org/bots/api#botcommandscope) of bot commands, covering all group and supergroup chats.
 */
export interface BotCommandScopeAllGroupChats {
    /**
     * Scope type, must be *all_group_chats*
     */
    type: string;
}
/**
 * Represents the [scope](https://core.telegram.org/bots/api#botcommandscope) of bot commands, covering all group and supergroup chat administrators.
 */
export interface BotCommandScopeAllChatAdministrators {
    /**
     * Scope type, must be *all_chat_administrators*
     */
    type: string;
}
/**
 * Represents the [scope](https://core.telegram.org/bots/api#botcommandscope) of bot commands, covering a specific chat.
 */
export interface BotCommandScopeChat {
    /**
     * Scope type, must be *chat*
     */
    type: string;
    /**
     * Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`). Channel direct messages chats and channel chats aren't supported.
     */
    chat_id: number | string;
}
/**
 * Represents the [scope](https://core.telegram.org/bots/api#botcommandscope) of bot commands, covering all administrators of a specific group or supergroup chat.
 */
export interface BotCommandScopeChatAdministrators {
    /**
     * Scope type, must be *chat_administrators*
     */
    type: string;
    /**
     * Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`). Channel direct messages chats and channel chats aren't supported.
     */
    chat_id: number | string;
}
/**
 * Represents the [scope](https://core.telegram.org/bots/api#botcommandscope) of bot commands, covering a specific member of a group or supergroup chat.
 */
export interface BotCommandScopeChatMember {
    /**
     * Scope type, must be *chat_member*
     */
    type: string;
    /**
     * Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`). Channel direct messages chats and channel chats aren't supported.
     */
    chat_id: number | string;
    /**
     * Unique identifier of the target user
     */
    user_id: number;
}
/**
 * This object represents the bot's name.
 */
export interface BotName {
    /**
     * The bot's name
     */
    name: string;
}
/**
 * This object represents the bot's description.
 */
export interface BotDescription {
    /**
     * The bot's description
     */
    description: string;
}
/**
 * This object represents the bot's short description.
 */
export interface BotShortDescription {
    /**
     * The bot's short description
     */
    short_description: string;
}
/**
 * This object describes the bot's menu button in a private chat.
 */
export type MenuButton = MenuButtonCommands | MenuButtonWebApp | MenuButtonDefault;
/**
 * Represents a menu button, which opens the bot's list of commands.
 */
export interface MenuButtonCommands {
    /**
     * Type of the button, must be *commands*
     */
    type: string;
}
/**
 * Represents a menu button, which launches a [Web App](https://core.telegram.org/bots/api/bots/webapps).
 */
export interface MenuButtonWebApp {
    /**
     * Type of the button, must be *web_app*
     */
    type: string;
    /**
     * Text on the button
     */
    text: string;
    /**
     * Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method [answerWebAppQuery](https://core.telegram.org/bots/api#answerwebappquery). Alternatively, a `t.me` link to a Web App of the bot can be specified in the object instead of the Web App's URL, in which case the Web App will be opened as if the user pressed the link.
     */
    web_app: WebAppInfo;
}
/**
 * Describes that no specific value for the menu button was set.
 */
export interface MenuButtonDefault {
    /**
     * Type of the button, must be *default*
     */
    type: string;
}
/**
 * This object describes the source of a chat boost.
 */
export type ChatBoostSource = ChatBoostSourcePremium | ChatBoostSourceGiftCode | ChatBoostSourceGiveaway;
/**
 * The boost was obtained by subscribing to Telegram Premium or by gifting a Telegram Premium subscription to another user.
 */
export interface ChatBoostSourcePremium {
    /**
     * Source of the boost, always “premium”
     */
    source: string;
    /**
     * User that boosted the chat
     */
    user: User;
}
/**
 * The boost was obtained by the creation of Telegram Premium gift codes to boost a chat. Each such code boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription.
 */
export interface ChatBoostSourceGiftCode {
    /**
     * Source of the boost, always “gift_code”
     */
    source: string;
    /**
     * User for which the gift code was created
     */
    user: User;
}
/**
 * The boost was obtained by the creation of a Telegram Premium or a Telegram Star giveaway. This boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription for Telegram Premium giveaways and *prize_star_count* / 500 times for one year for Telegram Star giveaways.
 */
export interface ChatBoostSourceGiveaway {
    /**
     * Source of the boost, always “giveaway”
     */
    source: string;
    /**
     * Identifier of a message in the chat with the giveaway; the message could have been deleted already. May be 0 if the message isn't sent yet.
     */
    giveaway_message_id: number;
    /**
     * User that won the prize in the giveaway if any; for Telegram Premium giveaways only
     */
    user: User | null;
    /**
     * The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only
     */
    prize_star_count: number | null;
    /**
     * *True*, if the giveaway was completed, but there was no user to win the prize
     */
    is_unclaimed: true | null;
}
/**
 * This object contains information about a chat boost.
 */
export interface ChatBoost {
    /**
     * Unique identifier of the boost
     */
    boost_id: string;
    /**
     * Point in time (Unix timestamp) when the chat was boosted
     */
    add_date: number;
    /**
     * Point in time (Unix timestamp) when the boost will automatically expire, unless the booster's Telegram Premium subscription is prolonged
     */
    expiration_date: number;
    /**
     * Source of the added boost
     */
    source: ChatBoostSource;
}
/**
 * This object represents a boost added to a chat or changed.
 */
export interface ChatBoostUpdated {
    /**
     * Chat which was boosted
     */
    chat: Chat;
    /**
     * Information about the chat boost
     */
    boost: ChatBoost;
}
/**
 * This object represents a boost removed from a chat.
 */
export interface ChatBoostRemoved {
    /**
     * Chat which was boosted
     */
    chat: Chat;
    /**
     * Unique identifier of the boost
     */
    boost_id: string;
    /**
     * Point in time (Unix timestamp) when the boost was removed
     */
    remove_date: number;
    /**
     * Source of the removed boost
     */
    source: ChatBoostSource;
}
/**
 * Describes a service message about the chat owner leaving the chat.
 */
export interface ChatOwnerLeft {
    /**
     * The user which will be the new owner of the chat if the previous owner does not return to the chat
     */
    new_owner: User | null;
}
/**
 * Describes a service message about an ownership change in the chat.
 */
export interface ChatOwnerChanged {
    /**
     * The new owner of the chat
     */
    new_owner: User;
}
/**
 * This object represents a list of boosts added to a chat by a user.
 */
export interface UserChatBoosts {
    /**
     * The list of boosts added to the chat by the user
     */
    boosts: ChatBoost[];
}
/**
 * Represents the rights of a business bot.
 */
export interface BusinessBotRights {
    /**
     * *True*, if the bot can send and edit messages in the private chats that had incoming messages in the last 24 hours
     */
    can_reply: true | null;
    /**
     * *True*, if the bot can mark incoming private messages as read
     */
    can_read_messages: true | null;
    /**
     * *True*, if the bot can delete messages sent by the bot
     */
    can_delete_sent_messages: true | null;
    /**
     * *True*, if the bot can delete all private messages in managed chats
     */
    can_delete_all_messages: true | null;
    /**
     * *True*, if the bot can edit the first and last name of the business account
     */
    can_edit_name: true | null;
    /**
     * *True*, if the bot can edit the bio of the business account
     */
    can_edit_bio: true | null;
    /**
     * *True*, if the bot can edit the profile photo of the business account
     */
    can_edit_profile_photo: true | null;
    /**
     * *True*, if the bot can edit the username of the business account
     */
    can_edit_username: true | null;
    /**
     * *True*, if the bot can change the privacy settings pertaining to gifts for the business account
     */
    can_change_gift_settings: true | null;
    /**
     * *True*, if the bot can view gifts and the amount of Telegram Stars owned by the business account
     */
    can_view_gifts_and_stars: true | null;
    /**
     * *True*, if the bot can convert regular gifts owned by the business account to Telegram Stars
     */
    can_convert_gifts_to_stars: true | null;
    /**
     * *True*, if the bot can transfer and upgrade gifts owned by the business account
     */
    can_transfer_and_upgrade_gifts: true | null;
    /**
     * *True*, if the bot can transfer Telegram Stars received by the business account to its own account, or use them to upgrade and transfer gifts
     */
    can_transfer_stars: true | null;
    /**
     * *True*, if the bot can post, edit and delete stories on behalf of the business account
     */
    can_manage_stories: true | null;
}
/**
 * Describes the connection of the bot with a business account.
 */
export interface BusinessConnection {
    /**
     * Unique identifier of the business connection
     */
    id: string;
    /**
     * Business account user that created the business connection
     */
    user: User;
    /**
     * Identifier of a private chat with the user who created the business connection. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
     */
    user_chat_id: number;
    /**
     * Date the connection was established in Unix time
     */
    date: number;
    /**
     * Rights of the business bot
     */
    rights: BusinessBotRights | null;
    /**
     * *True*, if the connection is active
     */
    is_enabled: boolean;
}
/**
 * This object is received when messages are deleted from a connected business account.
 */
export interface BusinessMessagesDeleted {
    /**
     * Unique identifier of the business connection
     */
    business_connection_id: string;
    /**
     * Information about a chat in the business account. The bot may not have access to the chat or the corresponding user.
     */
    chat: Chat;
    /**
     * The list of identifiers of deleted messages in the chat of the business account
     */
    message_ids: number[];
}
/**
 * Describes why a request was unsuccessful.
 */
export interface ResponseParameters {
    /**
     * The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
     */
    migrate_to_chat_id: number | null;
    /**
     * In case of exceeding flood control, the number of seconds left to wait before the request can be repeated
     */
    retry_after: number | null;
}
/**
 * This object represents the content of a media message to be sent.
 */
export type InputMedia = InputMediaAnimation | InputMediaDocument | InputMediaAudio | InputMediaPhoto | InputMediaVideo;
/**
 * Represents a photo to be sent.
 */
export interface InputMediaPhoto {
    /**
     * Type of the result, must be *photo*
     */
    type: string;
    /**
     * File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    media: string;
    /**
     * Caption of the photo to be sent, 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the photo caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * List of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * Pass *True*, if the caption must be shown above the message media
     */
    show_caption_above_media: boolean | null;
    /**
     * Pass *True* if the photo needs to be covered with a spoiler animation
     */
    has_spoiler: boolean | null;
}
/**
 * Represents a video to be sent.
 */
export interface InputMediaVideo {
    /**
     * Type of the result, must be *video*
     */
    type: string;
    /**
     * File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    media: string;
    /**
     * Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    thumbnail: string | null;
    /**
     * Cover for the video in the message. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    cover: string | null;
    /**
     * Start timestamp for the video in the message
     */
    start_timestamp: number | null;
    /**
     * Caption of the video to be sent, 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the video caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * List of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * Pass *True*, if the caption must be shown above the message media
     */
    show_caption_above_media: boolean | null;
    /**
     * Video width
     */
    width: number | null;
    /**
     * Video height
     */
    height: number | null;
    /**
     * Video duration in seconds
     */
    duration: number | null;
    /**
     * Pass *True* if the uploaded video is suitable for streaming
     */
    supports_streaming: boolean | null;
    /**
     * Pass *True* if the video needs to be covered with a spoiler animation
     */
    has_spoiler: boolean | null;
}
/**
 * Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
 */
export interface InputMediaAnimation {
    /**
     * Type of the result, must be *animation*
     */
    type: string;
    /**
     * File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    media: string;
    /**
     * Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    thumbnail: string | null;
    /**
     * Caption of the animation to be sent, 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the animation caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * List of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * Pass *True*, if the caption must be shown above the message media
     */
    show_caption_above_media: boolean | null;
    /**
     * Animation width
     */
    width: number | null;
    /**
     * Animation height
     */
    height: number | null;
    /**
     * Animation duration in seconds
     */
    duration: number | null;
    /**
     * Pass *True* if the animation needs to be covered with a spoiler animation
     */
    has_spoiler: boolean | null;
}
/**
 * Represents an audio file to be treated as music to be sent.
 */
export interface InputMediaAudio {
    /**
     * Type of the result, must be *audio*
     */
    type: string;
    /**
     * File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    media: string;
    /**
     * Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    thumbnail: string | null;
    /**
     * Caption of the audio to be sent, 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the audio caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * List of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * Duration of the audio in seconds
     */
    duration: number | null;
    /**
     * Performer of the audio
     */
    performer: string | null;
    /**
     * Title of the audio
     */
    title: string | null;
}
/**
 * Represents a general file to be sent.
 */
export interface InputMediaDocument {
    /**
     * Type of the result, must be *document*
     */
    type: string;
    /**
     * File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    media: string;
    /**
     * Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    thumbnail: string | null;
    /**
     * Caption of the document to be sent, 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the document caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * List of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * Disables automatic server-side content type detection for files uploaded using multipart/form-data. Always *True*, if the document is sent as part of an album.
     */
    disable_content_type_detection: boolean | null;
}
/**
 * This object represents the contents of a file to be uploaded. Must be posted using multipart/form-data in the usual way that files are uploaded via the browser.
 */
// deno-lint-ignore no-empty-interface
export interface InputFile {}
/**
 * This object describes the paid media to be sent.
 */
export type InputPaidMedia = InputPaidMediaPhoto | InputPaidMediaVideo;
/**
 * The paid media to send is a photo.
 */
export interface InputPaidMediaPhoto {
    /**
     * Type of the media, must be *photo*
     */
    type: string;
    /**
     * File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    media: string;
}
/**
 * The paid media to send is a video.
 */
export interface InputPaidMediaVideo {
    /**
     * Type of the media, must be *video*
     */
    type: string;
    /**
     * File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    media: string;
    /**
     * Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    thumbnail: string | null;
    /**
     * Cover for the video in the message. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    cover: string | null;
    /**
     * Start timestamp for the video in the message
     */
    start_timestamp: number | null;
    /**
     * Video width
     */
    width: number | null;
    /**
     * Video height
     */
    height: number | null;
    /**
     * Video duration in seconds
     */
    duration: number | null;
    /**
     * Pass *True* if the uploaded video is suitable for streaming
     */
    supports_streaming: boolean | null;
}
/**
 * This object describes a profile photo to set.
 */
export type InputProfilePhoto = InputProfilePhotoStatic | InputProfilePhotoAnimated;
/**
 * A static profile photo in the .JPG format.
 */
export interface InputProfilePhotoStatic {
    /**
     * Type of the profile photo, must be *static*
     */
    type: string;
    /**
     * The static profile photo. Profile photos can't be reused and can only be uploaded as a new file, so you can pass “attach://<file_attach_name>” if the photo was uploaded using multipart/form-data under <file_attach_name>. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    photo: string;
}
/**
 * An animated profile photo in the MPEG4 format.
 */
export interface InputProfilePhotoAnimated {
    /**
     * Type of the profile photo, must be *animated*
     */
    type: string;
    /**
     * The animated profile photo. Profile photos can't be reused and can only be uploaded as a new file, so you can pass “attach://<file_attach_name>” if the photo was uploaded using multipart/form-data under <file_attach_name>. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    animation: string;
    /**
     * Timestamp in seconds of the frame that will be used as the static profile photo. Defaults to 0.0.
     */
    main_frame_timestamp: number | null;
}
/**
 * This object describes the content of a story to post.
 */
export type InputStoryContent = InputStoryContentPhoto | InputStoryContentVideo;
/**
 * Describes a photo to post as a story.
 */
export interface InputStoryContentPhoto {
    /**
     * Type of the content, must be *photo*
     */
    type: string;
    /**
     * The photo to post as a story. The photo must be of the size 1080x1920 and must not exceed 10 MB. The photo can't be reused and can only be uploaded as a new file, so you can pass “attach://<file_attach_name>” if the photo was uploaded using multipart/form-data under <file_attach_name>. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    photo: string;
}
/**
 * Describes a video to post as a story.
 */
export interface InputStoryContentVideo {
    /**
     * Type of the content, must be *video*
     */
    type: string;
    /**
     * The video to post as a story. The video must be of the size 720x1280, streamable, encoded with H.265 codec, with key frames added each second in the MPEG4 format, and must not exceed 30 MB. The video can't be reused and can only be uploaded as a new file, so you can pass “attach://<file_attach_name>” if the video was uploaded using multipart/form-data under <file_attach_name>. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    video: string;
    /**
     * Precise duration of the video in seconds; 0-60
     */
    duration: number | null;
    /**
     * Timestamp in seconds of the frame that will be used as the static cover for the story. Defaults to 0.0.
     */
    cover_frame_timestamp: number | null;
    /**
     * Pass *True* if the video has no sound
     */
    is_animation: boolean | null;
}
/**
 * Colors with identifiers 0 (red), 1 (orange), 2 (purple/violet), 3 (green), 4 (cyan), 5 (blue), 6 (pink) can be customized by app themes. Additionally, the following colors in RGB format are currently in use.
 */
export enum AccentColors {
    /**
     * - Light colors: E15052 F9AE63
     * - Dark colors: FF9380 992F37
     */
    _7 = 7,
    /**
     * - Light colors: E0802B FAC534
     * - Dark colors: ECB04E C35714
     */
    _8 = 8,
    /**
     * - Light colors: A05FF3 F48FFF
     * - Dark colors: C697FF 5E31C8
     */
    _9 = 9,
    /**
     * - Light colors: 27A910 A7DC57
     * - Dark colors: A7EB6E 167E2D
     */
    _10 = 10,
    /**
     * - Light colors: 27ACCE 82E8D6
     * - Dark colors: 40D8D0 045C7F
     */
    _11 = 11,
    /**
     * - Light colors: 3391D4 7DD3F0
     * - Dark colors: 52BFFF 0B5494
     */
    _12 = 12,
    /**
     * - Light colors: DD4371 FFBE9F
     * - Dark colors: FF86A6 8E366E
     */
    _13 = 13,
    /**
     * - Light colors: 247BED F04856 FFFFFF
     * - Dark colors: 3FA2FE E5424F FFFFFF
     */
    _14 = 14,
    /**
     * - Light colors: D67722 1EA011 FFFFFF
     * - Dark colors: FF905E 32A527 FFFFFF
     */
    _15 = 15,
    /**
     * - Light colors: 179E42 E84A3F FFFFFF
     * - Dark colors: 66D364 D5444F FFFFFF
     */
    _16 = 16,
    /**
     * - Light colors: 2894AF 6FC456 FFFFFF
     * - Dark colors: 22BCE2 3DA240 FFFFFF
     */
    _17 = 17,
    /**
     * - Light colors: 0C9AB3 FFAD95 FFE6B5
     * - Dark colors: 22BCE2 FF9778 FFDA6B
     */
    _18 = 18,
    /**
     * - Light colors: 7757D6 F79610 FFDE8E
     * - Dark colors: 9791FF F2731D FFDB59
     */
    _19 = 19,
    /**
     * - Light colors: 1585CF F2AB1D FFFFFF
     * - Dark colors: 3DA6EB EEA51D FFFFFF
     */
    _20 = 20,
}
/**
 * Currently, the following colors in RGB format are in use for profile backgrounds.
 */
export enum ProfileAccentColors {
    /**
     * - Light colors: BA5650
     * - Dark colors: 9C4540
     */
    _0 = 0,
    /**
     * - Light colors: C27C3E
     * - Dark colors: 945E2C
     */
    _1 = 1,
    /**
     * - Light colors: 956AC8
     * - Dark colors: 715099
     */
    _2 = 2,
    /**
     * - Light colors: 49A355
     * - Dark colors: 33713B
     */
    _3 = 3,
    /**
     * - Light colors: 3E97AD
     * - Dark colors: 387E87
     */
    _4 = 4,
    /**
     * - Light colors: 5A8FBB
     * - Dark colors: 477194
     */
    _5 = 5,
    /**
     * - Light colors: B85378
     * - Dark colors: 944763
     */
    _6 = 6,
    /**
     * - Light colors: 7F8B95
     * - Dark colors: 435261
     */
    _7 = 7,
    /**
     * - Light colors: C9565D D97C57
     * - Dark colors: 994343 AC583E
     */
    _8 = 8,
    /**
     * - Light colors: CF7244 CC9433
     * - Dark colors: 8F552F A17232
     */
    _9 = 9,
    /**
     * - Light colors: 9662D4 B966B6
     * - Dark colors: 634691 9250A2
     */
    _10 = 10,
    /**
     * - Light colors: 3D9755 89A650
     * - Dark colors: 296A43 5F8F44
     */
    _11 = 11,
    /**
     * - Light colors: 3D95BA 50AD98
     * - Dark colors: 306C7C 3E987E
     */
    _12 = 12,
    /**
     * - Light colors: 538BC2 4DA8BD
     * - Dark colors: 38618C 458BA1
     */
    _13 = 13,
    /**
     * - Light colors: B04F74 D1666D
     * - Dark colors: 884160 A65259
     */
    _14 = 14,
    /**
     * - Light colors: 637482 7B8A97
     * - Dark colors: 53606E 384654
     */
    _15 = 15,
}
/**
 * A simple method for testing your bot's authentication token. Requires no parameters. Returns basic information about the bot in form of a [User](https://core.telegram.org/bots/api#user) object.
 */
// deno-lint-ignore no-empty-interface
export interface GetMeRequest {}
export type GetMeResponse = User;
/**
 * Use this method to log out from the cloud Bot API server before launching the bot locally. You **must** log out the bot before running it locally, otherwise there is no guarantee that the bot will receive updates. After a successful call, you can immediately log in on a local server, but will not be able to log in back to the cloud Bot API server for 10 minutes. Returns *True* on success. Requires no parameters.
 */
// deno-lint-ignore no-empty-interface
export interface LogOutRequest {}
export type LogOutResponse = true;
/**
 * Use this method to close the bot instance before moving it from one local server to another. You need to delete the webhook before calling this method to ensure that the bot isn't launched again after server restart. The method will return error 429 in the first 10 minutes after the bot is launched. Returns *True* on success. Requires no parameters.
 */
// deno-lint-ignore no-empty-interface
export interface CloseRequest {}
export type CloseResponse = true;
/**
 * Use this method to send text messages. On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
 */
export interface SendMessageRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message will be sent
     */
    business_connection_id: string | null;
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
     */
    message_thread_id: number | null;
    /**
     * Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
     */
    direct_messages_topic_id: number | null;
    /**
     * Text of the message to be sent, 1-4096 characters after entities parsing
     */
    text: string;
    /**
     * Mode for parsing entities in the message text. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * A JSON-serialized list of special entities that appear in message text, which can be specified instead of *parse_mode*
     */
    entities: MessageEntity[] | null;
    /**
     * Link preview generation options for the message
     */
    link_preview_options: LinkPreviewOptions | null;
    /**
     * Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
     */
    disable_notification: boolean | null;
    /**
     * Protects the contents of the sent message from forwarding and saving
     */
    protect_content: boolean | null;
    /**
     * Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
     */
    allow_paid_broadcast: boolean | null;
    /**
     * Unique identifier of the message effect to be added to the message; for private chats only
     */
    message_effect_id: string | null;
    /**
     * A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
     */
    suggested_post_parameters: SuggestedPostParameters | null;
    /**
     * Description of the message to reply to
     */
    reply_parameters: ReplyParameters | null;
    /**
     * Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/api/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user
     */
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | null;
}
export type SendMessageResponse = Message;
/**
 * Use this method to forward messages of any kind. Service messages and messages with protected content can't be forwarded. On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
 */
export interface ForwardMessageRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
     */
    message_thread_id: number | null;
    /**
     * Identifier of the direct messages topic to which the message will be forwarded; required if the message is forwarded to a direct messages chat
     */
    direct_messages_topic_id: number | null;
    /**
     * Unique identifier for the chat where the original message was sent (or channel username in the format `@channelusername`)
     */
    from_chat_id: number | string;
    /**
     * New start timestamp for the forwarded video in the message
     */
    video_start_timestamp: number | null;
    /**
     * Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
     */
    disable_notification: boolean | null;
    /**
     * Protects the contents of the forwarded message from forwarding and saving
     */
    protect_content: boolean | null;
    /**
     * Unique identifier of the message effect to be added to the message; only available when forwarding to private chats
     */
    message_effect_id: string | null;
    /**
     * A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only
     */
    suggested_post_parameters: SuggestedPostParameters | null;
    /**
     * Message identifier in the chat specified in *from_chat_id*
     */
    message_id: number;
}
export type ForwardMessageResponse = Message;
/**
 * Use this method to forward multiple messages of any kind. If some of the specified messages can't be found or forwarded, they are skipped. Service messages and messages with protected content can't be forwarded. Album grouping is kept for forwarded messages. On success, an array of [MessageId](https://core.telegram.org/bots/api#messageid) of the sent messages is returned.
 */
export interface ForwardMessagesRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
     */
    message_thread_id: number | null;
    /**
     * Identifier of the direct messages topic to which the messages will be forwarded; required if the messages are forwarded to a direct messages chat
     */
    direct_messages_topic_id: number | null;
    /**
     * Unique identifier for the chat where the original messages were sent (or channel username in the format `@channelusername`)
     */
    from_chat_id: number | string;
    /**
     * A JSON-serialized list of 1-100 identifiers of messages in the chat *from_chat_id* to forward. The identifiers must be specified in a strictly increasing order.
     */
    message_ids: number[];
    /**
     * Sends the messages [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
     */
    disable_notification: boolean | null;
    /**
     * Protects the contents of the forwarded messages from forwarding and saving
     */
    protect_content: boolean | null;
}
export type ForwardMessagesResponse = MessageId;
/**
 * Use this method to copy messages of any kind. Service messages, paid media messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz [poll](https://core.telegram.org/bots/api#poll) can be copied only if the value of the field *correct_option_id* is known to the bot. The method is analogous to the method [forwardMessage](https://core.telegram.org/bots/api#forwardmessage), but the copied message doesn't have a link to the original message. Returns the [MessageId](https://core.telegram.org/bots/api#messageid) of the sent message on success.
 */
export interface CopyMessageRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
     */
    message_thread_id: number | null;
    /**
     * Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
     */
    direct_messages_topic_id: number | null;
    /**
     * Unique identifier for the chat where the original message was sent (or channel username in the format `@channelusername`)
     */
    from_chat_id: number | string;
    /**
     * Message identifier in the chat specified in *from_chat_id*
     */
    message_id: number;
    /**
     * New start timestamp for the copied video in the message
     */
    video_start_timestamp: number | null;
    /**
     * New caption for media, 0-1024 characters after entities parsing. If not specified, the original caption is kept
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the new caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * A JSON-serialized list of special entities that appear in the new caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * Pass *True*, if the caption must be shown above the message media. Ignored if a new caption isn't specified.
     */
    show_caption_above_media: boolean | null;
    /**
     * Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
     */
    disable_notification: boolean | null;
    /**
     * Protects the contents of the sent message from forwarding and saving
     */
    protect_content: boolean | null;
    /**
     * Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
     */
    allow_paid_broadcast: boolean | null;
    /**
     * Unique identifier of the message effect to be added to the message; only available when copying to private chats
     */
    message_effect_id: string | null;
    /**
     * A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
     */
    suggested_post_parameters: SuggestedPostParameters | null;
    /**
     * Description of the message to reply to
     */
    reply_parameters: ReplyParameters | null;
    /**
     * Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/api/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user
     */
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | null;
}
export type CopyMessageResponse = MessageId;
/**
 * Use this method to copy messages of any kind. If some of the specified messages can't be found or copied, they are skipped. Service messages, paid media messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz [poll](https://core.telegram.org/bots/api#poll) can be copied only if the value of the field *correct_option_id* is known to the bot. The method is analogous to the method [forwardMessages](https://core.telegram.org/bots/api#forwardmessages), but the copied messages don't have a link to the original message. Album grouping is kept for copied messages. On success, an array of [MessageId](https://core.telegram.org/bots/api#messageid) of the sent messages is returned.
 */
export interface CopyMessagesRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
     */
    message_thread_id: number | null;
    /**
     * Identifier of the direct messages topic to which the messages will be sent; required if the messages are sent to a direct messages chat
     */
    direct_messages_topic_id: number | null;
    /**
     * Unique identifier for the chat where the original messages were sent (or channel username in the format `@channelusername`)
     */
    from_chat_id: number | string;
    /**
     * A JSON-serialized list of 1-100 identifiers of messages in the chat *from_chat_id* to copy. The identifiers must be specified in a strictly increasing order.
     */
    message_ids: number[];
    /**
     * Sends the messages [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
     */
    disable_notification: boolean | null;
    /**
     * Protects the contents of the sent messages from forwarding and saving
     */
    protect_content: boolean | null;
    /**
     * Pass *True* to copy the messages without their captions
     */
    remove_caption: boolean | null;
}
export type CopyMessagesResponse = MessageId;
/**
 * Use this method to send photos. On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
 */
export interface SendPhotoRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message will be sent
     */
    business_connection_id: string | null;
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
     */
    message_thread_id: number | null;
    /**
     * Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
     */
    direct_messages_topic_id: number | null;
    /**
     * Photo to send. Pass a file_id as String to send a photo that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a photo from the Internet, or upload a new photo using multipart/form-data. The photo must be at most 10 MB in size. The photo's width and height must not exceed 10000 in total. Width and height ratio must be at most 20. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    photo: InputFile | string;
    /**
     * Photo caption (may also be used when resending photos by *file_id*), 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the photo caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * Pass *True*, if the caption must be shown above the message media
     */
    show_caption_above_media: boolean | null;
    /**
     * Pass *True* if the photo needs to be covered with a spoiler animation
     */
    has_spoiler: boolean | null;
    /**
     * Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
     */
    disable_notification: boolean | null;
    /**
     * Protects the contents of the sent message from forwarding and saving
     */
    protect_content: boolean | null;
    /**
     * Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
     */
    allow_paid_broadcast: boolean | null;
    /**
     * Unique identifier of the message effect to be added to the message; for private chats only
     */
    message_effect_id: string | null;
    /**
     * A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
     */
    suggested_post_parameters: SuggestedPostParameters | null;
    /**
     * Description of the message to reply to
     */
    reply_parameters: ReplyParameters | null;
    /**
     * Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/api/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user
     */
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | null;
}
export type SendPhotoResponse = Message;
/**
 * Use this method to send audio files, if you want Telegram clients to display them in the music player. Your audio must be in the .MP3 or .M4A format. On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned. Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.
 */
export interface SendAudioRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message will be sent
     */
    business_connection_id: string | null;
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
     */
    message_thread_id: number | null;
    /**
     * Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
     */
    direct_messages_topic_id: number | null;
    /**
     * Audio file to send. Pass a file_id as String to send an audio file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an audio file from the Internet, or upload a new one using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    audio: InputFile | string;
    /**
     * Audio caption, 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the audio caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * Duration of the audio in seconds
     */
    duration: number | null;
    /**
     * Performer
     */
    performer: string | null;
    /**
     * Track name
     */
    title: string | null;
    /**
     * Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    thumbnail: InputFile | string | null;
    /**
     * Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
     */
    disable_notification: boolean | null;
    /**
     * Protects the contents of the sent message from forwarding and saving
     */
    protect_content: boolean | null;
    /**
     * Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
     */
    allow_paid_broadcast: boolean | null;
    /**
     * Unique identifier of the message effect to be added to the message; for private chats only
     */
    message_effect_id: string | null;
    /**
     * A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
     */
    suggested_post_parameters: SuggestedPostParameters | null;
    /**
     * Description of the message to reply to
     */
    reply_parameters: ReplyParameters | null;
    /**
     * Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/api/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user
     */
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | null;
}
export type SendAudioResponse = Message;
/**
 * Use this method to send general files. On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned. Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.
 */
export interface SendDocumentRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message will be sent
     */
    business_connection_id: string | null;
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
     */
    message_thread_id: number | null;
    /**
     * Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
     */
    direct_messages_topic_id: number | null;
    /**
     * File to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    document: InputFile | string;
    /**
     * Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    thumbnail: InputFile | string | null;
    /**
     * Document caption (may also be used when resending documents by *file_id*), 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the document caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * Disables automatic server-side content type detection for files uploaded using multipart/form-data
     */
    disable_content_type_detection: boolean | null;
    /**
     * Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
     */
    disable_notification: boolean | null;
    /**
     * Protects the contents of the sent message from forwarding and saving
     */
    protect_content: boolean | null;
    /**
     * Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
     */
    allow_paid_broadcast: boolean | null;
    /**
     * Unique identifier of the message effect to be added to the message; for private chats only
     */
    message_effect_id: string | null;
    /**
     * A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
     */
    suggested_post_parameters: SuggestedPostParameters | null;
    /**
     * Description of the message to reply to
     */
    reply_parameters: ReplyParameters | null;
    /**
     * Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/api/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user
     */
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | null;
}
export type SendDocumentResponse = Message;
/**
 * Use this method to send video files, Telegram clients support MPEG4 videos (other formats may be sent as [Document](https://core.telegram.org/bots/api#document)). On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned. Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
 */
export interface SendVideoRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message will be sent
     */
    business_connection_id: string | null;
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
     */
    message_thread_id: number | null;
    /**
     * Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
     */
    direct_messages_topic_id: number | null;
    /**
     * Video to send. Pass a file_id as String to send a video that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a video from the Internet, or upload a new video using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    video: InputFile | string;
    /**
     * Duration of sent video in seconds
     */
    duration: number | null;
    /**
     * Video width
     */
    width: number | null;
    /**
     * Video height
     */
    height: number | null;
    /**
     * Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    thumbnail: InputFile | string | null;
    /**
     * Cover for the video in the message. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    cover: InputFile | string | null;
    /**
     * Start timestamp for the video in the message
     */
    start_timestamp: number | null;
    /**
     * Video caption (may also be used when resending videos by *file_id*), 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the video caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * Pass *True*, if the caption must be shown above the message media
     */
    show_caption_above_media: boolean | null;
    /**
     * Pass *True* if the video needs to be covered with a spoiler animation
     */
    has_spoiler: boolean | null;
    /**
     * Pass *True* if the uploaded video is suitable for streaming
     */
    supports_streaming: boolean | null;
    /**
     * Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
     */
    disable_notification: boolean | null;
    /**
     * Protects the contents of the sent message from forwarding and saving
     */
    protect_content: boolean | null;
    /**
     * Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
     */
    allow_paid_broadcast: boolean | null;
    /**
     * Unique identifier of the message effect to be added to the message; for private chats only
     */
    message_effect_id: string | null;
    /**
     * A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
     */
    suggested_post_parameters: SuggestedPostParameters | null;
    /**
     * Description of the message to reply to
     */
    reply_parameters: ReplyParameters | null;
    /**
     * Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/api/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user
     */
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | null;
}
export type SendVideoResponse = Message;
/**
 * Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
 */
export interface SendAnimationRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message will be sent
     */
    business_connection_id: string | null;
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
     */
    message_thread_id: number | null;
    /**
     * Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
     */
    direct_messages_topic_id: number | null;
    /**
     * Animation to send. Pass a file_id as String to send an animation that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an animation from the Internet, or upload a new animation using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    animation: InputFile | string;
    /**
     * Duration of sent animation in seconds
     */
    duration: number | null;
    /**
     * Animation width
     */
    width: number | null;
    /**
     * Animation height
     */
    height: number | null;
    /**
     * Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    thumbnail: InputFile | string | null;
    /**
     * Animation caption (may also be used when resending animation by *file_id*), 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the animation caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * Pass *True*, if the caption must be shown above the message media
     */
    show_caption_above_media: boolean | null;
    /**
     * Pass *True* if the animation needs to be covered with a spoiler animation
     */
    has_spoiler: boolean | null;
    /**
     * Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
     */
    disable_notification: boolean | null;
    /**
     * Protects the contents of the sent message from forwarding and saving
     */
    protect_content: boolean | null;
    /**
     * Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
     */
    allow_paid_broadcast: boolean | null;
    /**
     * Unique identifier of the message effect to be added to the message; for private chats only
     */
    message_effect_id: string | null;
    /**
     * A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
     */
    suggested_post_parameters: SuggestedPostParameters | null;
    /**
     * Description of the message to reply to
     */
    reply_parameters: ReplyParameters | null;
    /**
     * Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/api/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user
     */
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | null;
}
export type SendAnimationResponse = Message;
/**
 * Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message. For this to work, your audio must be in an .OGG file encoded with OPUS, or in .MP3 format, or in .M4A format (other formats may be sent as [Audio](https://core.telegram.org/bots/api#audio) or [Document](https://core.telegram.org/bots/api#document)). On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future.
 */
export interface SendVoiceRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message will be sent
     */
    business_connection_id: string | null;
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
     */
    message_thread_id: number | null;
    /**
     * Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
     */
    direct_messages_topic_id: number | null;
    /**
     * Audio file to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    voice: InputFile | string;
    /**
     * Voice message caption, 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the voice message caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * Duration of the voice message in seconds
     */
    duration: number | null;
    /**
     * Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
     */
    disable_notification: boolean | null;
    /**
     * Protects the contents of the sent message from forwarding and saving
     */
    protect_content: boolean | null;
    /**
     * Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
     */
    allow_paid_broadcast: boolean | null;
    /**
     * Unique identifier of the message effect to be added to the message; for private chats only
     */
    message_effect_id: string | null;
    /**
     * A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
     */
    suggested_post_parameters: SuggestedPostParameters | null;
    /**
     * Description of the message to reply to
     */
    reply_parameters: ReplyParameters | null;
    /**
     * Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/api/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user
     */
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | null;
}
export type SendVoiceResponse = Message;
/**
 * As of [v.4.0](https://telegram.org/blog/video-messages-and-telescope), Telegram clients support rounded square MPEG4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
 */
export interface SendVideoNoteRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message will be sent
     */
    business_connection_id: string | null;
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
     */
    message_thread_id: number | null;
    /**
     * Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
     */
    direct_messages_topic_id: number | null;
    /**
     * Video note to send. Pass a file_id as String to send a video note that exists on the Telegram servers (recommended) or upload a new video using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files). Sending video notes by a URL is currently unsupported
     */
    video_note: InputFile | string;
    /**
     * Duration of sent video in seconds
     */
    duration: number | null;
    /**
     * Video width and height, i.e. diameter of the video message
     */
    length: number | null;
    /**
     * Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    thumbnail: InputFile | string | null;
    /**
     * Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
     */
    disable_notification: boolean | null;
    /**
     * Protects the contents of the sent message from forwarding and saving
     */
    protect_content: boolean | null;
    /**
     * Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
     */
    allow_paid_broadcast: boolean | null;
    /**
     * Unique identifier of the message effect to be added to the message; for private chats only
     */
    message_effect_id: string | null;
    /**
     * A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
     */
    suggested_post_parameters: SuggestedPostParameters | null;
    /**
     * Description of the message to reply to
     */
    reply_parameters: ReplyParameters | null;
    /**
     * Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/api/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user
     */
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | null;
}
export type SendVideoNoteResponse = Message;
/**
 * Use this method to send paid media. On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
 */
export interface SendPaidMediaRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message will be sent
     */
    business_connection_id: string | null;
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`). If the chat is a channel, all Telegram Star proceeds from this media will be credited to the chat's balance. Otherwise, they will be credited to the bot's balance.
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
     */
    message_thread_id: number | null;
    /**
     * Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
     */
    direct_messages_topic_id: number | null;
    /**
     * The number of Telegram Stars that must be paid to buy access to the media; 1-25000
     */
    star_count: number;
    /**
     * A JSON-serialized array describing the media to be sent; up to 10 items
     */
    media: InputPaidMedia[];
    /**
     * Bot-defined paid media payload, 0-128 bytes. This will not be displayed to the user, use it for your internal processes.
     */
    payload: string | null;
    /**
     * Media caption, 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the media caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * Pass *True*, if the caption must be shown above the message media
     */
    show_caption_above_media: boolean | null;
    /**
     * Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
     */
    disable_notification: boolean | null;
    /**
     * Protects the contents of the sent message from forwarding and saving
     */
    protect_content: boolean | null;
    /**
     * Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
     */
    allow_paid_broadcast: boolean | null;
    /**
     * A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
     */
    suggested_post_parameters: SuggestedPostParameters | null;
    /**
     * Description of the message to reply to
     */
    reply_parameters: ReplyParameters | null;
    /**
     * Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/api/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user
     */
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | null;
}
export type SendPaidMediaResponse = Message;
/**
 * Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of [Message](https://core.telegram.org/bots/api#message) objects that were sent is returned.
 */
export interface SendMediaGroupRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message will be sent
     */
    business_connection_id: string | null;
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
     */
    message_thread_id: number | null;
    /**
     * Identifier of the direct messages topic to which the messages will be sent; required if the messages are sent to a direct messages chat
     */
    direct_messages_topic_id: number | null;
    /**
     * A JSON-serialized array describing messages to be sent, must include 2-10 items
     */
    media: InputMediaAudio[];
    /**
     * Sends messages [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
     */
    disable_notification: boolean | null;
    /**
     * Protects the contents of the sent messages from forwarding and saving
     */
    protect_content: boolean | null;
    /**
     * Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
     */
    allow_paid_broadcast: boolean | null;
    /**
     * Unique identifier of the message effect to be added to the message; for private chats only
     */
    message_effect_id: string | null;
    /**
     * Description of the message to reply to
     */
    reply_parameters: ReplyParameters | null;
}
export type SendMediaGroupResponse = Message;
/**
 * Use this method to send point on the map. On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
 */
export interface SendLocationRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message will be sent
     */
    business_connection_id: string | null;
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
     */
    message_thread_id: number | null;
    /**
     * Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
     */
    direct_messages_topic_id: number | null;
    /**
     * Latitude of the location
     */
    latitude: number;
    /**
     * Longitude of the location
     */
    longitude: number;
    /**
     * The radius of uncertainty for the location, measured in meters; 0-1500
     */
    horizontal_accuracy: number | null;
    /**
     * Period in seconds during which the location will be updated (see [Live Locations](https://telegram.org/blog/live-locations), should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely.
     */
    live_period: number | null;
    /**
     * For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
     */
    heading: number | null;
    /**
     * For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
     */
    proximity_alert_radius: number | null;
    /**
     * Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
     */
    disable_notification: boolean | null;
    /**
     * Protects the contents of the sent message from forwarding and saving
     */
    protect_content: boolean | null;
    /**
     * Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
     */
    allow_paid_broadcast: boolean | null;
    /**
     * Unique identifier of the message effect to be added to the message; for private chats only
     */
    message_effect_id: string | null;
    /**
     * A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
     */
    suggested_post_parameters: SuggestedPostParameters | null;
    /**
     * Description of the message to reply to
     */
    reply_parameters: ReplyParameters | null;
    /**
     * Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/api/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user
     */
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | null;
}
export type SendLocationResponse = Message;
/**
 * Use this method to send information about a venue. On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
 */
export interface SendVenueRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message will be sent
     */
    business_connection_id: string | null;
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
     */
    message_thread_id: number | null;
    /**
     * Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
     */
    direct_messages_topic_id: number | null;
    /**
     * Latitude of the venue
     */
    latitude: number;
    /**
     * Longitude of the venue
     */
    longitude: number;
    /**
     * Name of the venue
     */
    title: string;
    /**
     * Address of the venue
     */
    address: string;
    /**
     * Foursquare identifier of the venue
     */
    foursquare_id: string | null;
    /**
     * Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
     */
    foursquare_type: string | null;
    /**
     * Google Places identifier of the venue
     */
    google_place_id: string | null;
    /**
     * Google Places type of the venue. (See [supported types](https://developers.google.com/places/web-service/supported_types).)
     */
    google_place_type: string | null;
    /**
     * Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
     */
    disable_notification: boolean | null;
    /**
     * Protects the contents of the sent message from forwarding and saving
     */
    protect_content: boolean | null;
    /**
     * Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
     */
    allow_paid_broadcast: boolean | null;
    /**
     * Unique identifier of the message effect to be added to the message; for private chats only
     */
    message_effect_id: string | null;
    /**
     * A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
     */
    suggested_post_parameters: SuggestedPostParameters | null;
    /**
     * Description of the message to reply to
     */
    reply_parameters: ReplyParameters | null;
    /**
     * Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/api/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user
     */
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | null;
}
export type SendVenueResponse = Message;
/**
 * Use this method to send phone contacts. On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
 */
export interface SendContactRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message will be sent
     */
    business_connection_id: string | null;
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
     */
    message_thread_id: number | null;
    /**
     * Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
     */
    direct_messages_topic_id: number | null;
    /**
     * Contact's phone number
     */
    phone_number: string;
    /**
     * Contact's first name
     */
    first_name: string;
    /**
     * Contact's last name
     */
    last_name: string | null;
    /**
     * Additional data about the contact in the form of a [vCard](https://en.wikipedia.org/wiki/VCard), 0-2048 bytes
     */
    vcard: string | null;
    /**
     * Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
     */
    disable_notification: boolean | null;
    /**
     * Protects the contents of the sent message from forwarding and saving
     */
    protect_content: boolean | null;
    /**
     * Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
     */
    allow_paid_broadcast: boolean | null;
    /**
     * Unique identifier of the message effect to be added to the message; for private chats only
     */
    message_effect_id: string | null;
    /**
     * A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
     */
    suggested_post_parameters: SuggestedPostParameters | null;
    /**
     * Description of the message to reply to
     */
    reply_parameters: ReplyParameters | null;
    /**
     * Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/api/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user
     */
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | null;
}
export type SendContactResponse = Message;
/**
 * Use this method to send a native poll. On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
 */
export interface SendPollRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message will be sent
     */
    business_connection_id: string | null;
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`). Polls can't be sent to channel direct messages chats.
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
     */
    message_thread_id: number | null;
    /**
     * Poll question, 1-300 characters
     */
    question: string;
    /**
     * Mode for parsing entities in the question. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details. Currently, only custom emoji entities are allowed
     */
    question_parse_mode: string | null;
    /**
     * A JSON-serialized list of special entities that appear in the poll question. It can be specified instead of *question_parse_mode*
     */
    question_entities: MessageEntity[] | null;
    /**
     * A JSON-serialized list of 2-12 answer options
     */
    options: InputPollOption[];
    /**
     * *True*, if the poll needs to be anonymous, defaults to *True*
     */
    is_anonymous: boolean | null;
    /**
     * Poll type, “quiz” or “regular”, defaults to “regular”
     */
    type: string | null;
    /**
     * *True*, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to *False*
     */
    allows_multiple_answers: boolean | null;
    /**
     * 0-based identifier of the correct answer option, required for polls in quiz mode
     */
    correct_option_id: number | null;
    /**
     * Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing
     */
    explanation: string | null;
    /**
     * Mode for parsing entities in the explanation. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    explanation_parse_mode: string | null;
    /**
     * A JSON-serialized list of special entities that appear in the poll explanation. It can be specified instead of *explanation_parse_mode*
     */
    explanation_entities: MessageEntity[] | null;
    /**
     * Amount of time in seconds the poll will be active after creation, 5-600. Can't be used together with *close_date*.
     */
    open_period: number | null;
    /**
     * Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 600 seconds in the future. Can't be used together with *open_period*.
     */
    close_date: number | null;
    /**
     * Pass *True* if the poll needs to be immediately closed. This can be useful for poll preview.
     */
    is_closed: boolean | null;
    /**
     * Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
     */
    disable_notification: boolean | null;
    /**
     * Protects the contents of the sent message from forwarding and saving
     */
    protect_content: boolean | null;
    /**
     * Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
     */
    allow_paid_broadcast: boolean | null;
    /**
     * Unique identifier of the message effect to be added to the message; for private chats only
     */
    message_effect_id: string | null;
    /**
     * Description of the message to reply to
     */
    reply_parameters: ReplyParameters | null;
    /**
     * Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/api/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user
     */
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | null;
}
export type SendPollResponse = Message;
/**
 * Use this method to send a checklist on behalf of a connected business account. On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
 */
export interface SendChecklistRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message will be sent
     */
    business_connection_id: string;
    /**
     * Unique identifier for the target chat
     */
    chat_id: number;
    /**
     * A JSON-serialized object for the checklist to send
     */
    checklist: InputChecklist;
    /**
     * Sends the message silently. Users will receive a notification with no sound.
     */
    disable_notification: boolean | null;
    /**
     * Protects the contents of the sent message from forwarding and saving
     */
    protect_content: boolean | null;
    /**
     * Unique identifier of the message effect to be added to the message
     */
    message_effect_id: string | null;
    /**
     * A JSON-serialized object for description of the message to reply to
     */
    reply_parameters: ReplyParameters | null;
    /**
     * A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards)
     */
    reply_markup: InlineKeyboardMarkup | null;
}
export type SendChecklistResponse = Message;
/**
 * Use this method to send an animated emoji that will display a random value. On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
 */
export interface SendDiceRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message will be sent
     */
    business_connection_id: string | null;
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
     */
    message_thread_id: number | null;
    /**
     * Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
     */
    direct_messages_topic_id: number | null;
    /**
     * Emoji on which the dice throw animation is based. Currently, must be one of “🎲”, “🎯”, “🏀”, “⚽”, “🎳”, or “🎰”. Dice can have values 1-6 for “🎲”, “🎯” and “🎳”, values 1-5 for “🏀” and “⚽”, and values 1-64 for “🎰”. Defaults to “🎲”
     */
    emoji: string | null;
    /**
     * Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
     */
    disable_notification: boolean | null;
    /**
     * Protects the contents of the sent message from forwarding
     */
    protect_content: boolean | null;
    /**
     * Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
     */
    allow_paid_broadcast: boolean | null;
    /**
     * Unique identifier of the message effect to be added to the message; for private chats only
     */
    message_effect_id: string | null;
    /**
     * A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
     */
    suggested_post_parameters: SuggestedPostParameters | null;
    /**
     * Description of the message to reply to
     */
    reply_parameters: ReplyParameters | null;
    /**
     * Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/api/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user
     */
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | null;
}
export type SendDiceResponse = Message;
/**
 * Use this method to stream a partial message to a user while the message is being generated; supported only for bots with forum topic mode enabled. Returns *True* on success.
 */
export interface SendMessageDraftRequest {
    /**
     * Unique identifier for the target private chat
     */
    chat_id: number;
    /**
     * Unique identifier for the target message thread
     */
    message_thread_id: number | null;
    /**
     * Unique identifier of the message draft; must be non-zero. Changes of drafts with the same identifier are animated
     */
    draft_id: number;
    /**
     * Text of the message to be sent, 1-4096 characters after entities parsing
     */
    text: string;
    /**
     * Mode for parsing entities in the message text. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * A JSON-serialized list of special entities that appear in message text, which can be specified instead of *parse_mode*
     */
    entities: MessageEntity[] | null;
}
export type SendMessageDraftResponse = true;
/**
 * Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns *True* on success.
 */
export interface SendChatActionRequest {
    /**
     * Unique identifier of the business connection on behalf of which the action will be sent
     */
    business_connection_id: string | null;
    /**
     * Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`). Channel chats and channel direct messages chats aren't supported.
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread or topic of a forum; for supergroups and private chats of bots with forum topic mode enabled only
     */
    message_thread_id: number | null;
    /**
     * Type of action to broadcast. Choose one, depending on what the user is about to receive: *typing* for [text messages](https://core.telegram.org/bots/api#sendmessage), *upload_photo* for [photos](https://core.telegram.org/bots/api#sendphoto), *record_video* or *upload_video* for [videos](https://core.telegram.org/bots/api#sendvideo), *record_voice* or *upload_voice* for [voice notes](https://core.telegram.org/bots/api#sendvoice), *upload_document* for [general files](https://core.telegram.org/bots/api#senddocument), *choose_sticker* for [stickers](https://core.telegram.org/bots/api#sendsticker), *find_location* for [location data](https://core.telegram.org/bots/api#sendlocation), *record_video_note* or *upload_video_note* for [video notes](https://core.telegram.org/bots/api#sendvideonote).
     */
    action: string;
}
export type SendChatActionResponse = true;
/**
 * Use this method to change the chosen reactions on a message. Service messages of some types can't be reacted to. Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel. Bots can't use paid reactions. Returns *True* on success.
 */
export interface SetMessageReactionRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Identifier of the target message. If the message belongs to a media group, the reaction is set to the first non-deleted message in the group instead.
     */
    message_id: number;
    /**
     * A JSON-serialized list of reaction types to set on the message. Currently, as non-premium users, bots can set up to one reaction per message. A custom emoji reaction can be used if it is either already present on the message or explicitly allowed by chat administrators. Paid reactions can't be used by bots.
     */
    reaction: ReactionType[] | null;
    /**
     * Pass *True* to set the reaction with a big animation
     */
    is_big: boolean | null;
}
export type SetMessageReactionResponse = true;
/**
 * Use this method to get a list of profile pictures for a user. Returns a [UserProfilePhotos](https://core.telegram.org/bots/api#userprofilephotos) object.
 */
export interface GetUserProfilePhotosRequest {
    /**
     * Unique identifier of the target user
     */
    user_id: number;
    /**
     * Sequential number of the first photo to be returned. By default, all photos are returned.
     */
    offset: number | null;
    /**
     * Limits the number of photos to be retrieved. Values between 1-100 are accepted. Defaults to 100.
     */
    limit: number | null;
}
export type GetUserProfilePhotosResponse = UserProfilePhotos;
/**
 * Use this method to get a list of profile audios for a user. Returns a [UserProfileAudios](https://core.telegram.org/bots/api#userprofileaudios) object.
 */
export interface GetUserProfileAudiosRequest {
    /**
     * Unique identifier of the target user
     */
    user_id: number;
    /**
     * Sequential number of the first audio to be returned. By default, all audios are returned.
     */
    offset: number | null;
    /**
     * Limits the number of audios to be retrieved. Values between 1-100 are accepted. Defaults to 100.
     */
    limit: number | null;
}
export type GetUserProfileAudiosResponse = UserProfileAudios;
/**
 * Changes the emoji status for a given user that previously allowed the bot to manage their emoji status via the Mini App method [requestEmojiStatusAccess](https://core.telegram.org/bots/api/bots/webapps#initializing-mini-apps). Returns *True* on success.
 */
export interface SetUserEmojiStatusRequest {
    /**
     * Unique identifier of the target user
     */
    user_id: number;
    /**
     * Custom emoji identifier of the emoji status to set. Pass an empty string to remove the status.
     */
    emoji_status_custom_emoji_id: string | null;
    /**
     * Expiration date of the emoji status, if any
     */
    emoji_status_expiration_date: number | null;
}
export type SetUserEmojiStatusResponse = true;
/**
 * Use this method to get basic information about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a [File](https://core.telegram.org/bots/api#file) object is returned. The file can then be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`, where `<file_path>` is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling [getFile](https://core.telegram.org/bots/api#getfile) again.
 */
export interface GetFileRequest {
    /**
     * File identifier to get information about
     */
    file_id: string;
}
export type GetFileResponse = File;
/**
 * Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless [unbanned](https://core.telegram.org/bots/api#unbanchatmember) first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
 */
export interface BanChatMemberRequest {
    /**
     * Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier of the target user
     */
    user_id: number;
    /**
     * Date when the user will be unbanned; Unix time. If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever. Applied for supergroups and channels only.
     */
    until_date: number | null;
    /**
     * Pass *True* to delete all messages from the chat for the user that is being removed. If *False*, the user will be able to see messages in the group that were sent before the user was removed. Always *True* for supergroups and channels.
     */
    revoke_messages: boolean | null;
}
export type BanChatMemberResponse = true;
/**
 * Use this method to unban a previously banned user in a supergroup or channel. The user will **not** return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it. So if the user is a member of the chat they will also be **removed** from the chat. If you don't want this, use the parameter *only_if_banned*. Returns *True* on success.
 */
export interface UnbanChatMemberRequest {
    /**
     * Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier of the target user
     */
    user_id: number;
    /**
     * Do nothing if the user is not banned
     */
    only_if_banned: boolean | null;
}
export type UnbanChatMemberResponse = true;
/**
 * Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass *True* for all permissions to lift restrictions from a user. Returns *True* on success.
 */
export interface RestrictChatMemberRequest {
    /**
     * Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier of the target user
     */
    user_id: number;
    /**
     * A JSON-serialized object for new user permissions
     */
    permissions: ChatPermissions;
    /**
     * Pass *True* if chat permissions are set independently. Otherwise, the *can_send_other_messages* and *can_add_web_page_previews* permissions will imply the *can_send_messages*, *can_send_audios*, *can_send_documents*, *can_send_photos*, *can_send_videos*, *can_send_video_notes*, and *can_send_voice_notes* permissions; the *can_send_polls* permission will imply the *can_send_messages* permission.
     */
    use_independent_chat_permissions: boolean | null;
    /**
     * Date when restrictions will be lifted for the user; Unix time. If user is restricted for more than 366 days or less than 30 seconds from the current time, they are considered to be restricted forever
     */
    until_date: number | null;
}
export type RestrictChatMemberResponse = true;
/**
 * Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass *False* for all boolean parameters to demote a user. Returns *True* on success.
 */
export interface PromoteChatMemberRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier of the target user
     */
    user_id: number;
    /**
     * Pass *True* if the administrator's presence in the chat is hidden
     */
    is_anonymous: boolean | null;
    /**
     * Pass *True* if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages, ignore slow mode, and send messages to the chat without paying Telegram Stars. Implied by any other administrator privilege.
     */
    can_manage_chat: boolean | null;
    /**
     * Pass *True* if the administrator can delete messages of other users
     */
    can_delete_messages: boolean | null;
    /**
     * Pass *True* if the administrator can manage video chats
     */
    can_manage_video_chats: boolean | null;
    /**
     * Pass *True* if the administrator can restrict, ban or unban chat members, or access supergroup statistics. For backward compatibility, defaults to *True* for promotions of channel administrators
     */
    can_restrict_members: boolean | null;
    /**
     * Pass *True* if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by him)
     */
    can_promote_members: boolean | null;
    /**
     * Pass *True* if the administrator can change chat title, photo and other settings
     */
    can_change_info: boolean | null;
    /**
     * Pass *True* if the administrator can invite new users to the chat
     */
    can_invite_users: boolean | null;
    /**
     * Pass *True* if the administrator can post stories to the chat
     */
    can_post_stories: boolean | null;
    /**
     * Pass *True* if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive
     */
    can_edit_stories: boolean | null;
    /**
     * Pass *True* if the administrator can delete stories posted by other users
     */
    can_delete_stories: boolean | null;
    /**
     * Pass *True* if the administrator can post messages in the channel, approve suggested posts, or access channel statistics; for channels only
     */
    can_post_messages: boolean | null;
    /**
     * Pass *True* if the administrator can edit messages of other users and can pin messages; for channels only
     */
    can_edit_messages: boolean | null;
    /**
     * Pass *True* if the administrator can pin messages; for supergroups only
     */
    can_pin_messages: boolean | null;
    /**
     * Pass *True* if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only
     */
    can_manage_topics: boolean | null;
    /**
     * Pass *True* if the administrator can manage direct messages within the channel and decline suggested posts; for channels only
     */
    can_manage_direct_messages: boolean | null;
}
export type PromoteChatMemberResponse = true;
/**
 * Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns *True* on success.
 */
export interface SetChatAdministratorCustomTitleRequest {
    /**
     * Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier of the target user
     */
    user_id: number;
    /**
     * New custom title for the administrator; 0-16 characters, emoji are not allowed
     */
    custom_title: string;
}
export type SetChatAdministratorCustomTitleResponse = true;
/**
 * Use this method to ban a channel chat in a supergroup or a channel. Until the chat is [unbanned](https://core.telegram.org/bots/api#unbanchatsenderchat), the owner of the banned chat won't be able to send messages on behalf of **any of their channels**. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights. Returns *True* on success.
 */
export interface BanChatSenderChatRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier of the target sender chat
     */
    sender_chat_id: number;
}
export type BanChatSenderChatResponse = true;
/**
 * Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights. Returns *True* on success.
 */
export interface UnbanChatSenderChatRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier of the target sender chat
     */
    sender_chat_id: number;
}
export type UnbanChatSenderChatResponse = true;
/**
 * Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the *can_restrict_members* administrator rights. Returns *True* on success.
 */
export interface SetChatPermissionsRequest {
    /**
     * Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
     */
    chat_id: number | string;
    /**
     * A JSON-serialized object for new default chat permissions
     */
    permissions: ChatPermissions;
    /**
     * Pass *True* if chat permissions are set independently. Otherwise, the *can_send_other_messages* and *can_add_web_page_previews* permissions will imply the *can_send_messages*, *can_send_audios*, *can_send_documents*, *can_send_photos*, *can_send_videos*, *can_send_video_notes*, and *can_send_voice_notes* permissions; the *can_send_polls* permission will imply the *can_send_messages* permission.
     */
    use_independent_chat_permissions: boolean | null;
}
export type SetChatPermissionsResponse = true;
/**
 * Use this method to generate a new primary invite link for a chat; any previously generated primary link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the new invite link as *String* on success.
 */
export interface ExportChatInviteLinkRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
}
export type ExportChatInviteLinkResponse = string;
/**
 * Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method [revokeChatInviteLink](https://core.telegram.org/bots/api#revokechatinvitelink). Returns the new invite link as [ChatInviteLink](https://core.telegram.org/bots/api#chatinvitelink) object.
 */
export interface CreateChatInviteLinkRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Invite link name; 0-32 characters
     */
    name: string | null;
    /**
     * Point in time (Unix timestamp) when the link will expire
     */
    expire_date: number | null;
    /**
     * The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
     */
    member_limit: number | null;
    /**
     * *True*, if users joining the chat via the link need to be approved by chat administrators. If *True*, *member_limit* can't be specified
     */
    creates_join_request: boolean | null;
}
export type CreateChatInviteLinkResponse = ChatInviteLink;
/**
 * Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a [ChatInviteLink](https://core.telegram.org/bots/api#chatinvitelink) object.
 */
export interface EditChatInviteLinkRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * The invite link to edit
     */
    invite_link: string;
    /**
     * Invite link name; 0-32 characters
     */
    name: string | null;
    /**
     * Point in time (Unix timestamp) when the link will expire
     */
    expire_date: number | null;
    /**
     * The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
     */
    member_limit: number | null;
    /**
     * *True*, if users joining the chat via the link need to be approved by chat administrators. If *True*, *member_limit* can't be specified
     */
    creates_join_request: boolean | null;
}
export type EditChatInviteLinkResponse = ChatInviteLink;
/**
 * Use this method to create a [subscription invite link](https://telegram.org/blog/superchannels-star-reactions-subscriptions#star-subscriptions) for a channel chat. The bot must have the *can_invite_users* administrator rights. The link can be edited using the method [editChatSubscriptionInviteLink](https://core.telegram.org/bots/api#editchatsubscriptioninvitelink) or revoked using the method [revokeChatInviteLink](https://core.telegram.org/bots/api#revokechatinvitelink). Returns the new invite link as a [ChatInviteLink](https://core.telegram.org/bots/api#chatinvitelink) object.
 */
export interface CreateChatSubscriptionInviteLinkRequest {
    /**
     * Unique identifier for the target channel chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Invite link name; 0-32 characters
     */
    name: string | null;
    /**
     * The number of seconds the subscription will be active for before the next payment. Currently, it must always be 2592000 (30 days).
     */
    subscription_period: number;
    /**
     * The amount of Telegram Stars a user must pay initially and after each subsequent subscription period to be a member of the chat; 1-10000
     */
    subscription_price: number;
}
export type CreateChatSubscriptionInviteLinkResponse = ChatInviteLink;
/**
 * Use this method to edit a subscription invite link created by the bot. The bot must have the *can_invite_users* administrator rights. Returns the edited invite link as a [ChatInviteLink](https://core.telegram.org/bots/api#chatinvitelink) object.
 */
export interface EditChatSubscriptionInviteLinkRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * The invite link to edit
     */
    invite_link: string;
    /**
     * Invite link name; 0-32 characters
     */
    name: string | null;
}
export type EditChatSubscriptionInviteLinkResponse = ChatInviteLink;
/**
 * Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the revoked invite link as [ChatInviteLink](https://core.telegram.org/bots/api#chatinvitelink) object.
 */
export interface RevokeChatInviteLinkRequest {
    /**
     * Unique identifier of the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * The invite link to revoke
     */
    invite_link: string;
}
export type RevokeChatInviteLinkResponse = ChatInviteLink;
/**
 * Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the *can_invite_users* administrator right. Returns *True* on success.
 */
export interface ApproveChatJoinRequestRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier of the target user
     */
    user_id: number;
}
export type ApproveChatJoinRequestResponse = true;
/**
 * Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the *can_invite_users* administrator right. Returns *True* on success.
 */
export interface DeclineChatJoinRequestRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier of the target user
     */
    user_id: number;
}
export type DeclineChatJoinRequestResponse = true;
/**
 * Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
 */
export interface SetChatPhotoRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * New chat photo, uploaded using multipart/form-data
     */
    photo: InputFile;
}
export type SetChatPhotoResponse = true;
/**
 * Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
 */
export interface DeleteChatPhotoRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
}
export type DeleteChatPhotoResponse = true;
/**
 * Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
 */
export interface SetChatTitleRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * New chat title, 1-128 characters
     */
    title: string;
}
export type SetChatTitleResponse = true;
/**
 * Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
 */
export interface SetChatDescriptionRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * New chat description, 0-255 characters
     */
    description: string | null;
}
export type SetChatDescriptionResponse = true;
/**
 * Use this method to add a message to the list of pinned messages in a chat. In private chats and channel direct messages chats, all non-service messages can be pinned. Conversely, the bot must be an administrator with the 'can_pin_messages' right or the 'can_edit_messages' right to pin messages in groups and channels respectively. Returns *True* on success.
 */
export interface PinChatMessageRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message will be pinned
     */
    business_connection_id: string | null;
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Identifier of a message to pin
     */
    message_id: number;
    /**
     * Pass *True* if it is not necessary to send a notification to all chat members about the new pinned message. Notifications are always disabled in channels and private chats.
     */
    disable_notification: boolean | null;
}
export type PinChatMessageResponse = true;
/**
 * Use this method to remove a message from the list of pinned messages in a chat. In private chats and channel direct messages chats, all messages can be unpinned. Conversely, the bot must be an administrator with the 'can_pin_messages' right or the 'can_edit_messages' right to unpin messages in groups and channels respectively. Returns *True* on success.
 */
export interface UnpinChatMessageRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message will be unpinned
     */
    business_connection_id: string | null;
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Identifier of the message to unpin. Required if *business_connection_id* is specified. If not specified, the most recent pinned message (by sending date) will be unpinned.
     */
    message_id: number | null;
}
export type UnpinChatMessageResponse = true;
/**
 * Use this method to clear the list of pinned messages in a chat. In private chats and channel direct messages chats, no additional rights are required to unpin all pinned messages. Conversely, the bot must be an administrator with the 'can_pin_messages' right or the 'can_edit_messages' right to unpin all pinned messages in groups and channels respectively. Returns *True* on success.
 */
export interface UnpinAllChatMessagesRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
}
export type UnpinAllChatMessagesResponse = true;
/**
 * Use this method for your bot to leave a group, supergroup or channel. Returns *True* on success.
 */
export interface LeaveChatRequest {
    /**
     * Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`). Channel direct messages chats aren't supported; leave the corresponding channel instead.
     */
    chat_id: number | string;
}
export type LeaveChatResponse = true;
/**
 * Use this method to get up-to-date information about the chat. Returns a [ChatFullInfo](https://core.telegram.org/bots/api#chatfullinfo) object on success.
 */
export interface GetChatRequest {
    /**
     * Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)
     */
    chat_id: number | string;
}
export type GetChatResponse = ChatFullInfo;
/**
 * Use this method to get a list of administrators in a chat, which aren't bots. Returns an Array of [ChatMember](https://core.telegram.org/bots/api#chatmember) objects.
 */
export interface GetChatAdministratorsRequest {
    /**
     * Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)
     */
    chat_id: number | string;
}
export type GetChatAdministratorsResponse = ChatMember[];
/**
 * Use this method to get the number of members in a chat. Returns *Int* on success.
 */
export interface GetChatMemberCountRequest {
    /**
     * Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)
     */
    chat_id: number | string;
}
export type GetChatMemberCountResponse = number;
/**
 * Use this method to get information about a member of a chat. The method is only guaranteed to work for other users if the bot is an administrator in the chat. Returns a [ChatMember](https://core.telegram.org/bots/api#chatmember) object on success.
 */
export interface GetChatMemberRequest {
    /**
     * Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier of the target user
     */
    user_id: number;
}
export type GetChatMemberResponse = ChatMember;
/**
 * Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field *can_set_sticker_set* optionally returned in [getChat](https://core.telegram.org/bots/api#getchat) requests to check if the bot can use this method. Returns *True* on success.
 */
export interface SetChatStickerSetRequest {
    /**
     * Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
     */
    chat_id: number | string;
    /**
     * Name of the sticker set to be set as the group sticker set
     */
    sticker_set_name: string;
}
export type SetChatStickerSetResponse = true;
/**
 * Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field *can_set_sticker_set* optionally returned in [getChat](https://core.telegram.org/bots/api#getchat) requests to check if the bot can use this method. Returns *True* on success.
 */
export interface DeleteChatStickerSetRequest {
    /**
     * Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
     */
    chat_id: number | string;
}
export type DeleteChatStickerSetResponse = true;
/**
 * Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters. Returns an Array of [Sticker](https://core.telegram.org/bots/api#sticker) objects.
 */
// deno-lint-ignore no-empty-interface
export interface GetForumTopicIconStickersRequest {}
export type GetForumTopicIconStickersResponse = Sticker[];
/**
 * Use this method to create a topic in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the *can_manage_topics* administrator right. Returns information about the created topic as a [ForumTopic](https://core.telegram.org/bots/api#forumtopic) object.
 */
export interface CreateForumTopicRequest {
    /**
     * Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
     */
    chat_id: number | string;
    /**
     * Topic name, 1-128 characters
     */
    name: string;
    /**
     * Color of the topic icon in RGB format. Currently, must be one of 7322096 (0x6FB9F0), 16766590 (0xFFD67E), 13338331 (0xCB86DB), 9367192 (0x8EEE98), 16749490 (0xFF93B2), or 16478047 (0xFB6F5F)
     */
    icon_color: number | null;
    /**
     * Unique identifier of the custom emoji shown as the topic icon. Use [getForumTopicIconStickers](https://core.telegram.org/bots/api#getforumtopiciconstickers) to get all allowed custom emoji identifiers.
     */
    icon_custom_emoji_id: string | null;
}
export type CreateForumTopicResponse = ForumTopic;
/**
 * Use this method to edit name and icon of a topic in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the *can_manage_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.
 */
export interface EditForumTopicRequest {
    /**
     * Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread of the forum topic
     */
    message_thread_id: number;
    /**
     * New topic name, 0-128 characters. If not specified or empty, the current name of the topic will be kept
     */
    name: string | null;
    /**
     * New unique identifier of the custom emoji shown as the topic icon. Use [getForumTopicIconStickers](https://core.telegram.org/bots/api#getforumtopiciconstickers) to get all allowed custom emoji identifiers. Pass an empty string to remove the icon. If not specified, the current icon will be kept
     */
    icon_custom_emoji_id: string | null;
}
export type EditForumTopicResponse = true;
/**
 * Use this method to close an open topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can_manage_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.
 */
export interface CloseForumTopicRequest {
    /**
     * Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread of the forum topic
     */
    message_thread_id: number;
}
export type CloseForumTopicResponse = true;
/**
 * Use this method to reopen a closed topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can_manage_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.
 */
export interface ReopenForumTopicRequest {
    /**
     * Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread of the forum topic
     */
    message_thread_id: number;
}
export type ReopenForumTopicResponse = true;
/**
 * Use this method to delete a forum topic along with all its messages in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the *can_delete_messages* administrator rights. Returns *True* on success.
 */
export interface DeleteForumTopicRequest {
    /**
     * Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread of the forum topic
     */
    message_thread_id: number;
}
export type DeleteForumTopicResponse = true;
/**
 * Use this method to clear the list of pinned messages in a forum topic in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the *can_pin_messages* administrator right in the supergroup. Returns *True* on success.
 */
export interface UnpinAllForumTopicMessagesRequest {
    /**
     * Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread of the forum topic
     */
    message_thread_id: number;
}
export type UnpinAllForumTopicMessagesResponse = true;
/**
 * Use this method to edit the name of the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can_manage_topics* administrator rights. Returns *True* on success.
 */
export interface EditGeneralForumTopicRequest {
    /**
     * Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
     */
    chat_id: number | string;
    /**
     * New topic name, 1-128 characters
     */
    name: string;
}
export type EditGeneralForumTopicResponse = true;
/**
 * Use this method to close an open 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can_manage_topics* administrator rights. Returns *True* on success.
 */
export interface CloseGeneralForumTopicRequest {
    /**
     * Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
     */
    chat_id: number | string;
}
export type CloseGeneralForumTopicResponse = true;
/**
 * Use this method to reopen a closed 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can_manage_topics* administrator rights. The topic will be automatically unhidden if it was hidden. Returns *True* on success.
 */
export interface ReopenGeneralForumTopicRequest {
    /**
     * Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
     */
    chat_id: number | string;
}
export type ReopenGeneralForumTopicResponse = true;
/**
 * Use this method to hide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can_manage_topics* administrator rights. The topic will be automatically closed if it was open. Returns *True* on success.
 */
export interface HideGeneralForumTopicRequest {
    /**
     * Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
     */
    chat_id: number | string;
}
export type HideGeneralForumTopicResponse = true;
/**
 * Use this method to unhide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can_manage_topics* administrator rights. Returns *True* on success.
 */
export interface UnhideGeneralForumTopicRequest {
    /**
     * Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
     */
    chat_id: number | string;
}
export type UnhideGeneralForumTopicResponse = true;
/**
 * Use this method to clear the list of pinned messages in a General forum topic. The bot must be an administrator in the chat for this to work and must have the *can_pin_messages* administrator right in the supergroup. Returns *True* on success.
 */
export interface UnpinAllGeneralForumTopicMessagesRequest {
    /**
     * Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
     */
    chat_id: number | string;
}
export type UnpinAllGeneralForumTopicMessagesResponse = true;
/**
 * Use this method to send answers to callback queries sent from [inline keyboards](https://core.telegram.org/bots/api/bots/features#inline-keyboards). The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, *True* is returned.
 */
export interface AnswerCallbackQueryRequest {
    /**
     * Unique identifier for the query to be answered
     */
    callback_query_id: string;
    /**
     * Text of the notification. If not specified, nothing will be shown to the user, 0-200 characters
     */
    text: string | null;
    /**
     * If *True*, an alert will be shown by the client instead of a notification at the top of the chat screen. Defaults to *false*.
     */
    show_alert: boolean | null;
    /**
     * URL that will be opened by the user's client. If you have created a [Game](https://core.telegram.org/bots/api#game) and accepted the conditions via [@BotFather](https://t.me/botfather), specify the URL that opens your game - note that this will only work if the query comes from a [*callback_game*](https://core.telegram.org/bots/api#inlinekeyboardbutton) button.
     *
     * Otherwise, you may use links like `t.me/your_bot?start=XXXX` that open your bot with a parameter.
     */
    url: string | null;
    /**
     * The maximum amount of time in seconds that the result of the callback query may be cached client-side. Telegram apps will support caching starting in version 3.14. Defaults to 0.
     */
    cache_time: number | null;
}
export type AnswerCallbackQueryResponse = true;
/**
 * Use this method to get the list of boosts added to a chat by a user. Requires administrator rights in the chat. Returns a [UserChatBoosts](https://core.telegram.org/bots/api#userchatboosts) object.
 */
export interface GetUserChatBoostsRequest {
    /**
     * Unique identifier for the chat or username of the channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier of the target user
     */
    user_id: number;
}
export type GetUserChatBoostsResponse = UserChatBoosts;
/**
 * Use this method to get information about the connection of the bot with a business account. Returns a [BusinessConnection](https://core.telegram.org/bots/api#businessconnection) object on success.
 */
export interface GetBusinessConnectionRequest {
    /**
     * Unique identifier of the business connection
     */
    business_connection_id: string;
}
export type GetBusinessConnectionResponse = BusinessConnection;
/**
 * Use this method to change the list of the bot's commands. See [this manual](https://core.telegram.org/bots/api/bots/features#commands) for more details about bot commands. Returns *True* on success.
 */
export interface SetMyCommandsRequest {
    /**
     * A JSON-serialized list of bot commands to be set as the list of the bot's commands. At most 100 commands can be specified.
     */
    commands: BotCommand[];
    /**
     * A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to [BotCommandScopeDefault](https://core.telegram.org/bots/api#botcommandscopedefault).
     */
    scope: BotCommandScope | null;
    /**
     * A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands
     */
    language_code: string | null;
}
export type SetMyCommandsResponse = true;
/**
 * Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, [higher level commands](https://core.telegram.org/bots/api#determining-list-of-commands) will be shown to affected users. Returns *True* on success.
 */
export interface DeleteMyCommandsRequest {
    /**
     * A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to [BotCommandScopeDefault](https://core.telegram.org/bots/api#botcommandscopedefault).
     */
    scope: BotCommandScope | null;
    /**
     * A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands
     */
    language_code: string | null;
}
export type DeleteMyCommandsResponse = true;
/**
 * Use this method to get the current list of the bot's commands for the given scope and user language. Returns an Array of [BotCommand](https://core.telegram.org/bots/api#botcommand) objects. If commands aren't set, an empty list is returned.
 */
export interface GetMyCommandsRequest {
    /**
     * A JSON-serialized object, describing scope of users. Defaults to [BotCommandScopeDefault](https://core.telegram.org/bots/api#botcommandscopedefault).
     */
    scope: BotCommandScope | null;
    /**
     * A two-letter ISO 639-1 language code or an empty string
     */
    language_code: string | null;
}
export type GetMyCommandsResponse = BotCommand[];
/**
 * Use this method to change the bot's name. Returns *True* on success.
 */
export interface SetMyNameRequest {
    /**
     * New bot name; 0-64 characters. Pass an empty string to remove the dedicated name for the given language.
     */
    name: string | null;
    /**
     * A two-letter ISO 639-1 language code. If empty, the name will be shown to all users for whose language there is no dedicated name.
     */
    language_code: string | null;
}
export type SetMyNameResponse = true;
/**
 * Use this method to get the current bot name for the given user language. Returns [BotName](https://core.telegram.org/bots/api#botname) on success.
 */
export interface GetMyNameRequest {
    /**
     * A two-letter ISO 639-1 language code or an empty string
     */
    language_code: string | null;
}
export type GetMyNameResponse = BotName;
/**
 * Use this method to change the bot's description, which is shown in the chat with the bot if the chat is empty. Returns *True* on success.
 */
export interface SetMyDescriptionRequest {
    /**
     * New bot description; 0-512 characters. Pass an empty string to remove the dedicated description for the given language.
     */
    description: string | null;
    /**
     * A two-letter ISO 639-1 language code. If empty, the description will be applied to all users for whose language there is no dedicated description.
     */
    language_code: string | null;
}
export type SetMyDescriptionResponse = true;
/**
 * Use this method to get the current bot description for the given user language. Returns [BotDescription](https://core.telegram.org/bots/api#botdescription) on success.
 */
export interface GetMyDescriptionRequest {
    /**
     * A two-letter ISO 639-1 language code or an empty string
     */
    language_code: string | null;
}
export type GetMyDescriptionResponse = BotDescription;
/**
 * Use this method to change the bot's short description, which is shown on the bot's profile page and is sent together with the link when users share the bot. Returns *True* on success.
 */
export interface SetMyShortDescriptionRequest {
    /**
     * New short description for the bot; 0-120 characters. Pass an empty string to remove the dedicated short description for the given language.
     */
    short_description: string | null;
    /**
     * A two-letter ISO 639-1 language code. If empty, the short description will be applied to all users for whose language there is no dedicated short description.
     */
    language_code: string | null;
}
export type SetMyShortDescriptionResponse = true;
/**
 * Use this method to get the current bot short description for the given user language. Returns [BotShortDescription](https://core.telegram.org/bots/api#botshortdescription) on success.
 */
export interface GetMyShortDescriptionRequest {
    /**
     * A two-letter ISO 639-1 language code or an empty string
     */
    language_code: string | null;
}
export type GetMyShortDescriptionResponse = BotShortDescription;
/**
 * Changes the profile photo of the bot. Returns *True* on success.
 */
export interface SetMyProfilePhotoRequest {
    /**
     * The new profile photo to set
     */
    photo: InputProfilePhoto;
}
export type SetMyProfilePhotoResponse = true;
/**
 * Removes the profile photo of the bot. Requires no parameters. Returns *True* on success.
 */
// deno-lint-ignore no-empty-interface
export interface RemoveMyProfilePhotoRequest {}
export type RemoveMyProfilePhotoResponse = true;
/**
 * Use this method to change the bot's menu button in a private chat, or the default menu button. Returns *True* on success.
 */
export interface SetChatMenuButtonRequest {
    /**
     * Unique identifier for the target private chat. If not specified, default bot's menu button will be changed
     */
    chat_id: number | null;
    /**
     * A JSON-serialized object for the bot's new menu button. Defaults to [MenuButtonDefault](https://core.telegram.org/bots/api#menubuttondefault)
     */
    menu_button: MenuButton | null;
}
export type SetChatMenuButtonResponse = true;
/**
 * Use this method to get the current value of the bot's menu button in a private chat, or the default menu button. Returns [MenuButton](https://core.telegram.org/bots/api#menubutton) on success.
 */
export interface GetChatMenuButtonRequest {
    /**
     * Unique identifier for the target private chat. If not specified, default bot's menu button will be returned
     */
    chat_id: number | null;
}
export type GetChatMenuButtonResponse = MenuButton;
/**
 * Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are free to modify the list before adding the bot. Returns *True* on success.
 */
export interface SetMyDefaultAdministratorRightsRequest {
    /**
     * A JSON-serialized object describing new default administrator rights. If not specified, the default administrator rights will be cleared.
     */
    rights: ChatAdministratorRights | null;
    /**
     * Pass *True* to change the default administrator rights of the bot in channels. Otherwise, the default administrator rights of the bot for groups and supergroups will be changed.
     */
    for_channels: boolean | null;
}
export type SetMyDefaultAdministratorRightsResponse = true;
/**
 * Use this method to get the current default administrator rights of the bot. Returns [ChatAdministratorRights](https://core.telegram.org/bots/api#chatadministratorrights) on success.
 */
export interface GetMyDefaultAdministratorRightsRequest {
    /**
     * Pass *True* to get default administrator rights of the bot in channels. Otherwise, default administrator rights of the bot for groups and supergroups will be returned.
     */
    for_channels: boolean | null;
}
export type GetMyDefaultAdministratorRightsResponse = ChatAdministratorRights;
/**
 * Returns the list of gifts that can be sent by the bot to users and channel chats. Requires no parameters. Returns a [Gifts](https://core.telegram.org/bots/api#gifts) object.
 */
// deno-lint-ignore no-empty-interface
export interface GetAvailableGiftsRequest {}
export type GetAvailableGiftsResponse = Gifts;
/**
 * Sends a gift to the given user or channel chat. The gift can't be converted to Telegram Stars by the receiver. Returns *True* on success.
 */
export interface SendGiftRequest {
    /**
     * Required if *chat_id* is not specified. Unique identifier of the target user who will receive the gift.
     */
    user_id: number | null;
    /**
     * Required if *user_id* is not specified. Unique identifier for the chat or username of the channel (in the format `@channelusername`) that will receive the gift.
     */
    chat_id: number | string | null;
    /**
     * Identifier of the gift; limited gifts can't be sent to channel chats
     */
    gift_id: string;
    /**
     * Pass *True* to pay for the gift upgrade from the bot's balance, thereby making the upgrade free for the receiver
     */
    pay_for_upgrade: boolean | null;
    /**
     * Text that will be shown along with the gift; 0-128 characters
     */
    text: string | null;
    /**
     * Mode for parsing entities in the text. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details. Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom_emoji” are ignored.
     */
    text_parse_mode: string | null;
    /**
     * A JSON-serialized list of special entities that appear in the gift text. It can be specified instead of *text_parse_mode*. Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom_emoji” are ignored.
     */
    text_entities: MessageEntity[] | null;
}
export type SendGiftResponse = true;
/**
 * Gifts a Telegram Premium subscription to the given user. Returns *True* on success.
 */
export interface GiftPremiumSubscriptionRequest {
    /**
     * Unique identifier of the target user who will receive a Telegram Premium subscription
     */
    user_id: number;
    /**
     * Number of months the Telegram Premium subscription will be active for the user; must be one of 3, 6, or 12
     */
    month_count: number;
    /**
     * Number of Telegram Stars to pay for the Telegram Premium subscription; must be 1000 for 3 months, 1500 for 6 months, and 2500 for 12 months
     */
    star_count: number;
    /**
     * Text that will be shown along with the service message about the subscription; 0-128 characters
     */
    text: string | null;
    /**
     * Mode for parsing entities in the text. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details. Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom_emoji” are ignored.
     */
    text_parse_mode: string | null;
    /**
     * A JSON-serialized list of special entities that appear in the gift text. It can be specified instead of *text_parse_mode*. Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom_emoji” are ignored.
     */
    text_entities: MessageEntity[] | null;
}
export type GiftPremiumSubscriptionResponse = true;
/**
 * Verifies a user [on behalf of the organization](https://telegram.org/verify#third-party-verification) which is represented by the bot. Returns *True* on success.
 */
export interface VerifyUserRequest {
    /**
     * Unique identifier of the target user
     */
    user_id: number;
    /**
     * Custom description for the verification; 0-70 characters. Must be empty if the organization isn't allowed to provide a custom verification description.
     */
    custom_description: string | null;
}
export type VerifyUserResponse = true;
/**
 * Verifies a chat [on behalf of the organization](https://telegram.org/verify#third-party-verification) which is represented by the bot. Returns *True* on success.
 */
export interface VerifyChatRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`). Channel direct messages chats can't be verified.
     */
    chat_id: number | string;
    /**
     * Custom description for the verification; 0-70 characters. Must be empty if the organization isn't allowed to provide a custom verification description.
     */
    custom_description: string | null;
}
export type VerifyChatResponse = true;
/**
 * Removes verification from a user who is currently verified [on behalf of the organization](https://telegram.org/verify#third-party-verification) represented by the bot. Returns *True* on success.
 */
export interface RemoveUserVerificationRequest {
    /**
     * Unique identifier of the target user
     */
    user_id: number;
}
export type RemoveUserVerificationResponse = true;
/**
 * Removes verification from a chat that is currently verified [on behalf of the organization](https://telegram.org/verify#third-party-verification) represented by the bot. Returns *True* on success.
 */
export interface RemoveChatVerificationRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
}
export type RemoveChatVerificationResponse = true;
/**
 * Marks incoming message as read on behalf of a business account. Requires the *can_read_messages* business bot right. Returns *True* on success.
 */
export interface ReadBusinessMessageRequest {
    /**
     * Unique identifier of the business connection on behalf of which to read the message
     */
    business_connection_id: string;
    /**
     * Unique identifier of the chat in which the message was received. The chat must have been active in the last 24 hours.
     */
    chat_id: number;
    /**
     * Unique identifier of the message to mark as read
     */
    message_id: number;
}
export type ReadBusinessMessageResponse = true;
/**
 * Delete messages on behalf of a business account. Requires the *can_delete_sent_messages* business bot right to delete messages sent by the bot itself, or the *can_delete_all_messages* business bot right to delete any message. Returns *True* on success.
 */
export interface DeleteBusinessMessagesRequest {
    /**
     * Unique identifier of the business connection on behalf of which to delete the messages
     */
    business_connection_id: string;
    /**
     * A JSON-serialized list of 1-100 identifiers of messages to delete. All messages must be from the same chat. See [deleteMessage](https://core.telegram.org/bots/api#deletemessage) for limitations on which messages can be deleted
     */
    message_ids: number[];
}
export type DeleteBusinessMessagesResponse = true;
/**
 * Changes the first and last name of a managed business account. Requires the *can_change_name* business bot right. Returns *True* on success.
 */
export interface SetBusinessAccountNameRequest {
    /**
     * Unique identifier of the business connection
     */
    business_connection_id: string;
    /**
     * The new value of the first name for the business account; 1-64 characters
     */
    first_name: string;
    /**
     * The new value of the last name for the business account; 0-64 characters
     */
    last_name: string | null;
}
export type SetBusinessAccountNameResponse = true;
/**
 * Changes the username of a managed business account. Requires the *can_change_username* business bot right. Returns *True* on success.
 */
export interface SetBusinessAccountUsernameRequest {
    /**
     * Unique identifier of the business connection
     */
    business_connection_id: string;
    /**
     * The new value of the username for the business account; 0-32 characters
     */
    username: string | null;
}
export type SetBusinessAccountUsernameResponse = true;
/**
 * Changes the bio of a managed business account. Requires the *can_change_bio* business bot right. Returns *True* on success.
 */
export interface SetBusinessAccountBioRequest {
    /**
     * Unique identifier of the business connection
     */
    business_connection_id: string;
    /**
     * The new value of the bio for the business account; 0-140 characters
     */
    bio: string | null;
}
export type SetBusinessAccountBioResponse = true;
/**
 * Changes the profile photo of a managed business account. Requires the *can_edit_profile_photo* business bot right. Returns *True* on success.
 */
export interface SetBusinessAccountProfilePhotoRequest {
    /**
     * Unique identifier of the business connection
     */
    business_connection_id: string;
    /**
     * The new profile photo to set
     */
    photo: InputProfilePhoto;
    /**
     * Pass *True* to set the public photo, which will be visible even if the main photo is hidden by the business account's privacy settings. An account can have only one public photo.
     */
    is_public: boolean | null;
}
export type SetBusinessAccountProfilePhotoResponse = true;
/**
 * Removes the current profile photo of a managed business account. Requires the *can_edit_profile_photo* business bot right. Returns *True* on success.
 */
export interface RemoveBusinessAccountProfilePhotoRequest {
    /**
     * Unique identifier of the business connection
     */
    business_connection_id: string;
    /**
     * Pass *True* to remove the public photo, which is visible even if the main photo is hidden by the business account's privacy settings. After the main photo is removed, the previous profile photo (if present) becomes the main photo.
     */
    is_public: boolean | null;
}
export type RemoveBusinessAccountProfilePhotoResponse = true;
/**
 * Changes the privacy settings pertaining to incoming gifts in a managed business account. Requires the *can_change_gift_settings* business bot right. Returns *True* on success.
 */
export interface SetBusinessAccountGiftSettingsRequest {
    /**
     * Unique identifier of the business connection
     */
    business_connection_id: string;
    /**
     * Pass *True*, if a button for sending a gift to the user or by the business account must always be shown in the input field
     */
    show_gift_button: boolean;
    /**
     * Types of gifts accepted by the business account
     */
    accepted_gift_types: AcceptedGiftTypes;
}
export type SetBusinessAccountGiftSettingsResponse = true;
/**
 * Returns the amount of Telegram Stars owned by a managed business account. Requires the *can_view_gifts_and_stars* business bot right. Returns [StarAmount](https://core.telegram.org/bots/api#staramount) on success.
 */
export interface GetBusinessAccountStarBalanceRequest {
    /**
     * Unique identifier of the business connection
     */
    business_connection_id: string;
}
export type GetBusinessAccountStarBalanceResponse = StarAmount;
/**
 * Transfers Telegram Stars from the business account balance to the bot's balance. Requires the *can_transfer_stars* business bot right. Returns *True* on success.
 */
export interface TransferBusinessAccountStarsRequest {
    /**
     * Unique identifier of the business connection
     */
    business_connection_id: string;
    /**
     * Number of Telegram Stars to transfer; 1-10000
     */
    star_count: number;
}
export type TransferBusinessAccountStarsResponse = true;
/**
 * Returns the gifts received and owned by a managed business account. Requires the *can_view_gifts_and_stars* business bot right. Returns [OwnedGifts](https://core.telegram.org/bots/api#ownedgifts) on success.
 */
export interface GetBusinessAccountGiftsRequest {
    /**
     * Unique identifier of the business connection
     */
    business_connection_id: string;
    /**
     * Pass *True* to exclude gifts that aren't saved to the account's profile page
     */
    exclude_unsaved: boolean | null;
    /**
     * Pass *True* to exclude gifts that are saved to the account's profile page
     */
    exclude_saved: boolean | null;
    /**
     * Pass *True* to exclude gifts that can be purchased an unlimited number of times
     */
    exclude_unlimited: boolean | null;
    /**
     * Pass *True* to exclude gifts that can be purchased a limited number of times and can be upgraded to unique
     */
    exclude_limited_upgradable: boolean | null;
    /**
     * Pass *True* to exclude gifts that can be purchased a limited number of times and can't be upgraded to unique
     */
    exclude_limited_non_upgradable: boolean | null;
    /**
     * Pass *True* to exclude unique gifts
     */
    exclude_unique: boolean | null;
    /**
     * Pass *True* to exclude gifts that were assigned from the TON blockchain and can't be resold or transferred in Telegram
     */
    exclude_from_blockchain: boolean | null;
    /**
     * Pass *True* to sort results by gift price instead of send date. Sorting is applied before pagination.
     */
    sort_by_price: boolean | null;
    /**
     * Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
     */
    offset: string | null;
    /**
     * The maximum number of gifts to be returned; 1-100. Defaults to 100
     */
    limit: number | null;
}
export type GetBusinessAccountGiftsResponse = OwnedGifts;
/**
 * Returns the gifts owned and hosted by a user. Returns [OwnedGifts](https://core.telegram.org/bots/api#ownedgifts) on success.
 */
export interface GetUserGiftsRequest {
    /**
     * Unique identifier of the user
     */
    user_id: number;
    /**
     * Pass *True* to exclude gifts that can be purchased an unlimited number of times
     */
    exclude_unlimited: boolean | null;
    /**
     * Pass *True* to exclude gifts that can be purchased a limited number of times and can be upgraded to unique
     */
    exclude_limited_upgradable: boolean | null;
    /**
     * Pass *True* to exclude gifts that can be purchased a limited number of times and can't be upgraded to unique
     */
    exclude_limited_non_upgradable: boolean | null;
    /**
     * Pass *True* to exclude gifts that were assigned from the TON blockchain and can't be resold or transferred in Telegram
     */
    exclude_from_blockchain: boolean | null;
    /**
     * Pass *True* to exclude unique gifts
     */
    exclude_unique: boolean | null;
    /**
     * Pass *True* to sort results by gift price instead of send date. Sorting is applied before pagination.
     */
    sort_by_price: boolean | null;
    /**
     * Offset of the first entry to return as received from the previous request; use an empty string to get the first chunk of results
     */
    offset: string | null;
    /**
     * The maximum number of gifts to be returned; 1-100. Defaults to 100
     */
    limit: number | null;
}
export type GetUserGiftsResponse = OwnedGifts;
/**
 * Returns the gifts owned by a chat. Returns [OwnedGifts](https://core.telegram.org/bots/api#ownedgifts) on success.
 */
export interface GetChatGiftsRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Pass *True* to exclude gifts that aren't saved to the chat's profile page. Always *True*, unless the bot has the *can_post_messages* administrator right in the channel.
     */
    exclude_unsaved: boolean | null;
    /**
     * Pass *True* to exclude gifts that are saved to the chat's profile page. Always *False*, unless the bot has the *can_post_messages* administrator right in the channel.
     */
    exclude_saved: boolean | null;
    /**
     * Pass *True* to exclude gifts that can be purchased an unlimited number of times
     */
    exclude_unlimited: boolean | null;
    /**
     * Pass *True* to exclude gifts that can be purchased a limited number of times and can be upgraded to unique
     */
    exclude_limited_upgradable: boolean | null;
    /**
     * Pass *True* to exclude gifts that can be purchased a limited number of times and can't be upgraded to unique
     */
    exclude_limited_non_upgradable: boolean | null;
    /**
     * Pass *True* to exclude gifts that were assigned from the TON blockchain and can't be resold or transferred in Telegram
     */
    exclude_from_blockchain: boolean | null;
    /**
     * Pass *True* to exclude unique gifts
     */
    exclude_unique: boolean | null;
    /**
     * Pass *True* to sort results by gift price instead of send date. Sorting is applied before pagination.
     */
    sort_by_price: boolean | null;
    /**
     * Offset of the first entry to return as received from the previous request; use an empty string to get the first chunk of results
     */
    offset: string | null;
    /**
     * The maximum number of gifts to be returned; 1-100. Defaults to 100
     */
    limit: number | null;
}
export type GetChatGiftsResponse = OwnedGifts;
/**
 * Converts a given regular gift to Telegram Stars. Requires the *can_convert_gifts_to_stars* business bot right. Returns *True* on success.
 */
export interface ConvertGiftToStarsRequest {
    /**
     * Unique identifier of the business connection
     */
    business_connection_id: string;
    /**
     * Unique identifier of the regular gift that should be converted to Telegram Stars
     */
    owned_gift_id: string;
}
export type ConvertGiftToStarsResponse = true;
/**
 * Upgrades a given regular gift to a unique gift. Requires the *can_transfer_and_upgrade_gifts* business bot right. Additionally requires the *can_transfer_stars* business bot right if the upgrade is paid. Returns *True* on success.
 */
export interface UpgradeGiftRequest {
    /**
     * Unique identifier of the business connection
     */
    business_connection_id: string;
    /**
     * Unique identifier of the regular gift that should be upgraded to a unique one
     */
    owned_gift_id: string;
    /**
     * Pass *True* to keep the original gift text, sender and receiver in the upgraded gift
     */
    keep_original_details: boolean | null;
    /**
     * The amount of Telegram Stars that will be paid for the upgrade from the business account balance. If `gift.prepaid_upgrade_star_count > 0`, then pass 0, otherwise, the *can_transfer_stars* business bot right is required and `gift.upgrade_star_count` must be passed.
     */
    star_count: number | null;
}
export type UpgradeGiftResponse = true;
/**
 * Transfers an owned unique gift to another user. Requires the *can_transfer_and_upgrade_gifts* business bot right. Requires *can_transfer_stars* business bot right if the transfer is paid. Returns *True* on success.
 */
export interface TransferGiftRequest {
    /**
     * Unique identifier of the business connection
     */
    business_connection_id: string;
    /**
     * Unique identifier of the regular gift that should be transferred
     */
    owned_gift_id: string;
    /**
     * Unique identifier of the chat which will own the gift. The chat must be active in the last 24 hours.
     */
    new_owner_chat_id: number;
    /**
     * The amount of Telegram Stars that will be paid for the transfer from the business account balance. If positive, then the *can_transfer_stars* business bot right is required.
     */
    star_count: number | null;
}
export type TransferGiftResponse = true;
/**
 * Posts a story on behalf of a managed business account. Requires the *can_manage_stories* business bot right. Returns [Story](https://core.telegram.org/bots/api#story) on success.
 */
export interface PostStoryRequest {
    /**
     * Unique identifier of the business connection
     */
    business_connection_id: string;
    /**
     * Content of the story
     */
    content: InputStoryContent;
    /**
     * Period after which the story is moved to the archive, in seconds; must be one of `6 * 3600`, `12 * 3600`, `86400`, or `2 * 86400`
     */
    active_period: number;
    /**
     * Caption of the story, 0-2048 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the story caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * A JSON-serialized list of clickable areas to be shown on the story
     */
    areas: StoryArea[] | null;
    /**
     * Pass *True* to keep the story accessible after it expires
     */
    post_to_chat_page: boolean | null;
    /**
     * Pass *True* if the content of the story must be protected from forwarding and screenshotting
     */
    protect_content: boolean | null;
}
export type PostStoryResponse = Story;
/**
 * Reposts a story on behalf of a business account from another business account. Both business accounts must be managed by the same bot, and the story on the source account must have been posted (or reposted) by the bot. Requires the *can_manage_stories* business bot right for both business accounts. Returns [Story](https://core.telegram.org/bots/api#story) on success.
 */
export interface RepostStoryRequest {
    /**
     * Unique identifier of the business connection
     */
    business_connection_id: string;
    /**
     * Unique identifier of the chat which posted the story that should be reposted
     */
    from_chat_id: number;
    /**
     * Unique identifier of the story that should be reposted
     */
    from_story_id: number;
    /**
     * Period after which the story is moved to the archive, in seconds; must be one of `6 * 3600`, `12 * 3600`, `86400`, or `2 * 86400`
     */
    active_period: number;
    /**
     * Pass *True* to keep the story accessible after it expires
     */
    post_to_chat_page: boolean | null;
    /**
     * Pass *True* if the content of the story must be protected from forwarding and screenshotting
     */
    protect_content: boolean | null;
}
export type RepostStoryResponse = Story;
/**
 * Edits a story previously posted by the bot on behalf of a managed business account. Requires the *can_manage_stories* business bot right. Returns [Story](https://core.telegram.org/bots/api#story) on success.
 */
export interface EditStoryRequest {
    /**
     * Unique identifier of the business connection
     */
    business_connection_id: string;
    /**
     * Unique identifier of the story to edit
     */
    story_id: number;
    /**
     * Content of the story
     */
    content: InputStoryContent;
    /**
     * Caption of the story, 0-2048 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the story caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * A JSON-serialized list of clickable areas to be shown on the story
     */
    areas: StoryArea[] | null;
}
export type EditStoryResponse = Story;
/**
 * Deletes a story previously posted by the bot on behalf of a managed business account. Requires the *can_manage_stories* business bot right. Returns *True* on success.
 */
export interface DeleteStoryRequest {
    /**
     * Unique identifier of the business connection
     */
    business_connection_id: string;
    /**
     * Unique identifier of the story to delete
     */
    story_id: number;
}
export type DeleteStoryResponse = true;
/**
 * Use this method to edit text and [game](https://core.telegram.org/bots/api#games) messages. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
 */
export interface EditMessageTextRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message to be edited was sent
     */
    business_connection_id: string | null;
    /**
     * Required if *inline_message_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string | null;
    /**
     * Required if *inline_message_id* is not specified. Identifier of the message to edit
     */
    message_id: number | null;
    /**
     * Required if *chat_id* and *message_id* are not specified. Identifier of the inline message
     */
    inline_message_id: string | null;
    /**
     * New text of the message, 1-4096 characters after entities parsing
     */
    text: string;
    /**
     * Mode for parsing entities in the message text. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * A JSON-serialized list of special entities that appear in message text, which can be specified instead of *parse_mode*
     */
    entities: MessageEntity[] | null;
    /**
     * Link preview generation options for the message
     */
    link_preview_options: LinkPreviewOptions | null;
    /**
     * A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards).
     */
    reply_markup: InlineKeyboardMarkup | null;
}
export type EditMessageTextResponse = Message | true;
/**
 * Use this method to edit captions of messages. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
 */
export interface EditMessageCaptionRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message to be edited was sent
     */
    business_connection_id: string | null;
    /**
     * Required if *inline_message_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string | null;
    /**
     * Required if *inline_message_id* is not specified. Identifier of the message to edit
     */
    message_id: number | null;
    /**
     * Required if *chat_id* and *message_id* are not specified. Identifier of the inline message
     */
    inline_message_id: string | null;
    /**
     * New caption of the message, 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the message caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * Pass *True*, if the caption must be shown above the message media. Supported only for animation, photo and video messages.
     */
    show_caption_above_media: boolean | null;
    /**
     * A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards).
     */
    reply_markup: InlineKeyboardMarkup | null;
}
export type EditMessageCaptionResponse = Message | true;
/**
 * Use this method to edit animation, audio, document, photo, or video messages, or to add media to text messages. If a message is part of a message album, then it can be edited only to an audio for audio albums, only to a document for document albums and to a photo or a video otherwise. When an inline message is edited, a new file can't be uploaded; use a previously uploaded file via its file_id or specify a URL. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
 */
export interface EditMessageMediaRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message to be edited was sent
     */
    business_connection_id: string | null;
    /**
     * Required if *inline_message_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string | null;
    /**
     * Required if *inline_message_id* is not specified. Identifier of the message to edit
     */
    message_id: number | null;
    /**
     * Required if *chat_id* and *message_id* are not specified. Identifier of the inline message
     */
    inline_message_id: string | null;
    /**
     * A JSON-serialized object for a new media content of the message
     */
    media: InputMedia;
    /**
     * A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards).
     */
    reply_markup: InlineKeyboardMarkup | null;
}
export type EditMessageMediaResponse = Message | true;
/**
 * Use this method to edit live location messages. A location can be edited until its *live_period* expires or editing is explicitly disabled by a call to [stopMessageLiveLocation](https://core.telegram.org/bots/api#stopmessagelivelocation). On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api#message) is returned, otherwise *True* is returned.
 */
export interface EditMessageLiveLocationRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message to be edited was sent
     */
    business_connection_id: string | null;
    /**
     * Required if *inline_message_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string | null;
    /**
     * Required if *inline_message_id* is not specified. Identifier of the message to edit
     */
    message_id: number | null;
    /**
     * Required if *chat_id* and *message_id* are not specified. Identifier of the inline message
     */
    inline_message_id: string | null;
    /**
     * Latitude of new location
     */
    latitude: number;
    /**
     * Longitude of new location
     */
    longitude: number;
    /**
     * New period in seconds during which the location can be updated, starting from the message send date. If 0x7FFFFFFF is specified, then the location can be updated forever. Otherwise, the new value must not exceed the current *live_period* by more than a day, and the live location expiration date must remain within the next 90 days. If not specified, then *live_period* remains unchanged
     */
    live_period: number | null;
    /**
     * The radius of uncertainty for the location, measured in meters; 0-1500
     */
    horizontal_accuracy: number | null;
    /**
     * Direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
     */
    heading: number | null;
    /**
     * The maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
     */
    proximity_alert_radius: number | null;
    /**
     * A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards).
     */
    reply_markup: InlineKeyboardMarkup | null;
}
export type EditMessageLiveLocationResponse = Message | true;
/**
 * Use this method to stop updating a live location message before *live_period* expires. On success, if the message is not an inline message, the edited [Message](https://core.telegram.org/bots/api#message) is returned, otherwise *True* is returned.
 */
export interface StopMessageLiveLocationRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message to be edited was sent
     */
    business_connection_id: string | null;
    /**
     * Required if *inline_message_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string | null;
    /**
     * Required if *inline_message_id* is not specified. Identifier of the message with live location to stop
     */
    message_id: number | null;
    /**
     * Required if *chat_id* and *message_id* are not specified. Identifier of the inline message
     */
    inline_message_id: string | null;
    /**
     * A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards).
     */
    reply_markup: InlineKeyboardMarkup | null;
}
export type StopMessageLiveLocationResponse = Message | true;
/**
 * Use this method to edit a checklist on behalf of a connected business account. On success, the edited [Message](https://core.telegram.org/bots/api#message) is returned.
 */
export interface EditMessageChecklistRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message will be sent
     */
    business_connection_id: string;
    /**
     * Unique identifier for the target chat
     */
    chat_id: number;
    /**
     * Unique identifier for the target message
     */
    message_id: number;
    /**
     * A JSON-serialized object for the new checklist
     */
    checklist: InputChecklist;
    /**
     * A JSON-serialized object for the new [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards) for the message
     */
    reply_markup: InlineKeyboardMarkup | null;
}
export type EditMessageChecklistResponse = Message;
/**
 * Use this method to edit only the reply markup of messages. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
 */
export interface EditMessageReplyMarkupRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message to be edited was sent
     */
    business_connection_id: string | null;
    /**
     * Required if *inline_message_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string | null;
    /**
     * Required if *inline_message_id* is not specified. Identifier of the message to edit
     */
    message_id: number | null;
    /**
     * Required if *chat_id* and *message_id* are not specified. Identifier of the inline message
     */
    inline_message_id: string | null;
    /**
     * A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards).
     */
    reply_markup: InlineKeyboardMarkup | null;
}
export type EditMessageReplyMarkupResponse = Message | true;
/**
 * Use this method to stop a poll which was sent by the bot. On success, the stopped [Poll](https://core.telegram.org/bots/api#poll) is returned.
 */
export interface StopPollRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message to be edited was sent
     */
    business_connection_id: string | null;
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Identifier of the original message with the poll
     */
    message_id: number;
    /**
     * A JSON-serialized object for a new message [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards).
     */
    reply_markup: InlineKeyboardMarkup | null;
}
export type StopPollResponse = Poll;
/**
 * Use this method to approve a suggested post in a direct messages chat. The bot must have the 'can_post_messages' administrator right in the corresponding channel chat. Returns *True* on success.
 */
export interface ApproveSuggestedPostRequest {
    /**
     * Unique identifier for the target direct messages chat
     */
    chat_id: number;
    /**
     * Identifier of a suggested post message to approve
     */
    message_id: number;
    /**
     * Point in time (Unix timestamp) when the post is expected to be published; omit if the date has already been specified when the suggested post was created. If specified, then the date must be not more than 2678400 seconds (30 days) in the future
     */
    send_date: number | null;
}
export type ApproveSuggestedPostResponse = true;
/**
 * Use this method to decline a suggested post in a direct messages chat. The bot must have the 'can_manage_direct_messages' administrator right in the corresponding channel chat. Returns *True* on success.
 */
export interface DeclineSuggestedPostRequest {
    /**
     * Unique identifier for the target direct messages chat
     */
    chat_id: number;
    /**
     * Identifier of a suggested post message to decline
     */
    message_id: number;
    /**
     * Comment for the creator of the suggested post; 0-128 characters
     */
    comment: string | null;
}
export type DeclineSuggestedPostResponse = true;
/**
 * Use this method to delete a message, including service messages, with the following limitations:
 * - A message can only be deleted if it was sent less than 48 hours ago.
 * - Service messages about a supergroup, channel, or forum topic creation can't be deleted.
 * - A dice message in a private chat can only be deleted if it was sent more than 24 hours ago.
 * - Bots can delete outgoing messages in private chats, groups, and supergroups.
 * - Bots can delete incoming messages in private chats.
 * - Bots granted *can_post_messages* permissions can delete outgoing messages in channels.
 * - If the bot is an administrator of a group, it can delete any message there.
 * - If the bot has *can_delete_messages* administrator right in a supergroup or a channel, it can delete any message there.
 * - If the bot has *can_manage_direct_messages* administrator right in a channel, it can delete any message in the corresponding direct messages chat.
 * Returns *True* on success.
 */
export interface DeleteMessageRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Identifier of the message to delete
     */
    message_id: number;
}
export type DeleteMessageResponse = true;
/**
 * Use this method to delete multiple messages simultaneously. If some of the specified messages can't be found, they are skipped. Returns *True* on success.
 */
export interface DeleteMessagesRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * A JSON-serialized list of 1-100 identifiers of messages to delete. See [deleteMessage](https://core.telegram.org/bots/api#deletemessage) for limitations on which messages can be deleted
     */
    message_ids: number[];
}
export type DeleteMessagesResponse = true;
/**
 * This object represents a sticker.
 */
export interface Sticker {
    /**
     * Identifier for this file, which can be used to download or reuse the file
     */
    file_id: string;
    /**
     * Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
     */
    file_unique_id: string;
    /**
     * Type of the sticker, currently one of “regular”, “mask”, “custom_emoji”. The type of the sticker is independent from its format, which is determined by the fields *is_animated* and *is_video*.
     */
    type: string;
    /**
     * Sticker width
     */
    width: number;
    /**
     * Sticker height
     */
    height: number;
    /**
     * *True*, if the sticker is [animated](https://telegram.org/blog/animated-stickers)
     */
    is_animated: boolean;
    /**
     * *True*, if the sticker is a [video sticker](https://telegram.org/blog/video-stickers-better-reactions)
     */
    is_video: boolean;
    /**
     * Sticker thumbnail in the .WEBP or .JPG format
     */
    thumbnail: PhotoSize | null;
    /**
     * Emoji associated with the sticker
     */
    emoji: string | null;
    /**
     * Name of the sticker set to which the sticker belongs
     */
    set_name: string | null;
    /**
     * For premium regular stickers, premium animation for the sticker
     */
    premium_animation: File | null;
    /**
     * For mask stickers, the position where the mask should be placed
     */
    mask_position: MaskPosition | null;
    /**
     * For custom emoji stickers, unique identifier of the custom emoji
     */
    custom_emoji_id: string | null;
    /**
     * *True*, if the sticker must be repainted to a text color in messages, the color of the Telegram Premium badge in emoji status, white color on chat photos, or another appropriate color in other places
     */
    needs_repainting: true | null;
    /**
     * File size in bytes
     */
    file_size: number | null;
}
/**
 * This object represents a sticker set.
 */
export interface StickerSet {
    /**
     * Sticker set name
     */
    name: string;
    /**
     * Sticker set title
     */
    title: string;
    /**
     * Type of stickers in the set, currently one of “regular”, “mask”, “custom_emoji”
     */
    sticker_type: string;
    /**
     * List of all set stickers
     */
    stickers: Sticker[];
    /**
     * Sticker set thumbnail in the .WEBP, .TGS, or .WEBM format
     */
    thumbnail: PhotoSize | null;
}
/**
 * This object describes the position on faces where a mask should be placed by default.
 */
export interface MaskPosition {
    /**
     * The part of the face relative to which the mask should be placed. One of “forehead”, “eyes”, “mouth”, or “chin”.
     */
    point: string;
    /**
     * Shift by X-axis measured in widths of the mask scaled to the face size, from left to right. For example, choosing -1.0 will place mask just to the left of the default mask position.
     */
    x_shift: number;
    /**
     * Shift by Y-axis measured in heights of the mask scaled to the face size, from top to bottom. For example, 1.0 will place the mask just below the default mask position.
     */
    y_shift: number;
    /**
     * Mask scaling coefficient. For example, 2.0 means double size.
     */
    scale: number;
}
/**
 * This object describes a sticker to be added to a sticker set.
 */
export interface InputSticker {
    /**
     * The added sticker. Pass a *file_id* as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new file using multipart/form-data under <file_attach_name> name. Animated and video stickers can't be uploaded via HTTP URL. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    sticker: string;
    /**
     * Format of the added sticker, must be one of “static” for a **.WEBP** or **.PNG** image, “animated” for a **.TGS** animation, “video” for a **.WEBM** video
     */
    format: string;
    /**
     * List of 1-20 emoji associated with the sticker
     */
    emoji_list: string[];
    /**
     * Position where the mask should be placed on faces. For “mask” stickers only.
     */
    mask_position: MaskPosition | null;
    /**
     * List of 0-20 search keywords for the sticker with total length of up to 64 characters. For “regular” and “custom_emoji” stickers only.
     */
    keywords: string[] | null;
}
/**
 * Use this method to send static .WEBP, [animated](https://telegram.org/blog/animated-stickers) .TGS, or [video](https://telegram.org/blog/video-stickers-better-reactions) .WEBM stickers. On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
 */
export interface SendStickerRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message will be sent
     */
    business_connection_id: string | null;
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
     */
    message_thread_id: number | null;
    /**
     * Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
     */
    direct_messages_topic_id: number | null;
    /**
     * Sticker to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a .WEBP sticker from the Internet, or upload a new .WEBP, .TGS, or .WEBM sticker using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files). Video and animated stickers can't be sent via an HTTP URL.
     */
    sticker: InputFile | string;
    /**
     * Emoji associated with the sticker; only for just uploaded stickers
     */
    emoji: string | null;
    /**
     * Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
     */
    disable_notification: boolean | null;
    /**
     * Protects the contents of the sent message from forwarding and saving
     */
    protect_content: boolean | null;
    /**
     * Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
     */
    allow_paid_broadcast: boolean | null;
    /**
     * Unique identifier of the message effect to be added to the message; for private chats only
     */
    message_effect_id: string | null;
    /**
     * A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
     */
    suggested_post_parameters: SuggestedPostParameters | null;
    /**
     * Description of the message to reply to
     */
    reply_parameters: ReplyParameters | null;
    /**
     * Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/api/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user
     */
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | null;
}
export type SendStickerResponse = Message;
/**
 * Use this method to get a sticker set. On success, a [StickerSet](https://core.telegram.org/bots/api#stickerset) object is returned.
 */
export interface GetStickerSetRequest {
    /**
     * Name of the sticker set
     */
    name: string;
}
export type GetStickerSetResponse = StickerSet;
/**
 * Use this method to get information about custom emoji stickers by their identifiers. Returns an Array of [Sticker](https://core.telegram.org/bots/api#sticker) objects.
 */
export interface GetCustomEmojiStickersRequest {
    /**
     * A JSON-serialized list of custom emoji identifiers. At most 200 custom emoji identifiers can be specified.
     */
    custom_emoji_ids: string[];
}
export type GetCustomEmojiStickersResponse = Sticker[];
/**
 * Use this method to upload a file with a sticker for later use in the [createNewStickerSet](https://core.telegram.org/bots/api#createnewstickerset), [addStickerToSet](https://core.telegram.org/bots/api#addstickertoset), or [replaceStickerInSet](https://core.telegram.org/bots/api#replacestickerinset) methods (the file can be used multiple times). Returns the uploaded [File](https://core.telegram.org/bots/api#file) on success.
 */
export interface UploadStickerFileRequest {
    /**
     * User identifier of sticker file owner
     */
    user_id: number;
    /**
     * A file with the sticker in .WEBP, .PNG, .TGS, or .WEBM format. See <a href="/stickers">[https://core.telegram.org/stickers](https://core.telegram.org/stickers)</a> for technical requirements. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files)
     */
    sticker: InputFile;
    /**
     * Format of the sticker, must be one of “static”, “animated”, “video”
     */
    sticker_format: string;
}
export type UploadStickerFileResponse = File;
/**
 * Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. Returns *True* on success.
 */
export interface CreateNewStickerSetRequest {
    /**
     * User identifier of created sticker set owner
     */
    user_id: number;
    /**
     * Short name of sticker set, to be used in `t.me/addstickers/` URLs (e.g., *animals*). Can contain only English letters, digits and underscores. Must begin with a letter, can't contain consecutive underscores and must end in `"_by_<bot_username>"`. `<bot_username>` is case insensitive. 1-64 characters.
     */
    name: string;
    /**
     * Sticker set title, 1-64 characters
     */
    title: string;
    /**
     * A JSON-serialized list of 1-50 initial stickers to be added to the sticker set
     */
    stickers: InputSticker[];
    /**
     * Type of stickers in the set, pass “regular”, “mask”, or “custom_emoji”. By default, a regular sticker set is created.
     */
    sticker_type: string | null;
    /**
     * Pass *True* if stickers in the sticker set must be repainted to the color of text when used in messages, the accent color if used as emoji status, white on chat photos, or another appropriate color based on context; for custom emoji sticker sets only
     */
    needs_repainting: boolean | null;
}
export type CreateNewStickerSetResponse = true;
/**
 * Use this method to add a new sticker to a set created by the bot. Emoji sticker sets can have up to 200 stickers. Other sticker sets can have up to 120 stickers. Returns *True* on success.
 */
export interface AddStickerToSetRequest {
    /**
     * User identifier of sticker set owner
     */
    user_id: number;
    /**
     * Sticker set name
     */
    name: string;
    /**
     * A JSON-serialized object with information about the added sticker. If exactly the same sticker had already been added to the set, then the set isn't changed.
     */
    sticker: InputSticker;
}
export type AddStickerToSetResponse = true;
/**
 * Use this method to move a sticker in a set created by the bot to a specific position. Returns *True* on success.
 */
export interface SetStickerPositionInSetRequest {
    /**
     * File identifier of the sticker
     */
    sticker: string;
    /**
     * New sticker position in the set, zero-based
     */
    position: number;
}
export type SetStickerPositionInSetResponse = true;
/**
 * Use this method to delete a sticker from a set created by the bot. Returns *True* on success.
 */
export interface DeleteStickerFromSetRequest {
    /**
     * File identifier of the sticker
     */
    sticker: string;
}
export type DeleteStickerFromSetResponse = true;
/**
 * Use this method to replace an existing sticker in a sticker set with a new one. The method is equivalent to calling [deleteStickerFromSet](https://core.telegram.org/bots/api#deletestickerfromset), then [addStickerToSet](https://core.telegram.org/bots/api#addstickertoset), then [setStickerPositionInSet](https://core.telegram.org/bots/api#setstickerpositioninset). Returns *True* on success.
 */
export interface ReplaceStickerInSetRequest {
    /**
     * User identifier of the sticker set owner
     */
    user_id: number;
    /**
     * Sticker set name
     */
    name: string;
    /**
     * File identifier of the replaced sticker
     */
    old_sticker: string;
    /**
     * A JSON-serialized object with information about the added sticker. If exactly the same sticker had already been added to the set, then the set remains unchanged.
     */
    sticker: InputSticker;
}
export type ReplaceStickerInSetResponse = true;
/**
 * Use this method to change the list of emoji assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns *True* on success.
 */
export interface SetStickerEmojiListRequest {
    /**
     * File identifier of the sticker
     */
    sticker: string;
    /**
     * A JSON-serialized list of 1-20 emoji associated with the sticker
     */
    emoji_list: string[];
}
export type SetStickerEmojiListResponse = true;
/**
 * Use this method to change search keywords assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns *True* on success.
 */
export interface SetStickerKeywordsRequest {
    /**
     * File identifier of the sticker
     */
    sticker: string;
    /**
     * A JSON-serialized list of 0-20 search keywords for the sticker with total length of up to 64 characters
     */
    keywords: string[] | null;
}
export type SetStickerKeywordsResponse = true;
/**
 * Use this method to change the [mask position](https://core.telegram.org/bots/api#maskposition) of a mask sticker. The sticker must belong to a sticker set that was created by the bot. Returns *True* on success.
 */
export interface SetStickerMaskPositionRequest {
    /**
     * File identifier of the sticker
     */
    sticker: string;
    /**
     * A JSON-serialized object with the position where the mask should be placed on faces. Omit the parameter to remove the mask position.
     */
    mask_position: MaskPosition | null;
}
export type SetStickerMaskPositionResponse = true;
/**
 * Use this method to set the title of a created sticker set. Returns *True* on success.
 */
export interface SetStickerSetTitleRequest {
    /**
     * Sticker set name
     */
    name: string;
    /**
     * Sticker set title, 1-64 characters
     */
    title: string;
}
export type SetStickerSetTitleResponse = true;
/**
 * Use this method to set the thumbnail of a regular or mask sticker set. The format of the thumbnail file must match the format of the stickers in the set. Returns *True* on success.
 */
export interface SetStickerSetThumbnailRequest {
    /**
     * Sticker set name
     */
    name: string;
    /**
     * User identifier of the sticker set owner
     */
    user_id: number;
    /**
     * A **.WEBP** or **.PNG** image with the thumbnail, must be up to 128 kilobytes in size and have a width and height of exactly 100px, or a **.TGS** animation with a thumbnail up to 32 kilobytes in size (see <a href="/stickers#animation-requirements">[https://core.telegram.org/stickers#animation-requirements](https://core.telegram.org/stickers#animation-requirements)</a> for animated sticker technical requirements), or a **.WEBM** video with the thumbnail up to 32 kilobytes in size; see <a href="/stickers#video-requirements">[https://core.telegram.org/stickers#video-requirements](https://core.telegram.org/stickers#video-requirements)</a> for video sticker technical requirements. Pass a *file_id* as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api#sending-files). Animated and video sticker set thumbnails can't be uploaded via HTTP URL. If omitted, then the thumbnail is dropped and the first sticker is used as the thumbnail.
     */
    thumbnail: InputFile | string | null;
    /**
     * Format of the thumbnail, must be one of “static” for a **.WEBP** or **.PNG** image, “animated” for a **.TGS** animation, or “video” for a **.WEBM** video
     */
    format: string;
}
export type SetStickerSetThumbnailResponse = true;
/**
 * Use this method to set the thumbnail of a custom emoji sticker set. Returns *True* on success.
 */
export interface SetCustomEmojiStickerSetThumbnailRequest {
    /**
     * Sticker set name
     */
    name: string;
    /**
     * Custom emoji identifier of a sticker from the sticker set; pass an empty string to drop the thumbnail and use the first sticker as the thumbnail.
     */
    custom_emoji_id: string | null;
}
export type SetCustomEmojiStickerSetThumbnailResponse = true;
/**
 * Use this method to delete a sticker set that was created by the bot. Returns *True* on success.
 */
export interface DeleteStickerSetRequest {
    /**
     * Sticker set name
     */
    name: string;
}
export type DeleteStickerSetResponse = true;
/**
 * This object represents an incoming inline query. When the user sends an empty query, your bot could return some default or trending results.
 */
export interface InlineQuery {
    /**
     * Unique identifier for this query
     */
    id: string;
    /**
     * Sender
     */
    from: User;
    /**
     * Text of the query (up to 256 characters)
     */
    query: string;
    /**
     * Offset of the results to be returned, can be controlled by the bot
     */
    offset: string;
    /**
     * Type of the chat from which the inline query was sent. Can be either “sender” for a private chat with the inline query sender, “private”, “group”, “supergroup”, or “channel”. The chat type should be always known for requests sent from official clients and most third-party clients, unless the request was sent from a secret chat
     */
    chat_type: string | null;
    /**
     * Sender location, only for bots that request user location
     */
    location: Location | null;
}
/**
 * Use this method to send answers to an inline query. On success, *True* is returned.
 * No more than **50** results per query are allowed.
 */
export interface AnswerInlineQueryRequest {
    /**
     * Unique identifier for the answered query
     */
    inline_query_id: string;
    /**
     * A JSON-serialized array of results for the inline query
     */
    results: InlineQueryResult[];
    /**
     * The maximum amount of time in seconds that the result of the inline query may be cached on the server. Defaults to 300.
     */
    cache_time: number | null;
    /**
     * Pass *True* if results may be cached on the server side only for the user that sent the query. By default, results may be returned to any user who sends the same query.
     */
    is_personal: boolean | null;
    /**
     * Pass the offset that a client should send in the next query with the same text to receive more results. Pass an empty string if there are no more results or if you don't support pagination. Offset length can't exceed 64 bytes.
     */
    next_offset: string | null;
    /**
     * A JSON-serialized object describing a button to be shown above inline query results
     */
    button: InlineQueryResultsButton | null;
}
export type AnswerInlineQueryResponse = true;
/**
 * This object represents a button to be shown above inline query results. You **must** use exactly one of the optional fields.
 */
export interface InlineQueryResultsButton {
    /**
     * Label text on the button
     */
    text: string;
    /**
     * Description of the [Web App](https://core.telegram.org/bots/api/bots/webapps) that will be launched when the user presses the button. The Web App will be able to switch back to the inline mode using the method [switchInlineQuery](https://core.telegram.org/bots/api/bots/webapps#initializing-mini-apps) inside the Web App.
     */
    web_app: WebAppInfo | null;
    /**
     * [Deep-linking](https://core.telegram.org/bots/api/bots/features#deep-linking) parameter for the /start message sent to the bot when a user presses the button. 1-64 characters, only `A-Z`, `a-z`, `0-9`, `_` and `-` are allowed.
     *
     * *Example:* An inline bot that sends YouTube videos can ask the user to connect the bot to their YouTube account to adapt search results accordingly. To do this, it displays a 'Connect your YouTube account' button above the results, or even before showing any. The user presses the button, switches to a private chat with the bot and, in doing so, passes a start parameter that instructs the bot to return an OAuth link. Once done, the bot can offer a [*switch_inline*](https://core.telegram.org/bots/api#inlinekeyboardmarkup) button so that the user can easily return to the chat where they wanted to use the bot's inline capabilities.
     */
    start_parameter: string | null;
}
/**
 * This object represents one result of an inline query.
 */
export type InlineQueryResult = InlineQueryResultCachedAudio | InlineQueryResultCachedDocument | InlineQueryResultCachedGif | InlineQueryResultCachedMpeg4Gif | InlineQueryResultCachedPhoto | InlineQueryResultCachedSticker | InlineQueryResultCachedVideo | InlineQueryResultCachedVoice | InlineQueryResultArticle | InlineQueryResultAudio | InlineQueryResultContact | InlineQueryResultGame | InlineQueryResultDocument | InlineQueryResultGif | InlineQueryResultLocation | InlineQueryResultMpeg4Gif | InlineQueryResultPhoto | InlineQueryResultVenue | InlineQueryResultVideo | InlineQueryResultVoice;
/**
 * Represents a link to an article or web page.
 */
export interface InlineQueryResultArticle {
    /**
     * Type of the result, must be *article*
     */
    type: string;
    /**
     * Unique identifier for this result, 1-64 Bytes
     */
    id: string;
    /**
     * Title of the result
     */
    title: string;
    /**
     * Content of the message to be sent
     */
    input_message_content: InputMessageContent;
    /**
     * [Inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards) attached to the message
     */
    reply_markup: InlineKeyboardMarkup | null;
    /**
     * URL of the result
     */
    url: string | null;
    /**
     * Short description of the result
     */
    description: string | null;
    /**
     * Url of the thumbnail for the result
     */
    thumbnail_url: string | null;
    /**
     * Thumbnail width
     */
    thumbnail_width: number | null;
    /**
     * Thumbnail height
     */
    thumbnail_height: number | null;
}
/**
 * Represents a link to a photo. By default, this photo will be sent by the user with optional caption. Alternatively, you can use *input_message_content* to send a message with the specified content instead of the photo.
 */
export interface InlineQueryResultPhoto {
    /**
     * Type of the result, must be *photo*
     */
    type: string;
    /**
     * Unique identifier for this result, 1-64 bytes
     */
    id: string;
    /**
     * A valid URL of the photo. Photo must be in **JPEG** format. Photo size must not exceed 5MB
     */
    photo_url: string;
    /**
     * URL of the thumbnail for the photo
     */
    thumbnail_url: string;
    /**
     * Width of the photo
     */
    photo_width: number | null;
    /**
     * Height of the photo
     */
    photo_height: number | null;
    /**
     * Title for the result
     */
    title: string | null;
    /**
     * Short description of the result
     */
    description: string | null;
    /**
     * Caption of the photo to be sent, 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the photo caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * List of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * Pass *True*, if the caption must be shown above the message media
     */
    show_caption_above_media: boolean | null;
    /**
     * [Inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards) attached to the message
     */
    reply_markup: InlineKeyboardMarkup | null;
    /**
     * Content of the message to be sent instead of the photo
     */
    input_message_content: InputMessageContent | null;
}
/**
 * Represents a link to an animated GIF file. By default, this animated GIF file will be sent by the user with optional caption. Alternatively, you can use *input_message_content* to send a message with the specified content instead of the animation.
 */
export interface InlineQueryResultGif {
    /**
     * Type of the result, must be *gif*
     */
    type: string;
    /**
     * Unique identifier for this result, 1-64 bytes
     */
    id: string;
    /**
     * A valid URL for the GIF file
     */
    gif_url: string;
    /**
     * Width of the GIF
     */
    gif_width: number | null;
    /**
     * Height of the GIF
     */
    gif_height: number | null;
    /**
     * Duration of the GIF in seconds
     */
    gif_duration: number | null;
    /**
     * URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
     */
    thumbnail_url: string;
    /**
     * MIME type of the thumbnail, must be one of “image/jpeg”, “image/gif”, or “video/mp4”. Defaults to “image/jpeg”
     */
    thumbnail_mime_type: string | null;
    /**
     * Title for the result
     */
    title: string | null;
    /**
     * Caption of the GIF file to be sent, 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * List of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * Pass *True*, if the caption must be shown above the message media
     */
    show_caption_above_media: boolean | null;
    /**
     * [Inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards) attached to the message
     */
    reply_markup: InlineKeyboardMarkup | null;
    /**
     * Content of the message to be sent instead of the GIF animation
     */
    input_message_content: InputMessageContent | null;
}
/**
 * Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default, this animated MPEG-4 file will be sent by the user with optional caption. Alternatively, you can use *input_message_content* to send a message with the specified content instead of the animation.
 */
export interface InlineQueryResultMpeg4Gif {
    /**
     * Type of the result, must be *mpeg4_gif*
     */
    type: string;
    /**
     * Unique identifier for this result, 1-64 bytes
     */
    id: string;
    /**
     * A valid URL for the MPEG4 file
     */
    mpeg4_url: string;
    /**
     * Video width
     */
    mpeg4_width: number | null;
    /**
     * Video height
     */
    mpeg4_height: number | null;
    /**
     * Video duration in seconds
     */
    mpeg4_duration: number | null;
    /**
     * URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
     */
    thumbnail_url: string;
    /**
     * MIME type of the thumbnail, must be one of “image/jpeg”, “image/gif”, or “video/mp4”. Defaults to “image/jpeg”
     */
    thumbnail_mime_type: string | null;
    /**
     * Title for the result
     */
    title: string | null;
    /**
     * Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * List of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * Pass *True*, if the caption must be shown above the message media
     */
    show_caption_above_media: boolean | null;
    /**
     * [Inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards) attached to the message
     */
    reply_markup: InlineKeyboardMarkup | null;
    /**
     * Content of the message to be sent instead of the video animation
     */
    input_message_content: InputMessageContent | null;
}
/**
 * Represents a link to a page containing an embedded video player or a video file. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use *input_message_content* to send a message with the specified content instead of the video.
 */
export interface InlineQueryResultVideo {
    /**
     * Type of the result, must be *video*
     */
    type: string;
    /**
     * Unique identifier for this result, 1-64 bytes
     */
    id: string;
    /**
     * A valid URL for the embedded video player or video file
     */
    video_url: string;
    /**
     * MIME type of the content of the video URL, “text/html” or “video/mp4”
     */
    mime_type: string;
    /**
     * URL of the thumbnail (JPEG only) for the video
     */
    thumbnail_url: string;
    /**
     * Title for the result
     */
    title: string;
    /**
     * Caption of the video to be sent, 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the video caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * List of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * Pass *True*, if the caption must be shown above the message media
     */
    show_caption_above_media: boolean | null;
    /**
     * Video width
     */
    video_width: number | null;
    /**
     * Video height
     */
    video_height: number | null;
    /**
     * Video duration in seconds
     */
    video_duration: number | null;
    /**
     * Short description of the result
     */
    description: string | null;
    /**
     * [Inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards) attached to the message
     */
    reply_markup: InlineKeyboardMarkup | null;
    /**
     * Content of the message to be sent instead of the video. This field is **required** if InlineQueryResultVideo is used to send an HTML-page as a result (e.g., a YouTube video).
     */
    input_message_content: InputMessageContent | null;
}
/**
 * Represents a link to an MP3 audio file. By default, this audio file will be sent by the user. Alternatively, you can use *input_message_content* to send a message with the specified content instead of the audio.
 */
export interface InlineQueryResultAudio {
    /**
     * Type of the result, must be *audio*
     */
    type: string;
    /**
     * Unique identifier for this result, 1-64 bytes
     */
    id: string;
    /**
     * A valid URL for the audio file
     */
    audio_url: string;
    /**
     * Title
     */
    title: string;
    /**
     * Caption, 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the audio caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * List of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * Performer
     */
    performer: string | null;
    /**
     * Audio duration in seconds
     */
    audio_duration: number | null;
    /**
     * [Inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards) attached to the message
     */
    reply_markup: InlineKeyboardMarkup | null;
    /**
     * Content of the message to be sent instead of the audio
     */
    input_message_content: InputMessageContent | null;
}
/**
 * Represents a link to a voice recording in an .OGG container encoded with OPUS. By default, this voice recording will be sent by the user. Alternatively, you can use *input_message_content* to send a message with the specified content instead of the the voice message.
 */
export interface InlineQueryResultVoice {
    /**
     * Type of the result, must be *voice*
     */
    type: string;
    /**
     * Unique identifier for this result, 1-64 bytes
     */
    id: string;
    /**
     * A valid URL for the voice recording
     */
    voice_url: string;
    /**
     * Recording title
     */
    title: string;
    /**
     * Caption, 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the voice message caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * List of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * Recording duration in seconds
     */
    voice_duration: number | null;
    /**
     * [Inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards) attached to the message
     */
    reply_markup: InlineKeyboardMarkup | null;
    /**
     * Content of the message to be sent instead of the voice recording
     */
    input_message_content: InputMessageContent | null;
}
/**
 * Represents a link to a file. By default, this file will be sent by the user with an optional caption. Alternatively, you can use *input_message_content* to send a message with the specified content instead of the file. Currently, only **.PDF** and **.ZIP** files can be sent using this method.
 */
export interface InlineQueryResultDocument {
    /**
     * Type of the result, must be *document*
     */
    type: string;
    /**
     * Unique identifier for this result, 1-64 bytes
     */
    id: string;
    /**
     * Title for the result
     */
    title: string;
    /**
     * Caption of the document to be sent, 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the document caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * List of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * A valid URL for the file
     */
    document_url: string;
    /**
     * MIME type of the content of the file, either “application/pdf” or “application/zip”
     */
    mime_type: string;
    /**
     * Short description of the result
     */
    description: string | null;
    /**
     * [Inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards) attached to the message
     */
    reply_markup: InlineKeyboardMarkup | null;
    /**
     * Content of the message to be sent instead of the file
     */
    input_message_content: InputMessageContent | null;
    /**
     * URL of the thumbnail (JPEG only) for the file
     */
    thumbnail_url: string | null;
    /**
     * Thumbnail width
     */
    thumbnail_width: number | null;
    /**
     * Thumbnail height
     */
    thumbnail_height: number | null;
}
/**
 * Represents a location on a map. By default, the location will be sent by the user. Alternatively, you can use *input_message_content* to send a message with the specified content instead of the location.
 */
export interface InlineQueryResultLocation {
    /**
     * Type of the result, must be *location*
     */
    type: string;
    /**
     * Unique identifier for this result, 1-64 Bytes
     */
    id: string;
    /**
     * Location latitude in degrees
     */
    latitude: number;
    /**
     * Location longitude in degrees
     */
    longitude: number;
    /**
     * Location title
     */
    title: string;
    /**
     * The radius of uncertainty for the location, measured in meters; 0-1500
     */
    horizontal_accuracy: number | null;
    /**
     * Period in seconds during which the location can be updated, should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely.
     */
    live_period: number | null;
    /**
     * For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
     */
    heading: number | null;
    /**
     * For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
     */
    proximity_alert_radius: number | null;
    /**
     * [Inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards) attached to the message
     */
    reply_markup: InlineKeyboardMarkup | null;
    /**
     * Content of the message to be sent instead of the location
     */
    input_message_content: InputMessageContent | null;
    /**
     * Url of the thumbnail for the result
     */
    thumbnail_url: string | null;
    /**
     * Thumbnail width
     */
    thumbnail_width: number | null;
    /**
     * Thumbnail height
     */
    thumbnail_height: number | null;
}
/**
 * Represents a venue. By default, the venue will be sent by the user. Alternatively, you can use *input_message_content* to send a message with the specified content instead of the venue.
 */
export interface InlineQueryResultVenue {
    /**
     * Type of the result, must be *venue*
     */
    type: string;
    /**
     * Unique identifier for this result, 1-64 Bytes
     */
    id: string;
    /**
     * Latitude of the venue location in degrees
     */
    latitude: number;
    /**
     * Longitude of the venue location in degrees
     */
    longitude: number;
    /**
     * Title of the venue
     */
    title: string;
    /**
     * Address of the venue
     */
    address: string;
    /**
     * Foursquare identifier of the venue if known
     */
    foursquare_id: string | null;
    /**
     * Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
     */
    foursquare_type: string | null;
    /**
     * Google Places identifier of the venue
     */
    google_place_id: string | null;
    /**
     * Google Places type of the venue. (See [supported types](https://developers.google.com/places/web-service/supported_types).)
     */
    google_place_type: string | null;
    /**
     * [Inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards) attached to the message
     */
    reply_markup: InlineKeyboardMarkup | null;
    /**
     * Content of the message to be sent instead of the venue
     */
    input_message_content: InputMessageContent | null;
    /**
     * Url of the thumbnail for the result
     */
    thumbnail_url: string | null;
    /**
     * Thumbnail width
     */
    thumbnail_width: number | null;
    /**
     * Thumbnail height
     */
    thumbnail_height: number | null;
}
/**
 * Represents a contact with a phone number. By default, this contact will be sent by the user. Alternatively, you can use *input_message_content* to send a message with the specified content instead of the contact.
 */
export interface InlineQueryResultContact {
    /**
     * Type of the result, must be *contact*
     */
    type: string;
    /**
     * Unique identifier for this result, 1-64 Bytes
     */
    id: string;
    /**
     * Contact's phone number
     */
    phone_number: string;
    /**
     * Contact's first name
     */
    first_name: string;
    /**
     * Contact's last name
     */
    last_name: string | null;
    /**
     * Additional data about the contact in the form of a [vCard](https://en.wikipedia.org/wiki/VCard), 0-2048 bytes
     */
    vcard: string | null;
    /**
     * [Inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards) attached to the message
     */
    reply_markup: InlineKeyboardMarkup | null;
    /**
     * Content of the message to be sent instead of the contact
     */
    input_message_content: InputMessageContent | null;
    /**
     * Url of the thumbnail for the result
     */
    thumbnail_url: string | null;
    /**
     * Thumbnail width
     */
    thumbnail_width: number | null;
    /**
     * Thumbnail height
     */
    thumbnail_height: number | null;
}
/**
 * Represents a [Game](https://core.telegram.org/bots/api#games).
 */
export interface InlineQueryResultGame {
    /**
     * Type of the result, must be *game*
     */
    type: string;
    /**
     * Unique identifier for this result, 1-64 bytes
     */
    id: string;
    /**
     * Short name of the game
     */
    game_short_name: string;
    /**
     * [Inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards) attached to the message
     */
    reply_markup: InlineKeyboardMarkup | null;
}
/**
 * Represents a link to a photo stored on the Telegram servers. By default, this photo will be sent by the user with an optional caption. Alternatively, you can use *input_message_content* to send a message with the specified content instead of the photo.
 */
export interface InlineQueryResultCachedPhoto {
    /**
     * Type of the result, must be *photo*
     */
    type: string;
    /**
     * Unique identifier for this result, 1-64 bytes
     */
    id: string;
    /**
     * A valid file identifier of the photo
     */
    photo_file_id: string;
    /**
     * Title for the result
     */
    title: string | null;
    /**
     * Short description of the result
     */
    description: string | null;
    /**
     * Caption of the photo to be sent, 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the photo caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * List of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * Pass *True*, if the caption must be shown above the message media
     */
    show_caption_above_media: boolean | null;
    /**
     * [Inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards) attached to the message
     */
    reply_markup: InlineKeyboardMarkup | null;
    /**
     * Content of the message to be sent instead of the photo
     */
    input_message_content: InputMessageContent | null;
}
/**
 * Represents a link to an animated GIF file stored on the Telegram servers. By default, this animated GIF file will be sent by the user with an optional caption. Alternatively, you can use *input_message_content* to send a message with specified content instead of the animation.
 */
export interface InlineQueryResultCachedGif {
    /**
     * Type of the result, must be *gif*
     */
    type: string;
    /**
     * Unique identifier for this result, 1-64 bytes
     */
    id: string;
    /**
     * A valid file identifier for the GIF file
     */
    gif_file_id: string;
    /**
     * Title for the result
     */
    title: string | null;
    /**
     * Caption of the GIF file to be sent, 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * List of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * Pass *True*, if the caption must be shown above the message media
     */
    show_caption_above_media: boolean | null;
    /**
     * [Inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards) attached to the message
     */
    reply_markup: InlineKeyboardMarkup | null;
    /**
     * Content of the message to be sent instead of the GIF animation
     */
    input_message_content: InputMessageContent | null;
}
/**
 * Represents a link to a video animation (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers. By default, this animated MPEG-4 file will be sent by the user with an optional caption. Alternatively, you can use *input_message_content* to send a message with the specified content instead of the animation.
 */
export interface InlineQueryResultCachedMpeg4Gif {
    /**
     * Type of the result, must be *mpeg4_gif*
     */
    type: string;
    /**
     * Unique identifier for this result, 1-64 bytes
     */
    id: string;
    /**
     * A valid file identifier for the MPEG4 file
     */
    mpeg4_file_id: string;
    /**
     * Title for the result
     */
    title: string | null;
    /**
     * Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * List of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * Pass *True*, if the caption must be shown above the message media
     */
    show_caption_above_media: boolean | null;
    /**
     * [Inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards) attached to the message
     */
    reply_markup: InlineKeyboardMarkup | null;
    /**
     * Content of the message to be sent instead of the video animation
     */
    input_message_content: InputMessageContent | null;
}
/**
 * Represents a link to a sticker stored on the Telegram servers. By default, this sticker will be sent by the user. Alternatively, you can use *input_message_content* to send a message with the specified content instead of the sticker.
 */
export interface InlineQueryResultCachedSticker {
    /**
     * Type of the result, must be *sticker*
     */
    type: string;
    /**
     * Unique identifier for this result, 1-64 bytes
     */
    id: string;
    /**
     * A valid file identifier of the sticker
     */
    sticker_file_id: string;
    /**
     * [Inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards) attached to the message
     */
    reply_markup: InlineKeyboardMarkup | null;
    /**
     * Content of the message to be sent instead of the sticker
     */
    input_message_content: InputMessageContent | null;
}
/**
 * Represents a link to a file stored on the Telegram servers. By default, this file will be sent by the user with an optional caption. Alternatively, you can use *input_message_content* to send a message with the specified content instead of the file.
 */
export interface InlineQueryResultCachedDocument {
    /**
     * Type of the result, must be *document*
     */
    type: string;
    /**
     * Unique identifier for this result, 1-64 bytes
     */
    id: string;
    /**
     * Title for the result
     */
    title: string;
    /**
     * A valid file identifier for the file
     */
    document_file_id: string;
    /**
     * Short description of the result
     */
    description: string | null;
    /**
     * Caption of the document to be sent, 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the document caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * List of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * [Inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards) attached to the message
     */
    reply_markup: InlineKeyboardMarkup | null;
    /**
     * Content of the message to be sent instead of the file
     */
    input_message_content: InputMessageContent | null;
}
/**
 * Represents a link to a video file stored on the Telegram servers. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use *input_message_content* to send a message with the specified content instead of the video.
 */
export interface InlineQueryResultCachedVideo {
    /**
     * Type of the result, must be *video*
     */
    type: string;
    /**
     * Unique identifier for this result, 1-64 bytes
     */
    id: string;
    /**
     * A valid file identifier for the video file
     */
    video_file_id: string;
    /**
     * Title for the result
     */
    title: string;
    /**
     * Short description of the result
     */
    description: string | null;
    /**
     * Caption of the video to be sent, 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the video caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * List of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * Pass *True*, if the caption must be shown above the message media
     */
    show_caption_above_media: boolean | null;
    /**
     * [Inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards) attached to the message
     */
    reply_markup: InlineKeyboardMarkup | null;
    /**
     * Content of the message to be sent instead of the video
     */
    input_message_content: InputMessageContent | null;
}
/**
 * Represents a link to a voice message stored on the Telegram servers. By default, this voice message will be sent by the user. Alternatively, you can use *input_message_content* to send a message with the specified content instead of the voice message.
 */
export interface InlineQueryResultCachedVoice {
    /**
     * Type of the result, must be *voice*
     */
    type: string;
    /**
     * Unique identifier for this result, 1-64 bytes
     */
    id: string;
    /**
     * A valid file identifier for the voice message
     */
    voice_file_id: string;
    /**
     * Voice message title
     */
    title: string;
    /**
     * Caption, 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the voice message caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * List of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * [Inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards) attached to the message
     */
    reply_markup: InlineKeyboardMarkup | null;
    /**
     * Content of the message to be sent instead of the voice message
     */
    input_message_content: InputMessageContent | null;
}
/**
 * Represents a link to an MP3 audio file stored on the Telegram servers. By default, this audio file will be sent by the user. Alternatively, you can use *input_message_content* to send a message with the specified content instead of the audio.
 */
export interface InlineQueryResultCachedAudio {
    /**
     * Type of the result, must be *audio*
     */
    type: string;
    /**
     * Unique identifier for this result, 1-64 bytes
     */
    id: string;
    /**
     * A valid file identifier for the audio file
     */
    audio_file_id: string;
    /**
     * Caption, 0-1024 characters after entities parsing
     */
    caption: string | null;
    /**
     * Mode for parsing entities in the audio caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * List of special entities that appear in the caption, which can be specified instead of *parse_mode*
     */
    caption_entities: MessageEntity[] | null;
    /**
     * [Inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards) attached to the message
     */
    reply_markup: InlineKeyboardMarkup | null;
    /**
     * Content of the message to be sent instead of the audio
     */
    input_message_content: InputMessageContent | null;
}
/**
 * This object represents the content of a message to be sent as a result of an inline query.
 */
export type InputMessageContent = InputTextMessageContent | InputLocationMessageContent | InputVenueMessageContent | InputContactMessageContent | InputInvoiceMessageContent;
/**
 * Represents the [content](https://core.telegram.org/bots/api#inputmessagecontent) of a text message to be sent as the result of an inline query.
 */
export interface InputTextMessageContent {
    /**
     * Text of the message to be sent, 1-4096 characters
     */
    message_text: string;
    /**
     * Mode for parsing entities in the message text. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
     */
    parse_mode: string | null;
    /**
     * List of special entities that appear in message text, which can be specified instead of *parse_mode*
     */
    entities: MessageEntity[] | null;
    /**
     * Link preview generation options for the message
     */
    link_preview_options: LinkPreviewOptions | null;
}
/**
 * Represents the [content](https://core.telegram.org/bots/api#inputmessagecontent) of a location message to be sent as the result of an inline query.
 */
export interface InputLocationMessageContent {
    /**
     * Latitude of the location in degrees
     */
    latitude: number;
    /**
     * Longitude of the location in degrees
     */
    longitude: number;
    /**
     * The radius of uncertainty for the location, measured in meters; 0-1500
     */
    horizontal_accuracy: number | null;
    /**
     * Period in seconds during which the location can be updated, should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely.
     */
    live_period: number | null;
    /**
     * For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
     */
    heading: number | null;
    /**
     * For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
     */
    proximity_alert_radius: number | null;
}
/**
 * Represents the [content](https://core.telegram.org/bots/api#inputmessagecontent) of a venue message to be sent as the result of an inline query.
 */
export interface InputVenueMessageContent {
    /**
     * Latitude of the venue in degrees
     */
    latitude: number;
    /**
     * Longitude of the venue in degrees
     */
    longitude: number;
    /**
     * Name of the venue
     */
    title: string;
    /**
     * Address of the venue
     */
    address: string;
    /**
     * Foursquare identifier of the venue, if known
     */
    foursquare_id: string | null;
    /**
     * Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
     */
    foursquare_type: string | null;
    /**
     * Google Places identifier of the venue
     */
    google_place_id: string | null;
    /**
     * Google Places type of the venue. (See [supported types](https://developers.google.com/places/web-service/supported_types).)
     */
    google_place_type: string | null;
}
/**
 * Represents the [content](https://core.telegram.org/bots/api#inputmessagecontent) of a contact message to be sent as the result of an inline query.
 */
export interface InputContactMessageContent {
    /**
     * Contact's phone number
     */
    phone_number: string;
    /**
     * Contact's first name
     */
    first_name: string;
    /**
     * Contact's last name
     */
    last_name: string | null;
    /**
     * Additional data about the contact in the form of a [vCard](https://en.wikipedia.org/wiki/VCard), 0-2048 bytes
     */
    vcard: string | null;
}
/**
 * Represents the [content](https://core.telegram.org/bots/api#inputmessagecontent) of an invoice message to be sent as the result of an inline query.
 */
export interface InputInvoiceMessageContent {
    /**
     * Product name, 1-32 characters
     */
    title: string;
    /**
     * Product description, 1-255 characters
     */
    description: string;
    /**
     * Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use it for your internal processes.
     */
    payload: string;
    /**
     * Payment provider token, obtained via [@BotFather](https://t.me/botfather). Pass an empty string for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    provider_token: string | null;
    /**
     * Three-letter ISO 4217 currency code, see [more on currencies](https://core.telegram.org/bots/api/bots/payments#supported-currencies). Pass “XTR” for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    currency: string;
    /**
     * Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.). Must contain exactly one item for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    prices: LabeledPrice[];
    /**
     * The maximum accepted amount for tips in the *smallest units* of the currency (integer, **not** float/double). For example, for a maximum tip of `US$ 1.45` pass `max_tip_amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/api/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    max_tip_amount: number | null;
    /**
     * A JSON-serialized array of suggested amounts of tip in the *smallest units* of the currency (integer, **not** float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed *max_tip_amount*.
     */
    suggested_tip_amounts: number[] | null;
    /**
     * A JSON-serialized object for data about the invoice, which will be shared with the payment provider. A detailed description of the required fields should be provided by the payment provider.
     */
    provider_data: string | null;
    /**
     * URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
     */
    photo_url: string | null;
    /**
     * Photo size in bytes
     */
    photo_size: number | null;
    /**
     * Photo width
     */
    photo_width: number | null;
    /**
     * Photo height
     */
    photo_height: number | null;
    /**
     * Pass *True* if you require the user's full name to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    need_name: boolean | null;
    /**
     * Pass *True* if you require the user's phone number to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    need_phone_number: boolean | null;
    /**
     * Pass *True* if you require the user's email address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    need_email: boolean | null;
    /**
     * Pass *True* if you require the user's shipping address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    need_shipping_address: boolean | null;
    /**
     * Pass *True* if the user's phone number should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    send_phone_number_to_provider: boolean | null;
    /**
     * Pass *True* if the user's email address should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    send_email_to_provider: boolean | null;
    /**
     * Pass *True* if the final price depends on the shipping method. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    is_flexible: boolean | null;
}
/**
 * Represents a [result](https://core.telegram.org/bots/api#inlinequeryresult) of an inline query that was chosen by the user and sent to their chat partner.
 */
export interface ChosenInlineResult {
    /**
     * The unique identifier for the result that was chosen
     */
    result_id: string;
    /**
     * The user that chose the result
     */
    from: User;
    /**
     * Sender location, only for bots that require user location
     */
    location: Location | null;
    /**
     * Identifier of the sent inline message. Available only if there is an [inline keyboard](https://core.telegram.org/bots/api#inlinekeyboardmarkup) attached to the message. Will be also received in [callback queries](https://core.telegram.org/bots/api#callbackquery) and can be used to [edit](https://core.telegram.org/bots/api#updating-messages) the message.
     */
    inline_message_id: string | null;
    /**
     * The query that was used to obtain the result
     */
    query: string;
}
/**
 * Use this method to set the result of an interaction with a [Web App](https://core.telegram.org/bots/api/bots/webapps) and send a corresponding message on behalf of the user to the chat from which the query originated. On success, a [SentWebAppMessage](https://core.telegram.org/bots/api#sentwebappmessage) object is returned.
 */
export interface AnswerWebAppQueryRequest {
    /**
     * Unique identifier for the query to be answered
     */
    web_app_query_id: string;
    /**
     * A JSON-serialized object describing the message to be sent
     */
    result: InlineQueryResult;
}
export type AnswerWebAppQueryResponse = SentWebAppMessage;
/**
 * Describes an inline message sent by a [Web App](https://core.telegram.org/bots/api/bots/webapps) on behalf of a user.
 */
export interface SentWebAppMessage {
    /**
     * Identifier of the sent inline message. Available only if there is an [inline keyboard](https://core.telegram.org/bots/api#inlinekeyboardmarkup) attached to the message.
     */
    inline_message_id: string | null;
}
/**
 * Stores a message that can be sent by a user of a Mini App. Returns a [PreparedInlineMessage](https://core.telegram.org/bots/api#preparedinlinemessage) object.
 */
export interface SavePreparedInlineMessageRequest {
    /**
     * Unique identifier of the target user that can use the prepared message
     */
    user_id: number;
    /**
     * A JSON-serialized object describing the message to be sent
     */
    result: InlineQueryResult;
    /**
     * Pass *True* if the message can be sent to private chats with users
     */
    allow_user_chats: boolean | null;
    /**
     * Pass *True* if the message can be sent to private chats with bots
     */
    allow_bot_chats: boolean | null;
    /**
     * Pass *True* if the message can be sent to group and supergroup chats
     */
    allow_group_chats: boolean | null;
    /**
     * Pass *True* if the message can be sent to channel chats
     */
    allow_channel_chats: boolean | null;
}
export type SavePreparedInlineMessageResponse = PreparedInlineMessage;
/**
 * Describes an inline message to be sent by a user of a Mini App.
 */
export interface PreparedInlineMessage {
    /**
     * Unique identifier of the prepared message
     */
    id: string;
    /**
     * Expiration date of the prepared message, in Unix time. Expired prepared messages can no longer be used
     */
    expiration_date: number;
}
/**
 * Use this method to send invoices. On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
 */
export interface SendInvoiceRequest {
    /**
     * Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
     */
    chat_id: number | string;
    /**
     * Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
     */
    message_thread_id: number | null;
    /**
     * Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
     */
    direct_messages_topic_id: number | null;
    /**
     * Product name, 1-32 characters
     */
    title: string;
    /**
     * Product description, 1-255 characters
     */
    description: string;
    /**
     * Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use it for your internal processes.
     */
    payload: string;
    /**
     * Payment provider token, obtained via [@BotFather](https://t.me/botfather). Pass an empty string for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    provider_token: string | null;
    /**
     * Three-letter ISO 4217 currency code, see [more on currencies](https://core.telegram.org/bots/api/bots/payments#supported-currencies). Pass “XTR” for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    currency: string;
    /**
     * Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.). Must contain exactly one item for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    prices: LabeledPrice[];
    /**
     * The maximum accepted amount for tips in the *smallest units* of the currency (integer, **not** float/double). For example, for a maximum tip of `US$ 1.45` pass `max_tip_amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/api/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    max_tip_amount: number | null;
    /**
     * A JSON-serialized array of suggested amounts of tips in the *smallest units* of the currency (integer, **not** float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed *max_tip_amount*.
     */
    suggested_tip_amounts: number[] | null;
    /**
     * Unique deep-linking parameter. If left empty, **forwarded copies** of the sent message will have a *Pay* button, allowing multiple users to pay directly from the forwarded message, using the same invoice. If non-empty, forwarded copies of the sent message will have a *URL* button with a deep link to the bot (instead of a *Pay* button), with the value used as the start parameter
     */
    start_parameter: string | null;
    /**
     * JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
     */
    provider_data: string | null;
    /**
     * URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service. People like it better when they see what they are paying for.
     */
    photo_url: string | null;
    /**
     * Photo size in bytes
     */
    photo_size: number | null;
    /**
     * Photo width
     */
    photo_width: number | null;
    /**
     * Photo height
     */
    photo_height: number | null;
    /**
     * Pass *True* if you require the user's full name to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    need_name: boolean | null;
    /**
     * Pass *True* if you require the user's phone number to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    need_phone_number: boolean | null;
    /**
     * Pass *True* if you require the user's email address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    need_email: boolean | null;
    /**
     * Pass *True* if you require the user's shipping address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    need_shipping_address: boolean | null;
    /**
     * Pass *True* if the user's phone number should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    send_phone_number_to_provider: boolean | null;
    /**
     * Pass *True* if the user's email address should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    send_email_to_provider: boolean | null;
    /**
     * Pass *True* if the final price depends on the shipping method. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    is_flexible: boolean | null;
    /**
     * Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
     */
    disable_notification: boolean | null;
    /**
     * Protects the contents of the sent message from forwarding and saving
     */
    protect_content: boolean | null;
    /**
     * Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
     */
    allow_paid_broadcast: boolean | null;
    /**
     * Unique identifier of the message effect to be added to the message; for private chats only
     */
    message_effect_id: string | null;
    /**
     * A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
     */
    suggested_post_parameters: SuggestedPostParameters | null;
    /**
     * Description of the message to reply to
     */
    reply_parameters: ReplyParameters | null;
    /**
     * A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards). If empty, one 'Pay `total price`' button will be shown. If not empty, the first button must be a Pay button.
     */
    reply_markup: InlineKeyboardMarkup | null;
}
export type SendInvoiceResponse = Message;
/**
 * Use this method to create a link for an invoice. Returns the created invoice link as *String* on success.
 */
export interface CreateInvoiceLinkRequest {
    /**
     * Unique identifier of the business connection on behalf of which the link will be created. For payments in [Telegram Stars](https://t.me/BotNews/90) only.
     */
    business_connection_id: string | null;
    /**
     * Product name, 1-32 characters
     */
    title: string;
    /**
     * Product description, 1-255 characters
     */
    description: string;
    /**
     * Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use it for your internal processes.
     */
    payload: string;
    /**
     * Payment provider token, obtained via [@BotFather](https://t.me/botfather). Pass an empty string for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    provider_token: string | null;
    /**
     * Three-letter ISO 4217 currency code, see [more on currencies](https://core.telegram.org/bots/api/bots/payments#supported-currencies). Pass “XTR” for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    currency: string;
    /**
     * Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.). Must contain exactly one item for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    prices: LabeledPrice[];
    /**
     * The number of seconds the subscription will be active for before the next payment. The currency must be set to “XTR” (Telegram Stars) if the parameter is used. Currently, it must always be 2592000 (30 days) if specified. Any number of subscriptions can be active for a given bot at the same time, including multiple concurrent subscriptions from the same user. Subscription price must no exceed 10000 Telegram Stars.
     */
    subscription_period: number | null;
    /**
     * The maximum accepted amount for tips in the *smallest units* of the currency (integer, **not** float/double). For example, for a maximum tip of `US$ 1.45` pass `max_tip_amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/api/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    max_tip_amount: number | null;
    /**
     * A JSON-serialized array of suggested amounts of tips in the *smallest units* of the currency (integer, **not** float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed *max_tip_amount*.
     */
    suggested_tip_amounts: number[] | null;
    /**
     * JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
     */
    provider_data: string | null;
    /**
     * URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
     */
    photo_url: string | null;
    /**
     * Photo size in bytes
     */
    photo_size: number | null;
    /**
     * Photo width
     */
    photo_width: number | null;
    /**
     * Photo height
     */
    photo_height: number | null;
    /**
     * Pass *True* if you require the user's full name to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    need_name: boolean | null;
    /**
     * Pass *True* if you require the user's phone number to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    need_phone_number: boolean | null;
    /**
     * Pass *True* if you require the user's email address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    need_email: boolean | null;
    /**
     * Pass *True* if you require the user's shipping address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    need_shipping_address: boolean | null;
    /**
     * Pass *True* if the user's phone number should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    send_phone_number_to_provider: boolean | null;
    /**
     * Pass *True* if the user's email address should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    send_email_to_provider: boolean | null;
    /**
     * Pass *True* if the final price depends on the shipping method. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
     */
    is_flexible: boolean | null;
}
export type CreateInvoiceLinkResponse = string;
/**
 * If you sent an invoice requesting a shipping address and the parameter *is_flexible* was specified, the Bot API will send an [Update](https://core.telegram.org/bots/api#update) with a *shipping_query* field to the bot. Use this method to reply to shipping queries. On success, *True* is returned.
 */
export interface AnswerShippingQueryRequest {
    /**
     * Unique identifier for the query to be answered
     */
    shipping_query_id: string;
    /**
     * Pass *True* if delivery to the specified address is possible and *False* if there are any problems (for example, if delivery to the specified address is not possible)
     */
    ok: boolean;
    /**
     * Required if *ok* is *True*. A JSON-serialized array of available shipping options.
     */
    shipping_options: ShippingOption[] | null;
    /**
     * Required if *ok* is *False*. Error message in human readable form that explains why it is impossible to complete the order (e.g. “Sorry, delivery to your desired address is unavailable”). Telegram will display this message to the user.
     */
    error_message: string | null;
}
export type AnswerShippingQueryResponse = true;
/**
 * Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an [Update](https://core.telegram.org/bots/api#update) with the field *pre_checkout_query*. Use this method to respond to such pre-checkout queries. On success, *True* is returned. **Note:** The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent.
 */
export interface AnswerPreCheckoutQueryRequest {
    /**
     * Unique identifier for the query to be answered
     */
    pre_checkout_query_id: string;
    /**
     * Specify *True* if everything is alright (goods are available, etc.) and the bot is ready to proceed with the order. Use *False* if there are any problems.
     */
    ok: boolean;
    /**
     * Required if *ok* is *False*. Error message in human readable form that explains the reason for failure to proceed with the checkout (e.g. "Sorry, somebody just bought the last of our amazing black T-shirts while you were busy filling out your payment details. Please choose a different color or garment!"). Telegram will display this message to the user.
     */
    error_message: string | null;
}
export type AnswerPreCheckoutQueryResponse = true;
/**
 * A method to get the current Telegram Stars balance of the bot. Requires no parameters. On success, returns a [StarAmount](https://core.telegram.org/bots/api#staramount) object.
 */
// deno-lint-ignore no-empty-interface
export interface GetMyStarBalanceRequest {}
export type GetMyStarBalanceResponse = StarAmount;
/**
 * Returns the bot's Telegram Star transactions in chronological order. On success, returns a [StarTransactions](https://core.telegram.org/bots/api#startransactions) object.
 */
export interface GetStarTransactionsRequest {
    /**
     * Number of transactions to skip in the response
     */
    offset: number | null;
    /**
     * The maximum number of transactions to be retrieved. Values between 1-100 are accepted. Defaults to 100.
     */
    limit: number | null;
}
export type GetStarTransactionsResponse = StarTransactions;
/**
 * Refunds a successful payment in [Telegram Stars](https://t.me/BotNews/90). Returns *True* on success.
 */
export interface RefundStarPaymentRequest {
    /**
     * Identifier of the user whose payment will be refunded
     */
    user_id: number;
    /**
     * Telegram payment identifier
     */
    telegram_payment_charge_id: string;
}
export type RefundStarPaymentResponse = true;
/**
 * Allows the bot to cancel or re-enable extension of a subscription paid in Telegram Stars. Returns *True* on success.
 */
export interface EditUserStarSubscriptionRequest {
    /**
     * Identifier of the user whose subscription will be edited
     */
    user_id: number;
    /**
     * Telegram payment identifier for the subscription
     */
    telegram_payment_charge_id: string;
    /**
     * Pass *True* to cancel extension of the user subscription; the subscription must be active up to the end of the current subscription period. Pass *False* to allow the user to re-enable a subscription that was previously canceled by the bot.
     */
    is_canceled: boolean;
}
export type EditUserStarSubscriptionResponse = true;
/**
 * This object represents a portion of the price for goods or services.
 */
export interface LabeledPrice {
    /**
     * Portion label
     */
    label: string;
    /**
     * Price of the product in the *smallest units* of the [currency](https://core.telegram.org/bots/api/bots/payments#supported-currencies) (integer, **not** float/double). For example, for a price of `US$ 1.45` pass `amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/api/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
     */
    amount: number;
}
/**
 * This object contains basic information about an invoice.
 */
export interface Invoice {
    /**
     * Product name
     */
    title: string;
    /**
     * Product description
     */
    description: string;
    /**
     * Unique bot deep-linking parameter that can be used to generate this invoice
     */
    start_parameter: string;
    /**
     * Three-letter ISO 4217 [currency](https://core.telegram.org/bots/api/bots/payments#supported-currencies) code, or “XTR” for payments in [Telegram Stars](https://t.me/BotNews/90)
     */
    currency: string;
    /**
     * Total price in the *smallest units* of the currency (integer, **not** float/double). For example, for a price of `US$ 1.45` pass `amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/api/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
     */
    total_amount: number;
}
/**
 * This object represents a shipping address.
 */
export interface ShippingAddress {
    /**
     * Two-letter [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code
     */
    country_code: string;
    /**
     * State, if applicable
     */
    state: string;
    /**
     * City
     */
    city: string;
    /**
     * First line for the address
     */
    street_line1: string;
    /**
     * Second line for the address
     */
    street_line2: string;
    /**
     * Address post code
     */
    post_code: string;
}
/**
 * This object represents information about an order.
 */
export interface OrderInfo {
    /**
     * User name
     */
    name: string | null;
    /**
     * User's phone number
     */
    phone_number: string | null;
    /**
     * User email
     */
    email: string | null;
    /**
     * User shipping address
     */
    shipping_address: ShippingAddress | null;
}
/**
 * This object represents one shipping option.
 */
export interface ShippingOption {
    /**
     * Shipping option identifier
     */
    id: string;
    /**
     * Option title
     */
    title: string;
    /**
     * List of price portions
     */
    prices: LabeledPrice[];
}
/**
 * This object contains basic information about a successful payment. Note that if the buyer initiates a chargeback with the relevant payment provider following this transaction, the funds may be debited from your balance. This is outside of Telegram's control.
 */
export interface SuccessfulPayment {
    /**
     * Three-letter ISO 4217 [currency](https://core.telegram.org/bots/api/bots/payments#supported-currencies) code, or “XTR” for payments in [Telegram Stars](https://t.me/BotNews/90)
     */
    currency: string;
    /**
     * Total price in the *smallest units* of the currency (integer, **not** float/double). For example, for a price of `US$ 1.45` pass `amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/api/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
     */
    total_amount: number;
    /**
     * Bot-specified invoice payload
     */
    invoice_payload: string;
    /**
     * Expiration date of the subscription, in Unix time; for recurring payments only
     */
    subscription_expiration_date: number | null;
    /**
     * *True*, if the payment is a recurring payment for a subscription
     */
    is_recurring: true | null;
    /**
     * *True*, if the payment is the first payment for a subscription
     */
    is_first_recurring: true | null;
    /**
     * Identifier of the shipping option chosen by the user
     */
    shipping_option_id: string | null;
    /**
     * Order information provided by the user
     */
    order_info: OrderInfo | null;
    /**
     * Telegram payment identifier
     */
    telegram_payment_charge_id: string;
    /**
     * Provider payment identifier
     */
    provider_payment_charge_id: string;
}
/**
 * This object contains basic information about a refunded payment.
 */
export interface RefundedPayment {
    /**
     * Three-letter ISO 4217 [currency](https://core.telegram.org/bots/api/bots/payments#supported-currencies) code, or “XTR” for payments in [Telegram Stars](https://t.me/BotNews/90). Currently, always “XTR”
     */
    currency: string;
    /**
     * Total refunded price in the *smallest units* of the currency (integer, **not** float/double). For example, for a price of `US$ 1.45`, `total_amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/api/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
     */
    total_amount: number;
    /**
     * Bot-specified invoice payload
     */
    invoice_payload: string;
    /**
     * Telegram payment identifier
     */
    telegram_payment_charge_id: string;
    /**
     * Provider payment identifier
     */
    provider_payment_charge_id: string | null;
}
/**
 * This object contains information about an incoming shipping query.
 */
export interface ShippingQuery {
    /**
     * Unique query identifier
     */
    id: string;
    /**
     * User who sent the query
     */
    from: User;
    /**
     * Bot-specified invoice payload
     */
    invoice_payload: string;
    /**
     * User specified shipping address
     */
    shipping_address: ShippingAddress;
}
/**
 * This object contains information about an incoming pre-checkout query.
 */
export interface PreCheckoutQuery {
    /**
     * Unique query identifier
     */
    id: string;
    /**
     * User who sent the query
     */
    from: User;
    /**
     * Three-letter ISO 4217 [currency](https://core.telegram.org/bots/api/bots/payments#supported-currencies) code, or “XTR” for payments in [Telegram Stars](https://t.me/BotNews/90)
     */
    currency: string;
    /**
     * Total price in the *smallest units* of the currency (integer, **not** float/double). For example, for a price of `US$ 1.45` pass `amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/api/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
     */
    total_amount: number;
    /**
     * Bot-specified invoice payload
     */
    invoice_payload: string;
    /**
     * Identifier of the shipping option chosen by the user
     */
    shipping_option_id: string | null;
    /**
     * Order information provided by the user
     */
    order_info: OrderInfo | null;
}
/**
 * This object contains information about a paid media purchase.
 */
export interface PaidMediaPurchased {
    /**
     * User who purchased the media
     */
    from: User;
    /**
     * Bot-specified paid media payload
     */
    paid_media_payload: string;
}
/**
 * This object describes the state of a revenue withdrawal operation.
 */
export type RevenueWithdrawalState = RevenueWithdrawalStatePending | RevenueWithdrawalStateSucceeded | RevenueWithdrawalStateFailed;
/**
 * The withdrawal is in progress.
 */
export interface RevenueWithdrawalStatePending {
    /**
     * Type of the state, always “pending”
     */
    type: string;
}
/**
 * The withdrawal succeeded.
 */
export interface RevenueWithdrawalStateSucceeded {
    /**
     * Type of the state, always “succeeded”
     */
    type: string;
    /**
     * Date the withdrawal was completed in Unix time
     */
    date: number;
    /**
     * An HTTPS URL that can be used to see transaction details
     */
    url: string;
}
/**
 * The withdrawal failed and the transaction was refunded.
 */
export interface RevenueWithdrawalStateFailed {
    /**
     * Type of the state, always “failed”
     */
    type: string;
}
/**
 * Contains information about the affiliate that received a commission via this transaction.
 */
export interface AffiliateInfo {
    /**
     * The bot or the user that received an affiliate commission if it was received by a bot or a user
     */
    affiliate_user: User | null;
    /**
     * The chat that received an affiliate commission if it was received by a chat
     */
    affiliate_chat: Chat | null;
    /**
     * The number of Telegram Stars received by the affiliate for each 1000 Telegram Stars received by the bot from referred users
     */
    commission_per_mille: number;
    /**
     * Integer amount of Telegram Stars received by the affiliate from the transaction, rounded to 0; can be negative for refunds
     */
    amount: number;
    /**
     * The number of 1/1000000000 shares of Telegram Stars received by the affiliate; from -999999999 to 999999999; can be negative for refunds
     */
    nanostar_amount: number | null;
}
/**
 * This object describes the source of a transaction, or its recipient for outgoing transactions.
 */
export type TransactionPartner = TransactionPartnerUser | TransactionPartnerChat | TransactionPartnerAffiliateProgram | TransactionPartnerFragment | TransactionPartnerTelegramAds | TransactionPartnerTelegramApi | TransactionPartnerOther;
/**
 * Describes a transaction with a user.
 */
export interface TransactionPartnerUser {
    /**
     * Type of the transaction partner, always “user”
     */
    type: string;
    /**
     * Type of the transaction, currently one of “invoice_payment” for payments via invoices, “paid_media_payment” for payments for paid media, “gift_purchase” for gifts sent by the bot, “premium_purchase” for Telegram Premium subscriptions gifted by the bot, “business_account_transfer” for direct transfers from managed business accounts
     */
    transaction_type: string;
    /**
     * Information about the user
     */
    user: User;
    /**
     * Information about the affiliate that received a commission via this transaction. Can be available only for “invoice_payment” and “paid_media_payment” transactions.
     */
    affiliate: AffiliateInfo | null;
    /**
     * Bot-specified invoice payload. Can be available only for “invoice_payment” transactions.
     */
    invoice_payload: string | null;
    /**
     * The duration of the paid subscription. Can be available only for “invoice_payment” transactions.
     */
    subscription_period: number | null;
    /**
     * Information about the paid media bought by the user; for “paid_media_payment” transactions only
     */
    paid_media: PaidMedia[] | null;
    /**
     * Bot-specified paid media payload. Can be available only for “paid_media_payment” transactions.
     */
    paid_media_payload: string | null;
    /**
     * The gift sent to the user by the bot; for “gift_purchase” transactions only
     */
    gift: Gift | null;
    /**
     * Number of months the gifted Telegram Premium subscription will be active for; for “premium_purchase” transactions only
     */
    premium_subscription_duration: number | null;
}
/**
 * Describes a transaction with a chat.
 */
export interface TransactionPartnerChat {
    /**
     * Type of the transaction partner, always “chat”
     */
    type: string;
    /**
     * Information about the chat
     */
    chat: Chat;
    /**
     * The gift sent to the chat by the bot
     */
    gift: Gift | null;
}
/**
 * Describes the affiliate program that issued the affiliate commission received via this transaction.
 */
export interface TransactionPartnerAffiliateProgram {
    /**
     * Type of the transaction partner, always “affiliate_program”
     */
    type: string;
    /**
     * Information about the bot that sponsored the affiliate program
     */
    sponsor_user: User | null;
    /**
     * The number of Telegram Stars received by the bot for each 1000 Telegram Stars received by the affiliate program sponsor from referred users
     */
    commission_per_mille: number;
}
/**
 * Describes a withdrawal transaction with Fragment.
 */
export interface TransactionPartnerFragment {
    /**
     * Type of the transaction partner, always “fragment”
     */
    type: string;
    /**
     * State of the transaction if the transaction is outgoing
     */
    withdrawal_state: RevenueWithdrawalState | null;
}
/**
 * Describes a withdrawal transaction to the Telegram Ads platform.
 */
export interface TransactionPartnerTelegramAds {
    /**
     * Type of the transaction partner, always “telegram_ads”
     */
    type: string;
}
/**
 * Describes a transaction with payment for [paid broadcasting](https://core.telegram.org/bots/api#paid-broadcasts).
 */
export interface TransactionPartnerTelegramApi {
    /**
     * Type of the transaction partner, always “telegram_api”
     */
    type: string;
    /**
     * The number of successful requests that exceeded regular limits and were therefore billed
     */
    request_count: number;
}
/**
 * Describes a transaction with an unknown source or recipient.
 */
export interface TransactionPartnerOther {
    /**
     * Type of the transaction partner, always “other”
     */
    type: string;
}
/**
 * Describes a Telegram Star transaction. Note that if the buyer initiates a chargeback with the payment provider from whom they acquired Stars (e.g., Apple, Google) following this transaction, the refunded Stars will be deducted from the bot's balance. This is outside of Telegram's control.
 */
export interface StarTransaction {
    /**
     * Unique identifier of the transaction. Coincides with the identifier of the original transaction for refund transactions. Coincides with *SuccessfulPayment.telegram_payment_charge_id* for successful incoming payments from users.
     */
    id: string;
    /**
     * Integer amount of Telegram Stars transferred by the transaction
     */
    amount: number;
    /**
     * The number of 1/1000000000 shares of Telegram Stars transferred by the transaction; from 0 to 999999999
     */
    nanostar_amount: number | null;
    /**
     * Date the transaction was created in Unix time
     */
    date: number;
    /**
     * Source of an incoming transaction (e.g., a user purchasing goods or services, Fragment refunding a failed withdrawal). Only for incoming transactions
     */
    source: TransactionPartner | null;
    /**
     * Receiver of an outgoing transaction (e.g., a user for a purchase refund, Fragment for a withdrawal). Only for outgoing transactions
     */
    receiver: TransactionPartner | null;
}
/**
 * Contains a list of Telegram Star transactions.
 */
export interface StarTransactions {
    /**
     * The list of transactions
     */
    transactions: StarTransaction[];
}
/**
 * Describes Telegram Passport data shared with the bot by the user.
 */
export interface PassportData {
    /**
     * Array with information about documents and other Telegram Passport elements that was shared with the bot
     */
    data: EncryptedPassportElement[];
    /**
     * Encrypted credentials required to decrypt the data
     */
    credentials: EncryptedCredentials;
}
/**
 * This object represents a file uploaded to Telegram Passport. Currently all Telegram Passport files are in JPEG format when decrypted and don't exceed 10MB.
 */
export interface PassportFile {
    /**
     * Identifier for this file, which can be used to download or reuse the file
     */
    file_id: string;
    /**
     * Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
     */
    file_unique_id: string;
    /**
     * File size in bytes
     */
    file_size: number;
    /**
     * Unix time when the file was uploaded
     */
    file_date: number;
}
/**
 * Describes documents or other Telegram Passport elements shared with the bot by the user.
 */
export interface EncryptedPassportElement {
    /**
     * Element type. One of “personal_details”, “passport”, “driver_license”, “identity_card”, “internal_passport”, “address”, “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”, “phone_number”, “email”.
     */
    type: string;
    /**
     * Base64-encoded encrypted Telegram Passport element data provided by the user; available only for “personal_details”, “passport”, “driver_license”, “identity_card”, “internal_passport” and “address” types. Can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api#encryptedcredentials).
     */
    data: string | null;
    /**
     * User's verified phone number; available only for “phone_number” type
     */
    phone_number: string | null;
    /**
     * User's verified email address; available only for “email” type
     */
    email: string | null;
    /**
     * Array of encrypted files with documents provided by the user; available only for “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration” and “temporary_registration” types. Files can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api#encryptedcredentials).
     */
    files: PassportFile[] | null;
    /**
     * Encrypted file with the front side of the document, provided by the user; available only for “passport”, “driver_license”, “identity_card” and “internal_passport”. The file can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api#encryptedcredentials).
     */
    front_side: PassportFile | null;
    /**
     * Encrypted file with the reverse side of the document, provided by the user; available only for “driver_license” and “identity_card”. The file can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api#encryptedcredentials).
     */
    reverse_side: PassportFile | null;
    /**
     * Encrypted file with the selfie of the user holding a document, provided by the user; available if requested for “passport”, “driver_license”, “identity_card” and “internal_passport”. The file can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api#encryptedcredentials).
     */
    selfie: PassportFile | null;
    /**
     * Array of encrypted files with translated versions of documents provided by the user; available if requested for “passport”, “driver_license”, “identity_card”, “internal_passport”, “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration” and “temporary_registration” types. Files can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api#encryptedcredentials).
     */
    translation: PassportFile[] | null;
    /**
     * Base64-encoded element hash for using in [PassportElementErrorUnspecified](https://core.telegram.org/bots/api#passportelementerrorunspecified)
     */
    hash: string;
}
/**
 * Describes data required for decrypting and authenticating [EncryptedPassportElement](https://core.telegram.org/bots/api#encryptedpassportelement). See the [Telegram Passport Documentation](https://core.telegram.org/bots/api/passport#receiving-information) for a complete description of the data decryption and authentication processes.
 */
export interface EncryptedCredentials {
    /**
     * Base64-encoded encrypted JSON-serialized data with unique user's payload, data hashes and secrets required for [EncryptedPassportElement](https://core.telegram.org/bots/api#encryptedpassportelement) decryption and authentication
     */
    data: string;
    /**
     * Base64-encoded data hash for data authentication
     */
    hash: string;
    /**
     * Base64-encoded secret, encrypted with the bot's public RSA key, required for data decryption
     */
    secret: string;
}
/**
 * Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns *True* on success.
 */
export interface SetPassportDataErrorsRequest {
    /**
     * User identifier
     */
    user_id: number;
    /**
     * A JSON-serialized array describing the errors
     */
    errors: PassportElementError[];
}
export type SetPassportDataErrorsResponse = true;
/**
 * This object represents an error in the Telegram Passport element which was submitted that should be resolved by the user.
 */
export type PassportElementError = PassportElementErrorDataField | PassportElementErrorFrontSide | PassportElementErrorReverseSide | PassportElementErrorSelfie | PassportElementErrorFile | PassportElementErrorFiles | PassportElementErrorTranslationFile | PassportElementErrorTranslationFiles | PassportElementErrorUnspecified;
/**
 * Represents an issue in one of the data fields that was provided by the user. The error is considered resolved when the field's value changes.
 */
export interface PassportElementErrorDataField {
    /**
     * Error source, must be *data*
     */
    source: string;
    /**
     * The section of the user's Telegram Passport which has the error, one of “personal_details”, “passport”, “driver_license”, “identity_card”, “internal_passport”, “address”
     */
    type: string;
    /**
     * Name of the data field which has the error
     */
    field_name: string;
    /**
     * Base64-encoded data hash
     */
    data_hash: string;
    /**
     * Error message
     */
    message: string;
}
/**
 * Represents an issue with the front side of a document. The error is considered resolved when the file with the front side of the document changes.
 */
export interface PassportElementErrorFrontSide {
    /**
     * Error source, must be *front_side*
     */
    source: string;
    /**
     * The section of the user's Telegram Passport which has the issue, one of “passport”, “driver_license”, “identity_card”, “internal_passport”
     */
    type: string;
    /**
     * Base64-encoded hash of the file with the front side of the document
     */
    file_hash: string;
    /**
     * Error message
     */
    message: string;
}
/**
 * Represents an issue with the reverse side of a document. The error is considered resolved when the file with reverse side of the document changes.
 */
export interface PassportElementErrorReverseSide {
    /**
     * Error source, must be *reverse_side*
     */
    source: string;
    /**
     * The section of the user's Telegram Passport which has the issue, one of “driver_license”, “identity_card”
     */
    type: string;
    /**
     * Base64-encoded hash of the file with the reverse side of the document
     */
    file_hash: string;
    /**
     * Error message
     */
    message: string;
}
/**
 * Represents an issue with the selfie with a document. The error is considered resolved when the file with the selfie changes.
 */
export interface PassportElementErrorSelfie {
    /**
     * Error source, must be *selfie*
     */
    source: string;
    /**
     * The section of the user's Telegram Passport which has the issue, one of “passport”, “driver_license”, “identity_card”, “internal_passport”
     */
    type: string;
    /**
     * Base64-encoded hash of the file with the selfie
     */
    file_hash: string;
    /**
     * Error message
     */
    message: string;
}
/**
 * Represents an issue with a document scan. The error is considered resolved when the file with the document scan changes.
 */
export interface PassportElementErrorFile {
    /**
     * Error source, must be *file*
     */
    source: string;
    /**
     * The section of the user's Telegram Passport which has the issue, one of “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
     */
    type: string;
    /**
     * Base64-encoded file hash
     */
    file_hash: string;
    /**
     * Error message
     */
    message: string;
}
/**
 * Represents an issue with a list of scans. The error is considered resolved when the list of files containing the scans changes.
 */
export interface PassportElementErrorFiles {
    /**
     * Error source, must be *files*
     */
    source: string;
    /**
     * The section of the user's Telegram Passport which has the issue, one of “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
     */
    type: string;
    /**
     * List of base64-encoded file hashes
     */
    file_hashes: string[];
    /**
     * Error message
     */
    message: string;
}
/**
 * Represents an issue with one of the files that constitute the translation of a document. The error is considered resolved when the file changes.
 */
export interface PassportElementErrorTranslationFile {
    /**
     * Error source, must be *translation_file*
     */
    source: string;
    /**
     * Type of element of the user's Telegram Passport which has the issue, one of “passport”, “driver_license”, “identity_card”, “internal_passport”, “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
     */
    type: string;
    /**
     * Base64-encoded file hash
     */
    file_hash: string;
    /**
     * Error message
     */
    message: string;
}
/**
 * Represents an issue with the translated version of a document. The error is considered resolved when a file with the document translation change.
 */
export interface PassportElementErrorTranslationFiles {
    /**
     * Error source, must be *translation_files*
     */
    source: string;
    /**
     * Type of element of the user's Telegram Passport which has the issue, one of “passport”, “driver_license”, “identity_card”, “internal_passport”, “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
     */
    type: string;
    /**
     * List of base64-encoded file hashes
     */
    file_hashes: string[];
    /**
     * Error message
     */
    message: string;
}
/**
 * Represents an issue in an unspecified place. The error is considered resolved when new data is added.
 */
export interface PassportElementErrorUnspecified {
    /**
     * Error source, must be *unspecified*
     */
    source: string;
    /**
     * Type of element of the user's Telegram Passport which has the issue
     */
    type: string;
    /**
     * Base64-encoded element hash
     */
    element_hash: string;
    /**
     * Error message
     */
    message: string;
}
/**
 * Use this method to send a game. On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
 */
export interface SendGameRequest {
    /**
     * Unique identifier of the business connection on behalf of which the message will be sent
     */
    business_connection_id: string | null;
    /**
     * Unique identifier for the target chat. Games can't be sent to channel direct messages chats and channel chats.
     */
    chat_id: number;
    /**
     * Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
     */
    message_thread_id: number | null;
    /**
     * Short name of the game, serves as the unique identifier for the game. Set up your games via [@BotFather](https://t.me/botfather).
     */
    game_short_name: string;
    /**
     * Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
     */
    disable_notification: boolean | null;
    /**
     * Protects the contents of the sent message from forwarding and saving
     */
    protect_content: boolean | null;
    /**
     * Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
     */
    allow_paid_broadcast: boolean | null;
    /**
     * Unique identifier of the message effect to be added to the message; for private chats only
     */
    message_effect_id: string | null;
    /**
     * Description of the message to reply to
     */
    reply_parameters: ReplyParameters | null;
    /**
     * A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/api/bots/features#inline-keyboards). If empty, one 'Play game_title' button will be shown. If not empty, the first button must launch the game.
     */
    reply_markup: InlineKeyboardMarkup | null;
}
export type SendGameResponse = Message;
/**
 * This object represents a game. Use BotFather to create and edit games, their short names will act as unique identifiers.
 */
export interface Game {
    /**
     * Title of the game
     */
    title: string;
    /**
     * Description of the game
     */
    description: string;
    /**
     * Photo that will be displayed in the game message in chats.
     */
    photo: PhotoSize[];
    /**
     * Brief description of the game or high scores included in the game message. Can be automatically edited to include current high scores for the game when the bot calls [setGameScore](https://core.telegram.org/bots/api#setgamescore), or manually edited using [editMessageText](https://core.telegram.org/bots/api#editmessagetext). 0-4096 characters.
     */
    text: string | null;
    /**
     * Special entities that appear in *text*, such as usernames, URLs, bot commands, etc.
     */
    text_entities: MessageEntity[] | null;
    /**
     * Animation that will be displayed in the game message in chats. Upload via [BotFather](https://t.me/botfather)
     */
    animation: Animation | null;
}
/**
 * A placeholder, currently holds no information. Use [BotFather](https://t.me/botfather) to set up your game.
 */
// deno-lint-ignore no-empty-interface
export interface CallbackGame {}
/**
 * Use this method to set the score of the specified user in a game message. On success, if the message is not an inline message, the [Message](https://core.telegram.org/bots/api#message) is returned, otherwise *True* is returned. Returns an error, if the new score is not greater than the user's current score in the chat and *force* is *False*.
 */
export interface SetGameScoreRequest {
    /**
     * User identifier
     */
    user_id: number;
    /**
     * New score, must be non-negative
     */
    score: number;
    /**
     * Pass *True* if the high score is allowed to decrease. This can be useful when fixing mistakes or banning cheaters
     */
    force: boolean | null;
    /**
     * Pass *True* if the game message should not be automatically edited to include the current scoreboard
     */
    disable_edit_message: boolean | null;
    /**
     * Required if *inline_message_id* is not specified. Unique identifier for the target chat
     */
    chat_id: number | null;
    /**
     * Required if *inline_message_id* is not specified. Identifier of the sent message
     */
    message_id: number | null;
    /**
     * Required if *chat_id* and *message_id* are not specified. Identifier of the inline message
     */
    inline_message_id: string | null;
}
export type SetGameScoreResponse = Message | true;
/**
 * Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. Returns an Array of [GameHighScore](https://core.telegram.org/bots/api#gamehighscore) objects.
 */
export interface GetGameHighScoresRequest {
    /**
     * Target user id
     */
    user_id: number;
    /**
     * Required if *inline_message_id* is not specified. Unique identifier for the target chat
     */
    chat_id: number | null;
    /**
     * Required if *inline_message_id* is not specified. Identifier of the sent message
     */
    message_id: number | null;
    /**
     * Required if *chat_id* and *message_id* are not specified. Identifier of the inline message
     */
    inline_message_id: string | null;
}
export type GetGameHighScoresResponse = GameHighScore[];
/**
 * This object represents one row of the high scores table for a game.
 */
export interface GameHighScore {
    /**
     * Position in high score table for the game
     */
    position: number;
    /**
     * User
     */
    user: User;
    /**
     * Score
     */
    score: number;
}
