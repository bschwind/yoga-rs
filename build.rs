extern crate bindgen;
extern crate cc;

use bindgen::{
    callbacks::{ParseCallbacks, TypeKind},
    NonCopyUnionStyle, RustTarget,
};
use cc::Build;
use std::{
    env,
    fs::{read_to_string, write},
    path::PathBuf,
    process::Command,
};

#[derive(Debug)]
struct BindgenCallbacks;

impl ParseCallbacks for BindgenCallbacks {
    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        enum_name.map(|name| original_variant_name.replace(name, ""))
    }

    fn add_derives(&self, info: &bindgen::callbacks::DeriveInfo<'_>) -> Vec<String> {
        if info.kind == TypeKind::Enum {
            ["PartialOrd", "Ord"].into_iter().map(String::from).collect()
        } else {
            vec![]
        }
    }

    fn add_attributes(&self, info: &bindgen::callbacks::AttributeInfo<'_>) -> Vec<String> {
        if info.kind == TypeKind::Enum {
            vec![r#"#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]"#.into()]
        } else {
            vec![]
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=src/yoga/yoga");

    Command::new("git")
        .args(["submodule", "init"])
        .status()
        .expect("Unable to initialize git submodules");
    Command::new("git")
        .args(["submodule", "update"])
        .status()
        .expect("Unable to update the submodule repositories");

    Build::new()
		.cpp(true)
		// https://github.com/facebook/yoga/blob/c5f826de8306e5fbe5963f944c75add827e096c3/BUCK#L13
		.std("c++20")
		// https://github.com/facebook/yoga/blob/c5f826de8306e5fbe5963f944c75add827e096c3/yoga_defs.bzl#L49-L56
		.flag("-fno-omit-frame-pointer")
		.flag("-fexceptions")
		.flag("-Wall")
		.flag("-O3")
		// https://github.com/facebook/yoga/blob/c5f826de8306e5fbe5963f944c75add827e096c3/yoga_defs.bzl#L58-L60
		.flag("-fPIC")
		// Include path
		.include("src/yoga")
		// C++ Files
        .file("src/yoga/yoga/algorithm/AbsoluteLayout.cpp")
        .file("src/yoga/yoga/algorithm/Baseline.cpp")
        .file("src/yoga/yoga/algorithm/Cache.cpp")
        .file("src/yoga/yoga/algorithm/CalculateLayout.cpp")
        .file("src/yoga/yoga/algorithm/FlexLine.cpp")
        .file("src/yoga/yoga/algorithm/PixelGrid.cpp")
        .file("src/yoga/yoga/config/Config.cpp")
        .file("src/yoga/yoga/event/event.cpp")
        .file("src/yoga/yoga/debug/AssertFatal.cpp")
        .file("src/yoga/yoga/debug/Log.cpp")
        .file("src/yoga/yoga/node/LayoutResults.cpp")
        .file("src/yoga/yoga/node/Node.cpp")
		.file("src/yoga/yoga/YGConfig.cpp")
		.file("src/yoga/yoga/YGEnums.cpp")
		.file("src/yoga/yoga/YGNode.cpp")
		.file("src/yoga/yoga/YGNodeLayout.cpp")
		.file("src/yoga/yoga/YGNodeStyle.cpp")
		.file("src/yoga/yoga/YGPixelGrid.cpp")
		.file("src/yoga/yoga/YGValue.cpp")
        // Enable if debugging this script
        // .cargo_debug(true)
		.compile("libyoga");

    let bindings = bindgen::Builder::default()
        .rust_target(RustTarget::stable(47, 0).unwrap_or_default())
        .clang_args(&["-x", "c++", "-std=c++20", "-Isrc/yoga"])
        .enable_cxx_namespaces()
        .allowlist_type("YG.*")
        .allowlist_function("YG.*")
        .allowlist_var("YG.*")
        .layout_tests(false)
        .formatter(bindgen::Formatter::Rustfmt)
        .rustified_enum("YG.*")
        .prepend_enum_name(false)
        .parse_callbacks(Box::new(BindgenCallbacks))
        .manually_drop_union(".*")
        .default_non_copy_union_style(NonCopyUnionStyle::ManuallyDrop)
        .header("src/wrapper.hpp")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_file = out_path.join("bindings.rs");

    bindings.write_to_file(&out_file).expect("Unable to write bindings!");

    // Patch bindings because bindgen is incorecctly detected float type
    let buf = read_to_string(&out_file).expect("Unable to read bindings");
    let patched = buf.replace(
        "pub const YGUndefined: f32 = f64::NAN;",
        "pub const YGUndefined: f32 = f32::NAN;",
    );
    write(&out_file, patched).expect("Unable to write patched bindings");

    println!("Bindings written to {}", &out_file.display());
}
