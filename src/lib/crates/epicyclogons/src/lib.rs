mod epicyclogons;
mod geometry;

use async_std::task::block_on;
use epicyclogons::run_app;
use wasm_bindgen::prelude::wasm_bindgen;

// web app entry_point
#[wasm_bindgen]
pub async fn main_web() {
	#[cfg(debug_assertions)]
	console_error_panic_hook::set_once();

	block_on(async {
		run_app().await;
	});
}
