use std::marker::Sized;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, Neg, Sub};
use std::convert::From;

use super::Scalar;
use super::normal::{Normal2, Normal3};
use super::{Metric, Vector, VectorSpace};

pub type Vector2f = Vector2<f32>;
pub type Vector2i = Vector2<i32>;
pub type Vector3f = Vector3<f32>;
pub type Vector3i = Vector3<i32>;

#[derive(Debug, Copy, Clone)]
pub struct Vector2<S: Scalar> {
    pub x: S,
    pub y: S,
}

impl<S: Scalar> Vector<S> for Vector2<S> {
    fn zero() -> Self {
        Vector2 {
            x: S::zero(),
            y: S::zero(),
        }
    }

    fn unit() -> Self {
        Vector2 {
            x: S::one(),
            y: S::one(),
        }
    }

    fn has_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }
    fn normalize(v: &Self) -> Self {
        v / v.norm()
    }
    fn normalize_inplace(&mut self) {
        let norm = self.norm();
        self.div_assign(norm);
    }
}

impl<S: Scalar> VectorSpace<S> for Vector2<S> {
    type Output = Self;
    type Scalar = S;
    fn abs_dot(self, rhs: Self) -> Self::Scalar {
        self.dot(rhs).abs()
    }
    fn dot(self, rhs: Self) -> Self::Scalar {
        self.x * rhs.x + self.y * rhs.y
    }
}
impl<'a, S: Scalar> VectorSpace<S> for &'a Vector2<S> {
    type Output = Vector3<S>;
    type Scalar = S;
    fn abs_dot(self, rhs: Self) -> Self::Scalar {
        self.dot(rhs).abs()
    }

    fn dot(self, rhs: Self) -> Self::Scalar {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<S: Scalar> Metric for Vector2<S> {
    type Output = S;
    fn distance(self, rhs: Self) -> Self::Output {
        (self - rhs).norm()
    }
    fn distance_squared(self, rhs: Self) -> Self::Output {
        (self - rhs).norm()
    }
    fn length_squared(self) -> Self::Output {
        self.x.powi(2) + self.y.powi(2)
    }
    fn norm(self) -> Self::Output {
        (self.length_squared()).sqrt()
    }
}

impl<'a, S: Scalar> Metric for &'a Vector2<S> {
    type Output = S;
    fn distance(self, rhs: Self) -> Self::Output {
        (*self - *rhs).norm()
    }
    fn distance_squared(self, rhs: Self) -> Self::Output {
        (*self - *rhs).norm()
    }
    fn length_squared(self) -> Self::Output {
        self.x.powi(2) + self.y.powi(2)
    }
    fn norm(self) -> Self::Output {
        (self.length_squared()).sqrt()
    }
}
impl<S: Scalar> Metric for Box<Vector2<S>> {
    type Output = S;
    fn distance(self, rhs: Self) -> Self::Output {
        (*self - *rhs).norm()
    }
    fn distance_squared(self, rhs: Self) -> Self::Output {
        (*self - *rhs).norm()
    }
    fn length_squared(self) -> Self::Output {
        self.x.powi(2) + self.y.powi(2)
    }
    fn norm(self) -> Self::Output {
        (self.length_squared()).sqrt()
    }
}

impl<S: Scalar> Vector2<S> {
    pub fn new(x: S, y: S) -> Vector2<S> {
        let v = Vector2 { x: x, y: y };
        assert!(!v.has_nan());
        v
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vector3<S: Scalar> {
    pub x: S,
    pub y: S,
    pub z: S,
}

impl<S: Scalar> VectorSpace<S> for Vector3<S> {
    type Output = Self;
    type Scalar = S;
    fn abs_dot(self, rhs: Self) -> Self::Scalar {
        self.dot(rhs).abs()
    }
    fn dot(self, rhs: Self) -> Self::Scalar {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
impl<'a, S: Scalar> VectorSpace<S> for &'a Vector3<S> {
    type Output = Vector3<S>;
    type Scalar = S;
    fn abs_dot(self, rhs: Self) -> Self::Scalar {
        self.dot(rhs).abs()
    }

    fn dot(self, rhs: Self) -> Self::Scalar {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<S: Scalar> Vector<S> for Vector3<S> {
    fn zero() -> Self {
        Vector3 {
            x: S::zero(),
            y: S::zero(),
            z: S::zero(),
        }
    }

    fn unit() -> Self {
        Vector3 {
            x: S::one(),
            y: S::one(),
            z: S::one(),
        }
    }

    fn has_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }
    fn normalize(v: &Self) -> Self {
        v / v.norm()
    }
    fn normalize_inplace(&mut self) {
        let norm = self.norm();
        self.div_assign(norm);
    }
}

impl<S: Scalar> Vector3<S> {
    pub fn new(x: S, y: S, z: S) -> Vector3<S> {
        let v = Vector3 { x: x, y: y, z: z };
        assert!(!v.has_nan());
        v
    }

    pub fn coordinate_system(&self) -> (Vector3<S>, Vector3<S>) {
        let v2 = if self.x.abs() > self.y.abs() {
            let denom = (self.x.powi(2) + self.z.powi(2)).sqrt();
            Self::new(-self.z, S::zero(), self.x) / denom
        } else {
            let denom = (self.y.powi(2) + self.z.powi(2)).sqrt();
            Self::new(S::zero(), self.z, -self.y) / denom
        };
        let v3 = self.cross(&v2);
        (v2, v3)
    }

    pub fn cross(self, rhs: &Self) -> Self {
        let x = self.y * rhs.z - self.z * rhs.y;
        let y = self.z * rhs.x - self.x * rhs.z;
        let z = self.x * rhs.y - self.y * rhs.x;
        Vector3::new(x, y, z)
    }
}

impl<S: Scalar> Metric for Vector3<S> {
    type Output = S;
    fn distance(self, rhs: Self) -> Self::Output {
        (self - rhs).norm()
    }
    fn distance_squared(self, rhs: Self) -> Self::Output {
        (self - rhs).norm()
    }
    fn length_squared(self) -> Self::Output {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }
    fn norm(self) -> Self::Output {
        (self.length_squared()).sqrt()
    }
}

impl<'a, S: Scalar> Metric for &'a Vector3<S> {
    type Output = S;
    fn distance(self, rhs: Self) -> Self::Output {
        (*self - *rhs).norm()
    }
    fn distance_squared(self, rhs: Self) -> Self::Output {
        (*self - *rhs).norm()
    }
    fn length_squared(self) -> Self::Output {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }
    fn norm(self) -> Self::Output {
        (self.length_squared()).sqrt()
    }
}
impl<S: Scalar> Metric for Box<Vector3<S>> {
    type Output = S;
    fn distance(self, rhs: Self) -> Self::Output {
        (*self - *rhs).norm()
    }
    fn distance_squared(self, rhs: Self) -> Self::Output {
        (*self - *rhs).norm()
    }
    fn length_squared(self) -> Self::Output {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }
    fn norm(self) -> Self::Output {
        (self.length_squared()).sqrt()
    }
}

impl<S: Scalar> Add for Vector3<S> {
    type Output = Vector3<S>;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl<'a, S: Scalar> Add for &'a Vector3<S> {
    type Output = Vector3<S>;
    fn add(self, other: Self) -> Self::Output {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<S: Scalar> Add<S> for Vector3<S> {
    type Output = Vector3<S>;
    fn add(self, other: S) -> Self {
        Self::new(self.x + other, self.y + other, self.z + other)
    }
}
impl<'a, S: Scalar> Add<Vector3<S>> for &'a Vector3<S> {
    type Output = Vector3<S>;
    fn add(self, other: Vector3<S>) -> Self::Output {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<S: Scalar> AddAssign for Vector3<S> {
    fn add_assign(&mut self, other: Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }
}
impl<S: Scalar> AddAssign<S> for Vector3<S> {
    fn add_assign(&mut self, other: S) {
        self.x = self.x + other;
        self.y = self.y + other;
        self.z = self.z + other;
    }
}

impl<S: Scalar> Neg for Vector3<S> {
    type Output = Self;

    fn neg(self) -> Self {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

impl<S: Scalar> Sub for Vector3<S> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
impl<'a, S: Scalar> Sub for &'a Vector3<S> {
    type Output = Vector3<S>;
    fn sub(self, other: Self) -> Self::Output {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<S: Scalar> Sub<S> for Vector3<S> {
    type Output = Self;
    fn sub(self, other: S) -> Self {
        Vector3::new(self.x - other, self.y - other, self.z - other)
    }
}

impl<'a, S: Scalar> Sub<S> for &'a Vector3<S> {
    type Output = Vector3<S>;
    fn sub(self, other: S) -> Self::Output {
        Vector3::new(self.x - other, self.y - other, self.z - other)
    }
}

impl<S: Scalar> Div<S> for Vector3<S> {
    type Output = Self;
    fn div(self, rhs: S) -> Self {
        assert!(rhs != S::zero());
        let inv = rhs.recip();
        self * inv
    }
}
impl<'a, S: Scalar> Div<S> for &'a Vector3<S> {
    type Output = Vector3<S>;
    fn div(self, rhs: S) -> Self::Output {
        assert!(rhs != S::zero());
        let inv = rhs.recip();
        self * inv
    }
}
impl<S: Scalar> DivAssign<S> for Vector3<S> {
    fn div_assign(&mut self, other: S) {
        self.x = self.x / other;
        self.y = self.y / other;
        self.z = self.z / other;
    }
}

impl<'a, S: Scalar> DivAssign<S> for &'a mut Vector3<S> {
    fn div_assign(&mut self, other: S) {
        self.x = self.x / other;
        self.y = self.y / other;
        self.z = self.z / other;
    }
}

impl<S: Scalar> Mul<S> for Vector3<S> {
    type Output = Self;
    fn mul(self, rhs: S) -> Self {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl<'a, S: Scalar> Mul<S> for &'a Vector3<S> {
    type Output = Vector3<S>;
    fn mul(self, rhs: S) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<S: Scalar> Index<u8> for Vector3<S> {
    type Output = S;
    fn index(&self, index: u8) -> &S {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vector Index out of range"),
        }
    }
}
impl<S: Scalar> Index<usize> for Vector3<S> {
    type Output = S;
    fn index(&self, index: usize) -> &S {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vector Index out of range"),
        }
    }
}
impl<S: Scalar> Index<u32> for Vector3<S> {
    type Output = S;
    fn index(&self, index: u32) -> &S {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vector3 Index (v[{}]) out of range", index),
        }
    }
}
impl<S: Scalar> IndexMut<u8> for Vector3<S> {
    fn index_mut<'a>(&'a mut self, index: u8) -> &'a mut S {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vector3 Index (v[{}]) out of range", index),
        }
    }
}
impl<S: Scalar> IndexMut<usize> for Vector3<S> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut S {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vector3 Index (v[{}]) out of range", index),
        }
    }
}
impl<S: Scalar> IndexMut<u32> for Vector3<S> {
    fn index_mut<'a>(&'a mut self, index: u32) -> &'a mut S {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vector3 Index (v[{}]) out of range", index),
        }
    }
}
impl<S: Scalar> Add for Vector2<S> {
    type Output = Vector2<S>;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}
impl<'a, S: Scalar> Add for &'a Vector2<S> {
    type Output = Vector2<S>;
    fn add(self, other: Self) -> Self::Output {
        Vector2::new(self.x + other.x, self.y + other.y)
    }
}

impl<S: Scalar> Add<S> for Vector2<S> {
    type Output = Vector2<S>;
    fn add(self, other: S) -> Self {
        Self::new(self.x + other, self.y + other)
    }
}
impl<'a, S: Scalar> Add<Vector3<S>> for &'a Vector2<S> {
    type Output = Vector2<S>;
    fn add(self, other: Vector3<S>) -> Self::Output {
        Vector2::new(self.x + other.x, self.y + other.y)
    }
}

impl<S: Scalar> AddAssign for Vector2<S> {
    fn add_assign(&mut self, other: Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}
impl<S: Scalar> AddAssign<S> for Vector2<S> {
    fn add_assign(&mut self, other: S) {
        self.x = self.x + other;
        self.y = self.y + other;
    }
}

impl<S: Scalar> Neg for Vector2<S> {
    type Output = Self;

    fn neg(self) -> Self {
        Vector2::new(-self.x, -self.y)
    }
}

impl<S: Scalar> Sub for Vector2<S> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vector2::new(self.x - other.x, self.y - other.y)
    }
}
impl<'a, S: Scalar> Sub for &'a Vector2<S> {
    type Output = Vector2<S>;
    fn sub(self, other: Self) -> Self::Output {
        Vector2::new(self.x - other.x, self.y - other.y)
    }
}

impl<S: Scalar> Sub<S> for Vector2<S> {
    type Output = Self;
    fn sub(self, other: S) -> Self {
        Vector2::new(self.x - other, self.y - other)
    }
}

impl<S: Scalar> Div<S> for Vector2<S> {
    type Output = Self;
    fn div(self, rhs: S) -> Self {
        assert!(rhs != S::zero());
        let inv = rhs.recip();
        self * inv
    }
}

impl<'a, S: Scalar> Div<S> for &'a Vector2<S> {
    type Output = Vector2<S>;
    fn div(self, rhs: S) -> Self::Output {
        assert!(rhs != S::zero());
        let inv = rhs.recip();
        self * inv
    }
}
impl<S: Scalar> DivAssign<S> for Vector2<S> {
    fn div_assign(&mut self, other: S) {
        self.x = self.x / other;
        self.y = self.y / other;
    }
}
impl<'a, S: Scalar> DivAssign<S> for &'a mut Vector2<S> {
    fn div_assign(&mut self, other: S) {
        self.x = self.x / other;
        self.y = self.y / other;
    }
}

impl<S: Scalar> Mul<S> for Vector2<S> {
    type Output = Self;
    fn mul(self, rhs: S) -> Self {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl<'a, S: Scalar> Mul<S> for &'a Vector2<S> {
    type Output = Vector2<S>;
    fn mul(self, rhs: S) -> Self::Output {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl<S: Scalar> Index<u8> for Vector2<S> {
    type Output = S;
    fn index(&self, index: u8) -> &S {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Vector Index out of range"),
        }
    }
}
impl<S: Scalar> Index<usize> for Vector2<S> {
    type Output = S;
    fn index(&self, index: usize) -> &S {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Vector Index out of range"),
        }
    }
}
impl<S: Scalar> Index<u32> for Vector2<S> {
    type Output = S;
    fn index(&self, index: u32) -> &S {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Vector3 Index (v[{}]) out of range", index),
        }
    }
}
impl<S: Scalar> IndexMut<u8> for Vector2<S> {
    fn index_mut<'a>(&'a mut self, index: u8) -> &'a mut S {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Vector3 Index (v[{}]) out of range", index),
        }
    }
}
impl<S: Scalar> IndexMut<usize> for Vector2<S> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut S {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Vector3 Index (v[{}]) out of range", index),
        }
    }
}
impl<S: Scalar> IndexMut<u32> for Vector2<S> {
    fn index_mut<'a>(&'a mut self, index: u32) -> &'a mut S {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Vector3 Index (v[{}]) out of range", index),
        }
    }
}
impl<S: Scalar> From<Normal3<S>> for Vector3<S> {
    fn from(v: Normal3<S>) -> Self {
        Vector3::new(v.x, v.y, v.z)
    }
}
impl<'a, S: Scalar> From<&'a Normal3<S>> for Vector3<S> {
    fn from(v: &'a Normal3<S>) -> Self {
        Vector3::new(v.x, v.y, v.z)
    }
}
impl<S: Scalar> From<Normal2<S>> for Vector2<S> {
    fn from(v: Normal2<S>) -> Self {
        Vector2::new(v.x, v.y)
    }
}
impl<'a, S: Scalar> From<&'a Normal2<S>> for Vector2<S> {
    fn from(v: &'a Normal2<S>) -> Self {
        Vector2::new(v.x, v.y)
    }
}
