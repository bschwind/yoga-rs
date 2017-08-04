Yoga-rs
=======

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

use yoga::{Node, Point, Percent};
use yoga::FlexStyle::*;
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
		Margin(Auto)
	);

	child.apply_styles(&child_styles);
	other_child.apply_styles(&child_styles);

	node.calculate_layout(512.0, 512.0, yoga::Direction::LTR);

	println!("Layout is {:#?}", child.get_layout());
}
```

Yoga Version
------------

[a20bde8444474e7a34352a78073de23c26e08fc5](https://github.com/facebook/yoga/tree/a20bde8444474e7a34352a78073de23c26e08fc5/yoga)
