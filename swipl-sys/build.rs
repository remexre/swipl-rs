extern crate bindgen;
extern crate flate2;
extern crate hashwriter;
extern crate reqwest;
extern crate sha2;
extern crate tar;

use std::env::var;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use bindgen::Builder;
use flate2::read::GzDecoder;
use hashwriter::HashWriter;
use tar::Archive;

const SWIPL_URL: &str =
    "http://www.swi-prolog.org/download/stable/src/swipl-7.6.4.tar.gz";
const SWIPL_HASH: &[u8] = b"\x2d\x3d\x7a\xab\xd6\xd9\x9a\x02\xdc\xc2\xda\x5d\x76\x04\xe3\x50\x03\x29\xe5\x41\xc6\xf8\x57\xed\xc5\xaa\x06\xa3\xb1\x26\x78\x91";
const SWIPL_SRC_DIR: &str = "swipl-7.6.4";

const HEADER: &str = r#"#include "SWI-Prolog.h"

atom_t PL_ATOM_nil(void) { return ATOM_nil; }
atom_t PL_ATOM_dot(void) { return ATOM_dot; }
"#;

fn main() {
    let out_path = PathBuf::from(var("OUT_DIR").unwrap());
    download_swipl(&out_path);
    extract_swipl(&out_path);
    build_swipl(&out_path);

    let header_path = out_path.join("swipl-sys.h");
    File::create(&header_path)
        .and_then(|mut f| writeln!(f, "{}", HEADER))
        .expect("Couldn't write the header");

    let include_path = out_path.join(SWIPL_SRC_DIR).join("src");

    Builder::default()
        .whitelist_function("PL_.*")
        .whitelist_type("PL_.*")
        .whitelist_var("PL_.*")
        .whitelist_var("(BUF|CVT|REP)_[0-9A-Z_]+")
        .clang_arg("-DPL_ARITY_AS_SIZE")
        .clang_arg(format!("-I{}", include_path.display()))
        .header(header_path.display().to_string())
        .generate()
        .expect("Couldn't generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}

fn download_swipl<P: AsRef<Path>>(out_path: P) {
    let file = File::create(out_path.as_ref().join("swipl.tar.gz"))
        .expect("Couldn't create file for SWIPL source tarball download");
    let mut w: HashWriter<sha2::Sha256, _> = HashWriter::from_writer(file);

    reqwest::get(SWIPL_URL)
        .expect("Couldn't download SWIPL source tarball")
        .error_for_status()
        .expect("Bad response when downloading source tarball")
        .copy_to(&mut w)
        .expect("Error when writing out source tarball");
    let hash = w.digest();
    assert_eq!(hash.as_slice(), SWIPL_HASH, "Download hash didn't match");
}

fn extract_swipl<P: AsRef<Path>>(out_path: P) {
    let out_path = out_path.as_ref();
    let file = File::open(out_path.join("swipl.tar.gz"))
        .expect("Couldn't open SWIPL source tarball");
    Archive::new(GzDecoder::new(file))
        .unpack(out_path)
        .expect("Couldn't extract SWIPL source tarball");
}

fn build_swipl<P: AsRef<Path>>(out_path: P) {
    let out_path = out_path.as_ref();

    let status = Command::new("bash")
        .arg("configure")
        .arg("--without-jpl")
        .current_dir(out_path.join(SWIPL_SRC_DIR))
        .env("CFLAGS", "-g")
        .status()
        .expect("Failed to start configuring SWIPL");
    assert!(status.success(), "Failed to configure SWIPL");

    // TODO: Support other arches
    let status = Command::new("make")
        .arg("-C")
        .arg("src")
        .arg("-j")
        .arg("../lib/x86_64-linux/libswipl.a")
        .current_dir(out_path.join(SWIPL_SRC_DIR))
        .status()
        .expect("Failed to start making SWIPL");
    assert!(status.success(), "Failed to make SWIPL");

    println!("cargo:rustc-link-lib=static=swipl");
    println!(
        "cargo:rustc-link-search={}",
        out_path
            .join(SWIPL_SRC_DIR)
            .join("lib")
            .join("x86_64-linux")
            .display()
    );
}
