extern crate bindgen;
extern crate cc;

use bindgen::RustTarget;
use cc::Build;
use std::env;
use std::path::PathBuf;

fn main() {
	Build::new()
		.cpp(true)
		// https://github.com/facebook/yoga/blob/c5f826de8306e5fbe5963f944c75add827e096c3/BUCK#L13
		.flag("-std=c++1y")
		// https://github.com/facebook/yoga/blob/c5f826de8306e5fbe5963f944c75add827e096c3/yoga_defs.bzl#L49-L56
		.flag("-fno-omit-frame-pointer")
		.flag("-fexceptions")
		.flag("-Wall")
		.flag("-O3")
		.flag("-ffast-math")
		// https://github.com/facebook/yoga/blob/c5f826de8306e5fbe5963f944c75add827e096c3/yoga_defs.bzl#L58-L60
		.flag("-fPIC")
		// C++ Files
		.file("src/c/Utils.cpp")
		.file("src/c/YGConfig.cpp")
		.file("src/c/YGEnums.cpp")
		.file("src/c/YGFloatOptional.cpp")
		.file("src/c/YGLayout.cpp")
		.file("src/c/YGNode.cpp")
		.file("src/c/YGNodePrint.cpp")
		.file("src/c/YGStyle.cpp")
		.file("src/c/Yoga.cpp")
		.compile("libyoga.a");

	let bindings = bindgen::Builder::default()
		.rust_target(RustTarget::Stable_1_21)
		.no_convert_floats()
		.enable_cxx_namespaces()
		.whitelist_type("YG.*")
		.whitelist_function("YG.*")
		.whitelist_var("YG.*")
		.layout_tests(false)
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
