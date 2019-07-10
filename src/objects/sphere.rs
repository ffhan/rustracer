use crate::vector::Vector;
use crate::base::{Color, Intersectable, Ray, Colorable, Drawable};

#[derive(PartialEq, Debug)]
pub struct Sphere {
    center: Vector,
    color: Color,
    radius: f64,
    n: f64,
}

impl Sphere {
    pub fn new(center: Vector, color: Color, radius: f64, n: f64) -> Sphere {
        Sphere{
            center: center,
            color: color,
            radius: radius,
            n: n,
        }
    }
    pub fn get_center(&self) -> &Vector {
        &self.center
    }
    pub fn get_radius(&self) -> f64 {
        self.radius
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        //Create a line segment between the ray origin and the center of the sphere
        let l: Vector = self.center.minus(ray.get_origin());
        //Use l as a hypotenuse and find the length of the adjacent side
        let adj2 = l.dot(ray.get_direction());
        //Find the length-squared of the opposite side
        //This is equivalent to (but faster than) (l.length() * l.length()) - (adj2 * adj2)
        let d2 = l.dot(&l) - (&adj2 * &adj2);
        //If that length-squared is less than radius squared, the ray intersects the sphere
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return None;
        }

        let thc = (radius2 - d2).sqrt();
        let t0 = adj2 - thc;
        let t1 = adj2 + thc;

        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }

        let distance = if t0 < t1 { t0 } else { t1 };
        Some(distance)
    }

    fn surface_normal(&self, hit_point: &Vector) -> Vector {
        hit_point.minus(&self.center).normalize()
    }
}

impl Drawable for Sphere {
    fn get_reflection_exponent(&self) -> f64 {
        self.n
    }
}

impl Colorable for Sphere {
    fn get_color(&self) -> &Color {
        &self.color
    }
}
