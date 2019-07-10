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
    fn get_intensity(&self) -> f64 {
        self.intensity
    }

    fn get_direction_to_light(&self, hit_point: &Vector) -> Vector {
        self.position.minus(hit_point).normalize()
    }
}
