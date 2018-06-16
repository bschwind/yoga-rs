extern crate bindgen;
extern crate cc;

use bindgen::RustTarget;
use cc::Build;
use std::env;
use std::path::PathBuf;

fn main() {
	let mut c = Build::new();

	c.warnings(false);
	c.cpp(true);
	c.flag("-std=c++11");
	c.file("src/c/Utils.cpp");
	c.file("src/c/YGConfig.cpp");
	c.file("src/c/YGEnums.cpp");
	c.file("src/c/YGFloatOptional.cpp");
	c.file("src/c/YGLayout.cpp");
	c.file("src/c/YGNode.cpp");
	c.file("src/c/YGNodePrint.cpp");
	c.file("src/c/YGStyle.cpp");
	c.file("src/c/Yoga.cpp");
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
