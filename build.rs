use std::env;
use std::path::Path;

#[cfg(feature = "svbony")]
fn link_svbony() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&dir).join("SVBCameraSDK/lib/arm64").display()
    );
    println!("cargo:rustc-link-lib=static=SVBCameraSDK");
}

fn main() {
    #[cfg(feature = "svbony")]
    link_svbony();
}
