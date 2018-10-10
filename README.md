Yoga-rs
=======
[![Build Status](https://travis-ci.org/bschwind/yoga-rs.svg?branch=master)](https://travis-ci.org/bschwind/yoga-rs)

A Rust wrapper for Facebook's Yoga layout library.

Dependencies
------------
- cargo
- rustc

Build
-----
    $ cargo build --release

Run Example
-----------
	$ cargo run --release --example layout

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
    $ cargo test


To generate the test cases, install `ChromeDriver` and the following Ruby Gems: `watir`, `watir-webdriver`. Run:
    $ ruby gentest/gentest.rb



Yoga Version
------------

[c5f826de8306e5fbe5963f944c75add827e096c3](https://github.com/facebook/yoga/tree/c5f826de8306e5fbe5963f944c75add827e096c3/yoga)
