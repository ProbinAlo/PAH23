//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use graph_serializer::{deserialize, serialize};
use js_sys::{BigUint64Array, JsString, Uint8Array};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_serialize() {
    let edges_1 = &[1u64];
    let edges_2 = &[0u64];
    let n_array = vec![JsString::from("a"), JsString::from("b")];
    let e_array = vec![
        BigUint64Array::from(edges_1.as_ref()),
        BigUint64Array::from(edges_2.as_ref()),
    ];
    let bytes = serialize(n_array, e_array);
    assert_eq!(
        vec![
            0, 0, 0, 0, 0, 0, 0, 2, 97, 0, 98, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0
        ],
        bytes.to_vec()
    );
}

#[wasm_bindgen_test]
fn test_deserialize() {
    let bytes: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 2, 97, 0, 98, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
        0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let result = deserialize(Uint8Array::from(bytes.as_ref()));

    let n_array = vec![JsString::from("a"), JsString::from("b")];
    let e_array = vec![
        vec![1u64],
        vec![0u64],
    ];

    assert_eq!(n_array, result.n_array());
    assert_eq!(e_array, result.e_array()
        .into_iter()
        .map(|arr| arr.to_vec())
        .collect::<Vec<_>>());
}
