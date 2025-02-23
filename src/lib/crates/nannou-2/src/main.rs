// native app entry_point

mod three_points;

use async_std::task::block_on;
use three_points::run_app;

fn main() {
	block_on(async {
		run_app().await;
	});
}
