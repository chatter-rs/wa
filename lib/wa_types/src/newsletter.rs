use std::collections::HashMap;
use std::fmt;

use macros::serde_derive_de_from_str;
use serde::Deserialize;
use strum::{Display, EnumString};
use utils::serde::jsontime::{UnixMicroString, UnixString};

use crate::{jid::JID, message::MessageServerID, user::ProfilePictureInfo};

#[derive(Clone, Debug, Display, EnumString)]
pub enum NewsletterVerificationState {
    #[strum(to_string = "verified", ascii_case_insensitive = true)]
    Verified,
    #[strum(to_string = "unverified", ascii_case_insensitive = true)]
    Unverified,
    #[strum(default)]
    UnknownVariant(String),
}

serde_derive_de_from_str!(NewsletterVerificationState);

#[derive(Clone, Debug, Display, EnumString)]
pub enum NewsletterPrivacy {
    #[strum(to_string = "private", ascii_case_insensitive = true)]
    Private,
    #[strum(to_string = "public", ascii_case_insensitive = true)]
    Public,
    #[strum(default)]
    UnknownVariant(String),
}

serde_derive_de_from_str!(NewsletterPrivacy);

#[derive(Clone, Debug, Display, EnumString)]
pub enum NewsletterReactionsMode {
    #[strum(to_string = "all")]
    All,
    #[strum(to_string = "basic")]
    Basic,
    #[strum(to_string = "none")]
    None,
    #[strum(to_string = "blocklist")]
    Blocklist,
    #[strum(default)]
    UnknownVariant(String),
}

serde_derive_de_from_str!(NewsletterReactionsMode);

#[derive(Clone, Debug, Display, EnumString)]
pub enum NewsletterState {
    #[strum(to_string = "active", ascii_case_insensitive = true)]
    Active,
    #[strum(to_string = "suspended", ascii_case_insensitive = true)]
    Suspended,
    #[strum(to_string = "geosuspended", ascii_case_insensitive = true)]
    GeoSuspended,
    #[strum(default)]
    UnknownVariant(String),
}

serde_derive_de_from_str!(NewsletterState);

#[derive(Clone, Debug, Display, EnumString)]
pub enum NewsletterMuteState {
    #[strum(to_string = "on", ascii_case_insensitive = true)]
    On,
    #[strum(to_string = "off", ascii_case_insensitive = true)]
    Off,
    #[strum(default)]
    UnknownVariant(String),
}

serde_derive_de_from_str!(NewsletterMuteState);

#[derive(Clone, Debug, Display, EnumString)]
pub enum NewsletterRole {
    #[strum(to_string = "subscriber", ascii_case_insensitive = true)]
    Subscriber,
    #[strum(to_string = "guest", ascii_case_insensitive = true)]
    Guest,
    #[strum(to_string = "admin", ascii_case_insensitive = true)]
    Admin,
    #[strum(to_string = "owner", ascii_case_insensitive = true)]
    Owner,
    #[strum(default)]
    UnknownVariant(String),
}

serde_derive_de_from_str!(NewsletterRole);

#[derive(Clone, Debug, Deserialize)]
pub struct WrappedNewsletterState {
    #[serde(rename = "type")]
    pub r#type: NewsletterState,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NewsletterMetadata {
    pub id: JID,
    pub state: WrappedNewsletterState,
    pub thread_metadata: NewsletterThreadMetadata,
    pub viewer_metadata: Option<NewsletterViewerMetadata>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NewsletterViewerMetadata {
    pub mute: NewsletterMuteState,
    pub role: NewsletterRole,
}

#[derive(Clone, Debug, Display, EnumString)]
pub enum NewsletterKeyType {
    #[strum(to_string = "JID")]
    JID,
    #[strum(to_string = "INVITE")]
    Invite,
    #[strum(default)]
    UnknownVariant(String),
}

serde_derive_de_from_str!(NewsletterKeyType);

#[derive(Clone, Debug, Deserialize)]
pub struct NewsletterReactionSettings {
    pub value: NewsletterReactionsMode,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NewsletterSettings {
    pub reaction_codes: NewsletterReactionSettings,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NewsletterThreadMetadata {
    pub creation_time: UnixString,
    #[serde(rename = "invite")]
    pub invite_code: String,
    pub name: NewsletterText,
    pub description: NewsletterText,
    #[serde(deserialize_with = "macros::serde_de_number_from_string")]
    pub subscriber_count: i64,
    #[serde(rename = "verification")]
    pub verification_state: NewsletterVerificationState,
    pub picture: Option<ProfilePictureInfo>,
    pub preview: ProfilePictureInfo,
    pub settings: NewsletterSettings,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NewsletterText {
    pub text: String,
    pub id: String,
    pub update_time: UnixMicroString,
}

#[derive(Clone, Debug)]
pub struct NewsletterMessage {
    pub message_server_id: MessageServerID,
    pub views_count: u64,
    pub reaction_counts: HashMap<String, u64>,

    /// This is only present when fetching messages, not in live updates.
    pub message: Option<wa_proto::items::wa_web_protobufs_e2e::Message>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GraphQLErrorExtensions {
    pub error_code: i32,
    pub is_retryable: bool,
    pub severity: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GraphQLError {
    pub extensions: GraphQLErrorExtensions,
    pub message: String,
    pub path: Vec<String>,
}

impl fmt::Display for GraphQLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} ({})",
            self.extensions.error_code, self.message, self.extensions.severity
        )
    }
}

impl std::error::Error for GraphQLError {}

#[derive(Clone, Debug, Deserialize)]
pub struct GraphQLErrors(pub Vec<GraphQLError>);

impl fmt::Display for GraphQLErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            return Ok(());
        }
        write!(f, "{} (and {} other errors)", self.0[0], self.0.len() - 1,)
    }
}

impl std::error::Error for GraphQLErrors {}

#[derive(Clone, Debug, Deserialize)]
pub struct GraphQLResponse {
    pub data: serde_json::Value,
    pub errors: GraphQLErrors,
}
