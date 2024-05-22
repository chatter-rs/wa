pub const DEFAULT_USER_SERVER: &str = "s.whatsapp.net";
pub const GROUP_SERVER: &str = "g.us";
pub const LEGACY_USER_SERVER: &str = "c.us";
pub const BROADCAST_SERVER: &str = "broadcast";
pub const HIDDEN_USER_SERVER: &str = "lid";
pub const MESSENGER_SERVER: &str = "msgr";
pub const INTEROP_SERVER: &str = "interop";
pub const NEWSLETTER_SERVER: &str = "newsletter";
pub const HOSTED_SERVER: &str = "hosted";

#[derive(Clone, Debug)]
pub struct JID {
    pub user: String,
    pub raw_agent: u8,
    pub device: u16,
    pub server: String,
    pub integrator: u16,
}

impl JID {
    pub fn actual_agent(&self) -> u8 {
        match self.server.as_str() {
            DEFAULT_USER_SERVER => 0,
            HIDDEN_USER_SERVER => 1,
            _ => self.raw_agent,
        }
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
}
