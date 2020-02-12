Yoga-rs
=======
[![Build Status](https://travis-ci.org/bschwind/yoga-rs.svg?branch=master)](https://travis-ci.org/bschwind/yoga-rs)

A Rust wrapper for Facebook's Yoga layout library.

You may also want to check out [stretch](https://github.com/vislyhq/stretch) as it is a pure Rust implementation.

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

```
$ cargo test
```

To generate the test cases:
Download the [ChromeDriver](http://chromedriver.chromium.org) binary and put it somewhere in your `$PATH`. Linux/MacOS example:

```
$ cp chromedriver /usr/local/bin
```

Install the required Ruby gem.

```
$ sudo gem install watir
```

Generate the tests.

```
$ ruby gentest/gentest.rb
```
