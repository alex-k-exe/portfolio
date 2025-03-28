// native app entry_point

mod epicycloids;
mod geometry;

use async_std::task::block_on;
use epicycloids::run_app;

fn main() {
	block_on(async {
		run_app().await;
	});
}
