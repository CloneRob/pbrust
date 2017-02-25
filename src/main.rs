#![macro_use]
extern crate nalgebra as na;
extern crate num;
extern crate alga;

use std::default::Default;
use num::Zero;
use na::{Point2, Point3, Vector3, Vector2};
mod util;
mod geometry;

use geometry::*;

fn main() {
    let v1 = Vector2::<f64>::x();
    let v2 = Vector2::<f64>::y();
    let alpha = 0.2f64;

    let v3 = v1.lerp(alpha, &v2);
    let b1  = bounds::Bounds2::<f32>::default();
    println!("{}", v3);
    println!("{:?}", b1);
}

