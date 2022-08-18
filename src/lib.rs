mod utils;

use wasm_bindgen::prelude::*;

use pinyin::to_pinyin_vec;
use pinyin::Pinyin;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn pinyin(hans: &str) -> JsValue {
    let v = to_pinyin_vec(hans, Pinyin::with_tone);
    return JsValue::from_serde(&v).unwrap();
}
