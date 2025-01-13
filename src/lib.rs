use async_std::task::block_on;
use crates::three_points::*;
use wasm_bindgen::prelude::wasm_bindgen;

mod crates {
    pub mod three_points;
}

// web app entry_point
#[wasm_bindgen]
pub async fn main_web() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    block_on(async {
        run_app().await;
    });
}
