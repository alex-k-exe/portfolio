use crates::three_points::run_app;
use tokio::runtime::Runtime;

mod crates {
    // mod epicycloids;
    // mod geometry;
    pub mod three_points;
}

fn main() {
    let runtime = Runtime::new().unwrap();

    runtime.block_on(async {
        run_app().await;
    });
}
