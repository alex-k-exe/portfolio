use nannou::prelude::*;

pub async fn run_app() {
	app::Builder::new_async(|app| Box::new(Model::new(app)))
		.update(update)
		.run_async()
		.await;
}

const FPS: u64 = 10;
const SQUARE_WIDTH: f32 = 30.;

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
	automata: Vec<Vec<bool>>,
	state: GameState,
}

impl Model {
	pub async fn new(app: &App) -> Self {
		app.new_window()
			.mouse_pressed(mouse_pressed)
			.key_pressed(key_pressed)
			.view(view)
			.build_async()
			.await
			.unwrap();

		Self::new_state()
	}

	fn new_state() -> Self {
		Model {
			automata: vec![vec![false; 15]; 15],
			state: GameState::Paused,
		}
	}
}

fn view(app: &App, model: &Model, frame: Frame) {
	let window = app.window_rect();
	let draw = app.draw();
	draw.background().color(BLACK);

	for (i, row) in model.automata.iter().enumerate() {
		for (j, cell) in row.iter().enumerate() {
			let x = SQUARE_WIDTH * (i as f32 + 0.5) - window.w() / 2.;
			let y = SQUARE_WIDTH * (j as f32 + 0.5) - window.h() / 2.;

			if *cell {
				draw.rect()
					.x_y(x, y)
					.w_h(SQUARE_WIDTH, SQUARE_WIDTH)
					.color(WHITE);
			} else {
				draw.rect()
					.x_y(x, y)
					.w_h(SQUARE_WIDTH, SQUARE_WIDTH)
					.no_fill()
					.stroke(WHITE)
					.stroke_weight(1.);
			}
		}
	}

	draw.ellipse()
		.x_y(app.mouse.x, app.mouse.y)
		.w_h(5., 5.)
		.color(RED);
	draw.to_frame(app, &frame).unwrap();
}

/** Update the automata according to the rules of Conway's Game of Life. */
pub fn update(app: &App, model: &mut Model, _update: Update) {
	if model.state == GameState::Paused || app.elapsed_frames() % FPS != 0 {
		return;
	}

	let mut new_automata = vec![vec![false; model.automata[0].len()]; model.automata.len()];

	// Iterate through each cell in the automata.
	for i in 0..model.automata.len() as usize {
		for j in 0..model.automata[0].len() as usize {
			// Count the number of live neighbors.
			let mut count = 0;
			for x in -1..=1 {
				for y in -1..=1 {
					if x == 0 && y == 0 {
						continue;
					}

					let i = i as i32 + x;
					let j = j as i32 + y;

					if i < 0 || i >= model.automata.len() as i32 {
						continue;
					}
					if j < 0 || j >= model.automata[0].len() as i32 {
						continue;
					}

					if model.automata[i as usize][j as usize] {
						count += 1;
					}
				}
			}

			if model.automata[i][j] {
				new_automata[i][j] = count == 2 || count == 3;
			} else {
				new_automata[i][j] = count == 3;
			}
		}
	}

	model.automata = new_automata;
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

	let i = ((mouse.x + window.w() / 2.) / SQUARE_WIDTH) as i32;
	let j = ((mouse.y + window.h() / 2.) / SQUARE_WIDTH) as i32;

	let i_is_invalid = i < 0 || i >= model.automata.len() as i32;
	let j_is_invalid = j < 0 || j >= model.automata[0].len() as i32;
	if i_is_invalid || j_is_invalid {
		return;
	}

	model.automata[i as usize][j as usize] ^= true;
}

/** When user presses the space bar, toggle the animation. */
fn key_pressed(_app: &App, model: &mut Model, key: Key) {
	match key {
		Key::Space => model.state = model.state.toggle(),
		Key::R => {
			*model = Model::new_state();
		}
		Key::Right => model.state = GameState::NextFrame,
		_ => {}
	}
}
