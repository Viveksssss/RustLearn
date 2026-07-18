use adder::*;
mod common;
#[test]
fn it_add_two() {
    assert_eq!(4, adder::add(2, 2));
}

#[test]
fn testss() {
    common::setup();
}
