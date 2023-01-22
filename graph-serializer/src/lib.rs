mod utils;

use crate::utils::set_panic_hook;
use js_sys::{BigUint64Array, JsString, Uint8Array};
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug)]
#[wasm_bindgen]
pub struct DeserializeResult {
    n_array: Vec<String>,
    e_array: Vec<Vec<u64>>,
}

#[wasm_bindgen]
impl DeserializeResult {
    #[wasm_bindgen(getter)]
    pub fn n_array(&self) -> Vec<JsString> {
        self.n_array.iter().map(|s| s.as_str().into()).collect()
    }

    #[wasm_bindgen(getter)]
    pub fn e_array(&self) -> Vec<BigUint64Array> {
        self.e_array
            .iter()
            .map(|v| <Vec<u64> as AsRef<[u64]>>::as_ref(v).into())
            .collect()
    }
}

#[wasm_bindgen]
pub fn serialize(n_array: Vec<JsString>, e_array: Vec<BigUint64Array>) -> Uint8Array {
    set_panic_hook();
    console_error_panic_hook::set_once();
    let n_array = n_array
        .into_iter()
        .map(|v| v.into())
        .collect::<Vec<String>>();
    let e_array = e_array
        .into_iter()
        .map(|v| v.to_vec())
        .collect::<Vec<Vec<u64>>>();

    let mut bytes: Vec<u8> = Vec::new();

    // Serialize `n_array`
    bytes.extend((n_array.len() as u64).to_be_bytes());
    for node_value in n_array {
        bytes.extend(node_value.bytes());
        bytes.push(0x0); // null terminator
    }

    // Serialize `e_array`
    bytes.extend((e_array.len() as u64).to_be_bytes());
    for node_edges in e_array {
        bytes.extend((node_edges.len() as u64).to_be_bytes());

        for n_idx in node_edges {
            bytes.extend(n_idx.to_be_bytes());
        }
    }

    Uint8Array::from(bytes.as_ref())
}

#[wasm_bindgen]
pub fn deserialize(bytes: Uint8Array) -> DeserializeResult {
    set_panic_hook();
    let bytes: Vec<u8> = bytes.to_vec();
    let mut cur_byte_idx;
    let mut n_array = Vec::new();
    let mut e_array = Vec::new();

    // deserialize to n_array
    let n_array_size = u64::from_be_bytes(<[u8; 8]>::try_from(&bytes[..8]).unwrap());
    cur_byte_idx = 8;

    let mut n_array_string_collected = 0;
    while n_array_string_collected < n_array_size {
        let mut string_bytes = vec![];
        while bytes[cur_byte_idx] != 0 {
            string_bytes.push(bytes[cur_byte_idx]);
            cur_byte_idx += 1;
        }
        cur_byte_idx += 1;
        n_array.push(String::from_utf8(string_bytes).unwrap());

        n_array_string_collected += 1;
    }

    // deserialize to e_array
    let e_array_size =
        u64::from_be_bytes(<[u8; 8]>::try_from(&bytes[cur_byte_idx..cur_byte_idx + 8]).unwrap());
    cur_byte_idx += 8;

    let mut e_array_edges_list_collected = 0;
    while e_array_edges_list_collected < e_array_size {
        let mut edges = vec![];
        let edges_list_size = u64::from_be_bytes(
            <[u8; 8]>::try_from(&bytes[cur_byte_idx..cur_byte_idx + 8]).unwrap(),
        );
        cur_byte_idx += 8;
        let mut edges_list_edges_collected = 0;
        while edges_list_edges_collected < edges_list_size {
            edges.push(u64::from_be_bytes(
                <[u8; 8]>::try_from(&bytes[cur_byte_idx..cur_byte_idx + 8]).unwrap(),
            ));
            cur_byte_idx += 8;
            edges_list_edges_collected += 1;
        }
        e_array.push(edges);

        e_array_edges_list_collected += 1;
    }

    DeserializeResult { n_array, e_array }
}
