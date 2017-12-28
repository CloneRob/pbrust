// extern crate cgmath as cg;
extern crate num;

use std::default::Default;
// use cg::{Point2, Point3, Vector3, Vector2};
// use cg::BaseFloat;
// use cg::prelude::*;
mod util;
mod geometry;

use geometry::{bounds, transform, Scalar};
use geometry::vector::Vector2;

fn func<T: Scalar>(delta: T) {
    let v2 = Vector2::new(delta, delta);
    let p1 = Vector2::new(delta, delta);
    let t = p1 - v2;
}

fn main() {
    println!("Hello World");
    let v1 = Vector2::<f64>::new(1.0, 0.0);
    let v2 = Vector2::<f64>::new(0.0, 1.0);
    let alpha = 0.2f64;

    let b1 = bounds::Bounds2::<f32>::default();
    println!("{:?}", b1);
}
