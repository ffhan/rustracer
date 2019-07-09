use crate::vector::Vector;
use image::{DynamicImage, GenericImage, RgbaImage, Rgba, RGBA};

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> bool;
}

#[derive(PartialEq, Debug)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color{
            red: red,
            green: green,
            blue: blue
        }
    }
}

pub struct Pixel {
    x: u32,
    y: u32,
}

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
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        //Create a line segment between the ray origin and the center of the sphere
        let l: Vector = self.center.minus(&ray.origin);
        //Use l as a hypotenuse and find the length of the adjacent side
        let adj2 = l.dot(&ray.direction);
        //Find the length-squared of the opposite side
        //This is equivalent to (but faster than) (l.length() * l.length()) - (adj2 * adj2)
        let d2 = l.dot(&l) - (&adj2 * &adj2);
        //If that length-squared is less than radius squared, the ray intersects the sphere
        d2 < (self.radius * self.radius)
    }
}

pub struct Scene {
    width: u32,
    height: u32,
    fov: f64,
    sphere: Sphere
}

impl Scene {
    pub fn get_width(&self) -> u32 {
        self.width
    }
    pub fn get_height(&self) -> u32 {
        self.height
    }
}

fn to_rgba(col: &Color) -> Rgba<u8> {
    Rgba([0, 255, 0, 255])
}

impl Scene {
    pub fn new(width: u32, height: u32, fov: f64, sphere: Sphere) -> Scene {
        Scene{
            width: width,
            height: height,
            fov: fov,
            sphere: sphere
        }
    }
    pub fn render(&self) -> DynamicImage {
        let mut image = DynamicImage::new_rgba8(self.width, self.height);
        let black = Rgba([0, 0, 0, 0]);
        for x in 0..self.width {
            for y in 0..self.height {
                let ray = Ray::new(x, y, self);

                if self.sphere.intersect(&ray) {
                    image.put_pixel(x, y, to_rgba(&self.sphere.color));
                } else {
                    image.put_pixel(x, y, black);
                }
            }
        }
        image
    }
}

pub struct Ray {
    origin: Vector,
    direction: Vector
}

impl Ray {
    pub fn new(x: u32, y: u32, scene: &Scene) -> Ray {
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let sensor_x = (((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio;
        let sensor_y = 1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0;

        Ray{
            origin: Vector::zero(),
            direction: Vector::new(sensor_x, sensor_y, -1.0).normalize(),
        }
    }
}
