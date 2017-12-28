// use cg;
// use cg::prelude::*;

// use num::{Zero, Float};
use std::cell::Cell;
use std::rc::Rc;
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub};
use std::cmp::{max, min};

use super::Scalar;
use super::{Metric, Vector, VectorSpace};
use super::vector::Vector3f;
use super::Point;
use super::point::Point3f;

#[derive(Debug, Clone)]
struct Medium {}

pub trait Ray {
    fn point(&self, t: f32) -> Point3f;
}

#[derive(Debug, Clone)]
struct Ray_ {
    pub o: Point3f,
    pub d: Vector3f,
    pub tmax: Cell<f32>,
    pub time: f32,
    pub medium: Option<Rc<Medium>>,
}

impl Ray_ {
    fn default() -> Self {
        Ray_ {
            o: Point3f::zero(),
            d: Vector3f::zero(),
            tmax: Cell::default(),
            time: 0.,
            medium: None,
        }
    }
    fn new(
        &self,
        o: &Point3f,
        d: &Vector3f,
        tmax: f32,
        time: f32,
        medium: Option<Rc<Medium>>,
    ) -> Ray_ {
        Ray_ {
            o: o.clone(),
            d: d.clone(),
            tmax: Cell::new(tmax),
            time: time,
            medium,
        }
    }
}

impl Ray for Ray_ {
    fn point(&self, t: f32) -> Point3f {
        self.o + self.d * t
    }
}

struct RayDifferential {
    pub ray: Ray_,
    pub has_differential: bool,
    pub rx_origin: Point3f,
    pub ry_origin: Point3f,
    pub rx_dir: Vector3f,
    pub ry_dir: Vector3f,
}

impl RayDifferential {
    fn default() -> Self {
        RayDifferential {
            ray: Ray_::default(),
            has_differential: false,
            rx_origin: Point3f::zero(),
            ry_origin: Point3f::zero(),
            rx_dir: Vector3f::zero(),
            ry_dir: Vector3f::zero(),
        }
    }
    fn new(ray: &Ray_) -> RayDifferential {
        RayDifferential {
            ray: ray.clone(),
            has_differential: false,
            rx_origin: Point3f::zero(),
            ry_origin: Point3f::zero(),
            rx_dir: Vector3f::zero(),
            ry_dir: Vector3f::zero(),
        }
    }

    fn scale_differentials(&mut self, s: f32) {
        self.rx_origin = self.ray.o + (self.rx_origin - self.ray.o) * s;
        self.ry_origin = self.ray.o + (self.ry_origin - self.ray.o) * s;
        self.rx_dir = self.ray.d + (self.rx_dir - self.ray.d) * s;
        self.ry_dir = self.ray.d + (self.ry_dir - self.ray.d) * s;
    }
}
