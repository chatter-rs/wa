mod version;

pub use version::PROTO_VERSION;

pub mod items {
    include!(concat!(env!("OUT_DIR"), "/wa_proto.rs"));
}
