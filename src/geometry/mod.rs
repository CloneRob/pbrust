pub mod bounds;
pub mod ray;
pub mod transform;

use ::cg;
use std::cmp;
use cg::BaseFloat;
use cg::prelude::*;
use ::util;


fn face_forward(v1: &cg::Vector3<f64>, v2: &cg::Vector3<f64>) -> cg::Vector3<f64> {
    if v1.dot(v2.clone()) < 0.0 {
        -*v1
    } else {
        v1.clone()
    }
}

pub trait Interpolate {
    type V1;
    type V2;
    type A: BaseFloat;
    fn lerp(&self, alpha: Self::A, v1: &Self::V1) -> Self::V2;
}

trait Combinable {
    type V;
    fn min_combination(&self, v: &Self::V) -> Self::V;
    fn max_combination(&self, v: &Self::V) -> Self::V;
}

impl Interpolate for f32 {
    type V1 = f32;
    type V2 = f32;
    type A = f32;
    fn lerp(&self, alpha: Self::A, v1: &Self::V1) -> Self::V2 {
        self * (Self::A::one() - alpha) + v1 * (alpha)
    }
}

impl Interpolate for f64 {
    type V1 = f64;
    type V2 = f64;
    type A = f64;
    fn lerp(&self, alpha: Self::A, v1: &Self::V1) -> Self::V2 {
        self * (Self::A::one() - alpha) + v1 * (alpha)
    }
}

impl<T> Interpolate for cg::Vector2<T> where 
    T: BaseFloat {
    type V1 = cg::Vector2<T>;
    type V2 = cg::Vector2<T>;
    type A = T;
    fn lerp(&self, alpha: Self::A, v1: &Self::V1) -> Self::V2 {
        self * (Self::A::one() - alpha) + v1 * (alpha)
    }
}

impl<T> Interpolate for cg::Vector3<T> where 
    T: BaseFloat {
    type V1 = cg::Vector3<T>;
    type V2 = cg::Vector3<T>;
    type A = T;
    fn lerp(&self, alpha: Self::A, v1: &Self::V1) -> Self::V2 {
        self * (Self::A::one() - alpha) + v1 * (alpha)
    }
}

impl<T> Interpolate for cg::Point2<T> where 
    T: BaseFloat {
    type V1 = cg::Point2<T>;
    type V2 = cg::Point2<T>;
    type A = T;
    fn lerp(&self, alpha: Self::A, v1: &Self::V1) -> Self::V2 {
        let mut p1 = self * (Self::A::one() - alpha);
        let p2 = v1 * alpha;

        p1.x += p2.x;
        p1.y += p2.y;

        p1
    }
}

impl<T> Interpolate for cg::Point3<T> where 
    T: BaseFloat {
    type V1 = cg::Point3<T>;
    type V2 = cg::Point3<T>;
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

impl<T> Combinable for cg::Vector3<T> where 
    T: BaseFloat {
    type V = cg::Vector3<T>;
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

impl<T> Combinable for cg::Point3<T> where 
    T: BaseFloat {
    type V = cg::Point3<T>;
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
impl<T> Combinable for cg::Vector3<T> where 
    T: cg::Scalar + cmp::Ord {
    type V = cg::Vector3<T>;
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

impl<T> Combinable for cg::Point3<T> where 
    T: cg::Scalar +  cmp::Ord + cmp::PartialOrd{
    type V = cg::Point3<T>;
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
    let v1 = cg::Vector2::<f64>::x();
    let v2 = cg::Vector2::<f64>::y();
    let alpha = 0.77f64;

    let v3 = v1.lerp(alpha, &v2);


    let v1 = cg::Vector3::<f64>::x();
    let v2 = cg::Vector3::<f64>::y();
    let r = cg::Vector3::new(1.0f64, 1.0, 0.0);
    assert_eq!(r, v1.max_combination(&v2));

}
