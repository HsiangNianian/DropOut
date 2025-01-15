fn main() {
    // This build script does nothing for now.
    // You can add custom build logic here if needed.
    println!("cargo:rerun-if-changed=build.rs");
}