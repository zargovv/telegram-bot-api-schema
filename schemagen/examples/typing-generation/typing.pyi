# this file is auto-generated

from enum import Enum
from typing import Literal, TypedDict

class Update(TypedDict):
    update_id: int
    message: Message | None
    edited_message: Message | None
    channel_post: Message | None
    edited_channel_post: Message | None
    business_connection: BusinessConnection | None
    business_message: Message | None
    edited_business_message: Message | None
    deleted_business_messages: BusinessMessagesDeleted | None
    message_reaction: MessageReactionUpdated | None
    message_reaction_count: MessageReactionCountUpdated | None
    inline_query: InlineQuery | None
    chosen_inline_result: ChosenInlineResult | None
    callback_query: CallbackQuery | None
    shipping_query: ShippingQuery | None
    pre_checkout_query: PreCheckoutQuery | None
    purchased_paid_media: PaidMediaPurchased | None
    poll: Poll | None
    poll_answer: PollAnswer | None
    my_chat_member: ChatMemberUpdated | None
    chat_member: ChatMemberUpdated | None
    chat_join_request: ChatJoinRequest | None
    chat_boost: ChatBoostUpdated | None
    removed_chat_boost: ChatBoostRemoved | None
class GetUpdatesRequest(TypedDict):
    offset: int | None
    limit: int | None
    timeout: int | None
    allowed_updates: list[str] | None
type GetUpdatesResponse = list[Update]
class SetWebhookRequest(TypedDict):
    url: str
    certificate: InputFile | None
    ip_address: str | None
    max_connections: int | None
    allowed_updates: list[str] | None
    drop_pending_updates: bool | None
    secret_token: str | None
type SetWebhookResponse = Literal[True]
class DeleteWebhookRequest(TypedDict):
    drop_pending_updates: bool | None
type DeleteWebhookResponse = Literal[True]
class GetWebhookInfoRequest(TypedDict):
    ...
type GetWebhookInfoResponse = WebhookInfo
class WebhookInfo(TypedDict):
    url: str
    has_custom_certificate: bool
    pending_update_count: int
    ip_address: str | None
    last_error_date: int | None
    last_error_message: str | None
    last_synchronization_error_date: int | None
    max_connections: int | None
    allowed_updates: list[str] | None
class User(TypedDict):
    id: int
    is_bot: bool
    first_name: str
    last_name: str | None
    username: str | None
    language_code: str | None
    is_premium: Literal[True] | None
    added_to_attachment_menu: Literal[True] | None
    can_join_groups: bool | None
    can_read_all_group_messages: bool | None
    supports_inline_queries: bool | None
    can_connect_to_business: bool | None
    has_main_web_app: bool | None
    has_topics_enabled: bool | None
    allows_users_to_create_topics: bool | None
class Chat(TypedDict):
    id: int
    type: str
    title: str | None
    username: str | None
    first_name: str | None
    last_name: str | None
    is_forum: Literal[True] | None
    is_direct_messages: Literal[True] | None
class ChatFullInfo(TypedDict):
    id: int
    type: str
    title: str | None
    username: str | None
    first_name: str | None
    last_name: str | None
    is_forum: Literal[True] | None
    is_direct_messages: Literal[True] | None
    accent_color_id: int
    max_reaction_count: int
    photo: ChatPhoto | None
    active_usernames: list[str] | None
    birthdate: Birthdate | None
    business_intro: BusinessIntro | None
    business_location: BusinessLocation | None
    business_opening_hours: BusinessOpeningHours | None
    personal_chat: Chat | None
    parent_chat: Chat | None
    available_reactions: list[ReactionType] | None
    background_custom_emoji_id: str | None
    profile_accent_color_id: int | None
    profile_background_custom_emoji_id: str | None
    emoji_status_custom_emoji_id: str | None
    emoji_status_expiration_date: int | None
    bio: str | None
    has_private_forwards: Literal[True] | None
    has_restricted_voice_and_video_messages: Literal[True] | None
    join_to_send_messages: Literal[True] | None
    join_by_request: Literal[True] | None
    description: str | None
    invite_link: str | None
    pinned_message: Message | None
    permissions: ChatPermissions | None
    accepted_gift_types: AcceptedGiftTypes
    can_send_paid_media: Literal[True] | None
    slow_mode_delay: int | None
    unrestrict_boost_count: int | None
    message_auto_delete_time: int | None
    has_aggressive_anti_spam_enabled: Literal[True] | None
    has_hidden_members: Literal[True] | None
    has_protected_content: Literal[True] | None
    has_visible_history: Literal[True] | None
    sticker_set_name: str | None
    can_set_sticker_set: Literal[True] | None
    custom_emoji_sticker_set_name: str | None
    linked_chat_id: int | None
    location: ChatLocation | None
    rating: UserRating | None
    first_profile_audio: Audio | None
    unique_gift_colors: UniqueGiftColors | None
    paid_message_star_count: int | None
class Message(TypedDict):
    message_id: int
    message_thread_id: int | None
    direct_messages_topic: DirectMessagesTopic | None
    from_: User | None
    sender_chat: Chat | None
    sender_boost_count: int | None
    sender_business_bot: User | None
    date: int
    business_connection_id: str | None
    chat: Chat
    forward_origin: MessageOrigin | None
    is_topic_message: Literal[True] | None
    is_automatic_forward: Literal[True] | None
    reply_to_message: Message | None
    external_reply: ExternalReplyInfo | None
    quote: TextQuote | None
    reply_to_story: Story | None
    reply_to_checklist_task_id: int | None
    via_bot: User | None
    edit_date: int | None
    has_protected_content: Literal[True] | None
    is_from_offline: Literal[True] | None
    is_paid_post: Literal[True] | None
    media_group_id: str | None
    author_signature: str | None
    paid_star_count: int | None
    text: str | None
    entities: list[MessageEntity] | None
    link_preview_options: LinkPreviewOptions | None
    suggested_post_info: SuggestedPostInfo | None
    effect_id: str | None
    animation: Animation | None
    audio: Audio | None
    document: Document | None
    paid_media: PaidMediaInfo | None
    photo: list[PhotoSize] | None
    sticker: Sticker | None
    story: Story | None
    video: Video | None
    video_note: VideoNote | None
    voice: Voice | None
    caption: str | None
    caption_entities: list[MessageEntity] | None
    show_caption_above_media: Literal[True] | None
    has_media_spoiler: Literal[True] | None
    checklist: Checklist | None
    contact: Contact | None
    dice: Dice | None
    game: Game | None
    poll: Poll | None
    venue: Venue | None
    location: Location | None
    new_chat_members: list[User] | None
    left_chat_member: User | None
    chat_owner_left: ChatOwnerLeft | None
    chat_owner_changed: ChatOwnerChanged | None
    new_chat_title: str | None
    new_chat_photo: list[PhotoSize] | None
    delete_chat_photo: Literal[True] | None
    group_chat_created: Literal[True] | None
    supergroup_chat_created: Literal[True] | None
    channel_chat_created: Literal[True] | None
    message_auto_delete_timer_changed: MessageAutoDeleteTimerChanged | None
    migrate_to_chat_id: int | None
    migrate_from_chat_id: int | None
    pinned_message: MaybeInaccessibleMessage | None
    invoice: Invoice | None
    successful_payment: SuccessfulPayment | None
    refunded_payment: RefundedPayment | None
    users_shared: UsersShared | None
    chat_shared: ChatShared | None
    gift: GiftInfo | None
    unique_gift: UniqueGiftInfo | None
    gift_upgrade_sent: GiftInfo | None
    connected_website: str | None
    write_access_allowed: WriteAccessAllowed | None
    passport_data: PassportData | None
    proximity_alert_triggered: ProximityAlertTriggered | None
    boost_added: ChatBoostAdded | None
    chat_background_set: ChatBackground | None
    checklist_tasks_done: ChecklistTasksDone | None
    checklist_tasks_added: ChecklistTasksAdded | None
    direct_message_price_changed: DirectMessagePriceChanged | None
    forum_topic_created: ForumTopicCreated | None
    forum_topic_edited: ForumTopicEdited | None
    forum_topic_closed: ForumTopicClosed | None
    forum_topic_reopened: ForumTopicReopened | None
    general_forum_topic_hidden: GeneralForumTopicHidden | None
    general_forum_topic_unhidden: GeneralForumTopicUnhidden | None
    giveaway_created: GiveawayCreated | None
    giveaway: Giveaway | None
    giveaway_winners: GiveawayWinners | None
    giveaway_completed: GiveawayCompleted | None
    paid_message_price_changed: PaidMessagePriceChanged | None
    suggested_post_approved: SuggestedPostApproved | None
    suggested_post_approval_failed: SuggestedPostApprovalFailed | None
    suggested_post_declined: SuggestedPostDeclined | None
    suggested_post_paid: SuggestedPostPaid | None
    suggested_post_refunded: SuggestedPostRefunded | None
    video_chat_scheduled: VideoChatScheduled | None
    video_chat_started: VideoChatStarted | None
    video_chat_ended: VideoChatEnded | None
    video_chat_participants_invited: VideoChatParticipantsInvited | None
    web_app_data: WebAppData | None
    reply_markup: InlineKeyboardMarkup | None
class MessageId(TypedDict):
    message_id: int
class InaccessibleMessage(TypedDict):
    chat: Chat
    message_id: int
    date: int
type MaybeInaccessibleMessage = Message | InaccessibleMessage
class MessageEntity(TypedDict):
    type: str
    offset: int
    length: int
    url: str | None
    user: User | None
    language: str | None
    custom_emoji_id: str | None
class TextQuote(TypedDict):
    text: str
    entities: list[MessageEntity] | None
    position: int
    is_manual: Literal[True] | None
class ExternalReplyInfo(TypedDict):
    origin: MessageOrigin
    chat: Chat | None
    message_id: int | None
    link_preview_options: LinkPreviewOptions | None
    animation: Animation | None
    audio: Audio | None
    document: Document | None
    paid_media: PaidMediaInfo | None
    photo: list[PhotoSize] | None
    sticker: Sticker | None
    story: Story | None
    video: Video | None
    video_note: VideoNote | None
    voice: Voice | None
    has_media_spoiler: Literal[True] | None
    checklist: Checklist | None
    contact: Contact | None
    dice: Dice | None
    game: Game | None
    giveaway: Giveaway | None
    giveaway_winners: GiveawayWinners | None
    invoice: Invoice | None
    location: Location | None
    poll: Poll | None
    venue: Venue | None
class ReplyParameters(TypedDict):
    message_id: int
    chat_id: int | str | None
    allow_sending_without_reply: bool | None
    quote: str | None
    quote_parse_mode: str | None
    quote_entities: list[MessageEntity] | None
    quote_position: int | None
    checklist_task_id: int | None
type MessageOrigin = MessageOriginUser | MessageOriginHiddenUser | MessageOriginChat | MessageOriginChannel
class MessageOriginUser(TypedDict):
    type: str
    date: int
    sender_user: User
class MessageOriginHiddenUser(TypedDict):
    type: str
    date: int
    sender_user_name: str
class MessageOriginChat(TypedDict):
    type: str
    date: int
    sender_chat: Chat
    author_signature: str | None
class MessageOriginChannel(TypedDict):
    type: str
    date: int
    chat: Chat
    message_id: int
    author_signature: str | None
class PhotoSize(TypedDict):
    file_id: str
    file_unique_id: str
    width: int
    height: int
    file_size: int | None
class Animation(TypedDict):
    file_id: str
    file_unique_id: str
    width: int
    height: int
    duration: int
    thumbnail: PhotoSize | None
    file_name: str | None
    mime_type: str | None
    file_size: int | None
class Audio(TypedDict):
    file_id: str
    file_unique_id: str
    duration: int
    performer: str | None
    title: str | None
    file_name: str | None
    mime_type: str | None
    file_size: int | None
    thumbnail: PhotoSize | None
class Document(TypedDict):
    file_id: str
    file_unique_id: str
    thumbnail: PhotoSize | None
    file_name: str | None
    mime_type: str | None
    file_size: int | None
class Story(TypedDict):
    chat: Chat
    id: int
class VideoQuality(TypedDict):
    file_id: str
    file_unique_id: str
    width: int
    height: int
    codec: str
    file_size: int | None
class Video(TypedDict):
    file_id: str
    file_unique_id: str
    width: int
    height: int
    duration: int
    thumbnail: PhotoSize | None
    cover: list[PhotoSize] | None
    start_timestamp: int | None
    qualities: list[VideoQuality] | None
    file_name: str | None
    mime_type: str | None
    file_size: int | None
class VideoNote(TypedDict):
    file_id: str
    file_unique_id: str
    length: int
    duration: int
    thumbnail: PhotoSize | None
    file_size: int | None
class Voice(TypedDict):
    file_id: str
    file_unique_id: str
    duration: int
    mime_type: str | None
    file_size: int | None
class PaidMediaInfo(TypedDict):
    star_count: int
    paid_media: list[PaidMedia]
type PaidMedia = PaidMediaPreview | PaidMediaPhoto | PaidMediaVideo
class PaidMediaPreview(TypedDict):
    type: str
    width: int | None
    height: int | None
    duration: int | None
class PaidMediaPhoto(TypedDict):
    type: str
    photo: list[PhotoSize]
class PaidMediaVideo(TypedDict):
    type: str
    video: Video
class Contact(TypedDict):
    phone_number: str
    first_name: str
    last_name: str | None
    user_id: int | None
    vcard: str | None
class Dice(TypedDict):
    emoji: str
    value: int
class PollOption(TypedDict):
    text: str
    text_entities: list[MessageEntity] | None
    voter_count: int
class InputPollOption(TypedDict):
    text: str
    text_parse_mode: str | None
    text_entities: list[MessageEntity] | None
class PollAnswer(TypedDict):
    poll_id: str
    voter_chat: Chat | None
    user: User | None
    option_ids: list[int]
class Poll(TypedDict):
    id: str
    question: str
    question_entities: list[MessageEntity] | None
    options: list[PollOption]
    total_voter_count: int
    is_closed: bool
    is_anonymous: bool
    type: str
    allows_multiple_answers: bool
    correct_option_id: int | None
    explanation: str | None
    explanation_entities: list[MessageEntity] | None
    open_period: int | None
    close_date: int | None
class ChecklistTask(TypedDict):
    id: int
    text: str
    text_entities: list[MessageEntity] | None
    completed_by_user: User | None
    completed_by_chat: Chat | None
    completion_date: int | None
class Checklist(TypedDict):
    title: str
    title_entities: list[MessageEntity] | None
    tasks: list[ChecklistTask]
    others_can_add_tasks: Literal[True] | None
    others_can_mark_tasks_as_done: Literal[True] | None
class InputChecklistTask(TypedDict):
    id: int
    text: str
    parse_mode: str | None
    text_entities: list[MessageEntity] | None
class InputChecklist(TypedDict):
    title: str
    parse_mode: str | None
    title_entities: list[MessageEntity] | None
    tasks: list[InputChecklistTask]
    others_can_add_tasks: bool | None
    others_can_mark_tasks_as_done: bool | None
class ChecklistTasksDone(TypedDict):
    checklist_message: Message | None
    marked_as_done_task_ids: list[int] | None
    marked_as_not_done_task_ids: list[int] | None
class ChecklistTasksAdded(TypedDict):
    checklist_message: Message | None
    tasks: list[ChecklistTask]
class Location(TypedDict):
    latitude: float
    longitude: float
    horizontal_accuracy: float | None
    live_period: int | None
    heading: int | None
    proximity_alert_radius: int | None
class Venue(TypedDict):
    location: Location
    title: str
    address: str
    foursquare_id: str | None
    foursquare_type: str | None
    google_place_id: str | None
    google_place_type: str | None
class WebAppData(TypedDict):
    data: str
    button_text: str
class ProximityAlertTriggered(TypedDict):
    traveler: User
    watcher: User
    distance: int
class MessageAutoDeleteTimerChanged(TypedDict):
    message_auto_delete_time: int
class ChatBoostAdded(TypedDict):
    boost_count: int
type BackgroundFill = BackgroundFillSolid | BackgroundFillGradient | BackgroundFillFreeformGradient
class BackgroundFillSolid(TypedDict):
    type: str
    color: int
class BackgroundFillGradient(TypedDict):
    type: str
    top_color: int
    bottom_color: int
    rotation_angle: int
class BackgroundFillFreeformGradient(TypedDict):
    type: str
    colors: list[int]
type BackgroundType = BackgroundTypeFill | BackgroundTypeWallpaper | BackgroundTypePattern | BackgroundTypeChatTheme
class BackgroundTypeFill(TypedDict):
    type: str
    fill: BackgroundFill
    dark_theme_dimming: int
class BackgroundTypeWallpaper(TypedDict):
    type: str
    document: Document
    dark_theme_dimming: int
    is_blurred: Literal[True] | None
    is_moving: Literal[True] | None
class BackgroundTypePattern(TypedDict):
    type: str
    document: Document
    fill: BackgroundFill
    intensity: int
    is_inverted: Literal[True] | None
    is_moving: Literal[True] | None
class BackgroundTypeChatTheme(TypedDict):
    type: str
    theme_name: str
class ChatBackground(TypedDict):
    type: BackgroundType
class ForumTopicCreated(TypedDict):
    name: str
    icon_color: int
    icon_custom_emoji_id: str | None
    is_name_implicit: Literal[True] | None
class ForumTopicClosed(TypedDict):
    ...
class ForumTopicEdited(TypedDict):
    name: str | None
    icon_custom_emoji_id: str | None
class ForumTopicReopened(TypedDict):
    ...
class GeneralForumTopicHidden(TypedDict):
    ...
class GeneralForumTopicUnhidden(TypedDict):
    ...
class SharedUser(TypedDict):
    user_id: int
    first_name: str | None
    last_name: str | None
    username: str | None
    photo: list[PhotoSize] | None
class UsersShared(TypedDict):
    request_id: int
    users: list[SharedUser]
class ChatShared(TypedDict):
    request_id: int
    chat_id: int
    title: str | None
    username: str | None
    photo: list[PhotoSize] | None
class WriteAccessAllowed(TypedDict):
    from_request: bool | None
    web_app_name: str | None
    from_attachment_menu: bool | None
class VideoChatScheduled(TypedDict):
    start_date: int
class VideoChatStarted(TypedDict):
    ...
class VideoChatEnded(TypedDict):
    duration: int
class VideoChatParticipantsInvited(TypedDict):
    users: list[User]
class PaidMessagePriceChanged(TypedDict):
    paid_message_star_count: int
class DirectMessagePriceChanged(TypedDict):
    are_direct_messages_enabled: bool
    direct_message_star_count: int | None
class SuggestedPostApproved(TypedDict):
    suggested_post_message: Message | None
    price: SuggestedPostPrice | None
    send_date: int
class SuggestedPostApprovalFailed(TypedDict):
    suggested_post_message: Message | None
    price: SuggestedPostPrice
class SuggestedPostDeclined(TypedDict):
    suggested_post_message: Message | None
    comment: str | None
class SuggestedPostPaid(TypedDict):
    suggested_post_message: Message | None
    currency: str
    amount: int | None
    star_amount: StarAmount | None
class SuggestedPostRefunded(TypedDict):
    suggested_post_message: Message | None
    reason: str
class GiveawayCreated(TypedDict):
    prize_star_count: int | None
class Giveaway(TypedDict):
    chats: list[Chat]
    winners_selection_date: int
    winner_count: int
    only_new_members: Literal[True] | None
    has_public_winners: Literal[True] | None
    prize_description: str | None
    country_codes: list[str] | None
    prize_star_count: int | None
    premium_subscription_month_count: int | None
class GiveawayWinners(TypedDict):
    chat: Chat
    giveaway_message_id: int
    winners_selection_date: int
    winner_count: int
    winners: list[User]
    additional_chat_count: int | None
    prize_star_count: int | None
    premium_subscription_month_count: int | None
    unclaimed_prize_count: int | None
    only_new_members: Literal[True] | None
    was_refunded: Literal[True] | None
    prize_description: str | None
class GiveawayCompleted(TypedDict):
    winner_count: int
    unclaimed_prize_count: int | None
    giveaway_message: Message | None
    is_star_giveaway: Literal[True] | None
class LinkPreviewOptions(TypedDict):
    is_disabled: bool | None
    url: str | None
    prefer_small_media: bool | None
    prefer_large_media: bool | None
    show_above_text: bool | None
class SuggestedPostPrice(TypedDict):
    currency: str
    amount: int
class SuggestedPostInfo(TypedDict):
    state: str
    price: SuggestedPostPrice | None
    send_date: int | None
class SuggestedPostParameters(TypedDict):
    price: SuggestedPostPrice | None
    send_date: int | None
class DirectMessagesTopic(TypedDict):
    topic_id: int
    user: User | None
class UserProfilePhotos(TypedDict):
    total_count: int
    photos: list[list[PhotoSize]]
class UserProfileAudios(TypedDict):
    total_count: int
    audios: list[Audio]
class File(TypedDict):
    file_id: str
    file_unique_id: str
    file_size: int | None
    file_path: str | None
class WebAppInfo(TypedDict):
    url: str
class ReplyKeyboardMarkup(TypedDict):
    keyboard: list[list[KeyboardButton]]
    is_persistent: bool | None
    resize_keyboard: bool | None
    one_time_keyboard: bool | None
    input_field_placeholder: str | None
    selective: bool | None
class KeyboardButton(TypedDict):
    text: str
    icon_custom_emoji_id: str | None
    style: str | None
    request_users: KeyboardButtonRequestUsers | None
    request_chat: KeyboardButtonRequestChat | None
    request_contact: bool | None
    request_location: bool | None
    request_poll: KeyboardButtonPollType | None
    web_app: WebAppInfo | None
class KeyboardButtonRequestUsers(TypedDict):
    request_id: int
    user_is_bot: bool | None
    user_is_premium: bool | None
    max_quantity: int | None
    request_name: bool | None
    request_username: bool | None
    request_photo: bool | None
class KeyboardButtonRequestChat(TypedDict):
    request_id: int
    chat_is_channel: bool
    chat_is_forum: bool | None
    chat_has_username: bool | None
    chat_is_created: bool | None
    user_administrator_rights: ChatAdministratorRights | None
    bot_administrator_rights: ChatAdministratorRights | None
    bot_is_member: bool | None
    request_title: bool | None
    request_username: bool | None
    request_photo: bool | None
class KeyboardButtonPollType(TypedDict):
    type: str | None
class ReplyKeyboardRemove(TypedDict):
    remove_keyboard: Literal[True]
    selective: bool | None
class InlineKeyboardMarkup(TypedDict):
    inline_keyboard: list[list[InlineKeyboardButton]]
class InlineKeyboardButton(TypedDict):
    text: str
    icon_custom_emoji_id: str | None
    style: str | None
    url: str | None
    callback_data: str | None
    web_app: WebAppInfo | None
    login_url: LoginUrl | None
    switch_inline_query: str | None
    switch_inline_query_current_chat: str | None
    switch_inline_query_chosen_chat: SwitchInlineQueryChosenChat | None
    copy_text: CopyTextButton | None
    callback_game: CallbackGame | None
    pay: bool | None
class LoginUrl(TypedDict):
    url: str
    forward_text: str | None
    bot_username: str | None
    request_write_access: bool | None
class SwitchInlineQueryChosenChat(TypedDict):
    query: str | None
    allow_user_chats: bool | None
    allow_bot_chats: bool | None
    allow_group_chats: bool | None
    allow_channel_chats: bool | None
class CopyTextButton(TypedDict):
    text: str
class CallbackQuery(TypedDict):
    id: str
    from_: User
    message: MaybeInaccessibleMessage | None
    inline_message_id: str | None
    chat_instance: str
    data: str | None
    game_short_name: str | None
class ForceReply(TypedDict):
    force_reply: Literal[True]
    input_field_placeholder: str | None
    selective: bool | None
class ChatPhoto(TypedDict):
    small_file_id: str
    small_file_unique_id: str
    big_file_id: str
    big_file_unique_id: str
class ChatInviteLink(TypedDict):
    invite_link: str
    creator: User
    creates_join_request: bool
    is_primary: bool
    is_revoked: bool
    name: str | None
    expire_date: int | None
    member_limit: int | None
    pending_join_request_count: int | None
    subscription_period: int | None
    subscription_price: int | None
class ChatAdministratorRights(TypedDict):
    is_anonymous: bool
    can_manage_chat: bool
    can_delete_messages: bool
    can_manage_video_chats: bool
    can_restrict_members: bool
    can_promote_members: bool
    can_change_info: bool
    can_invite_users: bool
    can_post_stories: bool
    can_edit_stories: bool
    can_delete_stories: bool
    can_post_messages: bool | None
    can_edit_messages: bool | None
    can_pin_messages: bool | None
    can_manage_topics: bool | None
    can_manage_direct_messages: bool | None
class ChatMemberUpdated(TypedDict):
    chat: Chat
    from_: User
    date: int
    old_chat_member: ChatMember
    new_chat_member: ChatMember
    invite_link: ChatInviteLink | None
    via_join_request: bool | None
    via_chat_folder_invite_link: bool | None
type ChatMember = ChatMemberOwner | ChatMemberAdministrator | ChatMemberMember | ChatMemberRestricted | ChatMemberLeft | ChatMemberBanned
class ChatMemberOwner(TypedDict):
    status: str
    user: User
    is_anonymous: bool
    custom_title: str | None
class ChatMemberAdministrator(TypedDict):
    status: str
    user: User
    can_be_edited: bool
    is_anonymous: bool
    can_manage_chat: bool
    can_delete_messages: bool
    can_manage_video_chats: bool
    can_restrict_members: bool
    can_promote_members: bool
    can_change_info: bool
    can_invite_users: bool
    can_post_stories: bool
    can_edit_stories: bool
    can_delete_stories: bool
    can_post_messages: bool | None
    can_edit_messages: bool | None
    can_pin_messages: bool | None
    can_manage_topics: bool | None
    can_manage_direct_messages: bool | None
    custom_title: str | None
class ChatMemberMember(TypedDict):
    status: str
    user: User
    until_date: int | None
class ChatMemberRestricted(TypedDict):
    status: str
    user: User
    is_member: bool
    can_send_messages: bool
    can_send_audios: bool
    can_send_documents: bool
    can_send_photos: bool
    can_send_videos: bool
    can_send_video_notes: bool
    can_send_voice_notes: bool
    can_send_polls: bool
    can_send_other_messages: bool
    can_add_web_page_previews: bool
    can_change_info: bool
    can_invite_users: bool
    can_pin_messages: bool
    can_manage_topics: bool
    until_date: int
class ChatMemberLeft(TypedDict):
    status: str
    user: User
class ChatMemberBanned(TypedDict):
    status: str
    user: User
    until_date: int
class ChatJoinRequest(TypedDict):
    chat: Chat
    from_: User
    user_chat_id: int
    date: int
    bio: str | None
    invite_link: ChatInviteLink | None
class ChatPermissions(TypedDict):
    can_send_messages: bool | None
    can_send_audios: bool | None
    can_send_documents: bool | None
    can_send_photos: bool | None
    can_send_videos: bool | None
    can_send_video_notes: bool | None
    can_send_voice_notes: bool | None
    can_send_polls: bool | None
    can_send_other_messages: bool | None
    can_add_web_page_previews: bool | None
    can_change_info: bool | None
    can_invite_users: bool | None
    can_pin_messages: bool | None
    can_manage_topics: bool | None
class Birthdate(TypedDict):
    day: int
    month: int
    year: int | None
class BusinessIntro(TypedDict):
    title: str | None
    message: str | None
    sticker: Sticker | None
class BusinessLocation(TypedDict):
    address: str
    location: Location | None
class BusinessOpeningHoursInterval(TypedDict):
    opening_minute: int
    closing_minute: int
class BusinessOpeningHours(TypedDict):
    time_zone_name: str
    opening_hours: list[BusinessOpeningHoursInterval]
class UserRating(TypedDict):
    level: int
    rating: int
    current_level_rating: int
    next_level_rating: int | None
class StoryAreaPosition(TypedDict):
    x_percentage: float
    y_percentage: float
    width_percentage: float
    height_percentage: float
    rotation_angle: float
    corner_radius_percentage: float
class LocationAddress(TypedDict):
    country_code: str
    state: str | None
    city: str | None
    street: str | None
type StoryAreaType = StoryAreaTypeLocation | StoryAreaTypeSuggestedReaction | StoryAreaTypeLink | StoryAreaTypeWeather | StoryAreaTypeUniqueGift
class StoryAreaTypeLocation(TypedDict):
    type: str
    latitude: float
    longitude: float
    address: LocationAddress | None
class StoryAreaTypeSuggestedReaction(TypedDict):
    type: str
    reaction_type: ReactionType
    is_dark: bool | None
    is_flipped: bool | None
class StoryAreaTypeLink(TypedDict):
    type: str
    url: str
class StoryAreaTypeWeather(TypedDict):
    type: str
    temperature: float
    emoji: str
    background_color: int
class StoryAreaTypeUniqueGift(TypedDict):
    type: str
    name: str
class StoryArea(TypedDict):
    position: StoryAreaPosition
    type: StoryAreaType
class ChatLocation(TypedDict):
    location: Location
    address: str
type ReactionType = ReactionTypeEmoji | ReactionTypeCustomEmoji | ReactionTypePaid
class ReactionTypeEmoji(TypedDict):
    type: str
    emoji: str
class ReactionTypeCustomEmoji(TypedDict):
    type: str
    custom_emoji_id: str
class ReactionTypePaid(TypedDict):
    type: str
class ReactionCount(TypedDict):
    type: ReactionType
    total_count: int
class MessageReactionUpdated(TypedDict):
    chat: Chat
    message_id: int
    user: User | None
    actor_chat: Chat | None
    date: int
    old_reaction: list[ReactionType]
    new_reaction: list[ReactionType]
class MessageReactionCountUpdated(TypedDict):
    chat: Chat
    message_id: int
    date: int
    reactions: list[ReactionCount]
class ForumTopic(TypedDict):
    message_thread_id: int
    name: str
    icon_color: int
    icon_custom_emoji_id: str | None
    is_name_implicit: Literal[True] | None
class GiftBackground(TypedDict):
    center_color: int
    edge_color: int
    text_color: int
class Gift(TypedDict):
    id: str
    sticker: Sticker
    star_count: int
    upgrade_star_count: int | None
    is_premium: Literal[True] | None
    has_colors: Literal[True] | None
    total_count: int | None
    remaining_count: int | None
    personal_total_count: int | None
    personal_remaining_count: int | None
    background: GiftBackground | None
    unique_gift_variant_count: int | None
    publisher_chat: Chat | None
class Gifts(TypedDict):
    gifts: list[Gift]
class UniqueGiftModel(TypedDict):
    name: str
    sticker: Sticker
    rarity_per_mille: int
    rarity: str | None
class UniqueGiftSymbol(TypedDict):
    name: str
    sticker: Sticker
    rarity_per_mille: int
class UniqueGiftBackdropColors(TypedDict):
    center_color: int
    edge_color: int
    symbol_color: int
    text_color: int
class UniqueGiftBackdrop(TypedDict):
    name: str
    colors: UniqueGiftBackdropColors
    rarity_per_mille: int
class UniqueGiftColors(TypedDict):
    model_custom_emoji_id: str
    symbol_custom_emoji_id: str
    light_theme_main_color: int
    light_theme_other_colors: list[int]
    dark_theme_main_color: int
    dark_theme_other_colors: list[int]
class UniqueGift(TypedDict):
    gift_id: str
    base_name: str
    name: str
    number: int
    model: UniqueGiftModel
    symbol: UniqueGiftSymbol
    backdrop: UniqueGiftBackdrop
    is_premium: Literal[True] | None
    is_burned: Literal[True] | None
    is_from_blockchain: Literal[True] | None
    colors: UniqueGiftColors | None
    publisher_chat: Chat | None
class GiftInfo(TypedDict):
    gift: Gift
    owned_gift_id: str | None
    convert_star_count: int | None
    prepaid_upgrade_star_count: int | None
    is_upgrade_separate: Literal[True] | None
    can_be_upgraded: Literal[True] | None
    text: str | None
    entities: list[MessageEntity] | None
    is_private: Literal[True] | None
    unique_gift_number: int | None
class UniqueGiftInfo(TypedDict):
    gift: UniqueGift
    origin: str
    last_resale_currency: str | None
    last_resale_amount: int | None
    owned_gift_id: str | None
    transfer_star_count: int | None
    next_transfer_date: int | None
type OwnedGift = OwnedGiftRegular | OwnedGiftUnique
class OwnedGiftRegular(TypedDict):
    type: str
    gift: Gift
    owned_gift_id: str | None
    sender_user: User | None
    send_date: int
    text: str | None
    entities: list[MessageEntity] | None
    is_private: Literal[True] | None
    is_saved: Literal[True] | None
    can_be_upgraded: Literal[True] | None
    was_refunded: Literal[True] | None
    convert_star_count: int | None
    prepaid_upgrade_star_count: int | None
    is_upgrade_separate: Literal[True] | None
    unique_gift_number: int | None
class OwnedGiftUnique(TypedDict):
    type: str
    gift: UniqueGift
    owned_gift_id: str | None
    sender_user: User | None
    send_date: int
    is_saved: Literal[True] | None
    can_be_transferred: Literal[True] | None
    transfer_star_count: int | None
    next_transfer_date: int | None
class OwnedGifts(TypedDict):
    total_count: int
    gifts: list[OwnedGift]
    next_offset: str | None
class AcceptedGiftTypes(TypedDict):
    unlimited_gifts: bool
    limited_gifts: bool
    unique_gifts: bool
    premium_subscription: bool
    gifts_from_channels: bool
class StarAmount(TypedDict):
    amount: int
    nanostar_amount: int | None
class BotCommand(TypedDict):
    command: str
    description: str
type BotCommandScope = BotCommandScopeDefault | BotCommandScopeAllPrivateChats | BotCommandScopeAllGroupChats | BotCommandScopeAllChatAdministrators | BotCommandScopeChat | BotCommandScopeChatAdministrators | BotCommandScopeChatMember
class BotCommandScopeDefault(TypedDict):
    type: str
class BotCommandScopeAllPrivateChats(TypedDict):
    type: str
class BotCommandScopeAllGroupChats(TypedDict):
    type: str
class BotCommandScopeAllChatAdministrators(TypedDict):
    type: str
class BotCommandScopeChat(TypedDict):
    type: str
    chat_id: int | str
class BotCommandScopeChatAdministrators(TypedDict):
    type: str
    chat_id: int | str
class BotCommandScopeChatMember(TypedDict):
    type: str
    chat_id: int | str
    user_id: int
class BotName(TypedDict):
    name: str
class BotDescription(TypedDict):
    description: str
class BotShortDescription(TypedDict):
    short_description: str
type MenuButton = MenuButtonCommands | MenuButtonWebApp | MenuButtonDefault
class MenuButtonCommands(TypedDict):
    type: str
class MenuButtonWebApp(TypedDict):
    type: str
    text: str
    web_app: WebAppInfo
class MenuButtonDefault(TypedDict):
    type: str
type ChatBoostSource = ChatBoostSourcePremium | ChatBoostSourceGiftCode | ChatBoostSourceGiveaway
class ChatBoostSourcePremium(TypedDict):
    source: str
    user: User
class ChatBoostSourceGiftCode(TypedDict):
    source: str
    user: User
class ChatBoostSourceGiveaway(TypedDict):
    source: str
    giveaway_message_id: int
    user: User | None
    prize_star_count: int | None
    is_unclaimed: Literal[True] | None
class ChatBoost(TypedDict):
    boost_id: str
    add_date: int
    expiration_date: int
    source: ChatBoostSource
class ChatBoostUpdated(TypedDict):
    chat: Chat
    boost: ChatBoost
class ChatBoostRemoved(TypedDict):
    chat: Chat
    boost_id: str
    remove_date: int
    source: ChatBoostSource
class ChatOwnerLeft(TypedDict):
    new_owner: User | None
class ChatOwnerChanged(TypedDict):
    new_owner: User
class UserChatBoosts(TypedDict):
    boosts: list[ChatBoost]
class BusinessBotRights(TypedDict):
    can_reply: Literal[True] | None
    can_read_messages: Literal[True] | None
    can_delete_sent_messages: Literal[True] | None
    can_delete_all_messages: Literal[True] | None
    can_edit_name: Literal[True] | None
    can_edit_bio: Literal[True] | None
    can_edit_profile_photo: Literal[True] | None
    can_edit_username: Literal[True] | None
    can_change_gift_settings: Literal[True] | None
    can_view_gifts_and_stars: Literal[True] | None
    can_convert_gifts_to_stars: Literal[True] | None
    can_transfer_and_upgrade_gifts: Literal[True] | None
    can_transfer_stars: Literal[True] | None
    can_manage_stories: Literal[True] | None
class BusinessConnection(TypedDict):
    id: str
    user: User
    user_chat_id: int
    date: int
    rights: BusinessBotRights | None
    is_enabled: bool
class BusinessMessagesDeleted(TypedDict):
    business_connection_id: str
    chat: Chat
    message_ids: list[int]
class ResponseParameters(TypedDict):
    migrate_to_chat_id: int | None
    retry_after: int | None
type InputMedia = InputMediaAnimation | InputMediaDocument | InputMediaAudio | InputMediaPhoto | InputMediaVideo
class InputMediaPhoto(TypedDict):
    type: str
    media: str
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    show_caption_above_media: bool | None
    has_spoiler: bool | None
class InputMediaVideo(TypedDict):
    type: str
    media: str
    thumbnail: str | None
    cover: str | None
    start_timestamp: int | None
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    show_caption_above_media: bool | None
    width: int | None
    height: int | None
    duration: int | None
    supports_streaming: bool | None
    has_spoiler: bool | None
class InputMediaAnimation(TypedDict):
    type: str
    media: str
    thumbnail: str | None
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    show_caption_above_media: bool | None
    width: int | None
    height: int | None
    duration: int | None
    has_spoiler: bool | None
class InputMediaAudio(TypedDict):
    type: str
    media: str
    thumbnail: str | None
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    duration: int | None
    performer: str | None
    title: str | None
class InputMediaDocument(TypedDict):
    type: str
    media: str
    thumbnail: str | None
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    disable_content_type_detection: bool | None
class InputFile(TypedDict):
    ...
type InputPaidMedia = InputPaidMediaPhoto | InputPaidMediaVideo
class InputPaidMediaPhoto(TypedDict):
    type: str
    media: str
class InputPaidMediaVideo(TypedDict):
    type: str
    media: str
    thumbnail: str | None
    cover: str | None
    start_timestamp: int | None
    width: int | None
    height: int | None
    duration: int | None
    supports_streaming: bool | None
type InputProfilePhoto = InputProfilePhotoStatic | InputProfilePhotoAnimated
class InputProfilePhotoStatic(TypedDict):
    type: str
    photo: str
class InputProfilePhotoAnimated(TypedDict):
    type: str
    animation: str
    main_frame_timestamp: float | None
type InputStoryContent = InputStoryContentPhoto | InputStoryContentVideo
class InputStoryContentPhoto(TypedDict):
    type: str
    photo: str
class InputStoryContentVideo(TypedDict):
    type: str
    video: str
    duration: float | None
    cover_frame_timestamp: float | None
    is_animation: bool | None
class AccentColors(Enum):
    _7 = 7
    _8 = 8
    _9 = 9
    _10 = 10
    _11 = 11
    _12 = 12
    _13 = 13
    _14 = 14
    _15 = 15
    _16 = 16
    _17 = 17
    _18 = 18
    _19 = 19
    _20 = 20
class ProfileAccentColors(Enum):
    _0 = 0
    _1 = 1
    _2 = 2
    _3 = 3
    _4 = 4
    _5 = 5
    _6 = 6
    _7 = 7
    _8 = 8
    _9 = 9
    _10 = 10
    _11 = 11
    _12 = 12
    _13 = 13
    _14 = 14
    _15 = 15
class GetMeRequest(TypedDict):
    ...
type GetMeResponse = User
class LogOutRequest(TypedDict):
    ...
type LogOutResponse = Literal[True]
class CloseRequest(TypedDict):
    ...
type CloseResponse = Literal[True]
class SendMessageRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str
    message_thread_id: int | None
    direct_messages_topic_id: int | None
    text: str
    parse_mode: str | None
    entities: list[MessageEntity] | None
    link_preview_options: LinkPreviewOptions | None
    disable_notification: bool | None
    protect_content: bool | None
    allow_paid_broadcast: bool | None
    message_effect_id: str | None
    suggested_post_parameters: SuggestedPostParameters | None
    reply_parameters: ReplyParameters | None
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | None
type SendMessageResponse = Message
class ForwardMessageRequest(TypedDict):
    chat_id: int | str
    message_thread_id: int | None
    direct_messages_topic_id: int | None
    from_chat_id: int | str
    video_start_timestamp: int | None
    disable_notification: bool | None
    protect_content: bool | None
    message_effect_id: str | None
    suggested_post_parameters: SuggestedPostParameters | None
    message_id: int
type ForwardMessageResponse = Message
class ForwardMessagesRequest(TypedDict):
    chat_id: int | str
    message_thread_id: int | None
    direct_messages_topic_id: int | None
    from_chat_id: int | str
    message_ids: list[int]
    disable_notification: bool | None
    protect_content: bool | None
type ForwardMessagesResponse = MessageId
class CopyMessageRequest(TypedDict):
    chat_id: int | str
    message_thread_id: int | None
    direct_messages_topic_id: int | None
    from_chat_id: int | str
    message_id: int
    video_start_timestamp: int | None
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    show_caption_above_media: bool | None
    disable_notification: bool | None
    protect_content: bool | None
    allow_paid_broadcast: bool | None
    message_effect_id: str | None
    suggested_post_parameters: SuggestedPostParameters | None
    reply_parameters: ReplyParameters | None
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | None
type CopyMessageResponse = MessageId
class CopyMessagesRequest(TypedDict):
    chat_id: int | str
    message_thread_id: int | None
    direct_messages_topic_id: int | None
    from_chat_id: int | str
    message_ids: list[int]
    disable_notification: bool | None
    protect_content: bool | None
    remove_caption: bool | None
type CopyMessagesResponse = MessageId
class SendPhotoRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str
    message_thread_id: int | None
    direct_messages_topic_id: int | None
    photo: InputFile | str
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    show_caption_above_media: bool | None
    has_spoiler: bool | None
    disable_notification: bool | None
    protect_content: bool | None
    allow_paid_broadcast: bool | None
    message_effect_id: str | None
    suggested_post_parameters: SuggestedPostParameters | None
    reply_parameters: ReplyParameters | None
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | None
type SendPhotoResponse = Message
class SendAudioRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str
    message_thread_id: int | None
    direct_messages_topic_id: int | None
    audio: InputFile | str
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    duration: int | None
    performer: str | None
    title: str | None
    thumbnail: InputFile | str | None
    disable_notification: bool | None
    protect_content: bool | None
    allow_paid_broadcast: bool | None
    message_effect_id: str | None
    suggested_post_parameters: SuggestedPostParameters | None
    reply_parameters: ReplyParameters | None
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | None
type SendAudioResponse = Message
class SendDocumentRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str
    message_thread_id: int | None
    direct_messages_topic_id: int | None
    document: InputFile | str
    thumbnail: InputFile | str | None
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    disable_content_type_detection: bool | None
    disable_notification: bool | None
    protect_content: bool | None
    allow_paid_broadcast: bool | None
    message_effect_id: str | None
    suggested_post_parameters: SuggestedPostParameters | None
    reply_parameters: ReplyParameters | None
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | None
type SendDocumentResponse = Message
class SendVideoRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str
    message_thread_id: int | None
    direct_messages_topic_id: int | None
    video: InputFile | str
    duration: int | None
    width: int | None
    height: int | None
    thumbnail: InputFile | str | None
    cover: InputFile | str | None
    start_timestamp: int | None
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    show_caption_above_media: bool | None
    has_spoiler: bool | None
    supports_streaming: bool | None
    disable_notification: bool | None
    protect_content: bool | None
    allow_paid_broadcast: bool | None
    message_effect_id: str | None
    suggested_post_parameters: SuggestedPostParameters | None
    reply_parameters: ReplyParameters | None
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | None
type SendVideoResponse = Message
class SendAnimationRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str
    message_thread_id: int | None
    direct_messages_topic_id: int | None
    animation: InputFile | str
    duration: int | None
    width: int | None
    height: int | None
    thumbnail: InputFile | str | None
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    show_caption_above_media: bool | None
    has_spoiler: bool | None
    disable_notification: bool | None
    protect_content: bool | None
    allow_paid_broadcast: bool | None
    message_effect_id: str | None
    suggested_post_parameters: SuggestedPostParameters | None
    reply_parameters: ReplyParameters | None
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | None
type SendAnimationResponse = Message
class SendVoiceRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str
    message_thread_id: int | None
    direct_messages_topic_id: int | None
    voice: InputFile | str
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    duration: int | None
    disable_notification: bool | None
    protect_content: bool | None
    allow_paid_broadcast: bool | None
    message_effect_id: str | None
    suggested_post_parameters: SuggestedPostParameters | None
    reply_parameters: ReplyParameters | None
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | None
type SendVoiceResponse = Message
class SendVideoNoteRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str
    message_thread_id: int | None
    direct_messages_topic_id: int | None
    video_note: InputFile | str
    duration: int | None
    length: int | None
    thumbnail: InputFile | str | None
    disable_notification: bool | None
    protect_content: bool | None
    allow_paid_broadcast: bool | None
    message_effect_id: str | None
    suggested_post_parameters: SuggestedPostParameters | None
    reply_parameters: ReplyParameters | None
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | None
type SendVideoNoteResponse = Message
class SendPaidMediaRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str
    message_thread_id: int | None
    direct_messages_topic_id: int | None
    star_count: int
    media: list[InputPaidMedia]
    payload: str | None
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    show_caption_above_media: bool | None
    disable_notification: bool | None
    protect_content: bool | None
    allow_paid_broadcast: bool | None
    suggested_post_parameters: SuggestedPostParameters | None
    reply_parameters: ReplyParameters | None
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | None
type SendPaidMediaResponse = Message
class SendMediaGroupRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str
    message_thread_id: int | None
    direct_messages_topic_id: int | None
    media: list[InputMediaAudio]
    disable_notification: bool | None
    protect_content: bool | None
    allow_paid_broadcast: bool | None
    message_effect_id: str | None
    reply_parameters: ReplyParameters | None
type SendMediaGroupResponse = Message
class SendLocationRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str
    message_thread_id: int | None
    direct_messages_topic_id: int | None
    latitude: float
    longitude: float
    horizontal_accuracy: float | None
    live_period: int | None
    heading: int | None
    proximity_alert_radius: int | None
    disable_notification: bool | None
    protect_content: bool | None
    allow_paid_broadcast: bool | None
    message_effect_id: str | None
    suggested_post_parameters: SuggestedPostParameters | None
    reply_parameters: ReplyParameters | None
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | None
type SendLocationResponse = Message
class SendVenueRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str
    message_thread_id: int | None
    direct_messages_topic_id: int | None
    latitude: float
    longitude: float
    title: str
    address: str
    foursquare_id: str | None
    foursquare_type: str | None
    google_place_id: str | None
    google_place_type: str | None
    disable_notification: bool | None
    protect_content: bool | None
    allow_paid_broadcast: bool | None
    message_effect_id: str | None
    suggested_post_parameters: SuggestedPostParameters | None
    reply_parameters: ReplyParameters | None
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | None
type SendVenueResponse = Message
class SendContactRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str
    message_thread_id: int | None
    direct_messages_topic_id: int | None
    phone_number: str
    first_name: str
    last_name: str | None
    vcard: str | None
    disable_notification: bool | None
    protect_content: bool | None
    allow_paid_broadcast: bool | None
    message_effect_id: str | None
    suggested_post_parameters: SuggestedPostParameters | None
    reply_parameters: ReplyParameters | None
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | None
type SendContactResponse = Message
class SendPollRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str
    message_thread_id: int | None
    question: str
    question_parse_mode: str | None
    question_entities: list[MessageEntity] | None
    options: list[InputPollOption]
    is_anonymous: bool | None
    type: str | None
    allows_multiple_answers: bool | None
    correct_option_id: int | None
    explanation: str | None
    explanation_parse_mode: str | None
    explanation_entities: list[MessageEntity] | None
    open_period: int | None
    close_date: int | None
    is_closed: bool | None
    disable_notification: bool | None
    protect_content: bool | None
    allow_paid_broadcast: bool | None
    message_effect_id: str | None
    reply_parameters: ReplyParameters | None
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | None
type SendPollResponse = Message
class SendChecklistRequest(TypedDict):
    business_connection_id: str
    chat_id: int
    checklist: InputChecklist
    disable_notification: bool | None
    protect_content: bool | None
    message_effect_id: str | None
    reply_parameters: ReplyParameters | None
    reply_markup: InlineKeyboardMarkup | None
type SendChecklistResponse = Message
class SendDiceRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str
    message_thread_id: int | None
    direct_messages_topic_id: int | None
    emoji: str | None
    disable_notification: bool | None
    protect_content: bool | None
    allow_paid_broadcast: bool | None
    message_effect_id: str | None
    suggested_post_parameters: SuggestedPostParameters | None
    reply_parameters: ReplyParameters | None
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | None
type SendDiceResponse = Message
class SendMessageDraftRequest(TypedDict):
    chat_id: int
    message_thread_id: int | None
    draft_id: int
    text: str
    parse_mode: str | None
    entities: list[MessageEntity] | None
type SendMessageDraftResponse = Literal[True]
class SendChatActionRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str
    message_thread_id: int | None
    action: str
type SendChatActionResponse = Literal[True]
class SetMessageReactionRequest(TypedDict):
    chat_id: int | str
    message_id: int
    reaction: list[ReactionType] | None
    is_big: bool | None
type SetMessageReactionResponse = Literal[True]
class GetUserProfilePhotosRequest(TypedDict):
    user_id: int
    offset: int | None
    limit: int | None
type GetUserProfilePhotosResponse = UserProfilePhotos
class GetUserProfileAudiosRequest(TypedDict):
    user_id: int
    offset: int | None
    limit: int | None
type GetUserProfileAudiosResponse = UserProfileAudios
class SetUserEmojiStatusRequest(TypedDict):
    user_id: int
    emoji_status_custom_emoji_id: str | None
    emoji_status_expiration_date: int | None
type SetUserEmojiStatusResponse = Literal[True]
class GetFileRequest(TypedDict):
    file_id: str
type GetFileResponse = File
class BanChatMemberRequest(TypedDict):
    chat_id: int | str
    user_id: int
    until_date: int | None
    revoke_messages: bool | None
type BanChatMemberResponse = Literal[True]
class UnbanChatMemberRequest(TypedDict):
    chat_id: int | str
    user_id: int
    only_if_banned: bool | None
type UnbanChatMemberResponse = Literal[True]
class RestrictChatMemberRequest(TypedDict):
    chat_id: int | str
    user_id: int
    permissions: ChatPermissions
    use_independent_chat_permissions: bool | None
    until_date: int | None
type RestrictChatMemberResponse = Literal[True]
class PromoteChatMemberRequest(TypedDict):
    chat_id: int | str
    user_id: int
    is_anonymous: bool | None
    can_manage_chat: bool | None
    can_delete_messages: bool | None
    can_manage_video_chats: bool | None
    can_restrict_members: bool | None
    can_promote_members: bool | None
    can_change_info: bool | None
    can_invite_users: bool | None
    can_post_stories: bool | None
    can_edit_stories: bool | None
    can_delete_stories: bool | None
    can_post_messages: bool | None
    can_edit_messages: bool | None
    can_pin_messages: bool | None
    can_manage_topics: bool | None
    can_manage_direct_messages: bool | None
type PromoteChatMemberResponse = Literal[True]
class SetChatAdministratorCustomTitleRequest(TypedDict):
    chat_id: int | str
    user_id: int
    custom_title: str
type SetChatAdministratorCustomTitleResponse = Literal[True]
class BanChatSenderChatRequest(TypedDict):
    chat_id: int | str
    sender_chat_id: int
type BanChatSenderChatResponse = Literal[True]
class UnbanChatSenderChatRequest(TypedDict):
    chat_id: int | str
    sender_chat_id: int
type UnbanChatSenderChatResponse = Literal[True]
class SetChatPermissionsRequest(TypedDict):
    chat_id: int | str
    permissions: ChatPermissions
    use_independent_chat_permissions: bool | None
type SetChatPermissionsResponse = Literal[True]
class ExportChatInviteLinkRequest(TypedDict):
    chat_id: int | str
type ExportChatInviteLinkResponse = str
class CreateChatInviteLinkRequest(TypedDict):
    chat_id: int | str
    name: str | None
    expire_date: int | None
    member_limit: int | None
    creates_join_request: bool | None
type CreateChatInviteLinkResponse = ChatInviteLink
class EditChatInviteLinkRequest(TypedDict):
    chat_id: int | str
    invite_link: str
    name: str | None
    expire_date: int | None
    member_limit: int | None
    creates_join_request: bool | None
type EditChatInviteLinkResponse = ChatInviteLink
class CreateChatSubscriptionInviteLinkRequest(TypedDict):
    chat_id: int | str
    name: str | None
    subscription_period: int
    subscription_price: int
type CreateChatSubscriptionInviteLinkResponse = ChatInviteLink
class EditChatSubscriptionInviteLinkRequest(TypedDict):
    chat_id: int | str
    invite_link: str
    name: str | None
type EditChatSubscriptionInviteLinkResponse = ChatInviteLink
class RevokeChatInviteLinkRequest(TypedDict):
    chat_id: int | str
    invite_link: str
type RevokeChatInviteLinkResponse = ChatInviteLink
class ApproveChatJoinRequestRequest(TypedDict):
    chat_id: int | str
    user_id: int
type ApproveChatJoinRequestResponse = Literal[True]
class DeclineChatJoinRequestRequest(TypedDict):
    chat_id: int | str
    user_id: int
type DeclineChatJoinRequestResponse = Literal[True]
class SetChatPhotoRequest(TypedDict):
    chat_id: int | str
    photo: InputFile
type SetChatPhotoResponse = Literal[True]
class DeleteChatPhotoRequest(TypedDict):
    chat_id: int | str
type DeleteChatPhotoResponse = Literal[True]
class SetChatTitleRequest(TypedDict):
    chat_id: int | str
    title: str
type SetChatTitleResponse = Literal[True]
class SetChatDescriptionRequest(TypedDict):
    chat_id: int | str
    description: str | None
type SetChatDescriptionResponse = Literal[True]
class PinChatMessageRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str
    message_id: int
    disable_notification: bool | None
type PinChatMessageResponse = Literal[True]
class UnpinChatMessageRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str
    message_id: int | None
type UnpinChatMessageResponse = Literal[True]
class UnpinAllChatMessagesRequest(TypedDict):
    chat_id: int | str
type UnpinAllChatMessagesResponse = Literal[True]
class LeaveChatRequest(TypedDict):
    chat_id: int | str
type LeaveChatResponse = Literal[True]
class GetChatRequest(TypedDict):
    chat_id: int | str
type GetChatResponse = ChatFullInfo
class GetChatAdministratorsRequest(TypedDict):
    chat_id: int | str
type GetChatAdministratorsResponse = list[ChatMember]
class GetChatMemberCountRequest(TypedDict):
    chat_id: int | str
type GetChatMemberCountResponse = int
class GetChatMemberRequest(TypedDict):
    chat_id: int | str
    user_id: int
type GetChatMemberResponse = ChatMember
class SetChatStickerSetRequest(TypedDict):
    chat_id: int | str
    sticker_set_name: str
type SetChatStickerSetResponse = Literal[True]
class DeleteChatStickerSetRequest(TypedDict):
    chat_id: int | str
type DeleteChatStickerSetResponse = Literal[True]
class GetForumTopicIconStickersRequest(TypedDict):
    ...
type GetForumTopicIconStickersResponse = list[Sticker]
class CreateForumTopicRequest(TypedDict):
    chat_id: int | str
    name: str
    icon_color: int | None
    icon_custom_emoji_id: str | None
type CreateForumTopicResponse = ForumTopic
class EditForumTopicRequest(TypedDict):
    chat_id: int | str
    message_thread_id: int
    name: str | None
    icon_custom_emoji_id: str | None
type EditForumTopicResponse = Literal[True]
class CloseForumTopicRequest(TypedDict):
    chat_id: int | str
    message_thread_id: int
type CloseForumTopicResponse = Literal[True]
class ReopenForumTopicRequest(TypedDict):
    chat_id: int | str
    message_thread_id: int
type ReopenForumTopicResponse = Literal[True]
class DeleteForumTopicRequest(TypedDict):
    chat_id: int | str
    message_thread_id: int
type DeleteForumTopicResponse = Literal[True]
class UnpinAllForumTopicMessagesRequest(TypedDict):
    chat_id: int | str
    message_thread_id: int
type UnpinAllForumTopicMessagesResponse = Literal[True]
class EditGeneralForumTopicRequest(TypedDict):
    chat_id: int | str
    name: str
type EditGeneralForumTopicResponse = Literal[True]
class CloseGeneralForumTopicRequest(TypedDict):
    chat_id: int | str
type CloseGeneralForumTopicResponse = Literal[True]
class ReopenGeneralForumTopicRequest(TypedDict):
    chat_id: int | str
type ReopenGeneralForumTopicResponse = Literal[True]
class HideGeneralForumTopicRequest(TypedDict):
    chat_id: int | str
type HideGeneralForumTopicResponse = Literal[True]
class UnhideGeneralForumTopicRequest(TypedDict):
    chat_id: int | str
type UnhideGeneralForumTopicResponse = Literal[True]
class UnpinAllGeneralForumTopicMessagesRequest(TypedDict):
    chat_id: int | str
type UnpinAllGeneralForumTopicMessagesResponse = Literal[True]
class AnswerCallbackQueryRequest(TypedDict):
    callback_query_id: str
    text: str | None
    show_alert: bool | None
    url: str | None
    cache_time: int | None
type AnswerCallbackQueryResponse = Literal[True]
class GetUserChatBoostsRequest(TypedDict):
    chat_id: int | str
    user_id: int
type GetUserChatBoostsResponse = UserChatBoosts
class GetBusinessConnectionRequest(TypedDict):
    business_connection_id: str
type GetBusinessConnectionResponse = BusinessConnection
class SetMyCommandsRequest(TypedDict):
    commands: list[BotCommand]
    scope: BotCommandScope | None
    language_code: str | None
type SetMyCommandsResponse = Literal[True]
class DeleteMyCommandsRequest(TypedDict):
    scope: BotCommandScope | None
    language_code: str | None
type DeleteMyCommandsResponse = Literal[True]
class GetMyCommandsRequest(TypedDict):
    scope: BotCommandScope | None
    language_code: str | None
type GetMyCommandsResponse = list[BotCommand]
class SetMyNameRequest(TypedDict):
    name: str | None
    language_code: str | None
type SetMyNameResponse = Literal[True]
class GetMyNameRequest(TypedDict):
    language_code: str | None
type GetMyNameResponse = BotName
class SetMyDescriptionRequest(TypedDict):
    description: str | None
    language_code: str | None
type SetMyDescriptionResponse = Literal[True]
class GetMyDescriptionRequest(TypedDict):
    language_code: str | None
type GetMyDescriptionResponse = BotDescription
class SetMyShortDescriptionRequest(TypedDict):
    short_description: str | None
    language_code: str | None
type SetMyShortDescriptionResponse = Literal[True]
class GetMyShortDescriptionRequest(TypedDict):
    language_code: str | None
type GetMyShortDescriptionResponse = BotShortDescription
class SetMyProfilePhotoRequest(TypedDict):
    photo: InputProfilePhoto
type SetMyProfilePhotoResponse = Literal[True]
class RemoveMyProfilePhotoRequest(TypedDict):
    ...
type RemoveMyProfilePhotoResponse = Literal[True]
class SetChatMenuButtonRequest(TypedDict):
    chat_id: int | None
    menu_button: MenuButton | None
type SetChatMenuButtonResponse = Literal[True]
class GetChatMenuButtonRequest(TypedDict):
    chat_id: int | None
type GetChatMenuButtonResponse = MenuButton
class SetMyDefaultAdministratorRightsRequest(TypedDict):
    rights: ChatAdministratorRights | None
    for_channels: bool | None
type SetMyDefaultAdministratorRightsResponse = Literal[True]
class GetMyDefaultAdministratorRightsRequest(TypedDict):
    for_channels: bool | None
type GetMyDefaultAdministratorRightsResponse = ChatAdministratorRights
class GetAvailableGiftsRequest(TypedDict):
    ...
type GetAvailableGiftsResponse = Gifts
class SendGiftRequest(TypedDict):
    user_id: int | None
    chat_id: int | str | None
    gift_id: str
    pay_for_upgrade: bool | None
    text: str | None
    text_parse_mode: str | None
    text_entities: list[MessageEntity] | None
type SendGiftResponse = Literal[True]
class GiftPremiumSubscriptionRequest(TypedDict):
    user_id: int
    month_count: int
    star_count: int
    text: str | None
    text_parse_mode: str | None
    text_entities: list[MessageEntity] | None
type GiftPremiumSubscriptionResponse = Literal[True]
class VerifyUserRequest(TypedDict):
    user_id: int
    custom_description: str | None
type VerifyUserResponse = Literal[True]
class VerifyChatRequest(TypedDict):
    chat_id: int | str
    custom_description: str | None
type VerifyChatResponse = Literal[True]
class RemoveUserVerificationRequest(TypedDict):
    user_id: int
type RemoveUserVerificationResponse = Literal[True]
class RemoveChatVerificationRequest(TypedDict):
    chat_id: int | str
type RemoveChatVerificationResponse = Literal[True]
class ReadBusinessMessageRequest(TypedDict):
    business_connection_id: str
    chat_id: int
    message_id: int
type ReadBusinessMessageResponse = Literal[True]
class DeleteBusinessMessagesRequest(TypedDict):
    business_connection_id: str
    message_ids: list[int]
type DeleteBusinessMessagesResponse = Literal[True]
class SetBusinessAccountNameRequest(TypedDict):
    business_connection_id: str
    first_name: str
    last_name: str | None
type SetBusinessAccountNameResponse = Literal[True]
class SetBusinessAccountUsernameRequest(TypedDict):
    business_connection_id: str
    username: str | None
type SetBusinessAccountUsernameResponse = Literal[True]
class SetBusinessAccountBioRequest(TypedDict):
    business_connection_id: str
    bio: str | None
type SetBusinessAccountBioResponse = Literal[True]
class SetBusinessAccountProfilePhotoRequest(TypedDict):
    business_connection_id: str
    photo: InputProfilePhoto
    is_public: bool | None
type SetBusinessAccountProfilePhotoResponse = Literal[True]
class RemoveBusinessAccountProfilePhotoRequest(TypedDict):
    business_connection_id: str
    is_public: bool | None
type RemoveBusinessAccountProfilePhotoResponse = Literal[True]
class SetBusinessAccountGiftSettingsRequest(TypedDict):
    business_connection_id: str
    show_gift_button: bool
    accepted_gift_types: AcceptedGiftTypes
type SetBusinessAccountGiftSettingsResponse = Literal[True]
class GetBusinessAccountStarBalanceRequest(TypedDict):
    business_connection_id: str
type GetBusinessAccountStarBalanceResponse = StarAmount
class TransferBusinessAccountStarsRequest(TypedDict):
    business_connection_id: str
    star_count: int
type TransferBusinessAccountStarsResponse = Literal[True]
class GetBusinessAccountGiftsRequest(TypedDict):
    business_connection_id: str
    exclude_unsaved: bool | None
    exclude_saved: bool | None
    exclude_unlimited: bool | None
    exclude_limited_upgradable: bool | None
    exclude_limited_non_upgradable: bool | None
    exclude_unique: bool | None
    exclude_from_blockchain: bool | None
    sort_by_price: bool | None
    offset: str | None
    limit: int | None
type GetBusinessAccountGiftsResponse = OwnedGifts
class GetUserGiftsRequest(TypedDict):
    user_id: int
    exclude_unlimited: bool | None
    exclude_limited_upgradable: bool | None
    exclude_limited_non_upgradable: bool | None
    exclude_from_blockchain: bool | None
    exclude_unique: bool | None
    sort_by_price: bool | None
    offset: str | None
    limit: int | None
type GetUserGiftsResponse = OwnedGifts
class GetChatGiftsRequest(TypedDict):
    chat_id: int | str
    exclude_unsaved: bool | None
    exclude_saved: bool | None
    exclude_unlimited: bool | None
    exclude_limited_upgradable: bool | None
    exclude_limited_non_upgradable: bool | None
    exclude_from_blockchain: bool | None
    exclude_unique: bool | None
    sort_by_price: bool | None
    offset: str | None
    limit: int | None
type GetChatGiftsResponse = OwnedGifts
class ConvertGiftToStarsRequest(TypedDict):
    business_connection_id: str
    owned_gift_id: str
type ConvertGiftToStarsResponse = Literal[True]
class UpgradeGiftRequest(TypedDict):
    business_connection_id: str
    owned_gift_id: str
    keep_original_details: bool | None
    star_count: int | None
type UpgradeGiftResponse = Literal[True]
class TransferGiftRequest(TypedDict):
    business_connection_id: str
    owned_gift_id: str
    new_owner_chat_id: int
    star_count: int | None
type TransferGiftResponse = Literal[True]
class PostStoryRequest(TypedDict):
    business_connection_id: str
    content: InputStoryContent
    active_period: int
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    areas: list[StoryArea] | None
    post_to_chat_page: bool | None
    protect_content: bool | None
type PostStoryResponse = Story
class RepostStoryRequest(TypedDict):
    business_connection_id: str
    from_chat_id: int
    from_story_id: int
    active_period: int
    post_to_chat_page: bool | None
    protect_content: bool | None
type RepostStoryResponse = Story
class EditStoryRequest(TypedDict):
    business_connection_id: str
    story_id: int
    content: InputStoryContent
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    areas: list[StoryArea] | None
type EditStoryResponse = Story
class DeleteStoryRequest(TypedDict):
    business_connection_id: str
    story_id: int
type DeleteStoryResponse = Literal[True]
class EditMessageTextRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str | None
    message_id: int | None
    inline_message_id: str | None
    text: str
    parse_mode: str | None
    entities: list[MessageEntity] | None
    link_preview_options: LinkPreviewOptions | None
    reply_markup: InlineKeyboardMarkup | None
type EditMessageTextResponse = Message | Literal[True]
class EditMessageCaptionRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str | None
    message_id: int | None
    inline_message_id: str | None
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    show_caption_above_media: bool | None
    reply_markup: InlineKeyboardMarkup | None
type EditMessageCaptionResponse = Message | Literal[True]
class EditMessageMediaRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str | None
    message_id: int | None
    inline_message_id: str | None
    media: InputMedia
    reply_markup: InlineKeyboardMarkup | None
type EditMessageMediaResponse = Message | Literal[True]
class EditMessageLiveLocationRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str | None
    message_id: int | None
    inline_message_id: str | None
    latitude: float
    longitude: float
    live_period: int | None
    horizontal_accuracy: float | None
    heading: int | None
    proximity_alert_radius: int | None
    reply_markup: InlineKeyboardMarkup | None
type EditMessageLiveLocationResponse = Message | Literal[True]
class StopMessageLiveLocationRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str | None
    message_id: int | None
    inline_message_id: str | None
    reply_markup: InlineKeyboardMarkup | None
type StopMessageLiveLocationResponse = Message | Literal[True]
class EditMessageChecklistRequest(TypedDict):
    business_connection_id: str
    chat_id: int
    message_id: int
    checklist: InputChecklist
    reply_markup: InlineKeyboardMarkup | None
type EditMessageChecklistResponse = Message
class EditMessageReplyMarkupRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str | None
    message_id: int | None
    inline_message_id: str | None
    reply_markup: InlineKeyboardMarkup | None
type EditMessageReplyMarkupResponse = Message | Literal[True]
class StopPollRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str
    message_id: int
    reply_markup: InlineKeyboardMarkup | None
type StopPollResponse = Poll
class ApproveSuggestedPostRequest(TypedDict):
    chat_id: int
    message_id: int
    send_date: int | None
type ApproveSuggestedPostResponse = Literal[True]
class DeclineSuggestedPostRequest(TypedDict):
    chat_id: int
    message_id: int
    comment: str | None
type DeclineSuggestedPostResponse = Literal[True]
class DeleteMessageRequest(TypedDict):
    chat_id: int | str
    message_id: int
type DeleteMessageResponse = Literal[True]
class DeleteMessagesRequest(TypedDict):
    chat_id: int | str
    message_ids: list[int]
type DeleteMessagesResponse = Literal[True]
class Sticker(TypedDict):
    file_id: str
    file_unique_id: str
    type: str
    width: int
    height: int
    is_animated: bool
    is_video: bool
    thumbnail: PhotoSize | None
    emoji: str | None
    set_name: str | None
    premium_animation: File | None
    mask_position: MaskPosition | None
    custom_emoji_id: str | None
    needs_repainting: Literal[True] | None
    file_size: int | None
class StickerSet(TypedDict):
    name: str
    title: str
    sticker_type: str
    stickers: list[Sticker]
    thumbnail: PhotoSize | None
class MaskPosition(TypedDict):
    point: str
    x_shift: float
    y_shift: float
    scale: float
class InputSticker(TypedDict):
    sticker: str
    format: str
    emoji_list: list[str]
    mask_position: MaskPosition | None
    keywords: list[str] | None
class SendStickerRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int | str
    message_thread_id: int | None
    direct_messages_topic_id: int | None
    sticker: InputFile | str
    emoji: str | None
    disable_notification: bool | None
    protect_content: bool | None
    allow_paid_broadcast: bool | None
    message_effect_id: str | None
    suggested_post_parameters: SuggestedPostParameters | None
    reply_parameters: ReplyParameters | None
    reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply | None
type SendStickerResponse = Message
class GetStickerSetRequest(TypedDict):
    name: str
type GetStickerSetResponse = StickerSet
class GetCustomEmojiStickersRequest(TypedDict):
    custom_emoji_ids: list[str]
type GetCustomEmojiStickersResponse = list[Sticker]
class UploadStickerFileRequest(TypedDict):
    user_id: int
    sticker: InputFile
    sticker_format: str
type UploadStickerFileResponse = File
class CreateNewStickerSetRequest(TypedDict):
    user_id: int
    name: str
    title: str
    stickers: list[InputSticker]
    sticker_type: str | None
    needs_repainting: bool | None
type CreateNewStickerSetResponse = Literal[True]
class AddStickerToSetRequest(TypedDict):
    user_id: int
    name: str
    sticker: InputSticker
type AddStickerToSetResponse = Literal[True]
class SetStickerPositionInSetRequest(TypedDict):
    sticker: str
    position: int
type SetStickerPositionInSetResponse = Literal[True]
class DeleteStickerFromSetRequest(TypedDict):
    sticker: str
type DeleteStickerFromSetResponse = Literal[True]
class ReplaceStickerInSetRequest(TypedDict):
    user_id: int
    name: str
    old_sticker: str
    sticker: InputSticker
type ReplaceStickerInSetResponse = Literal[True]
class SetStickerEmojiListRequest(TypedDict):
    sticker: str
    emoji_list: list[str]
type SetStickerEmojiListResponse = Literal[True]
class SetStickerKeywordsRequest(TypedDict):
    sticker: str
    keywords: list[str] | None
type SetStickerKeywordsResponse = Literal[True]
class SetStickerMaskPositionRequest(TypedDict):
    sticker: str
    mask_position: MaskPosition | None
type SetStickerMaskPositionResponse = Literal[True]
class SetStickerSetTitleRequest(TypedDict):
    name: str
    title: str
type SetStickerSetTitleResponse = Literal[True]
class SetStickerSetThumbnailRequest(TypedDict):
    name: str
    user_id: int
    thumbnail: InputFile | str | None
    format: str
type SetStickerSetThumbnailResponse = Literal[True]
class SetCustomEmojiStickerSetThumbnailRequest(TypedDict):
    name: str
    custom_emoji_id: str | None
type SetCustomEmojiStickerSetThumbnailResponse = Literal[True]
class DeleteStickerSetRequest(TypedDict):
    name: str
type DeleteStickerSetResponse = Literal[True]
class InlineQuery(TypedDict):
    id: str
    from_: User
    query: str
    offset: str
    chat_type: str | None
    location: Location | None
class AnswerInlineQueryRequest(TypedDict):
    inline_query_id: str
    results: list[InlineQueryResult]
    cache_time: int | None
    is_personal: bool | None
    next_offset: str | None
    button: InlineQueryResultsButton | None
type AnswerInlineQueryResponse = Literal[True]
class InlineQueryResultsButton(TypedDict):
    text: str
    web_app: WebAppInfo | None
    start_parameter: str | None
type InlineQueryResult = InlineQueryResultCachedAudio | InlineQueryResultCachedDocument | InlineQueryResultCachedGif | InlineQueryResultCachedMpeg4Gif | InlineQueryResultCachedPhoto | InlineQueryResultCachedSticker | InlineQueryResultCachedVideo | InlineQueryResultCachedVoice | InlineQueryResultArticle | InlineQueryResultAudio | InlineQueryResultContact | InlineQueryResultGame | InlineQueryResultDocument | InlineQueryResultGif | InlineQueryResultLocation | InlineQueryResultMpeg4Gif | InlineQueryResultPhoto | InlineQueryResultVenue | InlineQueryResultVideo | InlineQueryResultVoice
class InlineQueryResultArticle(TypedDict):
    type: str
    id: str
    title: str
    input_message_content: InputMessageContent
    reply_markup: InlineKeyboardMarkup | None
    url: str | None
    description: str | None
    thumbnail_url: str | None
    thumbnail_width: int | None
    thumbnail_height: int | None
class InlineQueryResultPhoto(TypedDict):
    type: str
    id: str
    photo_url: str
    thumbnail_url: str
    photo_width: int | None
    photo_height: int | None
    title: str | None
    description: str | None
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    show_caption_above_media: bool | None
    reply_markup: InlineKeyboardMarkup | None
    input_message_content: InputMessageContent | None
class InlineQueryResultGif(TypedDict):
    type: str
    id: str
    gif_url: str
    gif_width: int | None
    gif_height: int | None
    gif_duration: int | None
    thumbnail_url: str
    thumbnail_mime_type: str | None
    title: str | None
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    show_caption_above_media: bool | None
    reply_markup: InlineKeyboardMarkup | None
    input_message_content: InputMessageContent | None
class InlineQueryResultMpeg4Gif(TypedDict):
    type: str
    id: str
    mpeg4_url: str
    mpeg4_width: int | None
    mpeg4_height: int | None
    mpeg4_duration: int | None
    thumbnail_url: str
    thumbnail_mime_type: str | None
    title: str | None
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    show_caption_above_media: bool | None
    reply_markup: InlineKeyboardMarkup | None
    input_message_content: InputMessageContent | None
class InlineQueryResultVideo(TypedDict):
    type: str
    id: str
    video_url: str
    mime_type: str
    thumbnail_url: str
    title: str
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    show_caption_above_media: bool | None
    video_width: int | None
    video_height: int | None
    video_duration: int | None
    description: str | None
    reply_markup: InlineKeyboardMarkup | None
    input_message_content: InputMessageContent | None
class InlineQueryResultAudio(TypedDict):
    type: str
    id: str
    audio_url: str
    title: str
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    performer: str | None
    audio_duration: int | None
    reply_markup: InlineKeyboardMarkup | None
    input_message_content: InputMessageContent | None
class InlineQueryResultVoice(TypedDict):
    type: str
    id: str
    voice_url: str
    title: str
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    voice_duration: int | None
    reply_markup: InlineKeyboardMarkup | None
    input_message_content: InputMessageContent | None
class InlineQueryResultDocument(TypedDict):
    type: str
    id: str
    title: str
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    document_url: str
    mime_type: str
    description: str | None
    reply_markup: InlineKeyboardMarkup | None
    input_message_content: InputMessageContent | None
    thumbnail_url: str | None
    thumbnail_width: int | None
    thumbnail_height: int | None
class InlineQueryResultLocation(TypedDict):
    type: str
    id: str
    latitude: float
    longitude: float
    title: str
    horizontal_accuracy: float | None
    live_period: int | None
    heading: int | None
    proximity_alert_radius: int | None
    reply_markup: InlineKeyboardMarkup | None
    input_message_content: InputMessageContent | None
    thumbnail_url: str | None
    thumbnail_width: int | None
    thumbnail_height: int | None
class InlineQueryResultVenue(TypedDict):
    type: str
    id: str
    latitude: float
    longitude: float
    title: str
    address: str
    foursquare_id: str | None
    foursquare_type: str | None
    google_place_id: str | None
    google_place_type: str | None
    reply_markup: InlineKeyboardMarkup | None
    input_message_content: InputMessageContent | None
    thumbnail_url: str | None
    thumbnail_width: int | None
    thumbnail_height: int | None
class InlineQueryResultContact(TypedDict):
    type: str
    id: str
    phone_number: str
    first_name: str
    last_name: str | None
    vcard: str | None
    reply_markup: InlineKeyboardMarkup | None
    input_message_content: InputMessageContent | None
    thumbnail_url: str | None
    thumbnail_width: int | None
    thumbnail_height: int | None
class InlineQueryResultGame(TypedDict):
    type: str
    id: str
    game_short_name: str
    reply_markup: InlineKeyboardMarkup | None
class InlineQueryResultCachedPhoto(TypedDict):
    type: str
    id: str
    photo_file_id: str
    title: str | None
    description: str | None
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    show_caption_above_media: bool | None
    reply_markup: InlineKeyboardMarkup | None
    input_message_content: InputMessageContent | None
class InlineQueryResultCachedGif(TypedDict):
    type: str
    id: str
    gif_file_id: str
    title: str | None
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    show_caption_above_media: bool | None
    reply_markup: InlineKeyboardMarkup | None
    input_message_content: InputMessageContent | None
class InlineQueryResultCachedMpeg4Gif(TypedDict):
    type: str
    id: str
    mpeg4_file_id: str
    title: str | None
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    show_caption_above_media: bool | None
    reply_markup: InlineKeyboardMarkup | None
    input_message_content: InputMessageContent | None
class InlineQueryResultCachedSticker(TypedDict):
    type: str
    id: str
    sticker_file_id: str
    reply_markup: InlineKeyboardMarkup | None
    input_message_content: InputMessageContent | None
class InlineQueryResultCachedDocument(TypedDict):
    type: str
    id: str
    title: str
    document_file_id: str
    description: str | None
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    reply_markup: InlineKeyboardMarkup | None
    input_message_content: InputMessageContent | None
class InlineQueryResultCachedVideo(TypedDict):
    type: str
    id: str
    video_file_id: str
    title: str
    description: str | None
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    show_caption_above_media: bool | None
    reply_markup: InlineKeyboardMarkup | None
    input_message_content: InputMessageContent | None
class InlineQueryResultCachedVoice(TypedDict):
    type: str
    id: str
    voice_file_id: str
    title: str
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    reply_markup: InlineKeyboardMarkup | None
    input_message_content: InputMessageContent | None
class InlineQueryResultCachedAudio(TypedDict):
    type: str
    id: str
    audio_file_id: str
    caption: str | None
    parse_mode: str | None
    caption_entities: list[MessageEntity] | None
    reply_markup: InlineKeyboardMarkup | None
    input_message_content: InputMessageContent | None
type InputMessageContent = InputTextMessageContent | InputLocationMessageContent | InputVenueMessageContent | InputContactMessageContent | InputInvoiceMessageContent
class InputTextMessageContent(TypedDict):
    message_text: str
    parse_mode: str | None
    entities: list[MessageEntity] | None
    link_preview_options: LinkPreviewOptions | None
class InputLocationMessageContent(TypedDict):
    latitude: float
    longitude: float
    horizontal_accuracy: float | None
    live_period: int | None
    heading: int | None
    proximity_alert_radius: int | None
class InputVenueMessageContent(TypedDict):
    latitude: float
    longitude: float
    title: str
    address: str
    foursquare_id: str | None
    foursquare_type: str | None
    google_place_id: str | None
    google_place_type: str | None
class InputContactMessageContent(TypedDict):
    phone_number: str
    first_name: str
    last_name: str | None
    vcard: str | None
class InputInvoiceMessageContent(TypedDict):
    title: str
    description: str
    payload: str
    provider_token: str | None
    currency: str
    prices: list[LabeledPrice]
    max_tip_amount: int | None
    suggested_tip_amounts: list[int] | None
    provider_data: str | None
    photo_url: str | None
    photo_size: int | None
    photo_width: int | None
    photo_height: int | None
    need_name: bool | None
    need_phone_number: bool | None
    need_email: bool | None
    need_shipping_address: bool | None
    send_phone_number_to_provider: bool | None
    send_email_to_provider: bool | None
    is_flexible: bool | None
class ChosenInlineResult(TypedDict):
    result_id: str
    from_: User
    location: Location | None
    inline_message_id: str | None
    query: str
class AnswerWebAppQueryRequest(TypedDict):
    web_app_query_id: str
    result: InlineQueryResult
type AnswerWebAppQueryResponse = SentWebAppMessage
class SentWebAppMessage(TypedDict):
    inline_message_id: str | None
class SavePreparedInlineMessageRequest(TypedDict):
    user_id: int
    result: InlineQueryResult
    allow_user_chats: bool | None
    allow_bot_chats: bool | None
    allow_group_chats: bool | None
    allow_channel_chats: bool | None
type SavePreparedInlineMessageResponse = PreparedInlineMessage
class PreparedInlineMessage(TypedDict):
    id: str
    expiration_date: int
class SendInvoiceRequest(TypedDict):
    chat_id: int | str
    message_thread_id: int | None
    direct_messages_topic_id: int | None
    title: str
    description: str
    payload: str
    provider_token: str | None
    currency: str
    prices: list[LabeledPrice]
    max_tip_amount: int | None
    suggested_tip_amounts: list[int] | None
    start_parameter: str | None
    provider_data: str | None
    photo_url: str | None
    photo_size: int | None
    photo_width: int | None
    photo_height: int | None
    need_name: bool | None
    need_phone_number: bool | None
    need_email: bool | None
    need_shipping_address: bool | None
    send_phone_number_to_provider: bool | None
    send_email_to_provider: bool | None
    is_flexible: bool | None
    disable_notification: bool | None
    protect_content: bool | None
    allow_paid_broadcast: bool | None
    message_effect_id: str | None
    suggested_post_parameters: SuggestedPostParameters | None
    reply_parameters: ReplyParameters | None
    reply_markup: InlineKeyboardMarkup | None
type SendInvoiceResponse = Message
class CreateInvoiceLinkRequest(TypedDict):
    business_connection_id: str | None
    title: str
    description: str
    payload: str
    provider_token: str | None
    currency: str
    prices: list[LabeledPrice]
    subscription_period: int | None
    max_tip_amount: int | None
    suggested_tip_amounts: list[int] | None
    provider_data: str | None
    photo_url: str | None
    photo_size: int | None
    photo_width: int | None
    photo_height: int | None
    need_name: bool | None
    need_phone_number: bool | None
    need_email: bool | None
    need_shipping_address: bool | None
    send_phone_number_to_provider: bool | None
    send_email_to_provider: bool | None
    is_flexible: bool | None
type CreateInvoiceLinkResponse = str
class AnswerShippingQueryRequest(TypedDict):
    shipping_query_id: str
    ok: bool
    shipping_options: list[ShippingOption] | None
    error_message: str | None
type AnswerShippingQueryResponse = Literal[True]
class AnswerPreCheckoutQueryRequest(TypedDict):
    pre_checkout_query_id: str
    ok: bool
    error_message: str | None
type AnswerPreCheckoutQueryResponse = Literal[True]
class GetMyStarBalanceRequest(TypedDict):
    ...
type GetMyStarBalanceResponse = StarAmount
class GetStarTransactionsRequest(TypedDict):
    offset: int | None
    limit: int | None
type GetStarTransactionsResponse = StarTransactions
class RefundStarPaymentRequest(TypedDict):
    user_id: int
    telegram_payment_charge_id: str
type RefundStarPaymentResponse = Literal[True]
class EditUserStarSubscriptionRequest(TypedDict):
    user_id: int
    telegram_payment_charge_id: str
    is_canceled: bool
type EditUserStarSubscriptionResponse = Literal[True]
class LabeledPrice(TypedDict):
    label: str
    amount: int
class Invoice(TypedDict):
    title: str
    description: str
    start_parameter: str
    currency: str
    total_amount: int
class ShippingAddress(TypedDict):
    country_code: str
    state: str
    city: str
    street_line1: str
    street_line2: str
    post_code: str
class OrderInfo(TypedDict):
    name: str | None
    phone_number: str | None
    email: str | None
    shipping_address: ShippingAddress | None
class ShippingOption(TypedDict):
    id: str
    title: str
    prices: list[LabeledPrice]
class SuccessfulPayment(TypedDict):
    currency: str
    total_amount: int
    invoice_payload: str
    subscription_expiration_date: int | None
    is_recurring: Literal[True] | None
    is_first_recurring: Literal[True] | None
    shipping_option_id: str | None
    order_info: OrderInfo | None
    telegram_payment_charge_id: str
    provider_payment_charge_id: str
class RefundedPayment(TypedDict):
    currency: str
    total_amount: int
    invoice_payload: str
    telegram_payment_charge_id: str
    provider_payment_charge_id: str | None
class ShippingQuery(TypedDict):
    id: str
    from_: User
    invoice_payload: str
    shipping_address: ShippingAddress
class PreCheckoutQuery(TypedDict):
    id: str
    from_: User
    currency: str
    total_amount: int
    invoice_payload: str
    shipping_option_id: str | None
    order_info: OrderInfo | None
class PaidMediaPurchased(TypedDict):
    from_: User
    paid_media_payload: str
type RevenueWithdrawalState = RevenueWithdrawalStatePending | RevenueWithdrawalStateSucceeded | RevenueWithdrawalStateFailed
class RevenueWithdrawalStatePending(TypedDict):
    type: str
class RevenueWithdrawalStateSucceeded(TypedDict):
    type: str
    date: int
    url: str
class RevenueWithdrawalStateFailed(TypedDict):
    type: str
class AffiliateInfo(TypedDict):
    affiliate_user: User | None
    affiliate_chat: Chat | None
    commission_per_mille: int
    amount: int
    nanostar_amount: int | None
type TransactionPartner = TransactionPartnerUser | TransactionPartnerChat | TransactionPartnerAffiliateProgram | TransactionPartnerFragment | TransactionPartnerTelegramAds | TransactionPartnerTelegramApi | TransactionPartnerOther
class TransactionPartnerUser(TypedDict):
    type: str
    transaction_type: str
    user: User
    affiliate: AffiliateInfo | None
    invoice_payload: str | None
    subscription_period: int | None
    paid_media: list[PaidMedia] | None
    paid_media_payload: str | None
    gift: Gift | None
    premium_subscription_duration: int | None
class TransactionPartnerChat(TypedDict):
    type: str
    chat: Chat
    gift: Gift | None
class TransactionPartnerAffiliateProgram(TypedDict):
    type: str
    sponsor_user: User | None
    commission_per_mille: int
class TransactionPartnerFragment(TypedDict):
    type: str
    withdrawal_state: RevenueWithdrawalState | None
class TransactionPartnerTelegramAds(TypedDict):
    type: str
class TransactionPartnerTelegramApi(TypedDict):
    type: str
    request_count: int
class TransactionPartnerOther(TypedDict):
    type: str
class StarTransaction(TypedDict):
    id: str
    amount: int
    nanostar_amount: int | None
    date: int
    source: TransactionPartner | None
    receiver: TransactionPartner | None
class StarTransactions(TypedDict):
    transactions: list[StarTransaction]
class PassportData(TypedDict):
    data: list[EncryptedPassportElement]
    credentials: EncryptedCredentials
class PassportFile(TypedDict):
    file_id: str
    file_unique_id: str
    file_size: int
    file_date: int
class EncryptedPassportElement(TypedDict):
    type: str
    data: str | None
    phone_number: str | None
    email: str | None
    files: list[PassportFile] | None
    front_side: PassportFile | None
    reverse_side: PassportFile | None
    selfie: PassportFile | None
    translation: list[PassportFile] | None
    hash: str
class EncryptedCredentials(TypedDict):
    data: str
    hash: str
    secret: str
class SetPassportDataErrorsRequest(TypedDict):
    user_id: int
    errors: list[PassportElementError]
type SetPassportDataErrorsResponse = Literal[True]
type PassportElementError = PassportElementErrorDataField | PassportElementErrorFrontSide | PassportElementErrorReverseSide | PassportElementErrorSelfie | PassportElementErrorFile | PassportElementErrorFiles | PassportElementErrorTranslationFile | PassportElementErrorTranslationFiles | PassportElementErrorUnspecified
class PassportElementErrorDataField(TypedDict):
    source: str
    type: str
    field_name: str
    data_hash: str
    message: str
class PassportElementErrorFrontSide(TypedDict):
    source: str
    type: str
    file_hash: str
    message: str
class PassportElementErrorReverseSide(TypedDict):
    source: str
    type: str
    file_hash: str
    message: str
class PassportElementErrorSelfie(TypedDict):
    source: str
    type: str
    file_hash: str
    message: str
class PassportElementErrorFile(TypedDict):
    source: str
    type: str
    file_hash: str
    message: str
class PassportElementErrorFiles(TypedDict):
    source: str
    type: str
    file_hashes: list[str]
    message: str
class PassportElementErrorTranslationFile(TypedDict):
    source: str
    type: str
    file_hash: str
    message: str
class PassportElementErrorTranslationFiles(TypedDict):
    source: str
    type: str
    file_hashes: list[str]
    message: str
class PassportElementErrorUnspecified(TypedDict):
    source: str
    type: str
    element_hash: str
    message: str
class SendGameRequest(TypedDict):
    business_connection_id: str | None
    chat_id: int
    message_thread_id: int | None
    game_short_name: str
    disable_notification: bool | None
    protect_content: bool | None
    allow_paid_broadcast: bool | None
    message_effect_id: str | None
    reply_parameters: ReplyParameters | None
    reply_markup: InlineKeyboardMarkup | None
type SendGameResponse = Message
class Game(TypedDict):
    title: str
    description: str
    photo: list[PhotoSize]
    text: str | None
    text_entities: list[MessageEntity] | None
    animation: Animation | None
class CallbackGame(TypedDict):
    ...
class SetGameScoreRequest(TypedDict):
    user_id: int
    score: int
    force: bool | None
    disable_edit_message: bool | None
    chat_id: int | None
    message_id: int | None
    inline_message_id: str | None
type SetGameScoreResponse = Message | Literal[True]
class GetGameHighScoresRequest(TypedDict):
    user_id: int
    chat_id: int | None
    message_id: int | None
    inline_message_id: str | None
type GetGameHighScoresResponse = list[GameHighScore]
class GameHighScore(TypedDict):
    position: int
    user: User
    score: int
