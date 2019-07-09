use crate::vector::Vector;
use crate::base::{Color, Colorable};

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
    pub fn get_intensity(&self) -> f64 {
        self.intensity
    }
}

impl Colorable for DirectionalLight {
    fn get_color(&self) -> &Color {
        &self.color
    }
}
