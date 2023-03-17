extern crate adder;

mod common;

#[test]
fn it_works() {
    common::setup();
    let result = adder::add(2, 2);
    assert_eq!(result, 4);
}
