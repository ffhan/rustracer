use crate::base::Color;

pub struct Material {
    texture: Box<dyn Texture>,
    albedo: f64,
    glossiness: f64,
}

pub trait Texture {
    fn get_color(&self, x: f64, y: f64) -> Color;
}

pub struct ConstantTexture {
    color: Color,
}

impl ConstantTexture {
    pub fn new(color: Color) -> ConstantTexture {
        ConstantTexture {
            color: color
        }
    }
}

impl Texture for ConstantTexture {
    fn get_color(&self, _x: f64, _y: f64) -> Color {
        self.color.clone()
    }
}

impl Material {
    pub fn new(texture: Box<dyn Texture>, albedo: f64, glossiness: f64) -> Material {
        Material {
            texture: texture,
            albedo: albedo,
            glossiness: glossiness,
        }
    }
    pub fn new_constant(color: Color, albedo: f64, glossiness: f64) -> Material {
        Material {
            texture: Box::new(ConstantTexture::new(color)),
            albedo: albedo,
            glossiness: glossiness,
        }
    }
    pub fn get_texture(&self) -> &Box<dyn Texture> {
        &self.texture
    }
    pub fn get_glossiness(&self) -> f64 {
        self.glossiness
    }
    pub fn get_albedo(&self) -> f64 {
        self.albedo
    }
}


