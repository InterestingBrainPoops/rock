#![no_std]

use nalgebra::*;
#[allow(unused_imports)]
use num_traits::real::Real;
pub struct Rocket {
    pub position: Vector3<f64>,
    // <phi, cos>
    pub rotation: Vector2<f64>,
    pub velocity: Vector3<f64>,
    pub moment_of_inertia: f64,
    pub gimbal_rotation: Vector2<f64>,
    pub gimbal_location: f64,
    pub center_of_pressure: f64,
    pub coeff_of_drag: f64,
    pub cylinder_height: f64,
    pub cylinder_radius: f64,
}

impl Rocket {
    pub fn direction(&self) -> Vector3<f64> {
        // spherical to rectangular conversion
        Vector3::<f64>::new(
            self.rotation.y.cos() * self.rotation.x.sin(),
            self.rotation.y.sin() * self.rotation.x.sin(),
            self.rotation.x.cos(),
        )
    }
    pub fn cross_area(&self) -> f64 {
        0.0
    }
}
