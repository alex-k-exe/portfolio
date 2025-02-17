// https://github.com/nannou-org/nannou/blob/master/examples/draw/draw_blend.rs

use crate::three_points::{model, view, Model};
use nannou::prelude::*;
use nannou::wgpu::{Backends, DeviceDescriptor, Limits};
use std::cell::RefCell;

pub async fn run_app() {
	// Since ModelFn is not a closure we need this workaround to pass the calculated model
	thread_local!(static MODEL: RefCell<Option<Model>> = Default::default());

	app::Builder::new_async(|app| {
		Box::new(async move {
			create_window(app).await;
			model(app)
		})
	})
	.backends(Backends::PRIMARY | Backends::GL)
	.run_async()
	.await;
}

async fn create_window(app: &App) {
	let device_desc = DeviceDescriptor {
		limits: Limits {
			max_texture_dimension_2d: 8192,
			..Limits::downlevel_webgl2_defaults()
		},
		..Default::default()
	};

	app.new_window()
		.device_descriptor(device_desc)
		.title("nannou web test")
		// .raw_event(raw_event)
		// .key_pressed(key_pressed)
		// .key_released(key_released)
		// .mouse_pressed(mouse_pressed)
		// .mouse_moved(mouse_moved)
		// .mouse_released(mouse_released)
		// .mouse_wheel(mouse_wheel)
		// .touch(touch)
		.view(view)
		.build_async()
		.await
		.unwrap();
}
