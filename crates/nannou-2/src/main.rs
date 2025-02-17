// native app entry_point

mod sketch;
mod three_points;

use async_std::task::block_on;
use sketch::run_app;

fn main() {
	block_on(async {
		run_app().await;
	});
}
