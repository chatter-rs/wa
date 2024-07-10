use strum::{Display, EnumString};

use crate::jid::JID;
use crate::user::VerifiedName;

/// [`MessageID`] is the internal ID of a WhatsApp message.
#[derive(Clone, Debug)]
pub struct MessageID(pub String);

/// [`MessageServerID`] is the server ID of a WhatsApp newsletter message.
#[derive(Clone, Debug)]
pub struct MessageServerID(pub String);

/// [`MessageSource`] contains basic sender and chat information about a message.
#[derive(Clone, Debug)]
pub struct MessageSource {
    /// The chat where the message was sent.
    pub chat: JID,
    /// The user who sent the message.
    pub sender: JID,
    /// Whether the message was sent by the current user instead of someone else.
    pub is_from_me: bool,
    /// Whether the chat is a group chat or broadcast list.
    pub is_group: bool,

    /// When sending a read receipt to a broadcast list message, the chat is the broadcast list
    /// and sender is you, so the field contains the recipient of the read receipt.
    pub broadcast_list_owner: JID,
}

impl MessageSource {
    /// Returns true if the message was sent to a broadcast list instead of directly to the user.
    ///
    /// If this is true, it means the message shows up in the direct chat with the sender.
    pub fn is_incoming_broadcast(&self) -> bool {
        (!self.is_from_me || !self.broadcast_list_owner.is_empty()) && self.chat.is_broadcast_list()
    }

    /// Returns a log-friendly representation of who sent the message and where.
    pub fn source_string(&self) -> String {
        if self.sender != self.chat {
            format!("{} in {}", self.sender, self.chat)
        } else {
            self.chat.to_string()
        }
    }
}

/// [`DeviceSentMeta`] contains metadata from messages sent by another one of the user's own devices.
#[derive(Clone, Debug)]
pub struct DeviceSentMeta {
    /// The destination user. This should match the [`MessageInfo`].recipient field.
    pub destination_jid: String,
    pub phash: String,
}

#[derive(Clone, Debug, Display, EnumString)]
pub enum EditAttribute {
    #[strum(to_string = "")]
    Empty,
    #[strum(to_string = "1")]
    MessageEdit,
    #[strum(to_string = "2")]
    PinInChat,
    /// Only used in newsletters.
    #[strum(to_string = "3")]
    AdminEdit,
    #[strum(to_string = "7")]
    SenderRevoke,
    #[strum(to_string = "8")]
    AdminRevoke,
    #[strum(default)]
    UnknownVariant(String),
}

/// [`MessageInfo`] contains metadata about an incoming message.
#[derive(Clone, Debug)]
pub struct MessageInfo {
    pub source: MessageSource,
    pub id: MessageID,
    pub server_id: MessageServerID,
    pub r#type: String,
    pub push_name: String,
    pub timestamp: time::OffsetDateTime,
    pub category: String,
    pub multicast: bool,
    pub media_type: String,
    pub edit: EditAttribute,

    pub verified_name: Option<VerifiedName>,
    /// Metadata for direct messages sent from another one of the user's own devices.
    pub device_sent_meta: Option<DeviceSentMeta>,
}
