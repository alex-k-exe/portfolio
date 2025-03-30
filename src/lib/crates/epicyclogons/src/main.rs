// native app entry_point

mod epicyclogons;
mod geometry;

use async_std::task::block_on;
use epicyclogons::run_app;

fn main() {
	block_on(async {
		run_app().await;
	});
}
