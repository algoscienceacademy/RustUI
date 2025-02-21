use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    match target_os.as_str() {
        "ios" => {
            println!("cargo:rustc-link-lib=framework=UIKit");
            println!("cargo:rustc-link-lib=framework=Metal");
            println!("cargo:rustc-link-lib=framework=QuartzCore");
        }
        "android" => {
            println!("cargo:rustc-link-lib=dylib=GLESv3");
            println!("cargo:rustc-link-lib=dylib=EGL");
            println!("cargo:rustc-link-lib=dylib=android");
        }
        _ => {}
    }
}
