fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        println!("cargo:rerun-if-changed=resource.rc");
        println!("cargo:rerun-if-changed=ichi.ico");
        embed_resource::compile("resource.rc", embed_resource::NONE)
            .manifest_optional()
            .expect("Failed to compile resources");
    }
}
