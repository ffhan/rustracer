use crate::vector::Vector;
use crate::base::{Drawable, Intersectable, Ray, Color, Point2D, Textureable};
use crate::material::Material;

pub struct Plane {
    normal: Vector,
    point: Vector,
    material: Material
}

impl Plane {
    pub fn new(normal: Vector, point: Vector, material: Material) -> Plane {
        Plane {
            normal: normal,
            point: point,
            material: material,
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

impl Drawable for Plane {
    fn get_material(&self) -> &Material {
        &self.material
    }
}

impl Textureable for Plane {
    fn texture_coords(&self, hit_point: &Vector) -> Point2D {
        let mut x_axis = self.normal.cross(&Vector::new(
            0.0, 0.0, 1.0)
        );
        if x_axis.euclidian_distance() == 0.0 {
            x_axis = self.normal.cross(&Vector::new(
                0.0, 1.0, 0.0
            ));
        }
        let y_axis = self.normal.cross(&x_axis);

        let hit_vec = hit_point.minus(&self.point);

        Point2D {
            x: hit_vec.dot(&x_axis),
            y: hit_vec.dot(&y_axis),
        }
    }

    fn get_texture_color(&self, hit_point: &Vector) -> Color {
        let tex_coords = self.texture_coords(hit_point);
        self.material.get_texture().get_color(tex_coords.x, tex_coords.y)
    }
}
