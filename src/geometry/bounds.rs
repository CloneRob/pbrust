use std::convert::From;
use std::ops::Index;
use std::iter::Iterator;
use std::cmp::{min, max};

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
use std::cmp::{PartialOrd, PartialEq};

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
    fn maximum_extent(&self) -> Self::Scalar;
    fn lerp(&self, t: Self::Point) -> Self::Point;
    fn offset(&self, p: Self::Point) -> Self::Vector;
}


#[derive(Copy, Clone, Debug)]
pub struct Bounds2<S: Scalar> {
    p_min: Point2<S>,
    p_max: Point2<S>,
}
impl<S> Default for Bounds2<S>
    where S: Scalar + Bounded
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
        // b.p_min.x.min(p.x)
        // b.p_min.y.min(p.y)
        // b.p_min.z.min(p.z)

        // let x_max = b.p_max.x.max(p.x);
        // let y_max = b.p_max.y.max(p.y);
        // let z_max = b.p_max.z.max(p.z);
        let pmin = Point2::new(min(b.p_min.x, p.x), min(b.p_min.y, p.y));
        let pmax = Point2::new(max(b.p_max.x, p.x), max(b.p_max.y, p.y));
        Bounds2::from((pmin, pmax))
    }

    fn bounds_union(b1: &Self, b2: Self) -> Self {
        let pmin = Point2::new(min(b1.p_min.x, b2.p_min.x), min(b1.p_min.y, b2.p_min.y));
        let pmax = Point2::new(max(b1.p_max.x, b2.p_max.x), max(b1.p_max.y, b2.p_max.y));
        Bounds2::from((pmin, pmax))
    }
    fn intersect(b1: &Self, b2: &Self) -> Self {
        let pmin = Point2::new(max(b1.p_min.x, b2.p_min.x), max(b1.p_min.y, b2.p_min.y));
        let pmax = Point2::new(min(b1.p_max.x, b2.p_max.x), min(b1.p_max.y, b2.p_max.y));
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
    fn maximum_extent(&self) -> Self::Scalar {
        unimplemented!()
    }
    fn lerp(&self, t: Self::Point) -> Self::Point {
        let x = super::lerp(t.x, self.p_min.x, self.p_max.x);
        let y = super::lerp(t.y, self.p_min.y, self.p_max.y);
        Point2::new(x, y)
    }

    fn offset(&self, p: Self::Point) -> Self::Vector {
        unimplemented!()
    }
}

// impl<S> Bounds2<S> where S: Scalar{

//     fn zero() -> Bounds2<S> {
//         Bounds2 {
//             p_min: Point2::zero(),
//             p_max: Point2::zero(),
//         }
//     }
//     fn diagonal(&self) -> Vector2<S> {
//         Vector2::from(self.p_max - self.p_min)
//     }

//     fn area(&self) -> S {
//         let a = self.p_max - self.p_min;
//         a.x * a.y
//     }

//     fn expand(&self, delta: S) -> Bounds2<S> {
//         let delta_vec = Vector2::new(delta, delta);
//         // let p_min = self.p_min - delta_vec;
//         // let p_max = self.p_max + delta_vec;
//         let p_min = self.p_min;
//         let p_max = self.p_max;
//         Bounds2 {
//             p_min: p_min,
//             p_max: p_max,
//         }
//     }

//     fn maximum_extend(&self) -> u8 {
//         let diag = self.diagonal();
//         if diag.x > diag.y {
//             0
//         } else {
//             1
//         }
//     }

//     fn offset(&self, p: &Point2<S>) -> Vector2<S> {
//         let mut offset = &*p - &self.p_min;
//         if self.p_max.x > self.p_min.x {
//             offset.x = offset.x / (self.p_max.x - self.p_min.x);
//         }
//         if self.p_max.y > self.p_min.y {
//             offset.y = offset.y / (self.p_max.y - self.p_min.y);
//         }
//         offset
//     }
//     fn lerp(&self, t: &Point2<S>) -> Point2<S> {
//         let x = super::lerp(t.x, self.p_min.x, self.p_max.x);
//         let y = super::lerp(t.y, self.p_min.y, self.p_max.y);
//         Point2::new(x, y)
//     }
// }

// impl<S> Default for Bounds2<S> where
//     S: Scalar + Bounded {
//     fn default() -> Bounds2<S> {
//         let min = Bounded::min_value();
//         let max = Bounded::max_value();

//         Bounds2 {
//             p_min: Point2::new(min, min),
//             p_max: Point2::new(max, max),
//         }
//     }
// }

// impl<'p, S> From<&'p Point2<S>> for Bounds2<S> where
//     S: Scalar {
//     fn from(t: &'p Point2<S>) -> Self {
//         Bounds2 {
//             p_min: t.clone(),
//             p_max: t.clone(),
//         }
//     }
// }

// impl<S> PartialEq for Bounds2<S> where
//     S: Scalar {
//     fn eq(&self, other: &Bounds2<S>) -> bool {
//         if self.p_min == other.p_min &&
//            self.p_max == other.p_max {
//                true
//            } else {
//                false
//            }
//     }
// }
