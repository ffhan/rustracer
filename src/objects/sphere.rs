use crate::vector::Vector;
use crate::base::{Color, Intersectable, Ray};

#[derive(PartialEq, Debug)]
pub struct Sphere {
    center: Vector,
    color: Color,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vector, color: Color, radius: f64) -> Sphere {
        Sphere{
            center: center,
            color: color,
            radius: radius
        }
    }
    pub fn get_center(&self) -> &Vector {
        &self.center
    }
    pub fn get_color(&self) -> &Color {
        &self.color
    }
    pub fn get_radius(&self) -> f64 {
        self.radius
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        //Create a line segment between the ray origin and the center of the sphere
        let l: Vector = self.center.minus(ray.get_origin());
        //Use l as a hypotenuse and find the length of the adjacent side
        let adj2 = l.dot(ray.get_direction());
        //Find the length-squared of the opposite side
        //This is equivalent to (but faster than) (l.length() * l.length()) - (adj2 * adj2)
        let d2 = l.dot(&l) - (&adj2 * &adj2);
        //If that length-squared is less than radius squared, the ray intersects the sphere
        d2 < (self.radius * self.radius)
    }
}
