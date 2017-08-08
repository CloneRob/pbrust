// use std::error::Error;
// use std::fmt;

// use cg;
// use cg::prelude::*;
// use cg::BaseFloat;
// use cg::Rad;
// use cg::{Rotation, Rotation3};

// #[derive(Debug)]
// struct InversionError;

// impl fmt::Display for InversionError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Error inverting matrix")
//     }
// }

// impl Error for InversionError {
//     fn description(&self) -> &str {
//         "Numeric Error when matrix inversion is not successful"
//     }
// }

// struct Transform<T> where T: BaseFloat {
//     m: cg::Matrix4<T>,
//     m_inv: cg::Matrix4<T>,
// }

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

//     fn invert_matrix(m: &cg::Matrix4<T>) -> Result<cg::Matrix4<T>, InversionError> {
//         let inverse = m.invert();
//         if let Some(inv) = inverse {
//             Ok(inv)
//         } else {
//             Err(InversionError)
//         }
//     }

//     fn build(m: cg::Matrix4<T>, inv: cg::Matrix4<T>) -> Transform<T> {
//         Transform {
//             m: m,
//             m_inv: inv,
//         }
//     }

//     fn inverse(&self) -> Transform<T> {
//         Transform::build(self.m_inv.clone(), self.m.clone())
//     } 

//     fn transpose(&self) -> Transform<T> {
//         Transform::build(self.m.transpose(), self.m_inv.transpose())
//     } 

//     fn translate(delta: &cg::Vector3<T>) -> Transform<T> {
//         let m = cg::Matrix4::from_translation(delta.clone());
//         let m_inv = cg::Matrix4::from_translation(-delta.clone());
//         Transform::build(m, m_inv)
//     }

//     fn scale(x: T, y: T, z: T) -> Transform<T> {
//         let m = cg::Matrix4::from_nonuniform_scale(x, y, z);
//         let m_inv = cg::Matrix4::from_nonuniform_scale(T::one() / x, T::one() / y, T::one() / z);
//         Transform::build(m, m_inv)
//     }

//     fn rotate_x(theta: T) -> Transform<T> {
//         let theta = Rad(theta);
//         let m = cg::Matrix4::from_angle_x(theta);
//         let m_inv = m.transpose();

//         Transform::build(m, m_inv)
//     }

//     fn rotate(theta: T, axis: &cg::Vector3<T>) -> Transform<T> {
//         let normed_axis = axis.normalize();
//         let m = cg::Matrix4::from_axis_angle(normed_axis, Rad(theta));
//         let m_inv = m.transpose();

//         Transform::build(m, m_inv)
//     }

//     fn look_at(pos: &cg::Point3<T>, look: &cg::Point3<T>, up: &cg::Vector3<T>) -> Result<Transform<T>, InversionError> {
//         let m = cg::Matrix4::look_at(pos.clone(), look.clone(), up.clone()); 
//         if let Ok(inv) = Transform::invert_matrix(&m) {
//             Ok(Transform::build(m, inv))
//         } else {
//             Err(InversionError)
//         }
//     }
// }