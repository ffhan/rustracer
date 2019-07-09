use crate::scene::Scene;
use crate::vector::Vector;
use core::borrow::Borrow;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
}

pub trait Colorable {
    fn get_color(&self) -> Color;
}

pub trait Drawable: Intersectable + Colorable {
}

#[derive(PartialEq, Debug)]
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

impl <'a> Intersection<'a> {
    pub fn new(distance: f64, object: &'a Box<dyn Drawable>) -> Intersection {
        Intersection{
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
