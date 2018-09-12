//cargo run --features "baz" for sho baz in cfg.rs
pub fn start() {
    if cfg!(feature = "baz") {
        println!("BAZ");
    }
}
