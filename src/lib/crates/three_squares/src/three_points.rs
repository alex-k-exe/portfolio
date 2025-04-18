use nannou::{
	prelude::*,
	wgpu::{DeviceDescriptor, Limits},
};

pub async fn run_app() {
	app::Builder::new_async(|app| Box::new(model(app)))
		.run_async()
		.await;
}

pub struct Model {
	primary_points: [Point2; 3],
	secondary_points: Vec<(Point2, f32)>,
	selected_point: i8,
}

const POINT_WIDTH: f32 = 5.;
const GREYSCALE: bool = false;

pub async fn model(app: &App) -> Model {
	let device_desc = DeviceDescriptor {
		limits: Limits {
			max_texture_dimension_2d: 8192,
			..Limits::downlevel_webgl2_defaults()
		},
		..Default::default()
	};

	app.new_window()
		.device_descriptor(device_desc)
		.view(view)
		.mouse_pressed(mouse_pressed)
		.build_async()
		.await
		.unwrap();

	let window = app.window_rect();

	let primary_points = [pt2(240., 63.), pt2(-127., 127.), pt2(-255., -96.)];

	let mut secondary_points: Vec<(Vec2, f32)> = vec![];
	let mut current_point = pt2(-window.w() / 2., window.h() / 2.);

	while current_point.y > -window.h() / 2. {
		secondary_points.push((current_point, 0.));

		if current_point[0] + POINT_WIDTH * 1.5 > window.w() / 2. {
			current_point[0] = -window.w() / 2.;
			current_point[1] -= POINT_WIDTH;
		} else {
			current_point[0] += POINT_WIDTH;
		}
	}

	color_secondary_points(primary_points, &mut secondary_points);
	Model {
		primary_points,
		secondary_points,
		selected_point: -1,
	}
}

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
	match button {
		MouseButton::Left => {}
		_ => return,
	}

	let mut pressed_point: i8 = 0;
	for point in model.primary_points {
		let rect = Rect::from_xy_wh(point, vec2(15., 15.));
		if rect.contains(pt2(app.mouse.x, app.mouse.y)) {
			break;
		}
		pressed_point += 1;
	}

	// if a point is pressed
	if pressed_point < model.primary_points.len() as i8 {
		if pressed_point == model.selected_point {
			model.selected_point = -1;
		} else {
			model.selected_point = pressed_point;
		}
		return;
	}

	if model.selected_point == -1 {
		return;
	}

	model.primary_points[model.selected_point as usize] = pt2(app.mouse.x, app.mouse.y);
	color_secondary_points(model.primary_points, &mut model.secondary_points);
}

pub fn view(app: &App, model: &Model, frame: Frame) {
	let draw = app.draw();

	for point in &model.secondary_points {
		if GREYSCALE {
			draw.rect()
				.xy(point.0)
				.wh(vec2(POINT_WIDTH, POINT_WIDTH))
				.rgb(point.1, point.1, point.1);
		} else {
			draw.rect()
				.xy(point.0)
				.wh(vec2(POINT_WIDTH, POINT_WIDTH))
				.hsv(point.1, 50., 50.);
		}
	}

	let mut index = 0;
	for point in model.primary_points {
		draw.rect()
			.xy(point)
			.wh(vec2(15., 15.))
			.color(if model.selected_point == index {
				if GREYSCALE {
					RED
				} else {
					WHITE
				}
			} else {
				if GREYSCALE {
					BLUE
				} else {
					BLACK
				}
			});
		index += 1;
	}

	// put everything on the frame
	draw.to_frame(app, &frame).unwrap();
}

fn color_secondary_points(primary_points: [Point2; 3], secondary_points: &mut Vec<(Vec2, f32)>) {
	let mut max_distance: f32 = 0.;
	let mut min_distance: f32 = 99999.;
	for secondary in secondary_points {
		let mut total_distance: f32 = 0.;
		for primary in primary_points {
			total_distance += secondary.0.distance(primary);
		}
		total_distance /= primary_points.len() as f32;

		if max_distance < total_distance {
			max_distance = total_distance;
		} else if min_distance > total_distance {
			min_distance = total_distance;
		}
		secondary.1 = clamp(
			map_range(
				total_distance,
				min_distance,
				max_distance,
				0.,
				if GREYSCALE { 1. } else { 255. },
			),
			0.,
			if GREYSCALE { 1. } else { 255. },
		);
	}
}
