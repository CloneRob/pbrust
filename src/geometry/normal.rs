use std::convert::{From, Into};

use cg::{Vector3};
use cg::{BaseFloat};
use cg::prelude::*;

use num::Bounded;

use std::ops::Add;
use std::cmp::{PartialOrd, PartialEq};

#[derive(Copy, Clone, Debug)]
struct Normal3<T> where T: BaseFloat {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: BaseFloat> Normal3<T> {
    pub fn new(x: T, y: T, z: T) -> Normal3<T> {
        Normal3 {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn cross(self, other: Normal3<T>) -> Normal3<T> {
        Normal3::new((self.y * other.z) - (self.z * other.y),
                     (self.z * other.x) - (self.x * other.z),
                     (self.x * other.y) - (self.y * other.x))
    }
}

impl<T> From<Vector3<T>> for Normal3<T> where T: BaseFloat{
    fn from(v: Vector3<T>) -> Normal3<T> {
        Normal3 {
            x: v.x,
            y: v.y,
            z: v.z
        }
    }
}

impl<'a, T> From<&'a Vector3<T>> for Normal3<T> where T: BaseFloat{
    fn from(v: &'a Vector3<T>) -> Normal3<T> {
        Normal3 {
            x: v.x,
            y: v.y,
            z: v.z
        }
    }
}

impl<T> Into<Vector3<T>> for Normal3<T> where T: BaseFloat {
    fn into(self) -> Vector3<T> {
        Vector3::new(self.x, self.y, self.z)
    }
}
impl<'a, T> Into<Vector3<T>> for &'a Normal3<T> where T: BaseFloat {
    fn into(self) -> Vector3<T> {
        Vector3::new(self.x, self.y, self.z)
    }
}

impl<T: BaseFloat> MetricSpace for Normal3<T> {
    type Metric = T;

    fn distance2(self, other: Self) -> Self::Metric {
         
        let dist = ((self.x - other.x) + (self.y - other.y) + (self.z - other.z));
        dist
    } 
    // fn distance(self, other: Self) -> Self::Metric {
    //     T::One()
    // } 
}

// impl<T: BaseFloat> VectorSpace for Normal3<T> {

// }