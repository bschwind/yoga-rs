extern crate yoga;

use yoga::{Direction, FlexDirection, Node, StyleUnit, Undefined};

/// Tests that a Node is NOT marked dirty if it's styles are updated
/// with identical values to what they already were (i.e. the styles
/// are unchanged even though they were set)
#[test]
fn test_copy_style_same() {
    // Initially dirty
    let mut node0 = Node::new();
    assert!(node0.is_dirty());

    // Made clean by calculating layout
    node0.calculate_layout(Undefined, Undefined, Direction::LTR);
    assert!(!node0.is_dirty());

    // Not re-dirtied by updating to identical styles
    let node1 = Node::new();
    node0.copy_style(&node1);
    assert!(!node0.is_dirty());
}

/// Tests that a Node IS marked dirty if it's styles are updated
/// with different values to what they previously were
#[test]
fn test_copy_style_modified() {
    let mut node0 = Node::new();
    node0.calculate_layout(Undefined, Undefined, Direction::LTR);
    assert!(!node0.is_dirty());
    assert_eq!(FlexDirection::Column, node0.get_flex_direction());
    assert_eq!(node0.get_style_max_height(), StyleUnit::UndefinedValue);

    let mut node1 = Node::new();
    node1.set_flex_direction(FlexDirection::Row);
    node1.set_max_height(StyleUnit::Point(10.0.into()));

    node0.copy_style(&node1);
    assert!(node0.is_dirty());
    assert_eq!(FlexDirection::Row, node0.get_flex_direction());
    assert_eq!(node0.get_style_max_height(), StyleUnit::Point(10.0.into()));
}

#[test]
fn test_copy_style_modified_same() {
    let mut node0 = Node::new();
    node0.set_flex_direction(FlexDirection::Row);
    node0.set_max_height(StyleUnit::Point(10.0.into()));
    node0.calculate_layout(Undefined, Undefined, Direction::LTR);
    assert!(!node0.is_dirty());

    let mut node1 = Node::new();
    node1.set_flex_direction(FlexDirection::Row);
    node1.set_max_height(StyleUnit::Point(10.0.into()));

    node0.copy_style(&node1);
    assert!(!node0.is_dirty());
}
