extern crate cgmath as cg;
extern crate num;

use std::default::Default;
use cg::{Point2, Point3, Vector3, Vector2};
use cg::BaseFloat;
use cg::prelude::*;
mod util;
mod geometry;


use geometry::{bounds, Interpolate};

// fn func<T>(delta: T) {
//     let v2 = Vector2::new(delta, delta);
//     let p1 = Vector2::new(delta, delta);
//     let t = p1 - v2;
// }

fn main() {
    let v1 = Vector2::<f64>::unit_x();
    let v2 = Vector2::<f64>::unit_y();
    // let v1 = 1.0f64;
    // let v2 = 0.7f64;
    let alpha = 0.2f64;

    let v3 = Interpolate::lerp(&v1, alpha, &v2);
    let b1  = bounds::Bounds2::<f32>::default();
    println!("{:?}", v3);
    println!("{:?}", b1);
}

