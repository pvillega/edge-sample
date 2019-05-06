//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate edge-sample;
use wasm_bindgen_test::*;
use edge-sample::linear_regression;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_greet() {
    assert_eq!("Hello, wasm-worker!", "Hello, wasm-worker!");
}
