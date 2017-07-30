use std::error::Error;
use std::fmt;

use cg;
use cg::prelude::*;
use cg::BaseFloat;

#[derive(Debug)]
struct InversionError;

impl fmt::Display for InversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error inverting matrix")
    }
}

impl Error for InversionError {
    fn description(&self) -> &str {
        "Numeric Error when matrix inversion is not successful"
    }
}

struct Transform<T> where T: BaseFloat {
    m: cg::Matrix4<T>,
    m_inv: cg::Matrix4<T>,
}

impl<T> Transform<T> where T: BaseFloat {
    fn new(m: cg::Matrix4<T>) -> Result<Transform<T>, InversionError> {
        let inverse = m.invert();
        if let Some(inv) = inverse {
            let transform = Transform {
                m: m,
                m_inv: inv,
            };
            Ok(transform)
        } else {
            Err(InversionError)
        }
    }
}