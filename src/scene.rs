use core::borrow::Borrow;
use std::f64::MAX;

use image::{DynamicImage, GenericImage, Rgba};

use crate::base::{Color, Colorable, Drawable, Intersectable, Intersection, Ray};
use crate::lighting::directional::DirectionalLight;
use crate::lighting::Lighting;

const SHADOW_BIAS: f64 = 1e-13;

pub struct Scene {
    width: u32,
    height: u32,
    fov: f64,
    objects: Vec<Box<dyn Drawable>>,
    lights: Vec<Box<dyn Lighting>>,
}

fn to_rgba(col: &Color) -> Rgba<u8> {
    let [r, g, b] = col.get();
    Rgba([r, g, b, 255])
}

impl Scene {
    pub fn new(width: u32, height: u32, fov: f64) -> Scene {
        Scene {
            width: width,
            height: height,
            fov: fov,
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }
    pub fn render(&self) -> DynamicImage {
        let start_time = std::time::SystemTime::now();

        let mut image = DynamicImage::new_rgba8(self.width, self.height);
        let black = Rgba([128, 128, 255, 255]);
        for x in 0..self.width {
            for y in 0..self.height {
                let ray = Ray::new(x, y, self);

                let trace = self.trace(&ray);
                if trace.is_some() {
                    image.put_pixel(x, y, to_rgba(&self.get_color(&ray, &trace.unwrap())));
                } else {
                    image.put_pixel(x, y, black);
                }
            }
        }
        let duration = std::time::SystemTime::now().duration_since(start_time);
        println!("Rendered the image in {:?}", duration);
        image
    }
    pub fn add_object(&mut self, obj: Box<dyn Drawable>) {
        self.objects.push(obj);
    }
    pub fn add_light(&mut self, light: Box<dyn Lighting>) { self.lights.push(light); }
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
        let mut objs = Vec::new();
        for s in self.objects.iter() {
            let distance = s.intersect(ray);
            if distance.is_some() {
                objs.push(Intersection::new(distance.unwrap(), s));
            }
        }
        objs.into_iter().min_by(|i1, i2| i1.get_distance().partial_cmp(&i2.get_distance()).unwrap())
    }
    fn get_color(&self, ray: &Ray, intersection: &Intersection) -> Color {
        let hit_point = ray.get_origin()
            .plus(&ray.get_direction()
                .factor(intersection.get_distance()));
        let surface_normal = intersection.get_object().surface_normal(&hit_point);

        let mut color = [0_f64, 0.0, 0.0];

        for light in self.lights.iter() {
            let direction_to_light = light.get_direction_to_light(&hit_point);

            // calculate if the point is a shadow
            let shadow_ray = Ray::from(
                hit_point.clone() + (surface_normal.factor(SHADOW_BIAS)),
                direction_to_light.clone()
            );
            let in_light = self.trace(&shadow_ray).is_none();

            let light_intensity = if in_light {surface_normal.normalize()
                .dot(&direction_to_light.normalize())
                .max(0.0) * light.get_intensity(&hit_point)} else {0.0};
            let light_reflected = 1.0; // todo: implementiraj

            let light_color = light.get_color().get();
            let obj_color = intersection.get_object().get_color().get();

            color[0] += light_color[0] as f64 * obj_color[0] as f64 * light_intensity * light_reflected / 255.0;
            color[1] += light_color[1] as f64 * obj_color[1] as f64 * light_intensity * light_reflected / 255.0;
            color[2] += light_color[2] as f64 * obj_color[2] as f64 * light_intensity * light_reflected / 255.0;

//            println!("Light intensity {} light color {:?} obj color {:?} color {:?}", light_intensity,
//                     light_color, obj_color, color);
        }
        let color = color.iter().map(|e| {
            e.round().max(0.0).min(255.0) as u8
        }).collect::<Vec<u8>>();
        Color::new(*color.get(0).unwrap(),
                   *color.get(1).unwrap(),
                   *color.get(2).unwrap())
    }
}
