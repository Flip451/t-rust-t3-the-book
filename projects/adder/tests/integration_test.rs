extern crate adder;
// use adder; でも機能する
// adder はこのパッケージの名前であることに注意
// src/main.rs, src/libs.rs は、パッケージと同じ名前を持つバイナリクレートのクレートルート

mod common;

#[test]
fn it_adds_two() {
  common::setup();
  assert_eq!(4, adder::add_two(2));
}