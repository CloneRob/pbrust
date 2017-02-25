extern crate nalgebra as na;
extern crate num;
extern crate alga;

pub mod bounds;
pub mod ray;

use std::cmp;
use num::{One, Zero, Bounded, Float, Integer};
use alga::general::{ClosedSub, ClosedMul, ClosedAdd, ClosedDiv};
use ::util;

pub trait naBase: na::Scalar + One + Zero + Bounded + ClosedSub + ClosedMul + ClosedAdd + ClosedDiv{}
pub trait naFloat: Float + naBase {}
pub trait naInt: Integer + naBase {}
pub trait naNum: Float + Integer + naBase {}

impl naBase for f32 {}
impl naBase for f64 {}
impl naBase for i32 {}
impl naBase for i64 {}

fn face_forward(v1: &na::Vector3<f64>, v2: &na::Vector3<f64>) -> na::Vector3<f64> {
    if v1.dot(v2) < 0.0 {
        -v1
    } else {
        v1.clone()
    }
}

pub trait Interpolatable {
    type V1;
    type V2;
    type A;
    fn lerp(&self, alpha: Self::A, v1: &Self::V1) -> Self::V2;
}

trait Combinable {
    type V;
    fn min_combination(&self, v: &Self::V) -> Self::V;
    fn max_combination(&self, v: &Self::V) -> Self::V;
}

impl Interpolatable for f32 {
    type V1 = f32;
    type V2 = f32;
    type A = f32;
    fn lerp(&self, alpha: Self::A, v1: &Self::V1) -> Self::V2 {
        self * (Self::A::one() - alpha) + v1 * (alpha)
    }
}

impl Interpolatable for f64 {
    type V1 = f64;
    type V2 = f64;
    type A = f64;
    fn lerp(&self, alpha: Self::A, v1: &Self::V1) -> Self::V2 {
        self * (Self::A::one() - alpha) + v1 * (alpha)
    }
}

impl<T> Interpolatable for na::Vector2<T> where 
    T: naBase {
    type V1 = na::Vector2<T>;
    type V2 = na::Vector2<T>;
    type A = T;
    fn lerp(&self, alpha: Self::A, v1: &Self::V1) -> Self::V2 {
        self * (Self::A::one() - alpha) + v1 * (alpha)
    }
}

impl<T> Interpolatable for na::Vector3<T> where 
    T: naBase {
    type V1 = na::Vector3<T>;
    type V2 = na::Vector3<T>;
    type A = T;
    fn lerp(&self, alpha: Self::A, v1: &Self::V1) -> Self::V2 {
        self * (Self::A::one() - alpha) + v1 * (alpha)
    }
}

impl<T> Interpolatable for na::Point2<T> where 
    T: naBase {
    type V1 = na::Point2<T>;
    type V2 = na::Point2<T>;
    type A = T;
    fn lerp(&self, alpha: Self::A, v1: &Self::V1) -> Self::V2 {
        let mut p1 = self * (Self::A::one() - alpha);
        let p2 = v1 * alpha;

        p1.x += p2.x;
        p1.y += p2.y;

        p1
    }
}

impl<T> Interpolatable for na::Point3<T> where 
    T: naBase {
    type V1 = na::Point3<T>;
    type V2 = na::Point3<T>;
    type A = T;
    fn lerp(&self, alpha: Self::A, v1: &Self::V1) -> Self::V2 {
        let mut p1 = self * (Self::A::one() - alpha);
        let p2 = v1 * alpha;

        p1.x += p2.x;
        p1.y += p2.y;
        p1.z += p2.z;

        p1
    }
}

impl<T> Combinable for na::Vector3<T> where 
    T: na::Scalar + cmp::PartialOrd {
    type V = na::Vector3<T>;
    fn min_combination(&self, v: &Self::V) -> Self::V {
        return Self::V::new(util::minf(self.x, v.x), 
                                    util::minf(self.y, v.y),
                                    util::minf(self.z, v.z));
    }
    fn max_combination(&self, v: &Self::V) -> Self::V {
        return Self::V::new(util::maxf(self.x, v.x), 
                                    util::maxf(self.y, v.y),
                                    util::maxf(self.z, v.z));
    }
}

impl<T> Combinable for na::Point3<T> where 
    T: na::Scalar +  cmp::Ord + cmp::PartialOrd{
    type V = na::Point3<T>;
    fn min_combination(&self, v: &Self::V) -> Self::V {
        return Self::V::new(util::minf(self.x, v.x), 
                                    util::minf(self.y, v.y),
                                    util::minf(self.z, v.z));
    }
    fn max_combination(&self, v: &Self::V) -> Self::V {
        return Self::V::new(util::maxf(self.x, v.x), 
                                    util::maxf(self.y, v.y),
                                    util::maxf(self.z, v.z));
    }
}

/*
impl<T> Combinable for na::Vector3<T> where 
    T: na::Scalar + cmp::Ord {
    type V = na::Vector3<T>;
    fn min_combination(&self, v: &Self::V) -> Self::V {
        return Self::V::new(cmp::min(self.x, v.x), 
                                    cmp::min(self.y, v.y),
                                    cmp::min(self.z, v.z));
    }
    fn max_combination(&self, v: &Self::V) -> Self::V {
        return Self::V::new(cmp::max(self.x, v.x), 
                                    cmp::max(self.y, v.y),
                                    cmp::max(self.z, v.z));
    }
}

impl<T> Combinable for na::Point3<T> where 
    T: na::Scalar +  cmp::Ord + cmp::PartialOrd{
    type V = na::Point3<T>;
    fn min_combination(&self, v: &Self::V) -> Self::V {
        return Self::V::new(cmp::min(self.x, v.x), 
                                    cmp::min(self.y, v.y),
                                    cmp::min(self.z, v.z));
    }
    fn max_combination(&self, v: &Self::V) -> Self::V {
        return Self::V::new(cmp::max(self.x, v.x), 
                                    cmp::max(self.y, v.y),
                                    cmp::max(self.z, v.z));
    }
}
*/

#[test]
fn lerp_test() {
    let v1 = na::Vector2::<f64>::x();
    let v2 = na::Vector2::<f64>::y();
    let alpha = 0.77f64;

    let v3 = v1.lerp(alpha, &v2);


    let v1 = na::Vector3::<f64>::x();
    let v2 = na::Vector3::<f64>::y();
    let r = na::Vector3::new(1.0f64, 1.0, 0.0);
    assert_eq!(r, v1.max_combination(&v2));

}
