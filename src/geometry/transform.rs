use std::mem;
use std::error::Error;
use std::fmt;
use std::marker::Sized;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, Neg, Sub};
use std::convert::From;

use super::Scalar;
use super::normal::{Normal2, Normal3};
use super::{radians, Metric, Vector, VectorSpace};
use super::vector::{Vector2, Vector3};
use super::point::{Point2, Point3};

#[derive(Debug)]
pub struct InvError;

impl fmt::Display for InvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error inverting matrix")
    }
}

impl Error for InvError {
    fn description(&self) -> &str {
        "Numeric Error when matrix inversion is not successful"
    }
}

#[derive(Debug, Clone)]
pub struct Matrix4<S: Scalar> {
    mat: [[S; 4]; 4],
}

impl<S> Matrix4<S>
where
    S: Scalar,
{
    pub fn new() -> Self {
        let mut mat = [[S::zero(); 4]; 4];
        for i in 0..4 {
            mat[i][i] = S::one();
        }
        Matrix4 { mat }
    }
    pub fn from_values(
        m00: S,
        m01: S,
        m02: S,
        m03: S,
        m10: S,
        m11: S,
        m12: S,
        m13: S,
        m20: S,
        m21: S,
        m22: S,
        m23: S,
        m30: S,
        m31: S,
        m32: S,
        m33: S,
    ) -> Self {
        let mat = [
            [m00, m01, m02, m03],
            [m10, m11, m12, m13],
            [m20, m21, m22, m23],
            [m30, m31, m32, m33],
        ];
        Matrix4 { mat }
    }
    fn transpose(&self) -> Self {
        Matrix4::from_values(
            self.mat[0][0],
            self.mat[1][0],
            self.mat[2][0],
            self.mat[3][0],
            self.mat[0][1],
            self.mat[1][1],
            self.mat[2][1],
            self.mat[3][1],
            self.mat[0][2],
            self.mat[1][2],
            self.mat[2][2],
            self.mat[3][2],
            self.mat[0][3],
            self.mat[1][3],
            self.mat[2][3],
            self.mat[3][3],
        )
    }
    fn inverse(&self) -> Result<Self, InvError> {
        let mut indxc = [0usize; 4];
        let mut indxr = [0usize; 4];
        let mut ipiv = [0usize; 4];

        let mut minv = self.mat.clone();
        for i in 0usize..4 {
            let mut irow = 0usize;
            let mut icol = 0usize;
            let mut big = S::zero();
            for j in 0usize..4 {
                if ipiv[j] != 1 {
                    for k in 0usize..4 {
                        if ipiv[k] == 0 {
                            if minv[j][k].abs() >= big {
                                big = minv[j][k].abs();
                                irow = j;
                                icol = k;
                            }
                        } else if ipiv[k] > 1 {
                            return Err(InvError);
                        }
                    }
                }
            }
            ipiv[icol] += 1;
            if irow != icol {
                for k in 0usize..4 {
                    let rv = minv[irow][k];
                    let cv = minv[icol][k];
                    minv[irow][k] = cv;
                    minv[icol][k] = rv;
                }
            }
            indxr[i] = irow;
            indxc[i] = icol;
            if minv[icol][irow] == S::zero() {
                return Err(InvError);
            }

            let pivinv = S::one() / minv[icol][icol];
            minv[icol][icol] = S::one();
            for j in 0usize..4 {
                minv[icol][j] = minv[icol][j] * pivinv;
            }

            for j in 0usize..4 {
                if j != icol {
                    let save = minv[j][icol];
                    minv[j][icol] = S::zero();
                    for k in 0usize..4 {
                        minv[j][k] = minv[j][k] - minv[icol][k] * save;
                    }
                }
            }
        }
        for j in 0usize..4 {
            for k in 0usize..4 {}
        }
        return Ok(Matrix4 { mat: minv });
    }
}
impl<S: Scalar> Mul<Matrix4<S>> for Matrix4<S> {
    type Output = Self;
    fn mul(self, rhs: Matrix4<S>) -> Self {
        &self * &rhs
    }
}
impl<'a, S: Scalar> Mul<&'a Matrix4<S>> for &'a Matrix4<S> {
    type Output = Matrix4<S>;
    fn mul(self, rhs: &'a Matrix4<S>) -> Self::Output {
        let mut new_mat = [[S::zero(); 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                new_mat[i][j] = self.mat[i][0] * rhs.mat[0][j] + self.mat[i][1] * rhs.mat[1][j]
                    + self.mat[i][2] * rhs.mat[2][j]
                    + self.mat[i][3] * rhs.mat[3][j];
            }
        }
        Matrix4 { mat: new_mat }
    }
}
#[derive(Debug, Clone)]
pub struct Transform<S: Scalar> {
    m: Matrix4<S>,
    m_inv: Matrix4<S>,
}

impl<S: Scalar> Transform<S> {
    pub fn new(m: Matrix4<S>) -> Result<Self, InvError> {
        let m_inv = m.inverse()?;
        Ok(Transform { m, m_inv })
    }

    pub fn from(m: &Matrix4<S>, m_inv: &Matrix4<S>) -> Self {
        Transform {
            m: m.clone(),
            m_inv: m_inv.clone(),
        }
    }

    pub fn inverse(&self) -> Self {
        Transform {
            m: self.m_inv.clone(),
            m_inv: self.m.clone(),
        }
    }

    pub fn translate(delta: &Vector3<S>) -> Self {
        let mut m = Matrix4::new();
        m.mat[0][3] = delta.x;
        m.mat[1][3] = delta.y;
        m.mat[2][3] = delta.z;
        let mut m_inv = Matrix4::new();
        m.mat[0][3] = -delta.x;
        m.mat[1][3] = -delta.y;
        m.mat[2][3] = -delta.z;

        Transform { m, m_inv }
    }

    pub fn scale(x: S, y: S, z: S) -> Self {
        let mut m = Matrix4::new();
        m.mat[1][1] = x;
        m.mat[2][2] = y;
        m.mat[3][3] = z;
        let mut m_inv = Matrix4::new();
        m.mat[1][1] = x.recip();
        m.mat[2][2] = y.recip();
        m.mat[3][3] = z.recip();

        Transform { m, m_inv }
    }

    pub fn rotate_x(theta: S) -> Self {
        let sin_theta = radians(theta).sin();
        let cos_theta = radians(theta).cos();
        let mut m = Matrix4::new();
        m.mat[1][1] = cos_theta;
        m.mat[1][2] = -sin_theta;
        m.mat[2][1] = sin_theta;
        m.mat[1][1] = cos_theta;

        let m_inv = m.transpose();
        Transform { m, m_inv }
    }

    pub fn rotate_y(theta: S) -> Self {
        let sin_theta = radians(theta).sin();
        let cos_theta = radians(theta).cos();
        let mut m = Matrix4::new();
        m.mat[0][0] = cos_theta;
        m.mat[2][0] = -sin_theta;
        m.mat[0][2] = sin_theta;
        m.mat[2][2] = cos_theta;

        let m_inv = m.transpose();
        Transform { m, m_inv }
    }

    pub fn rotate_z(theta: S) -> Self {
        let sin_theta = radians(theta).sin();
        let cos_theta = radians(theta).cos();
        let mut m = Matrix4::new();
        m.mat[0][0] = cos_theta;
        m.mat[0][1] = -sin_theta;
        m.mat[1][0] = sin_theta;
        m.mat[1][1] = cos_theta;

        let m_inv = m.transpose();
        Transform { m, m_inv }
    }

    pub fn rotate(theta: S, axis: &Vector3<S>) -> Self {
        let a = Vector::normalize(axis);
        let sin_theta = radians(theta).sin();
        let cos_theta = radians(theta).cos();

        let mut m = Matrix4::new();

        // Compute rotations of first basis vectors
        m.mat[0][0] = a.x * a.x + (S::one() - a.x * a.x) * cos_theta;
        m.mat[0][1] = a.x * a.y + (S::one() - cos_theta) - a.z * sin_theta;
        m.mat[0][2] = a.x * a.z + (S::one() - cos_theta) - a.y * sin_theta;
        m.mat[0][3] = S::zero();

        // Compute rotations of second basis vectors
        m.mat[1][0] = a.x * a.y * (S::one() - cos_theta) + a.z * sin_theta;
        m.mat[1][1] = a.y * a.y + (S::one() - a.y * a.y) * cos_theta;
        m.mat[1][2] = a.y * a.z * (S::one() - cos_theta) - a.x * sin_theta;
        m.mat[1][3] = S::zero();

        // Compute rotations of third basis vectors
        m.mat[2][0] = a.x * a.z * (S::one() - cos_theta) - a.y * sin_theta;
        m.mat[2][1] = a.y * a.z * (S::one() - cos_theta) + a.x * sin_theta;
        m.mat[2][2] = a.z * a.z + (S::one() - a.z * a.z) * cos_theta;
        m.mat[2][3] = S::zero();

        let m_inv = m.transpose();
        Transform { m, m_inv }
    }

    pub fn look_at(pos: &Point3<S>, look: &Point3<S>, up: &Vector3<S>) -> Result<Self, InvError> {
        let mut c_t_w = Matrix4::new();
        c_t_w.mat[0][3] = pos.x;
        c_t_w.mat[1][3] = pos.y;
        c_t_w.mat[2][3] = pos.z;

        let dir = Vector::normalize(&(look - pos));
        let normed_up = Vector::normalize(up);
        let left = Vector::normalize(&normed_up.cross(&dir));
        let new_up = dir.cross(&left);

        c_t_w.mat[0][0] = left.x;
        c_t_w.mat[1][0] = left.y;
        c_t_w.mat[2][0] = left.z;

        c_t_w.mat[0][1] = new_up.x;
        c_t_w.mat[1][1] = new_up.y;
        c_t_w.mat[2][1] = new_up.z;

        c_t_w.mat[0][2] = dir.x;
        c_t_w.mat[1][2] = dir.y;
        c_t_w.mat[2][2] = dir.z;

        let w_t_c = c_t_w.inverse()?;
        Ok(Transform {
            m: w_t_c,
            m_inv: c_t_w,
        })
    }
}
impl<S: Scalar> Mul<Transform<S>> for Transform<S> {
    type Output = Transform<S>;
    fn mul(self, rhs: Transform<S>) -> Self::Output {
        &self * &rhs
    }
}
impl<'a, S: Scalar> Mul<Transform<S>> for &'a Transform<S> {
    type Output = Transform<S>;
    fn mul(self, rhs: Transform<S>) -> Self::Output {
        self * &rhs
    }
}
impl<'a, S: Scalar> Mul<&'a Transform<S>> for &'a Transform<S> {
    type Output = Transform<S>;
    fn mul(self, rhs: &'a Transform<S>) -> Self::Output {
        let m = &self.m * &rhs.m;
        let m_inv = &self.m_inv * &rhs.m_inv;
        Transform { m, m_inv }
    }
}
impl<S: Scalar> Mul<Point3<S>> for Transform<S> {
    type Output = Point3<S>;
    fn mul(self, rhs: Point3<S>) -> Self::Output {
        &self * &rhs
    }
}
impl<'a, S: Scalar> Mul<Point3<S>> for &'a Transform<S> {
    type Output = Point3<S>;
    fn mul(self, rhs: Point3<S>) -> Self::Output {
        self * &rhs
    }
}
impl<'a, S: Scalar> Mul<&'a Point3<S>> for Transform<S> {
    type Output = Point3<S>;
    fn mul(self, rhs: &'a Point3<S>) -> Self::Output {
        &self * rhs
    }
}
impl<'a, S: Scalar> Mul<&'a Point3<S>> for &'a Transform<S> {
    type Output = Point3<S>;
    fn mul(self, rhs: &'a Point3<S>) -> Self::Output {
        let mut p = Point3::new(
            self.m.mat[0][0] * rhs.x + self.m.mat[0][1] * rhs.y + self.m.mat[0][2] * rhs.z
                + self.m.mat[0][3],
            self.m.mat[1][0] * rhs.x + self.m.mat[1][1] * rhs.y + self.m.mat[1][2] * rhs.z
                + self.m.mat[1][3],
            self.m.mat[2][0] * rhs.x + self.m.mat[2][1] * rhs.y + self.m.mat[2][2] * rhs.z
                + self.m.mat[2][3],
        );
        let wp = self.m.mat[3][0] * rhs.x + self.m.mat[3][1] * rhs.y + self.m.mat[3][2] * rhs.z
            + self.m.mat[3][3];
        if wp == S::one() {
            p
        } else {
            p / wp
        }
    }
}
impl<S: Scalar> Mul<Vector3<S>> for Transform<S> {
    type Output = Vector3<S>;
    fn mul(self, rhs: Vector3<S>) -> Self::Output {
        &self * &rhs
    }
}
impl<'a, S: Scalar> Mul<Vector3<S>> for &'a Transform<S> {
    type Output = Vector3<S>;
    fn mul(self, rhs: Vector3<S>) -> Self::Output {
        self * &rhs
    }
}
impl<'a, S: Scalar> Mul<&'a Vector3<S>> for Transform<S> {
    type Output = Vector3<S>;
    fn mul(self, rhs: &'a Vector3<S>) -> Self::Output {
        &self * rhs
    }
}
impl<'a, S: Scalar> Mul<&'a Vector3<S>> for &'a Transform<S> {
    type Output = Vector3<S>;
    fn mul(self, rhs: &'a Vector3<S>) -> Self::Output {
        Vector3::new(
            self.m.mat[0][0] * rhs.x + self.m.mat[0][1] * rhs.y + self.m.mat[0][2] * rhs.z,
            self.m.mat[1][0] * rhs.x + self.m.mat[1][1] * rhs.y + self.m.mat[1][2] * rhs.z,
            self.m.mat[2][0] * rhs.x + self.m.mat[2][1] * rhs.y + self.m.mat[2][2] * rhs.z,
        )
    }
}
impl<'a, S: Scalar> Mul<Normal3<S>> for Transform<S> {
    type Output = Normal3<S>;
    fn mul(self, rhs: Normal3<S>) -> Self::Output {
        &self * &rhs
    }
}

impl<'a, S: Scalar> Mul<&'a Normal3<S>> for Transform<S> {
    type Output = Normal3<S>;
    fn mul(self, rhs: &'a Normal3<S>) -> Self::Output {
        &self * rhs
    }
}

impl<'a, S: Scalar> Mul<Normal3<S>> for &'a Transform<S> {
    type Output = Normal3<S>;
    fn mul(self, rhs: Normal3<S>) -> Self::Output {
        self * &rhs
    }
}
impl<'a, S: Scalar> Mul<&'a Normal3<S>> for &'a Transform<S> {
    type Output = Normal3<S>;
    fn mul(self, rhs: &'a Normal3<S>) -> Self::Output {
        Normal3::new(
            self.m_inv.mat[0][0] * rhs.x + self.m_inv.mat[1][0] * rhs.y + self.m_inv.mat[2][0] * rhs.z,
            self.m_inv.mat[0][1] * rhs.x + self.m_inv.mat[1][1] * rhs.y + self.m_inv.mat[2][1] * rhs.z,
            self.m_inv.mat[0][2] * rhs.x + self.m_inv.mat[1][2] * rhs.y + self.m_inv.mat[2][2] * rhs.z,
        )
    }
}
