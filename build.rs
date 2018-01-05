fn main() {
    #[cfg(any(feature = "gtk", windows))]
    println!("cargo:rustc-cfg=gtk");
}
