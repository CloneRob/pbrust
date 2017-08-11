// use std::marker::Sized;
use std::ops::{Add, AddAssign, Neg, Sub, Mul, Div, Index, IndexMut};
use std::cmp::{min, max};


use super::Scalar;
use super::{Vector, VectorSpace, Metric};
use super::Point;
use super::vector::{Vector2, Vector3};

type Point2f = Point2<f32>;
type Point2i = Point2<i32>;
type Point3f = Point3<f32>;
type Point3i = Point3<i32>;

#[derive(Debug, Copy, Clone)]
pub struct Point2<S: Scalar> {
    pub x: S,
    pub y: S,
}

impl<S: Scalar> Point<S> for Point2<S> {
    fn zero() -> Self {
        Point2 {
            x: S::zero(),
            y: S::zero(),
        }
    }

    fn unit() -> Self {
        Point2 {
            x: S::one(),
            y: S::one(),
        }
    }

    fn has_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }
    fn distance(p1: &Self, p2: &Self) -> S {
        (&*p1 - p2).norm()
    }
    fn distance_squared(p1: &Self, p2: &Self) -> S {
        (&*p1 - p2).length_squared()
    }
    fn lerp(t: S, p1: &Self, p2: &Self) -> Self {
        p1 * (S::one() - t) + p2 * t
    }
    fn min(p1: &Self, p2: &Self) -> Self {
        Point2::new(min(p1.x, p2.x), min(p1.y, p2.y))
    }
    fn max(p1: &Self, p2: &Self) -> Self {
        Point2::new(max(p1.x, p2.x), max(p1.y, p2.y))
    }

    fn floor(&self) -> Self {
        Point2::new(self.x.floor(), self.y.floor())
    }

    fn ceil(&self) -> Self {
        Point2::new(self.x.ceil(), self.y.ceil())
    }

    fn abs(&self) -> Self {
        Point2::new(self.x.abs(), self.y.abs())
    }
}


impl<S: Scalar> Point2<S> {
    pub fn new(x: S, y: S) -> Point2<S> {
        let v = Point2 { x: x, y: y };
        assert!(!v.has_nan());
        v
    }
    pub fn permute(&self, x: u8, y: u8) -> Self {
        Point2::new(self[x], self[y])
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Point3<S: Scalar> {
    pub x: S,
    pub y: S,
    pub z: S,
}


impl<S: Scalar> Point<S> for Point3<S> {
    fn zero() -> Self {
        Point3 {
            x: S::zero(),
            y: S::zero(),
            z: S::zero(),
        }
    }

    fn unit() -> Self {
        Point3 {
            x: S::one(),
            y: S::one(),
            z: S::one(),
        }
    }

    fn has_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }
    fn distance(p1: &Self, p2: &Self) -> S {
        (&*p1 - p2).norm()
    }
    fn distance_squared(p1: &Self, p2: &Self) -> S {
        (&*p1 - p2).length_squared()
    }
    fn lerp(t: S, p1: &Point3<S>, p2: &Point3<S>) -> Self {
        p1 * (S::one() - t) + p2 * t
    }
    fn min(p1: &Self, p2: &Self) -> Self {
        Point3::new(min(p1.x, p2.x), min(p1.y, p2.y), min(p1.z, p2.z))
    }
    fn max(p1: &Self, p2: &Self) -> Self {
        Point3::new(max(p1.x, p2.x), max(p1.y, p2.y), max(p1.z, p2.z))
    }

    fn floor(&self) -> Self {
        Point3::new(self.x.floor(), self.y.floor(), self.z.floor())
    }

    fn ceil(&self) -> Self {
        Point3::new(self.x.ceil(), self.y.ceil(), self.z.ceil())
    }

    fn abs(&self) -> Self {
        Point3::new(self.x.abs(), self.y.abs(), self.z.abs())
    }
}

impl<S: Scalar> Point3<S> {
    pub fn new(x: S, y: S, z: S) -> Point3<S> {
        let v = Point3 { x: x, y: y, z: z };
        assert!(!v.has_nan());
        v
    }
    pub fn permute(&self, x: u8, y: u8, z: u8) -> Self {
        Point3::new(self[x], self[y], self[z])
    }
}

impl<S: Scalar> Add for Point3<S> {
    type Output = Point3<S>;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl<'a, S: Scalar> Add for &'a Point3<S> {
    type Output = Point3<S>;
    fn add(self, other: Self) -> Self::Output {
        Point3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<S: Scalar> Add<S> for Point3<S> {
    type Output = Point3<S>;
    fn add(self, other: S) -> Self {
        Self::new(self.x + other, self.y + other, self.z + other)
    }
}
impl<'a, S: Scalar> Add<Point3<S>> for &'a Point3<S> {
    type Output = Point3<S>;
    fn add(self, other: Point3<S>) -> Self::Output {
        Point3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<S: Scalar> Add<Vector3<S>> for Point3<S> {
    type Output = Point3<S>;
    fn add(self, other: Vector3<S>) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl<'a, S: Scalar> Add<&'a Vector3<S>> for &'a Point3<S> {
    type Output = Point3<S>;
    fn add(self, other: &'a Vector3<S>) -> Self::Output {
        Point3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<'a, S: Scalar> AddAssign<&'a Point3<S>> for Point3<S> {
    fn add_assign(&mut self, other: &'a Point3<S>) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }
}
impl<S: Scalar> AddAssign<S> for Point3<S> {
    fn add_assign(&mut self, other: S) {
        self.x = self.x + other;
        self.y = self.y + other;
        self.z = self.z + other;
    }
}
impl<S: Scalar> AddAssign<Vector3<S>> for Point3<S> {
    fn add_assign(&mut self, other: Vector3<S>) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }
}
impl<'a, S: Scalar> AddAssign<&'a Vector3<S>> for Point3<S> {
    fn add_assign(&mut self, other: &'a Vector3<S>) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }
}

impl<S: Scalar> Neg for Point3<S> {
    type Output = Self;

    fn neg(self) -> Self {
        Point3::new(-self.x, -self.y, -self.z)
    }
}

impl<S: Scalar> Sub for Point3<S> {
    type Output = Vector3<S>;
    fn sub(self, other: Self) -> Self::Output {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
impl<'a, S: Scalar> Sub for &'a Point3<S> {
    type Output = Vector3<S>;
    fn sub(self, other: Self) -> Self::Output {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<S: Scalar> Sub<S> for Point3<S> {
    type Output = Self;
    fn sub(self, other: S) -> Self::Output {
        Point3::new(self.x - other, self.y - other, self.z - other)
    }
}

impl<'a, S: Scalar> Sub<S> for &'a Point3<S> {
    type Output = Vector3<S>;
    fn sub(self, other: S) -> Self::Output {
        Vector3::new(self.x - other, self.y - other, self.z - other)
    }
}

impl<S: Scalar> Sub<Vector3<S>> for Point3<S> {
    type Output = Point3<S>;
    fn sub(self, other: Vector3<S>) -> Self::Output {
        Point3::new(self.x - other.x, self.y - other.x, self.z - other.z)
    }
}

impl<'a, S: Scalar> Sub<&'a Vector3<S>> for &'a Point3<S> {
    type Output = Point3<S>;
    fn sub(self, other: &'a Vector3<S>) -> Self::Output {
        Point3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<S: Scalar> Div<S> for Point3<S> {
    type Output = Self;
    fn div(self, rhs: S) -> Self {
        assert!(rhs != S::zero());
        let inv = rhs.recip();
        self * inv
    }
}
impl<'a, S: Scalar> Div<S> for &'a Point3<S> {
    type Output = Point3<S>;
    fn div(self, rhs: S) -> Self::Output {
        assert!(rhs != S::zero());
        let inv = rhs.recip();
        self * inv
    }
}

impl<S: Scalar> Mul<S> for Point3<S> {
    type Output = Self;
    fn mul(self, rhs: S) -> Self {
        Point3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl<'a, S: Scalar> Mul<S> for &'a Point3<S> {
    type Output = Point3<S>;
    fn mul(self, rhs: S) -> Self::Output {
        Point3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<S: Scalar> Index<u8> for Point3<S> {
    type Output = S;
    fn index(&self, index: u8) -> &S {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Point Index out of range"),
        }
    }
}
impl<S: Scalar> Index<usize> for Point3<S> {
    type Output = S;
    fn index(&self, index: usize) -> &S {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Point Index out of range"),
        }
    }
}
impl<S: Scalar> Index<u32> for Point3<S> {
    type Output = S;
    fn index(&self, index: u32) -> &S {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Point3 Index (v[{}]) out of range", index),
        }
    }
}
impl<S: Scalar> IndexMut<u8> for Point3<S> {
    fn index_mut<'a>(&'a mut self, index: u8) -> &'a mut S {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Point3 Index (v[{}]) out of range", index),
        }
    }
}
impl<S: Scalar> IndexMut<usize> for Point3<S> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut S {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Point3 Index (v[{}]) out of range", index),
        }
    }
}
impl<S: Scalar> IndexMut<u32> for Point3<S> {
    fn index_mut<'a>(&'a mut self, index: u32) -> &'a mut S {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Point3 Index (v[{}]) out of range", index),
        }
    }
}
impl<S: Scalar> Add for Point2<S> {
    type Output = Point2<S>;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}
impl<'a, S: Scalar> Add for &'a Point2<S> {
    type Output = Point2<S>;
    fn add(self, other: Self) -> Self::Output {
        Point2::new(self.x + other.x, self.y + other.y)
    }
}

impl<S: Scalar> Add<S> for Point2<S> {
    type Output = Point2<S>;
    fn add(self, other: S) -> Self {
        Self::new(self.x + other, self.y + other)
    }
}
impl<'a, S: Scalar> Add<Point3<S>> for &'a Point2<S> {
    type Output = Point2<S>;
    fn add(self, other: Point3<S>) -> Self::Output {
        Point2::new(self.x + other.x, self.y + other.y)
    }
}
impl<S: Scalar> Add<Vector2<S>> for Point2<S> {
    type Output = Point2<S>;
    fn add(self, other: Vector2<S>) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}
impl<'a, S: Scalar> Add<&'a Vector2<S>> for &'a Point2<S> {
    type Output = Point2<S>;
    fn add(self, other: &'a Vector2<S>) -> Self::Output {
        Point2::new(self.x + other.x, self.y + other.y)
    }
}

impl<S: Scalar> AddAssign for Point2<S> {
    fn add_assign(&mut self, other: Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}
impl<S: Scalar> AddAssign<S> for Point2<S> {
    fn add_assign(&mut self, other: S) {
        self.x = self.x + other;
        self.y = self.y + other;
    }
}

impl<S: Scalar> Neg for Point2<S> {
    type Output = Self;

    fn neg(self) -> Self {
        Point2::new(-self.x, -self.y)
    }
}

impl<S: Scalar> Sub for Point2<S> {
    type Output = Vector2<S>;
    fn sub(self, other: Self) -> Self::Output {
        Vector2::new(self.x - other.x, self.y - other.y)
    }
}
impl<'a, S: Scalar> Sub for &'a Point2<S> {
    type Output = Vector2<S>;
    fn sub(self, other: Self) -> Self::Output {
        Vector2::new(self.x - other.x, self.y - other.y)
    }
}
impl<S: Scalar> Sub<Vector2<S>> for Point2<S> {
    type Output = Self;
    fn sub(self, other: Vector2<S>) -> Self::Output {
        Point2::new(self.x - other.x, self.y - other.y)
    }
}
impl<'a, S: Scalar> Sub<&'a Vector2<S>> for &'a Point2<S> {
    type Output = Point2<S>;
    fn sub(self, other: &'a Vector2<S>) -> Self::Output {
        Point2::new(self.x - other.x, self.y - other.y)
    }
}

impl<S: Scalar> Sub<S> for Point2<S> {
    type Output = Self;
    fn sub(self, other: S) -> Self {
        Point2::new(self.x - other, self.y - other)
    }
}

impl<S: Scalar> Div<S> for Point2<S> {
    type Output = Self;
    fn div(self, rhs: S) -> Self {
        assert!(rhs != S::zero());
        let inv = rhs.recip();
        self * inv
    }
}

impl<'a, S: Scalar> Div<S> for &'a Point2<S> {
    type Output = Point2<S>;
    fn div(self, rhs: S) -> Self::Output {
        assert!(rhs != S::zero());
        let inv = rhs.recip();
        self * inv
    }
}

impl<S: Scalar> Mul<S> for Point2<S> {
    type Output = Self;
    fn mul(self, rhs: S) -> Self {
        Point2::new(self.x * rhs, self.y * rhs)
    }
}

impl<'a, S: Scalar> Mul<S> for &'a Point2<S> {
    type Output = Point2<S>;
    fn mul(self, rhs: S) -> Self::Output {
        Point2::new(self.x * rhs, self.y * rhs)
    }
}

impl<S: Scalar> Index<u8> for Point2<S> {
    type Output = S;
    fn index(&self, index: u8) -> &S {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Point Index out of range"),
        }
    }
}
impl<S: Scalar> Index<usize> for Point2<S> {
    type Output = S;
    fn index(&self, index: usize) -> &S {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Point Index out of range"),
        }
    }
}
impl<S: Scalar> Index<u32> for Point2<S> {
    type Output = S;
    fn index(&self, index: u32) -> &S {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Point3 Index (v[{}]) out of range", index),
        }
    }
}
impl<S: Scalar> IndexMut<u8> for Point2<S> {
    fn index_mut<'a>(&'a mut self, index: u8) -> &'a mut S {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Point3 Index (v[{}]) out of range", index),
        }
    }
}
impl<S: Scalar> IndexMut<usize> for Point2<S> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut S {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Point3 Index (v[{}]) out of range", index),
        }
    }
}
impl<S: Scalar> IndexMut<u32> for Point2<S> {
    fn index_mut<'a>(&'a mut self, index: u32) -> &'a mut S {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Point3 Index (v[{}]) out of range", index),
        }
    }
}