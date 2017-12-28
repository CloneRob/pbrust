use super::Scalar;
use super::{Metric, Vector, VectorSpace};
use super::vector::Vector3f;
use super::point::Point3f;
use super::normal::Normal3f;
use super::{Medium, MediumInterface};

//TODO Create Interaction Trait and implement SurfaceInteraction
pub struct Interaction {
    p: Point3f,
    time: f32,
    p_error: Vector3f,
    wo: Vector3f,
    n: Normal3f,
    medium_interface: MediumInterface,
}

