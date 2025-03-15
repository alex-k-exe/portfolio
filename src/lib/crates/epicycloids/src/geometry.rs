use core::panic;
use geo::{ClosestPoint, Coord, LineString};
use geom::bounding_rect;
use nannou::prelude::*;

pub const NO_VERTICES_ERROR: &str = "Polygon should have vertices";

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Line {
	pub point1: Point2,
	pub point2: Point2,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Polygon {
	pub points: Vec<Point2>,
}

impl Polygon {
	pub fn new(radius: f32, sides: usize) -> Self {
		let points = (0..sides)
			.map(|side| {
				let angle = ((180. / sides as f32)
					* (1. + (if sides % 2 == 1 { 1. } else { 0. }) + 2. * side as f32))
					.to_radians();
				// Get the sine of the radian to find the x co-ordinate of this point of the circle
				// and multiply it by the radius.
				let x = angle.sin() * radius;
				// Do the same with cosine to find the y co-ordinate.
				let y = angle.cos() * radius;
				pt2(x, y)
			})
			.collect();
		Polygon { points }
	}

	/** Align self to be above, below, or to the right or left of polygon */
	pub fn align_above(&mut self, polygon: &Polygon) {
		let bounding_boxes = [
			bounding_rect(self.points.clone()).expect(NO_VERTICES_ERROR),
			bounding_rect(polygon.points.clone()).expect(NO_VERTICES_ERROR),
		];

		self.translate(vec2(
			(bounding_boxes[1].x.start + bounding_boxes[1].x.end) / 2.
				- (bounding_boxes[0].x.start + bounding_boxes[0].x.end) / 2.,
			bounding_boxes[1].y.end - bounding_boxes[0].y.start,
		));
	}

	pub fn translate(&mut self, translation: Vec2) {
		for point in &mut self.points {
			point.x += translation.x;
			point.y += translation.y;
		}
	}

	/** Rotate clockwise, angle is in radians */
	pub fn rotate_around_point(&mut self, centre: Point2, angle: f32) {
		for point in &mut self.points {
			rotate_point(point, centre, angle.sin(), angle.cos());
		}
	}

	pub fn distance_to_point(&self, point: Point2) -> f32 {
		let points: Vec<Coord<f32>> = self
			.points
			.iter()
			.map(|p| geo::coord! {x: p.x, y: p.y})
			.collect();
		let polygon = geo::Polygon::new(LineString::from(points), vec![]);
		let closest = polygon.closest_point(&geo::point! {x: point.x, y: point.y});
		match closest {
			geo::Closest::SinglePoint(p) => pt2(p.x(), p.y()).distance(point),
			geo::Closest::Intersection(_) => 0.,
			geo::Closest::Indeterminate => panic!("Unable to determine closest point"),
		}
	}
}

/**
Assumes that b is the centre point, angle is in radians
*/
pub fn angle_between_points(a: Point2, b: Point2, c: Point2) -> f32 {
	let ba = a - b;
	let bc = c - b;

	let dot_product = ba.dot(bc);
	let magnitude_ba = ba.length();
	let magnitude_bc = bc.length();

	if dot_product == 0. || magnitude_ba == 0. || magnitude_bc == 0. {
		return 0.;
	}

	(dot_product / magnitude_ba / magnitude_bc).acos()
}

/** Rotate a point clockwise around another point */
pub fn rotate_point(point: &mut Point2, centre: Point2, sin: f32, cos: f32) {
	// Translate the point to the origin (relative to the centre)
	let translated_x = point.x - centre.x;
	let translated_y = point.y - centre.y;

	// Perform the rotation
	let rotated_x = translated_x * cos - translated_y * sin;
	let rotated_y = translated_x * sin + translated_y * cos;

	// Translate the point back
	point.x = rotated_x + centre.x;
	point.y = rotated_y + centre.y;
}
