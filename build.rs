extern crate bindgen;
extern crate cc;

use bindgen::RustTarget;
use cc::Build;
use std::env;
use std::path::PathBuf;

fn main() {
	let mut c = Build::new();

	c.warnings(false);
	c.flag("-std=c99");
	c.file("src/c/YGEnums.c");
	c.file("src/c/YGNodeList.c");
	c.file("src/c/Yoga.c");
	c.compile("libyoga.a");

	let bindings = bindgen::Builder::default()
		.rust_target(RustTarget::Stable_1_21)
		.blacklist_type("max_align_t") // This fails `cargo test` so disable for now
		.blacklist_type("FP_INFINITE")
		.blacklist_type("FP_NAN")
		.blacklist_type("FP_NORMAL")
		.blacklist_type("FP_SUBNORMAL")
		.blacklist_type("FP_ZERO")
		.rustfmt_bindings(false)
		.rustified_enum("YG.*")
		.header("src/c/wrapper.h")
		.generate()
		.expect("Unable to generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Unable to write bindings!");
}
