use std::env;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

fn main() {
    let interrupts = env::var_os("CARGO_FEATURE_INTERRUPTS").is_some();
    // let static_ram = env::var_os("CARGO_FEATURE_STATIC_RAM").is_some();

    // Pass our linker script to the top crate
    let script = "1bitsy.ld";
    let script_int_txt = "interrupts_text.ld";
    let script_int_sym = "interrupts_sym.ld";
    let src = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    fs::copy(src.join(script), out.join(script)).unwrap();

    if interrupts {
        fs::copy(src.join(script_int_txt), out.join(script_int_txt)).unwrap();
        fs::copy(src.join(script_int_sym), out.join(script_int_sym)).unwrap();
    } else {
        File::create(out.join(script_int_txt));
        File::create(out.join(script_int_sym));
    }

    // if static_ram {
    //     fs::
    // } else {
    //
    // }

    println!("cargo:rustc-link-search={}", out.display());
}
