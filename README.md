Yoga-rs
=======

[![Github CI](https://github.com/bschwind/yoga-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/bschwind/yoga-rs/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/yoga.svg)](https://crates.io/crates/yoga)
[![docs.rs](https://img.shields.io/docsrs/yoga)](https://docs.rs/yoga)

A Rust wrapper for Facebook's [Yoga](https://github.com/facebook/yoga) layout library.

You may also want to check out [Taffy](https://github.com/dioxuslabs/taffy) (a revived fork of the abandoned [stretch](https://github.com/vislyhq/stretch)) as it is a pure Rust implementation.

Dependencies
------------
- cargo
- rustc
- libc++-dev (LLVM’s libc++)

Build
-----
    $ cargo build --release

Run Example
-----------
	$ cargo run --release --example layout

Format Code
-----------
	$ cargo +nightly fmt

Example Code
------------
```rust
#[macro_use]
extern crate yoga;

use yoga::prelude::*;
use yoga::Node;
use yoga::StyleUnit::{Auto, UndefinedValue};

fn main() {
	let mut node = Node::new();

	let mut child = Node::new();
	let mut other_child = Node::new();

	node.insert_child(&mut child, 0);
	node.insert_child(&mut other_child, 1);

	style!(node,
		Margin(10 pt),
		MarginLeft(Auto),
		PaddingHorizontal(4 pt),
		Left(16 %),
		Bottom(UndefinedValue)
	);

	let child_styles = make_styles!(
		Width(32 pt),
		Height(32 pt),
		FlexGrow(1.0),
		Margin(Auto)
	);

	child.apply_styles(&child_styles);
	other_child.apply_styles(&child_styles);

	node.calculate_layout(512.0, 512.0, yoga::Direction::LTR);

	println!("Layout is {:#?}", child.get_layout());
}
```

Testing
-------
The unit tests are automatically generated based on upstream fixtures and should not be edited manually.

```
$ cargo test
```

To generate the test cases:
Download the [ChromeDriver](http://chromedriver.chromium.org) binary and put it somewhere in your `$PATH`. Linux/MacOS example:

```
$ cp chromedriver /usr/local/bin
```

Chromedriver can also be installed via homebrew.

Then run the following

```bash
$ cd gentest
$ npm install # Install the required node modules
$ npm run gentest # Generate the tests
$ cargo +nightly fmt # Format the tests for consistency
```
