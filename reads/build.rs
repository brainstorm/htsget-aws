
fn main() {
    let mut cfg = cc::Build::new();
    cfg.warnings(true).static_flag(true).pic(true);
    println!("cargo:rustc-link-lib=static=hts");
    //println!("cargo:rustc-link-lib=static=z");
    //println!("cargo:rustc-link-lib=static=lzma");
    //println!("cargo:rustc-link-lib=static=dl");
    //println!("cargo:rustc-link-lib=static=util");
    //println!("cargo:rustc-link-lib=compiler_rt");
    //println!("cargo:rustc-link-lib=static=pthread");
}
