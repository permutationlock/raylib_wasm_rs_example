fn main() {
    use std::path::Path;
    let arch   = build_target::target_arch().unwrap();
    if arch == build_target::Arch::WASM32 {
        let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        println!(
            "cargo:rustc-link-search=native={}",
            Path::new(&dir).join("lib_wasm").display()
        );
    }
}
