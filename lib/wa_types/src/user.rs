use std::collections::HashMap;

use macros::serde_derive_de_from_str;
use serde::Deserialize;
use strum::{Display, EnumString};
use subenum::subenum;
use wa_proto::items::wa_web_protobufs_vname_cert::{
    verified_name_certificate::Details, VerifiedNameCertificate,
};

use crate::jid::JID;

/// [`VerifiedName`] contains verified WhatsApp business details.
#[derive(Clone, Debug)]
pub struct VerifiedName {
    pub certificate: VerifiedNameCertificate,
    pub details: Details,
}

/// [`UserInfo`] contains the info about a WhatsApp user.
#[derive(Clone, Debug)]
pub struct UserInfo {
    pub verified_name: VerifiedName,
    pub status: String,
    pub picture_id: String,
    pub devices: JID,
}

#[derive(Clone, Debug, Display, EnumString)]
pub enum ProfilePictureType {
    #[strum(to_string = "image")]
    FullResolution,
    #[strum(to_string = "preview")]
    Thumbnail,
    #[strum(default)]
    UnknownVariant(String),
}

serde_derive_de_from_str!(ProfilePictureType);

/// [`ProfilePictureInfo`] contains the ID and the URL for a WhatsApp user's profile picture or group's photo.
#[derive(Clone, Debug, Deserialize)]
pub struct ProfilePictureInfo {
    /// The full URL for the image, can be downloaded with a simple HTTP request.
    pub url: String,
    /// The ID of the image. This is the same as [`UserInfo`].picture_id.
    pub id: String,
    /// The quality of the image.
    #[serde(rename = "type")]
    pub r#type: ProfilePictureType,

    /// The path to the image, probably not very useful.
    pub direct_path: String,
}

/// [`ContactInfo`] contains the cached names of a WhatsApp user.
#[derive(Clone, Debug)]
pub struct ContactInfo {
    pub first_name: String,
    pub full_name: String,
    pub push_name: String,
    pub business_name: String,
}

/// [`LocalChatSettings`] contains the cached local settings for a chat.
#[derive(Clone, Debug)]
pub struct LocalChatSettings {
    pub muted_until: time::OffsetDateTime,
    pub pinned: bool,
    pub archived: bool,
}

/// [`IsOnWhatsAppResponse`] contains information received in response to checking if a phone number is on WhatsApp.
#[derive(Clone, Debug)]
pub struct IsOnWhatsAppResponse {
    /// The query string used.
    pub query: String,
    /// The canonical user ID.
    pub jid: JID,
    /// Whether the phone is registered or not.
    pub is_in: bool,

    /// If the phone is a business, the verified business details.
    pub verified_name: Option<VerifiedName>,
}

/// [`BusinessMessageLinkTarget`] contains the info that is found using a business message link.
#[derive(Clone, Debug)]
pub struct BusinessMessageLinkTarget {
    /// The JID of the business.
    pub jid: JID,

    /// The notify / push name of the business.
    pub push_name: String,
    /// The verified business name.
    pub verified_name: String,
    /// Some boolean, seems to be true?
    pub is_signed: bool,
    /// Tulir guesses the level of verification, starting from "unknown".
    pub verified_level: String,

    /// The message that WhatsApp clients will pre-fill in the input box when clicking the link.
    pub message: String,
}

/// [`ContactQRLinkTarget`] contains the info that is found using a contact QR link.
#[derive(Clone, Debug)]
pub struct ContactQRLinkTarget {
    pub jid: JID,
    /// Might always be "contact".
    pub r#type: String,
    /// The notify / push name of the user.
    pub push_name: String,
}

/// [`PrivacySetting`] is an individual setting value in the user's privacy settings.
#[subenum(
    PrivacySettingGroupAdd,
    PrivacySettingLastSeen,
    PrivacySettingStatus,
    PrivacySettingProfile,
    PrivacySettingReadReceipts,
    PrivacySettingOnline,
    PrivacySettingCallAdd
)]
#[derive(Clone, Debug, Display, EnumString, PartialEq)]
pub enum PrivacySetting {
    #[strum(to_string = "")]
    Undefined,
    #[subenum(
        PrivacySettingGroupAdd,
        PrivacySettingLastSeen,
        PrivacySettingStatus,
        PrivacySettingProfile,
        PrivacySettingReadReceipts,
        PrivacySettingOnline,
        PrivacySettingCallAdd
    )]
    #[strum(to_string = "all")]
    All,
    #[subenum(
        PrivacySettingGroupAdd,
        PrivacySettingLastSeen,
        PrivacySettingStatus,
        PrivacySettingProfile
    )]
    #[strum(to_string = "contacts")]
    Contacts,
    #[subenum(
        PrivacySettingGroupAdd,
        PrivacySettingLastSeen,
        PrivacySettingStatus,
        PrivacySettingProfile
    )]
    #[strum(to_string = "contact_blacklist")]
    ContactBlacklist,
    #[subenum(PrivacySettingOnline)]
    #[strum(to_string = "match_last_seen")]
    MatchLastSeen,
    #[subenum(PrivacySettingCallAdd)]
    #[strum(to_string = "known")]
    Known,
    #[subenum(
        PrivacySettingGroupAdd,
        PrivacySettingLastSeen,
        PrivacySettingStatus,
        PrivacySettingProfile,
        PrivacySettingReadReceipts
    )]
    #[strum(to_string = "none")]
    None,
    #[strum(default)]
    UnknownVariant(String),
}

/// [`PrivacySettingType`] is the type of privacy setting.
#[derive(Clone, Debug, Display, EnumString)]
pub enum PrivacySettingType {
    #[strum(to_string = "groupadd")]
    GroupAdd,
    #[strum(to_string = "last")]
    LastSeen,
    #[strum(to_string = "status")]
    Status,
    #[strum(to_string = "profile")]
    Profile,
    #[strum(to_string = "readreceipts")]
    ReadReceipts,
    #[strum(to_string = "online")]
    Online,
    #[strum(to_string = "calladd")]
    CallAdd,
    #[strum(default)]
    UnknownVariant(String),
}

/// [`PrivacySettings`] contains the user's privacy settings.
#[derive(Clone, Debug)]
pub struct PrivacySettings {
    pub group_add: PrivacySettingGroupAdd,
    pub last_seen: PrivacySettingLastSeen,
    pub status: PrivacySettingStatus,
    pub profile: PrivacySettingProfile,
    pub read_receipts: PrivacySettingReadReceipts,
    pub call_add: PrivacySettingCallAdd,
    pub online: PrivacySettingOnline,
}

/// [`StatusPrivacyType`] is the type of list in [`StatusPrivacy`].
#[derive(Clone, Debug, Display, EnumString)]
pub enum StatusPrivacyType {
    #[strum(to_string = "contacts")]
    Contacts,
    #[strum(to_string = "blacklist")]
    Blacklist,
    #[strum(to_string = "whitelist")]
    Whitelist,
    #[strum(default)]
    UnknownVariant(String),
}

/// [`StatusPrivacy`] contains the settings for who to send status messages to by default.
#[derive(Clone, Debug)]
pub struct StatusPrivacy {
    pub r#type: StatusPrivacyType,
    pub list: Vec<JID>,

    pub is_default: bool,
}

/// [`Blocklist`] contains the user's current list of blocked users.
#[derive(Clone, Debug)]
pub struct Blocklist {
    pub hash: String,
    pub jids: Vec<JID>,
}

/// [`BusinessHoursConfig`] contains business operating hours of a WhatsApp business.
#[derive(Clone, Debug)]
pub struct BusinessHoursConfig {
    pub day_of_week: String,
    pub mode: String,
    pub open_time: String,
    pub close_time: String,
}

/// [`BusinessCategory`] contains a WhatsApp business category.
#[derive(Clone, Debug)]
pub struct BusinessCategory {
    pub id: String,
    pub name: String,
}

/// [`BusinessProfile`] contains the profile information of a WhatsApp business.
#[derive(Clone, Debug)]
pub struct BusinessProfile {
    pub jid: JID,
    pub address: String,
    pub email: String,
    pub categories: Vec<BusinessCategory>,
    pub profile_options: HashMap<String, String>,
    pub business_hours_time_zone: String,
    pub business_hours: Vec<BusinessHoursConfig>,
}
