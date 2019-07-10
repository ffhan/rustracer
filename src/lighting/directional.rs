use crate::vector::Vector;
use crate::base::{Color, Colorable};
use crate::lighting::Lighting;

pub struct DirectionalLight {
    direction: Vector,
    color: Color,
    intensity: f64,
}

impl DirectionalLight {
    pub fn new(direction: Vector, color: Color, intensity: f64) -> DirectionalLight {
        DirectionalLight {
            direction: direction,
            color: color,
            intensity: intensity,
        }
    }
    pub fn get_direction(&self) -> &Vector {
        &self.direction
    }
}

impl Colorable for DirectionalLight {
    fn get_color(&self) -> &Color {
        &self.color
    }
}

impl Lighting for DirectionalLight {
    fn get_intensity(&self, _hit_point: &Vector) -> f64 {
        self.intensity
    }

    fn get_direction_to_light(&self, _hit_point: &Vector) -> Vector {
        self.direction.neg().normalize()
    }
}
