use std::mem;
use std::error::Error;
use std::fmt;
use std::marker::Sized;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, Neg, Sub};
use std::convert::From;

use super::Scalar;
use super::normal::{Normal2, Normal3};
use super::{Metric, Vector, VectorSpace};

#[derive(Debug)]
struct InvError;

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

struct Matrix4<S: Scalar> {
    mat: [[S; 4]; 4],
}

impl<S> Matrix4<S>
where
    S: Scalar,
{
    fn new() -> Self {
        let mut mat = [[S::zero(); 4]; 4];
        for i in 0..4 {
            mat[i][i] = S::one();
        }
        Matrix4 { mat }
    }
    fn from_values(
        m00: S, m01: S, m02: S, m03: S,
        m10: S, m11: S, m12: S, m13: S,
        m20: S, m21: S, m22: S, m23: S,
        m30: S, m31: S, m32: S, m33: S,
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
            self.mat[0][0], self.mat[1][0], self.mat[2][0], self.mat[3][0],
            self.mat[0][1], self.mat[1][1], self.mat[2][1], self.mat[3][1],
            self.mat[0][2], self.mat[1][2], self.mat[2][2], self.mat[3][2],
            self.mat[0][3], self.mat[1][3], self.mat[2][3], self.mat[3][3],
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
struct Transform<S>
where
    S: Scalar,
{
    m: Matrix4<S>,
    m_inv: Matrix4<S>,
}

// impl<T> Transform<T> where T: BaseFloat {
//     fn new(m: cg::Matrix4<T>) -> Result<Transform<T>, InversionError> {
//         if let Ok(m_inv) = Transform::invert_matrix(&m) {
//             let transform = Transform {
//                 m: m,
//                 m_inv: m_inv,
//             };
//             return Ok(transform);
//         } else {
//             Err(InversionError)
//         }
//     }
//
//     fn invert_matrix(m: &cg::Matrix4<T>) -> Result<cg::Matrix4<T>, InversionError> {
//         let inverse = m.invert();
//         if let Some(inv) = inverse {
//             Ok(inv)
//         } else {
//             Err(InversionError)
//         }
//     }
//
//     fn build(m: cg::Matrix4<T>, inv: cg::Matrix4<T>) -> Transform<T> {
//         Transform {
//             m: m,
//             m_inv: inv,
//         }
//     }
//
//     fn inverse(&self) -> Transform<T> {
//         Transform::build(self.m_inv.clone(), self.m.clone())
//     }
//
//     fn transpose(&self) -> Transform<T> {
//         Transform::build(self.m.transpose(), self.m_inv.transpose())
//     }
//
//     fn translate(delta: &cg::Vector3<T>) -> Transform<T> {
//         let m = cg::Matrix4::from_translation(delta.clone());
//         let m_inv = cg::Matrix4::from_translation(-delta.clone());
//         Transform::build(m, m_inv)
//     }
//
//     fn scale(x: T, y: T, z: T) -> Transform<T> {
//         let m = cg::Matrix4::from_nonuniform_scale(x, y, z);
//         let m_inv = cg::Matrix4::from_nonuniform_scale(T::one() / x, T::one() / y, T::one() / z);
//         Transform::build(m, m_inv)
//     }
//
//     fn rotate_x(theta: T) -> Transform<T> {
//         let theta = Rad(theta);
//         let m = cg::Matrix4::from_angle_x(theta);
//         let m_inv = m.transpose();
//
//         Transform::build(m, m_inv)
//     }
//
//     fn rotate(theta: T, axis: &cg::Vector3<T>) -> Transform<T> {
//         let normed_axis = axis.normalize();
//         let m = cg::Matrix4::from_axis_angle(normed_axis, Rad(theta));
//         let m_inv = m.transpose();
//
//         Transform::build(m, m_inv)
//     }
//
//     fn look_at(pos: &cg::Point3<T>, look: &cg::Point3<T>, up: &cg::Vector3<T>) -> Result<Transform<T>, InversionError> {
//         let m = cg::Matrix4::look_at(pos.clone(), look.clone(), up.clone());
//         if let Ok(inv) = Transform::invert_matrix(&m) {
//             Ok(Transform::build(m, inv))
//         } else {
//             Err(InversionError)
//         }
//     }
// }
