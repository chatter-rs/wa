pub const PROTO_VERSION: [u32; 3] = [2, 3000, 1014340090];

pub mod items {
    include!(concat!(env!("OUT_DIR"), "/mod_items.rs"));
}
