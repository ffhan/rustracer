use crate::vector::Vector;
use crate::base::{Color, Intersectable, Ray, Drawable, Point2D, Textureable};
use std::f64::consts::PI;
use crate::material::Material;

pub struct Sphere {
    center: Vector,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vector, radius: f64, material: Material) -> Sphere {
        Sphere{
            center: center,
            radius: radius,
            material: material
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
    fn get_material(&self) -> &Material {
        &self.material
    }
}

impl Textureable for Sphere {
    fn texture_coords(&self, hit_point: &Vector) -> Point2D {
        let hit_vec = hit_point.minus(&self.center);
        let phi = hit_vec.get_z().atan2(hit_vec.get_x());
        let theta = (hit_vec.get_y() / self.radius).acos();
        Point2D {
            x: (1.0 + phi / PI) * 0.5,
            y: theta / PI,
        }
    }

    fn get_texture_color(&self, hit_point: &Vector) -> Color {
        let tex_coords = self.texture_coords(hit_point);
        self.material.get_texture().get_color(tex_coords.x, tex_coords.y)
    }
}
