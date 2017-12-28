use std::convert::From;
use std::ops::Index;
use std::iter::Iterator;
use std::cmp::{max, min};

use super::Scalar;
use super::Vector;
use super::vector::{Vector2, Vector3};
use super::Point;
use super::point::{Point2, Point3};
// use cg::{Point2, Point3, Vector2, Vector3};
// use cg::{Scalar};
// use cg::prelude::*;

use num::Bounded;

use std::ops::Add;
use std::cmp::{PartialEq, PartialOrd};

// use ::geometry::{Interpolate};

pub trait Bounds {
    type Scalar;
    type Point;
    type Vector;
    fn corner(&self, corner: u8) -> Self::Point;
    fn point_union(b: &Self, p: &Self::Point) -> Self;
    fn bounds_union(b1: &Self, b2: Self) -> Self;
    fn intersect(b1: &Self, b2: &Self) -> Self;
    fn overlaps(b1: &Self, b2: &Self) -> bool;
    fn inside(p: &Self::Point, b: &Self) -> bool;
    fn inside_exclusive(p: &Self::Point, b: &Self) -> bool;
    fn expand(b1: &Self, delta: Self::Scalar) -> Self;
    fn diagonal(&self) -> Self::Vector;
    fn surface_area(&self) -> Self::Scalar;
    fn volume(&self) -> Self::Scalar;
    fn maximum_extent(&self) -> u8;
    fn lerp(&self, t: &Self::Point) -> Self::Point;
    fn offset(&self, p: &Self::Point) -> Self::Vector;
    fn bounding_sphere(&self) -> (Self::Point, f32);
}

#[derive(Copy, Clone, Debug)]
pub struct Bounds2<S: Scalar> {
    p_min: Point2<S>,
    p_max: Point2<S>,
}
impl<S> Default for Bounds2<S>
where
    S: Scalar + Bounded,
{
    fn default() -> Bounds2<S> {
        let min = Bounded::min_value();
        let max = Bounded::max_value();

        Bounds2 {
            p_min: Point2::new(min, min),
            p_max: Point2::new(max, max),
        }
    }
}

impl<'a, S: Scalar> From<&'a Point2<S>> for Bounds2<S> {
    fn from(p: &'a Point2<S>) -> Self {
        Bounds2 {
            p_min: p.clone(),
            p_max: p.clone(),
        }
    }
}
impl<S: Scalar> From<(Point2<S>, Point2<S>)> for Bounds2<S> {
    fn from(points: (Point2<S>, Point2<S>)) -> Self {
        Bounds2 {
            p_min: points.0,
            p_max: points.1,
        }
    }
}
impl<'a, S: Scalar> From<(&'a Point2<S>, &'a Point2<S>)> for Bounds2<S> {
    fn from(points: (&'a Point2<S>, &'a Point2<S>)) -> Self {
        Bounds2 {
            p_min: points.0.clone(),
            p_max: points.1.clone(),
        }
    }
}

impl<S: Scalar> Index<u8> for Bounds2<S> {
    type Output = Point2<S>;
    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.p_min,
            1 => &self.p_max,
            _ => panic!("Bounds2 Index (b[{}]) out of range", index),
        }
    }
}
impl<S: Scalar> Bounds for Bounds2<S> {
    type Scalar = S;
    type Point = Point2<S>;
    type Vector = Vector2<S>;

    fn corner(&self, corner: u8) -> Self::Point {
        let x = self[corner & 1].x;
        let y = self[corner & 2].y;
        Point2::new(x, y)
    }
    fn point_union(b: &Self, p: &Self::Point) -> Self {
        let pmin = Point::min(&b.p_min, &p);
        let pmax = Point::max(&b.p_max, &p);
        Bounds2::from((pmin, pmax))
    }

    fn bounds_union(b1: &Self, b2: Self) -> Self {
        let pmin = Point::min(&b1.p_min, &b2.p_min);
        let pmax = Point::max(&b1.p_max, &b2.p_max);
        Bounds2::from((pmin, pmax))
    }
    fn intersect(b1: &Self, b2: &Self) -> Self {
        let pmin = Point::max(&b1.p_min, &b2.p_min);
        let pmax = Point::min(&b1.p_max, &b2.p_max);
        Bounds2::from((pmin, pmax))
    }
    fn overlaps(b1: &Self, b2: &Self) -> bool {
        let x = (b1.p_max.x >= b2.p_min.x) && (b1.p_min.x <= b2.p_max.x);
        let y = (b1.p_max.y >= b2.p_min.y) && (b1.p_min.y <= b2.p_max.y);
        // let z = (b1.p_max.z >= b2.p_min.z) && (b1.p_min.z <= b2.p_max.z);
        x && y
    }
    fn inside(p: &Self::Point, b: &Self) -> bool {
        let x = p.x >= b.p_min.x && p.x <= b.p_max.x;
        let y = p.y >= b.p_min.y && p.y <= b.p_max.y;
        // let z = p.z >= b.p_min.z && p.z <= b.p_max.z;
        x && y
    }
    fn inside_exclusive(p: &Self::Point, b: &Self) -> bool {
        let x = p.x >= b.p_min.x && p.x < b.p_max.x;
        let y = p.y >= b.p_min.y && p.y < b.p_max.y;
        // let z = p.z >= b.p_min.z && p.z < b.p_max.z;
        x && y
    }
    fn expand(b1: &Self, delta: Self::Scalar) -> Self {
        let delta_vec = Vector2::new(delta, delta);
        let p_min = b1.p_min - delta_vec;
        let p_max = b1.p_max + delta_vec;
        Bounds2::from((p_min, p_max))
    }
    fn diagonal(&self) -> Self::Vector {
        self.p_max - self.p_min
    }
    fn surface_area(&self) -> Self::Scalar {
        let d = self.diagonal();
        d.x * d.y
    }
    fn volume(&self) -> Self::Scalar {
        S::zero()
    }
    fn maximum_extent(&self) -> u8 {
        let d = self.diagonal();
        if d.x > d.y {
            0
        } else {
            1
        }
    }
    fn lerp(&self, t: &Self::Point) -> Self::Point {
        let x = super::lerp(t.x, self.p_min.x, self.p_max.x);
        let y = super::lerp(t.y, self.p_min.y, self.p_max.y);
        Point2::new(x, y)
    }

    fn offset(&self, p: &Self::Point) -> Self::Vector {
        let mut offset = &*p - &self.p_min;
        if self.p_max.x > self.p_min.x {
            offset.x = offset.x / (self.p_max.x - self.p_min.x);
        }
        if self.p_max.y > self.p_min.y {
            offset.y = offset.y / (self.p_max.y - self.p_min.y);
        }
        offset
    }
    fn bounding_sphere(&self) -> (Self::Point, f32) {
        let center = (self.p_min + self.p_max) / (S::one() + S::one());
        let radius = if Bounds::inside(&center, self) {
            Point::distance(&center, &self.p_max)
                .to_f32()
                .expect("Panic on bounding sphere radius calculation cast")
        } else {
            S::zero()
                .to_f32()
                .expect("Panic on casting S::zero() to f32")
        };
        (center, radius)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Bounds3<S: Scalar> {
    p_min: Point3<S>,
    p_max: Point3<S>,
}
impl<S> Default for Bounds3<S>
where
    S: Scalar + Bounded,
{
    fn default() -> Bounds3<S> {
        let min = Bounded::min_value();
        let max = Bounded::max_value();

        Bounds3 {
            p_min: Point3::new(min, min, min),
            p_max: Point3::new(max, max, max),
        }
    }
}

impl<'a, S: Scalar> From<&'a Point3<S>> for Bounds3<S> {
    fn from(p: &'a Point3<S>) -> Self {
        Bounds3 {
            p_min: p.clone(),
            p_max: p.clone(),
        }
    }
}
impl<S: Scalar> From<(Point3<S>, Point3<S>)> for Bounds3<S> {
    fn from(points: (Point3<S>, Point3<S>)) -> Self {
        Bounds3 {
            p_min: points.0,
            p_max: points.1,
        }
    }
}
impl<'a, S: Scalar> From<(&'a Point3<S>, &'a Point3<S>)> for Bounds3<S> {
    fn from(points: (&'a Point3<S>, &'a Point3<S>)) -> Self {
        Bounds3 {
            p_min: points.0.clone(),
            p_max: points.1.clone(),
        }
    }
}

impl<S: Scalar> Index<u8> for Bounds3<S> {
    type Output = Point3<S>;
    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.p_min,
            1 => &self.p_max,
            _ => panic!("Bounds2 Index (b[{}]) out of range", index),
        }
    }
}
impl<S: Scalar> Bounds for Bounds3<S> {
    type Scalar = S;
    type Point = Point3<S>;
    type Vector = Vector3<S>;

    fn corner(&self, corner: u8) -> Self::Point {
        let x = self[corner & 1].x;
        let y = self[corner & 2].y;
        let z = self[corner & 4].z;
        Point3::new(x, y, z)
    }
    fn point_union(b: &Self, p: &Self::Point) -> Self {
        // b.p_min.x.min(p.x)
        // b.p_min.y.min(p.y)
        // b.p_min.z.min(p.z)

        // let x_max = b.p_max.x.max(p.x);
        // let y_max = b.p_max.y.max(p.y);
        // let z_max = b.p_max.z.max(p.z);
        let pmin = Point::min(&b.p_min, &p);
        let pmax = Point::max(&b.p_max, &p);
        Bounds3::from((pmin, pmax))
    }

    fn bounds_union(b1: &Self, b2: Self) -> Self {
        let pmin = Point::min(&b1.p_min, &b2.p_min);
        let pmax = Point::max(&b1.p_max, &b2.p_max);
        Bounds3::from((pmin, pmax))
    }
    fn intersect(b1: &Self, b2: &Self) -> Self {
        let pmin = Point::max(&b1.p_min, &b2.p_min);
        let pmax = Point::min(&b1.p_max, &b2.p_max);
        Bounds3::from((pmin, pmax))
    }
    fn overlaps(b1: &Self, b2: &Self) -> bool {
        let x = (b1.p_max.x >= b2.p_min.x) && (b1.p_min.x <= b2.p_max.x);
        let y = (b1.p_max.y >= b2.p_min.y) && (b1.p_min.y <= b2.p_max.y);
        let z = (b1.p_max.z >= b2.p_min.z) && (b1.p_min.z <= b2.p_max.z);
        x && y && z
    }
    fn inside(p: &Self::Point, b: &Self) -> bool {
        let x = p.x >= b.p_min.x && p.x <= b.p_max.x;
        let y = p.y >= b.p_min.y && p.y <= b.p_max.y;
        let z = p.z >= b.p_min.z && p.z <= b.p_max.z;
        x && y && z
    }
    fn inside_exclusive(p: &Self::Point, b: &Self) -> bool {
        let x = p.x >= b.p_min.x && p.x < b.p_max.x;
        let y = p.y >= b.p_min.y && p.y < b.p_max.y;
        let z = p.z >= b.p_min.z && p.z < b.p_max.z;
        x && y && z
    }
    fn expand(b1: &Self, delta: Self::Scalar) -> Self {
        let delta_vec = Vector3::new(delta, delta, delta);
        let p_min = b1.p_min - delta_vec;
        let p_max = b1.p_max + delta_vec;
        Bounds3::from((p_min, p_max))
    }
    fn diagonal(&self) -> Self::Vector {
        self.p_max - self.p_min
    }
    fn surface_area(&self) -> Self::Scalar {
        let d = self.diagonal();
        S::one() + S::one() * (d.x * d.y + d.x * d.z + d.y * d.z)
    }
    fn volume(&self) -> Self::Scalar {
        let d = self.diagonal();
        d.x * d.y * d.z
    }
    fn maximum_extent(&self) -> u8 {
        let d = self.diagonal();
        if d.x > d.y && d.x > d.z {
            0
        } else if d.y > d.z {
            1
        } else {
            2
        }
    }
    fn lerp(&self, t: &Self::Point) -> Self::Point {
        let x = super::lerp(t.x, self.p_min.x, self.p_max.x);
        let y = super::lerp(t.y, self.p_min.y, self.p_max.y);
        let z = super::lerp(t.z, self.p_min.z, self.p_max.z);
        Point3::new(x, y, z)
    }

    fn offset(&self, p: &Self::Point) -> Self::Vector {
        let mut offset = &*p - &self.p_min;
        if self.p_max.x > self.p_min.x {
            offset.x = offset.x / (self.p_max.x - self.p_min.x);
        }
        if self.p_max.y > self.p_min.y {
            offset.y = offset.y / (self.p_max.y - self.p_min.y);
        }
        offset
    }
    fn bounding_sphere(&self) -> (Self::Point, f32) {
        let center = (self.p_min + self.p_max) / (S::one() + S::one());
        let radius = if Bounds::inside(&center, self) {
            Point::distance(&center, &self.p_max)
                .to_f32()
                .expect("Panic on bounding sphere radius calculation cast")
        } else {
            S::zero()
                .to_f32()
                .expect("Panic on casting S::zero() to f32")
        };
        (center, radius)
    }
}
