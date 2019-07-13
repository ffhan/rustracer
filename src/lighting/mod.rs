use crate::vector::Vector;
use crate::base::Colorable;

pub mod directional;
pub mod spherical;

pub trait Lighting: Colorable {
    fn get_intensity(&self, hit_point: &Vector) -> f64;
    fn get_direction_to_light(&self, hit_point: &Vector) -> Vector;
}
