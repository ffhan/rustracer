use crate::vector::Vector;
use crate::base::{Color, Colorable};
use crate::lighting::Lighting;

pub struct SphericalLight {
    position: Vector,
    color: Color,
    intensity: f64,
}

impl SphericalLight {
    pub fn new(position: Vector, color: Color, intensity: f64) -> SphericalLight {
        SphericalLight {
            position: position,
            color: color,
            intensity: intensity,
        }
    }
}

impl Colorable for SphericalLight {
    fn get_color(&self) -> &Color {
        &self.color
    }
}

impl Lighting for SphericalLight {
    fn get_intensity(&self, hit_point: &Vector) -> f64 {
        let r2 = self.position.minus(&hit_point).normalize();
        self.intensity / (4.0 * ::std::f64::consts::PI * r2.euclidian_distance().powf(2.0))
    }

    fn get_direction_to_light(&self, hit_point: &Vector) -> Vector {
        self.position.minus(hit_point).normalize()
    }
}
