use crate::vector::Vector;
use crate::base::{Drawable, Intersectable, Ray, Colorable, Color};

pub struct Plane {
    normal: Vector,
    point: Vector,
    color: Color,
    n: f64
}

impl Plane {
    pub fn new(normal: Vector, point: Vector, color: Color, n: f64) -> Plane {
        Plane {
            normal: normal,
            point: point,
            color: color,
            n: n,
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let dot_product = self.normal.dot(ray.get_direction());
        if dot_product > 1e-4 {
            return None;
        }
        let v = self.point.minus(ray.get_origin());
        let distance = v.dot(&self.normal) / dot_product;
        if distance >= 0.0 {
            return Some(distance);
        }
        None
    }

    fn surface_normal(&self, _: &Vector) -> Vector {
        self.normal.clone()
    }
}

impl Colorable for Plane {
    fn get_color(&self) -> &Color {
        &self.color
    }
}

impl Drawable for Plane {
    fn get_reflection_exponent(&self) -> f64 {
        self.n
    }
}
