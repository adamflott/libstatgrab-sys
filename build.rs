fn main() {
    println!("cargo:rustc-link-lib=statgrab");
    // TODO use pkg-config
    println!("cargo:rustc-link-search=/nix/store/qwyr9n73avfk0g7slfvipmzwsljgxswl-libstatgrab-0.92.1/include");
    println!("cargo:rustc-link-search=/nix/store/qwyr9n73avfk0g7slfvipmzwsljgxswl-libstatgrab-0.92.1/lib");

}
