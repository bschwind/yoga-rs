extern crate bindgen;
extern crate gcc;

use gcc::Build;
use std::env;
use std::path::PathBuf;

fn main() {
	let mut builder = Build::new();

	builder
		.cpp(true)
		// https://github.com/facebook/yoga/blob/7f44ec512e7d150b7312336ed7908ac86441b4cc/BUCK#L13
		.flag("-std=c++1y")
		// https://github.com/facebook/yoga/blob/7f44ec512e7d150b7312336ed7908ac86441b4cc/yoga_defs.bzl#L28
		.flag("-fno-omit-frame-pointer")
		.flag("-fexceptions")
		.flag("-Wall")
		.flag("-Werror")
		.flag("-O3")
		// https://github.com/facebook/yoga/blob/7f44ec512e7d150b7312336ed7908ac86441b4cc/yoga_defs.bzl#L36
		.flag("-fPIC")
		.file("src/c/Utils.cpp")
		.file("src/c/YGEnums.cpp")
		.file("src/c/YGNode.cpp")
		.file("src/c/YGNodePrint.cpp")
		.file("src/c/Yoga.cpp");

	println!("Compiler is {:?}", builder.get_compiler());

	builder.compile("libyoga.a");

	let bindings = bindgen::Builder::default()
		.clang_arg("-std=c++11")
		.no_unstable_rust()
		.no_convert_floats()
		.enable_cxx_namespaces()
		.whitelisted_type("YG.*")
		.whitelisted_function("YG.*")
		.layout_tests(false)
		.header("src/c/wrapper.hpp")
		.generate()
		.expect("Unable to generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Unable to write bindings!");
}
