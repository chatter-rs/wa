use strum::{Display, EnumString};

#[derive(Clone, Debug, Display, EnumString)]
pub enum Presence {
    #[strum(to_string = "available")]
    Available,
    #[strum(to_string = "unavailable")]
    Unavailable,
    #[strum(default)]
    UnknownVariant(String),
}

#[derive(Clone, Debug, Display, EnumString)]
pub enum ChatPresence {
    #[strum(to_string = "composing")]
    Composing,
    #[strum(to_string = "paused")]
    Paused,
    #[strum(default)]
    UnknownVariant(String),
}

#[derive(Clone, Debug, Display, EnumString)]
pub enum ChatPresenceMedia {
    #[strum(to_string = "")]
    Text,
    #[strum(to_string = "audio")]
    Audio,
    #[strum(default)]
    UnknownVariant(String),
}

#[derive(Clone, Debug, Display, EnumString)]
pub enum ReceiptType {
    /// [`ReceiptType::Delivered`] means the message was delivered to the device (but the user might not have noticed).
    #[strum(to_string = "")]
    Delivered,
    /// [`ReceiptType::Sender`] is sent by your devices when a message you sent is delivered to them.
    #[strum(to_string = "sender")]
    Sender,
    /// [`ReceiptType::Retry`] means the message was delivered to the device, but decrypting the message failed.
    #[strum(to_string = "retry")]
    Retry,
    /// [`ReceiptType::Read`] means the user opened the chat and saw the message.
    #[strum(to_string = "read")]
    Read,
    /// [`ReceiptType::ReadSelf`] means the current user read a message from a different device, and has read receipts disabled in privacy settings.
    #[strum(to_string = "read-self")]
    ReadSelf,
    /// [`ReceiptType::Played`] means the user opened a view-once media message.
    ///
    /// This is dispatched for both incoming and outgoing messages when played. If the current
    /// user opened the media, it means media should be removed from all devices. If a recipient
    /// opened a media, it's just a notification for the sender that the media was viewed.
    #[strum(to_string = "played")]
    Played,
    /// [`ReceiptType::PlayedSelf`] probably means the current user opened a view-once media
    /// message from a different device, and has read receipts disabled in privacy settings.
    #[strum(to_string = "played-self")]
    PlayedSelf,
    #[strum(to_string = "server-error")]
    ServerError,
    #[strum(to_string = "inactive")]
    Inactive,
    #[strum(to_string = "peer_msg")]
    PeerMsg,
    #[strum(to_string = "hist_sync")]
    HistorySync,
    #[strum(default)]
    UnknownVariant(String),
}
