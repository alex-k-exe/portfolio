// native app entry_point

mod game_of_life;

use async_std::task::block_on;
use game_of_life::run_app;

fn main() {
	block_on(async {
		run_app().await;
	});
}
