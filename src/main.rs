// extern crate cgmath as cg;
extern crate num;

use std::default::Default;
// use cg::{Point2, Point3, Vector3, Vector2};
// use cg::BaseFloat;
// use cg::prelude::*;
mod util;
mod geometry;

use geometry::{Vector, VectorSpace, Metric, Point, Scalar};
use geometry::point::Point3;
use geometry::vector::Vector2;
use geometry::transform::{Matrix4, Transform};

fn main() {
    let point = Point3::unit();
    if let Ok(T) = Transform::new(Matrix4::<f32>::new()) {
        if let Ok(K) = Transform::new(Matrix4::<f32>::new()) {
            let p = &K * &T * &point;
            println!("{:?}", p);
        }
    }
}
