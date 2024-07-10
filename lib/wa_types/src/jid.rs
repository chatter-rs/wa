use std::{fmt, num::ParseIntError, str::FromStr};

use macros::{serde_derive_de_from_str, serde_derive_se_to_string};
use thiserror::Error;

pub const DEFAULT_USER_SERVER: &str = "s.whatsapp.net";
pub const GROUP_SERVER: &str = "g.us";
pub const LEGACY_USER_SERVER: &str = "c.us";
pub const BROADCAST_SERVER: &str = "broadcast";
pub const HIDDEN_USER_SERVER: &str = "lid";
pub const MESSENGER_SERVER: &str = "msgr";
pub const INTEROP_SERVER: &str = "interop";
pub const NEWSLETTER_SERVER: &str = "newsletter";
pub const HOSTED_SERVER: &str = "hosted";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JID {
    pub user: String,
    pub raw_agent: u8,
    pub device: u16,
    pub server: String,
    pub integrator: u16,
}

serde_derive_de_from_str!(JID);
serde_derive_se_to_string!(JID);

impl JID {
    /// Creates a new regular JID.
    pub fn new(user: String, server: String) -> Self {
        JID {
            user,
            server,
            raw_agent: 0,
            device: 0,
            integrator: 0,
        }
    }

    /// Creates a new AD JID.
    pub fn new_ad_jid(user: String, agent: u8, device: u8) -> Self {
        let (server, raw_agent) = match agent {
            0 => (DEFAULT_USER_SERVER.to_string(), agent),
            1 => (HIDDEN_USER_SERVER.to_string(), 0),
            _ => (HOSTED_SERVER.to_string(), agent),
        };
        JID {
            user,
            raw_agent,
            server,
            device: device as u16,
            integrator: 0,
        }
    }

    pub fn actual_agent(&self) -> u8 {
        match self.server.as_str() {
            DEFAULT_USER_SERVER => 0,
            HIDDEN_USER_SERVER => 1,
            _ => self.raw_agent,
        }
    }

    /// Returns the user as an integer. This is only safe to run on normal users, not on groups
    /// or broadcast lists.
    pub fn user_int(&self) -> u64 {
        self.user.parse::<u64>().unwrap()
    }

    /// Returns a version of the [`JID`] struct that doesn't have the agent and device set.
    pub fn to_non_ad(self) -> Self {
        Self {
            user: self.user,
            server: self.server,
            integrator: self.integrator,
            raw_agent: 0,
            device: 0,
        }
    }

    /// Returns the Signal protocol address for the user.
    pub fn signal_address(&self) -> libsignal_protocol::ProtocolAddress {
        let agent = self.actual_agent();
        let user = if agent == 0 {
            self.user.clone()
        } else {
            format!("{}_{}", self.user, agent)
        };
        libsignal_protocol::ProtocolAddress::new(
            user,
            libsignal_protocol::DeviceId::from(self.device as u32),
        )
    }

    /// Returns true if the [`JID`] has no server (which is required for all JIDs).
    pub fn is_empty(&self) -> bool {
        self.server.is_empty()
    }

    /// Returns true if the JID is a broadcast list, but not the status broadcast.
    pub fn is_broadcast_list(&self) -> bool {
        self.server == BROADCAST_SERVER && self.user != "status"
    }

    pub fn ad_string(&self) -> String {
        format!(
            "{}.{}:{}@{}",
            self.user, self.raw_agent, self.device, self.server
        )
    }
}

#[derive(Error, Debug)]
pub enum JIDParseError {
    #[error("unexpected number of dots")]
    UnexpectedDots,
    #[error("unexpected number of colons")]
    UnexpectedColons,
    #[error("failed to parse agent: {0}")]
    InvalidAgent(ParseIntError),
    #[error("failed to parse device: {0}")]
    InvalidDevice(ParseIntError),
}

impl FromStr for JID {
    type Err = JIDParseError;

    fn from_str(jid: &str) -> Result<Self, Self::Err> {
        let parts = jid.split('@').collect::<Vec<_>>();
        if parts.len() == 1 {
            return Ok(JID::new("".to_string(), parts[0].to_string()));
        }

        let mut parsed_jid = JID::new(parts[0].to_string(), parts[1].to_string());
        if parsed_jid.user.contains('.') {
            let parts = parsed_jid
                .user
                .split('.')
                .map(String::from)
                .collect::<Vec<_>>();
            if parts.len() != 2 {
                return Err(JIDParseError::UnexpectedDots);
            }
            parsed_jid.user.clone_from(&parts[0]);
            let ad = &parts[1];
            let parts = ad.split(':').collect::<Vec<_>>();
            if parts.len() > 2 {
                return Err(JIDParseError::UnexpectedColons);
            }
            let agent = parts[0]
                .parse::<u8>()
                .map_err(JIDParseError::InvalidAgent)?;
            parsed_jid.raw_agent = agent;
            if parts.len() == 2 {
                let device = parts[1]
                    .parse::<u16>()
                    .map_err(JIDParseError::InvalidDevice)?;
                parsed_jid.device = device;
            }
        } else if parsed_jid.user.contains(':') {
            let parts = parsed_jid
                .user
                .split(':')
                .map(String::from)
                .collect::<Vec<_>>();
            if parts.len() != 2 {
                return Err(JIDParseError::UnexpectedColons);
            }
            parsed_jid.user.clone_from(&parts[0]);
            let device = parts[1]
                .parse::<u16>()
                .map_err(JIDParseError::InvalidDevice)?;
            parsed_jid.device = device;
        }
        Ok(parsed_jid)
    }
}

impl fmt::Display for JID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.raw_agent > 0 {
            write!(
                f,
                "{}.{}:{}@{}",
                self.user, self.raw_agent, self.device, self.server
            )
        } else if self.device > 0 {
            write!(f, "{}:{}@{}", self.user, self.device, self.server,)
        } else if !self.user.is_empty() {
            write!(f, "{}@{}", self.user, self.server)
        } else {
            write!(f, "{}", self.server)
        }
    }
}
