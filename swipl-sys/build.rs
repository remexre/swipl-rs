extern crate bindgen;
extern crate pkg_config;

use std::env::var;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use bindgen::Builder;
use pkg_config::Config;

fn main() {
    let out_path = PathBuf::from(var("OUT_DIR").unwrap());

    let program = Config::new()
        .atleast_version("7.6.0")
        .statik(true)
        .probe("swipl")
        .expect("Couldn't find SWI Prolog");

    let header_path = out_path.join("swipl-sys.h");
    File::create(&header_path)
        .and_then(|mut f| writeln!(f, "#include <SWI-Prolog.h>"))
        .expect("Couldn't write the header");

    let flags = program
        .include_paths
        .iter()
        .map(|p| format!("-I{}", p.display()));
    Builder::default()
        .blacklist_type("max_align_t")
        .clang_args(flags)
        .header(header_path.display().to_string())
        .generate()
        .expect("Couldn't generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
