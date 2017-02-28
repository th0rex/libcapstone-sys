extern crate bindgen;

use std::env;
use std::io::Write;
use std::path::PathBuf;

#[cfg(not(target_os="windows"))]
fn default_include_dir(_: env::VarError) -> String {
    "/usr/include".to_owned()
}

#[cfg(target_os="windows")]
fn default_include_dir(_: env::VarError) -> String {
    panic!("No default directory for includes in windows")
}

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let in_path = PathBuf::from(env::var("CAPSTONE_INCLUDE_DIR")
        .unwrap_or_else(default_include_dir));

    let wrapper = out_path.join("wrapper.h");
    {
        let mut file = std::fs::File::create(&wrapper).unwrap();
        file.write_all(b"#include <capstone/capstone.h>").unwrap();
    }

    println!("cargo:rerun-if-changed={}",
             in_path.join("capstone").join("capstone.h").display());

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(wrapper.to_str().unwrap())
        .clang_arg(format!("-I{}", in_path.display()))
        .link("capstone")
        .generate_comments(true)
        .bitfield_enum("cs_.*")
        .bitfield_enum("x86_.*")
        .bitfield_enum("arm_.*")
        .bitfield_enum("arm64_.*")
        .bitfield_enum("mips_.*")
        .bitfield_enum("ppc_.*")
        .bitfield_enum("sparc_.*")
        .bitfield_enum("sysz_.*")
        .bitfield_enum("xcore_.*")
        // Only generate bindings for all types that start with cs_ and types included by them.
        // This avoids generating bindings for standard c functions.
        .whitelisted_type("cs_.*")
        .whitelisted_function("cs_.*")
        .prepend_enum_name(false)
        .raw_line(DEBUG_IMPL)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // TODO: For some reason .link("capstone") won't work.
    // Remove this once it does again.
    // TODO: Add support for library search paths.
    println!("cargo:rustc-link-lib=dylib=capstone");
}

const DEBUG_IMPL: &'static str =
    "
use std::fmt;
impl fmt::Debug for cs_insn {
    fn fmt(&self, f: &mut fmt::Formatter) -> \
     fmt::Result {
        f.debug_struct(\"cs_insn\")
            .field(\"id\", &self.id)
            \
     .field(\"address\", &self.address)
            .field(\"size\", &self.size)
            \
     .field(\"bytes\", &self.bytes)
            .field(\"mnemonic\", &self.mnemonic)
            \
     .field(\"detail\", &self.detail)
            .finish()
    }
}
";