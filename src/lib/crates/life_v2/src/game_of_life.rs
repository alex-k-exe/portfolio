use nannou::{
	prelude::*,
	wgpu::{DeviceDescriptor, Limits},
};

pub async fn run_app() {
	app::Builder::new_async(|app| Box::new(Model::new(app)))
		.update(update)
		.run_async()
		.await;
}

const FPS: u64 = 5;
const CELL_WIDTH: f32 = 60.;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum GameState {
	Paused,
	NextFrame,
	Running,
}

impl GameState {
	/** Toggle the game state between Paused and Running. */
	fn toggle(&mut self) -> Self {
		match self {
			GameState::Paused => GameState::Running,
			GameState::Running => GameState::Paused,
			GameState::NextFrame => GameState::Paused,
		}
	}
}

#[derive(Debug, Clone)]
pub struct Model {
	// Inner Vectors are the rows, f32 values are how much each cell is rotated
	cells: Vec<Vec<f32>>,
	state: GameState,
}

impl Model {
	pub async fn new(app: &App) -> Self {
		let device_desc = DeviceDescriptor {
			limits: Limits {
				max_texture_dimension_2d: 8192,
				..Limits::downlevel_webgl2_defaults()
			},
			..Default::default()
		};

		app.new_window()
			.device_descriptor(device_desc)
			.key_pressed(key_pressed)
			.view(view)
			.build_async()
			.await
			.unwrap();

		Self::random_state()
	}

	fn clear_state() -> Self {
		Model {
			cells: vec![vec![0.; 10]; 10],
			state: GameState::Paused,
		}
	}

	fn random_state() -> Self {
		let mut model = Self::clear_state();

		for row in &mut model.cells {
			for cell in row {
				*cell = random_f32() * 2. * PI;
			}
		}
		model
	}
}

fn view(app: &App, model: &Model, frame: Frame) {
	let window = app.window_rect();
	let draw = app.draw();
	draw.background().color(BLACK);

	for (i, row) in model.cells.iter().enumerate() {
		for (j, cell_rotation) in row.iter().enumerate() {
			let x = CELL_WIDTH * (i as f32 + 0.5) - window.w() / 2.;
			let y = CELL_WIDTH * (j as f32 + 0.5) - window.h() / 2.;

			draw.rect()
				.x_y(x, y)
				.w_h(CELL_WIDTH * 0.75, CELL_WIDTH * 0.25)
				.rotate(*cell_rotation)
				.no_fill()
				.stroke(GREY)
				.stroke_weight(1.);
		}
	}

	draw.to_frame(app, &frame).unwrap();
}

/** Update the cells according to the rules of Conway's Game of Life. */
pub fn update(app: &App, model: &mut Model, _update: Update) {
	if model.state == GameState::Paused || app.elapsed_frames() % FPS != 0 {
		return;
	}

	let mut new_cells = vec![vec![0.; model.cells[0].len()]; model.cells.len()];

	// Iterate through the cells.
	for i in 0..model.cells.len() as i32 {
		for j in 0..model.cells[0].len() as i32 {
			// Count the number of neighbours and the sum of their angles
			let mut count = 0;
			let mut sum = 0.;
			// TODO: refactor this into one loop
			for x in -1..=1 {
				for y in -1..=1 {
					if x == 0 && y == 0 {
						continue;
					}

					let i = i + x;
					let j = j + y;

					if i < 0 || i >= model.cells.len() as i32 {
						continue;
					}
					if j < 0 || j >= model.cells[0].len() as i32 {
						continue;
					}

					count += 1;
					sum += model.cells[i as usize][j as usize];
				}
			}

			new_cells[i as usize][j as usize] = sum / count as f32;
		}
	}

	model.cells = new_cells;
	if model.state == GameState::NextFrame {
		model.state = GameState::Paused;
	}
}

/** When user presses the space bar, toggle the animation. */
fn key_pressed(_app: &App, model: &mut Model, key: Key) {
	match key {
		Key::Space => model.state = model.state.toggle(),
		Key::R => {
			*model = Model::random_state();
		}
		Key::Back => {
			*model = Model::clear_state();
		}
		Key::Right => model.state = GameState::NextFrame,
		_ => {}
	}
}
