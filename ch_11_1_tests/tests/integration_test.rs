use ch_11_1_tests;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, ch_11_1_tests::add_two(2));
}
