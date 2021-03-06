use crate::scene::Scene;
use crate::vector::Vector;
use crate::material::Material;

pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn surface_normal(&self, hit_point: &Vector) -> Vector;
}

pub trait Textureable {
    fn texture_coords(&self, hit_point: &Vector) -> Point2D;
    fn get_texture_color(&self, hit_point: &Vector) -> Color;
}

pub trait Drawable: Intersectable + Textureable {
    fn get_material(&self) -> &Material;
}

pub trait Colorable {
    fn get_color(&self) -> &Color;
}

#[derive(PartialEq, Debug, Clone)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color {
            red: red,
            green: green,
            blue: blue,
        }
    }
    pub fn get(&self) -> [u8; 3] {
        [self.red, self.green, self.blue]
    }
}

pub struct Ray {
    origin: Vector,
    direction: Vector,
}

impl Ray {
    pub fn new(x: u32, y: u32, scene: &Scene) -> Ray {
        let fov_adjustment = (scene.get_fov().to_radians() / 2.0).tan();
        let aspect_ratio = (scene.get_width() as f64) / (scene.get_height() as f64);
        let sensor_x = ((((x as f64 + 0.5) / scene.get_width() as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.get_height() as f64) * 2.0) * fov_adjustment;

        Ray {
            origin: Vector::zero(),
            direction: Vector::new(sensor_x, sensor_y, -1.0).normalize(),
        }
    }
    pub fn from(origin: Vector, direction: Vector) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }
    pub fn from_reflection(normal: &Vector, incident: &Vector, intersection: &Vector, bias: f64) -> Ray {
        Ray {
            origin: intersection.plus(&normal.factor(bias)),
            direction: incident.minus(&normal.factor(2.0 * incident.dot(&normal)))
        }
    }
    pub fn get_origin(&self) -> &Vector {
        &self.origin
    }
    pub fn get_direction(&self) -> &Vector {
        &self.direction
    }
}

pub struct Intersection<'a> {
    distance: f64,
    object: &'a Box<dyn Drawable>,
}

impl<'a> Intersection<'a> {
    pub fn new(distance: f64, object: &'a Box<dyn Drawable>) -> Intersection {
        Intersection {
            distance: distance,
            object: object,
        }
    }
    pub fn get_distance(&self) -> f64 {
        self.distance
    }
    pub fn get_object(&self) -> &'a Box<dyn Drawable> {
        self.object
    }
}
