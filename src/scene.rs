use crate::base::{Color, Ray, Intersectable, Intersection, Colorable, Drawable};
use image::{DynamicImage, GenericImage, Rgba};
use core::borrow::Borrow;

pub struct Scene {
    width: u32,
    height: u32,
    fov: f64,
    objects: Vec<Box<dyn Drawable>>,
}

fn to_rgba(col: &Color) -> Rgba<u8> {
    let [r, g, b] = col.get();
    Rgba([r, g, b, 255])
}

impl Scene {
    pub fn new(width: u32, height: u32, fov: f64) -> Scene {
        Scene{
            width: width,
            height: height,
            fov: fov,
            objects: Vec::new(),
        }
    }
    pub fn render(&self) -> DynamicImage {
        let mut image = DynamicImage::new_rgba8(self.width, self.height);
        let black = Rgba([0, 0, 0, 255]);
        for x in 0..self.width {
            for y in 0..self.height {
                let ray = Ray::new(x, y, self);

                let trace = self.trace(&ray);
                if trace.is_some() {
                    let obj = trace.unwrap().get_object().get_color();
                    image.put_pixel(x, y, to_rgba(&obj));
                } else {
                    image.put_pixel(x, y, black);
                }
            }
        }
        image
    }
    pub fn add(&mut self, obj: Box<dyn Drawable>) {
        self.objects.push(obj);
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
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.objects.iter()
            .map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)).unwrap())
            .min_by(|i1, i2| i1.get_distance().partial_cmp(&i2.get_distance()).unwrap())
    }
}
