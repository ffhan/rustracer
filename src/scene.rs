use crate::objects::sphere::Sphere;
use crate::base::{Color, Ray, Intersectable};
use image::{DynamicImage, GenericImage, Rgba};

pub struct Scene {
    width: u32,
    height: u32,
    fov: f64,
    sphere: Sphere
}

fn to_rgba(col: &Color) -> Rgba<u8> {
    let [r, g, b] = col.get();
    Rgba([r, g, b, 255])
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
                    image.put_pixel(x, y, to_rgba(self.sphere.get_color()));
                } else {
                    image.put_pixel(x, y, black);
                }
            }
        }
        image
    }
    pub fn get_width(&self) -> u32 {
        self.width
    }
    pub fn get_height(&self) -> u32 {
        self.height
    }
    pub fn get_fov(&self) -> f64 {
        self.fov
    }
}
