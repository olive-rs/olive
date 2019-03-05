#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    // TODO: export a start fn which leaks the wasm_bindgen start
}
