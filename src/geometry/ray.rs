use na;
use num::{Zero, Float};

pub trait BaseRay {
    fn get_position(&self) -> na::Point3<f64>;
}

struct Medium {
    x: i32,
}

struct Ray<'med> {
    o: na::Point3<f64>,
    d: na::Vector3<f64>,
    medium: Option<&'med Medium>,
    tmax: f64,
    time: f64,
}

impl<'med> Default for Ray<'med> {
    fn default() -> Ray<'med> {
        Ray {
            o: na::Point3::origin(),
            d: na::Vector3::zero(),
            medium: None,
            tmax: Float::infinity(),
            time: 0.0,
        }
    }
}

impl<'med> BaseRay for Ray<'med> {
    fn get_position(&self) -> na::Point3<f64> {
        return self.time * (self.o + self.d);
    }
}

struct RayDifferential<'med> {
    ray: Ray<'med>,
    has_differential: bool,
    rx_origin: na::Point3<f64>,
    ry_origin: na::Point3<f64>,
    rx_direction: na::Vector3<f64>,
    ry_direction: na::Vector3<f64>,
}

impl<'med> Default for RayDifferential<'med> {
    fn default() -> RayDifferential<'med> {
        RayDifferential {
            ray: Ray::default(),
            has_differential: false,
            rx_origin: na::Point3::origin(),
            ry_origin: na::Point3::origin(),
            rx_direction: na::Vector3::zero(),
            ry_direction: na::Vector3::zero(),
        }
    }
}

impl<'med> BaseRay for RayDifferential<'med> {
    fn get_position(&self) -> na::Point3<f64> {
        return self.ray.time * (self.ray.o + self.ray.d);
    }
}

impl<'med> From<Ray<'med>> for RayDifferential<'med> {
    fn from(r: Ray<'med>) -> RayDifferential<'med> {
        RayDifferential {
            ray: r,
            has_differential: false,
            rx_origin: na::Point3::origin(),
            ry_origin: na::Point3::origin(),
            rx_direction: na::Vector3::zero(),
            ry_direction: na::Vector3::zero(),
        }
    }
}

impl<'med> RayDifferential<'med> {
    fn scale_differentials(&mut self, scaler: f64) {
        self.rx_origin = self.ray.o + (self.rx_origin - self.ray.o) * scaler;
        self.ry_origin = self.ray.o + (self.ry_origin - self.ray.o) * scaler;
        self.rx_direction = self.ray.d + (self.rx_direction - self.ray.d) * scaler;
        self.ry_direction = self.ray.d + (self.ry_direction - self.ray.d) * scaler;
    }
}
