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

const FPS: u64 = 10;
const CELL_WIDTH: f32 = 30.;

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
	// Elements of outer Vec are the rows, bool values are the cells
	cells: Vec<Vec<bool>>,
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
			.mouse_pressed(mouse_pressed)
			.key_pressed(key_pressed)
			.view(view)
			.build_async()
			.await
			.unwrap();

		Self::random_state()
	}

	fn clear_state() -> Self {
		Model {
			cells: vec![vec![false; 20]; 20],
			state: GameState::Paused,
		}
	}

	fn random_state() -> Self {
		let mut model = Self::clear_state();

		for row in &mut model.cells {
			for cell in row {
				*cell = random_f32() > 0.5;
			}
		}
		model
	}
}

fn view(app: &App, model: &Model, frame: Frame) {
	let window = app.window_rect();
	let draw = app.draw();
	draw.background().color(WHITE);

	for (i, row) in model.cells.iter().enumerate() {
		for (j, cell) in row.iter().enumerate() {
			let x = CELL_WIDTH * (i as f32 + 0.5) - window.w() / 2.;
			let y = CELL_WIDTH * (j as f32 + 0.5) - window.h() / 2.;

			if *cell {
				draw.rect()
					.x_y(x, y)
					.w_h(CELL_WIDTH, CELL_WIDTH)
					.color(BLACK)
					.stroke(GREY)
					.stroke_weight(1.);
			} else {
				draw.rect()
					.x_y(x, y)
					.w_h(CELL_WIDTH, CELL_WIDTH)
					.stroke(GREY)
					.stroke_weight(1.);
			}
		}
	}

	draw.to_frame(app, &frame).unwrap();
}

/** Update the cells according to the rules of Conway's Game of Life. */
pub fn update(app: &App, model: &mut Model, _update: Update) {
	if model.state == GameState::Paused || app.elapsed_frames() % FPS != 0 {
		return;
	}

	let mut new_cells = vec![vec![false; model.cells[0].len()]; model.cells.len()];

	// Iterate through the cells.
	for i in 0..model.cells.len() as usize {
		for j in 0..model.cells[0].len() as usize {
			// Count the number of live neighbors.
			let mut count = 0;
			// TODO: refactor this into one loop
			for x in -1..=1 {
				for y in -1..=1 {
					if x == 0 && y == 0 {
						continue;
					}

					let i = i as i32 + x;
					let j = j as i32 + y;

					if i < 0 || i >= model.cells.len() as i32 {
						continue;
					}
					if j < 0 || j >= model.cells[0].len() as i32 {
						continue;
					}

					if model.cells[i as usize][j as usize] {
						count += 1;
					}
				}
			}

			if model.cells[i][j] {
				new_cells[i][j] = count == 2 || count == 3;
			} else {
				new_cells[i][j] = count == 3;
			}
		}
	}

	model.cells = new_cells;
	if model.state == GameState::NextFrame {
		model.state = GameState::Paused;
	}
}

/** When user clicks left mouse button, find the cell that was clicked and toggle it. */
fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
	match button {
		MouseButton::Left => {}
		_ => return,
	}
	if model.state == GameState::Running {
		return;
	}

	let window = app.window_rect();
	let mouse = app.mouse.position();

	let i = ((mouse.x + window.w() / 2.) / CELL_WIDTH) as i32;
	let j = ((mouse.y + window.h() / 2.) / CELL_WIDTH) as i32;

	let i_is_invalid = i < 0 || i >= model.cells.len() as i32;
	let j_is_invalid = j < 0 || j >= model.cells[0].len() as i32;
	if i_is_invalid || j_is_invalid {
		return;
	}

	model.cells[i as usize][j as usize] ^= true;
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
