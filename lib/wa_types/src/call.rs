use crate::jid::JID;

#[derive(Clone, Debug)]
pub struct BasicCallMeta {
    pub from: JID,
    pub timestamp: time::OffsetDateTime,
    pub call_creator: JID,
    pub call_id: String,
}

#[derive(Clone, Debug)]
pub struct CallRemoteMeta {
    /// The platform of the caller's WhatsApp client.
    pub remote_platform: String,
    /// The version of the caller's WhatsApp client.
    pub remote_version: String,
}
