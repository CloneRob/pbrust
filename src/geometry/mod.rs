use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, Neg, Sub};

pub mod bounds;
pub mod ray;
pub mod transform;
pub mod normal;
pub mod vector;
pub mod point;

use std::cmp;
use util;
use num;

use num::{Float, Integer};
// pub trait Scalar:  Float + Integer {}
pub trait Scalar: Float {}
impl Scalar for f32 {}
impl Scalar for f64 {}

pub trait Metric<RHS = Self> {
    type Output;
    fn distance(self, rhs: RHS) -> Self::Output;
    fn distance_squared(self, rhs: RHS) -> Self::Output;
    fn norm(self) -> Self::Output;
    fn length_squared(self) -> Self::Output;
}

pub trait VectorSpace<S: Scalar, RHS = Self>
where
    Self: Add,
    Self: Sub,
    Self: Mul<S>,
    Self: Div<S>,
    Self: Sized,
{
    type Scalar;
    type Output;
    fn abs_dot(self, rhs: RHS) -> Self::Scalar;
    fn dot(self, rhs: RHS) -> Self::Scalar;
}

pub trait Vector<S: Scalar>
where
    Self: VectorSpace<S>,
    Self: Metric<Output = S>,
    Self: Sized,
{
    fn zero() -> Self;
    fn unit() -> Self;
    fn has_nan(&self) -> bool;
    fn normalize(v: &Self) -> Self;
    fn normalize_inplace(&mut self);
}

pub trait Point<S: Scalar> {
    fn zero() -> Self;
    fn unit() -> Self;
    fn has_nan(&self) -> bool;

    fn distance(p1: &Self, p2: &Self) -> S;
    fn distance_squared(p1: &Self, p2: &Self) -> S;
    fn lerp(t: S, p1: &Self, p2: &Self) -> Self;
    fn min(p1: &Self, p2: &Self) -> Self;
    fn max(p1: &Self, p2: &Self) -> Self;
    fn floor(&self) -> Self;
    fn ceil(&self) -> Self;
    fn abs(&self) -> Self;
}

pub fn lerp<S: Scalar>(t: S, v1: S, v2: S) -> S {
    return (S::one() - t) * v1 + t * v2;
}

pub fn radians<S: Scalar>(deg: S) -> S {
    deg.to_radians()
}
