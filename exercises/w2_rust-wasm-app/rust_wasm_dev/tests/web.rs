// https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/usage.html
// the tests must be in the root of the crate, or within a pub mod
#![cfg(target_arch = "wasm32")]

// extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

//This macro macro is used to indicate that the test is intended to execute in a web browser as opposed to Node.js
wasm_bindgen_test_configure!(run_in_browser);
// This attribute defines a test accessible to WebAssembly and headless web browser testing
#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn fail() {
    assert_eq!(1, 2);
}
