use thiserror::Error;

/// [`ORIGIN`] is the Origin header for all WhatsApp websocket connections.
pub const ORIGIN: &str = "https://web.whatsapp.com";
/// [`URL`] is the websocket URL for the new multidevice protocol.
pub const URL: &str = "wss://web.whatsapp.com/ws/chat";

pub const NOISE_START_PATTERN: &str = "Noise_XX_25519_AESGCM_SHA256\x00\x00\x00\x00";
pub const WA_MAGIC_VALUE: u8 = 6;

pub const WA_CONN_HEADER: [u8; 4] = [b'W', b'A', WA_MAGIC_VALUE, wa_binary::token::DICT_VERSION];

pub const FRAME_MAX_SIZE: usize = 2 << 23;
pub const FRAME_LENGTH_SIZE: usize = 3;

#[derive(Error, Clone, Debug)]
pub enum SocketError {
    #[error("frame too large")]
    FrameTooLarge,
    #[error("frame socket is closed")]
    SocketClosed,
    #[error("frame socket is already open")]
    SocketAlreadyOpen,
}
