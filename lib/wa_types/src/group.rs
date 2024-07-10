use strum::{Display, EnumString};

use crate::jid::JID;

#[derive(Clone, Debug, Display, EnumString)]
pub enum GroupMemberAddMode {
    #[strum(to_string = "admin_add")]
    Admin,
    #[strum(default)]
    UnknownVariant(String),
}

/// [`GroupInfo`] contains basic information about a group chat on WhatsApp.
#[derive(Clone, Debug)]
pub struct GroupInfo {
    pub jid: JID,
    pub owner_jid: JID,

    pub name: GroupName,
    pub topic: GroupTopic,
    pub locked: GroupLocked,
    pub announce: GroupAnnounce,
    pub ephemeral: GroupEphemeral,
    pub incognito: GroupIncognito,

    pub parent: GroupParent,
    pub linked_parent: GroupLinkedParent,
    pub is_default_sub: GroupIsDefaultSub,

    pub group_created: time::OffsetDateTime,

    pub participant_version_id: String,
    pub participants: Vec<GroupParticipant>,

    pub member_add_mode: GroupMemberAddMode,
}

#[derive(Clone, Debug)]
pub struct GroupParent {
    pub is_parent: bool,
    pub default_membership_approval_mode: String, // request_required
}

#[derive(Clone, Debug)]
pub struct GroupLinkedParent {
    pub linked_parent_jid: JID,
}

#[derive(Clone, Debug)]
pub struct GroupIsDefaultSub {
    pub is_default_sub_group: bool,
}

/// [`GroupName`] contains the name of a group along with metadata of who set it and when.
#[derive(Clone, Debug)]
pub struct GroupName {
    pub name: String,
    pub name_set_at: time::OffsetDateTime,
    pub name_set_by: JID,
}

/// [`GroupTopic`] contains the topic (description) of a group along with metadata of who set it and when.
#[derive(Clone, Debug)]
pub struct GroupTopic {
    pub topic: String,
    pub topic_id: String,
    pub topic_set_at: time::OffsetDateTime,
    pub topic_set_by: JID,
    pub topic_deleted: bool,
}

/// [`GroupLocked`] specifies whether the group info can only be edited by admins.
#[derive(Clone, Debug)]
pub struct GroupLocked {
    pub is_locked: bool,
}

/// [`GroupAnnounce`] specifies whether only admins can send messages in the group.
#[derive(Clone, Debug)]
pub struct GroupAnnounce {
    pub is_announce: bool,
    pub announce_version_id: String,
}

#[derive(Clone, Debug)]
pub struct GroupIncognito {
    pub is_incognito: bool,
}

/// [`GroupParticipant`] contains info about a participant of a WhatsApp group chat.
#[derive(Clone, Debug)]
pub struct GroupParticipant {
    pub jid: JID,
    pub lid: JID,
    pub is_admin: bool,
    pub is_super_admin: bool,

    /// This is only present for anonymous users in announcement groups, it's an obfuscated phone
    /// number.
    pub display_name: Option<String>,

    /// When creating a group, adding some participants may fail. In such cases, the error code
    /// will be here.
    pub error: Option<i32>,
    /// When creating a group, adding some participants may fail. In such cases, the original
    /// add request will be here.
    pub add_request: Option<GroupPartipantAddRequest>,
}

#[derive(Clone, Debug)]
pub struct GroupPartipantAddRequest {
    pub code: String,
    pub expiration: time::OffsetDateTime,
}

/// [`GroupEphemeral`] contains the group's disappearing messages settings.
#[derive(Clone, Debug)]
pub struct GroupEphemeral {
    pub is_ephemeral: bool,
    pub disappearing_timer: u64,
}

#[derive(Clone, Debug)]
pub struct GroupDelete {
    pub deleted: bool,
    pub reason: String,
}

#[derive(Clone, Debug, Display, EnumString)]
pub enum GroupLinkChangeType {
    #[strum(to_string = "parent_group")]
    Parent,
    #[strum(to_string = "sub_group")]
    Sub,
    #[strum(to_string = "sibling_group")]
    Sibling,
    #[strum(default)]
    UnknownVariant(String),
}

#[derive(Clone, Debug, Display, EnumString)]
pub enum GroupUnlinkReason {
    #[strum(to_string = "unlink_group")]
    Default,
    #[strum(to_string = "delete_parent")]
    Delete,
    #[strum(default)]
    UnknownVariant(String),
}

#[derive(Clone, Debug)]
pub struct GroupLinkTarget {
    pub jid: JID,
    pub name: GroupName,
    pub is_default_sub: GroupIsDefaultSub,
}

#[derive(Clone, Debug)]
pub struct GroupLinkChange {
    pub r#type: GroupLinkChangeType,
    pub unlink_reason: GroupUnlinkReason,
    pub group: GroupLinkTarget,
}
