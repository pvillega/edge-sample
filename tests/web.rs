//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate edge;
use wasm_bindgen_test::*;
use edge::{greet, parse};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_greet() {
    assert_eq!(greet(), "Hello, wasm-worker!");
}

#[wasm_bindgen_test]
fn test_parse() {
    let expected = "\nHTML output:\n<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n";

    assert_eq!(parse(), expected);
}
