#![recursion_limit = "256"]
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    portfolio::server::run().await;
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // WASM entry point is in lib.rs
}
