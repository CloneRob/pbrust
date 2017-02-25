use std::convert::From;

use na;
use std::ops::Add;
use na::{Point2, Point3, Vector2, Vector3};
use num::{Zero, Bounded, Integer};
use std::cmp::{PartialOrd, PartialEq};

use ::geometry::{Interpolatable, naFloat, naInt, naBase};


trait InsideBounds {
    type B;
    fn inside(&self, b: &Self::B) -> bool;
    fn inside_exclusive(&self, b: &Self::B) -> bool;
}

impl<T: naBase + PartialOrd> InsideBounds for Point2<T> {
    type B = Bounds2<T>;
    fn inside(&self, b: &Self::B) -> bool {
       self.x >= b.p_min.x && self.x <= b.p_max.x && self.y >= b.p_min.y && self.y <= b.p_max.y
    }
    fn inside_exclusive(&self, b: &Self::B) -> bool {
       self.x >= b.p_min.x && self.x < b.p_max.x && self.y >= b.p_min.y && self.y < b.p_max.y
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Bounds2<T> where 
    T: naBase {
    p_min: Point2<T>,
    p_max: Point2<T>,
}

impl<T: naBase> Bounds2<T> {

    fn zero() -> Bounds2<T> {
        Bounds2 {
            p_min: Point2::origin(),
            p_max: Point2::origin(),
        }
    }
    fn diagonal(&self) -> Vector2<T> {
        Vector2::from(self.p_max - self.p_min)
    }

    fn area(&self) -> T {
        let a = self.p_max - self.p_min;
        a.x * a.y
    }

    fn from_vectors(v1: Vector2<T>, v2: Vector2<T>) -> Bounds2<T> {
        Bounds2 {
            p_min: Point2::from_coordinates(v1),
            p_max: Point2::from_coordinates(v2),
        }
    }

    fn expand(&self, delta: T) -> Bounds2<T> {
        let delta_vec = Vector2::new(delta, delta);
        Bounds2 {
            p_min: self.p_min - delta_vec,
            p_max: self.p_max + delta_vec,
        }
    }
}

impl<T> Bounds2<T> where
    T: naBase + PartialOrd {
    fn maximum_extend(&self) -> u8 {
        let diag = self.diagonal();
        if diag.x > diag.y {
            0
        } else {
            1
        }
    }

    fn offset(&self, p: &Point2<T>) -> Point2<T> {
        let mut o = p - self.p_min;
        if self.p_max.x > self.p_min.x {
            o.x /= self.p_max.x - self.p_min.x;
        }
        if self.p_max.y > self.p_min.y {
            o.y /= self.p_max.y - self.p_min.y;
        }
        Point2::from_coordinates(o)
    }
}

impl<T> Bounds2<T> where
    T: naBase + Interpolatable<A=T, V1=T, V2=T> {
    fn lerp(&self, t: &Point2<T>) -> Point2<T> {
        Point2::new(self.p_min.x.lerp(t.x, &self.p_max.x),
                    self.p_min.y.lerp(t.y, &self.p_max.y))
    }
}

impl<T> Default for Bounds2<T> where 
    T: naBase {
    fn default() -> Bounds2<T> {
        let min = Bounded::min_value();
        let max = Bounded::max_value();

        Bounds2 {
            p_min: Point2::new(min, min),
            p_max: Point2::new(max, max),
        }
    }
}

impl<'p, T> From<&'p Point2<T>> for Bounds2<T> where 
    T: naBase {
    fn from(t: &'p Point2<T>) -> Self {
        Bounds2 {
            p_min: t.clone(), 
            p_max: t.clone(),
        }    
    }
}

impl<T> PartialEq for Bounds2<T> where 
    T: naBase + PartialOrd {
    fn eq(&self, other: &Bounds2<T>) -> bool {
        if self.p_min == other.p_min &&
           self.p_max == other.p_max {
               true
           } else {
               false
           }
    }
}
