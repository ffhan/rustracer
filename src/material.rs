use crate::base::Color;

pub struct Material {
    texture: Box<dyn Texture>,
    albedo: f64,
    glossiness: f64,
    surface_type: SurfaceType,
}

pub enum SurfaceType {
    Diffuse,
    Reflective { reflectivity: f64 },
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
    pub fn new(texture: Box<dyn Texture>, surface_type: SurfaceType, albedo: f64, glossiness: f64) -> Material {
        Material {
            texture: texture,
            albedo: albedo,
            glossiness: glossiness,
            surface_type: surface_type,
        }
    }
    pub fn new_constant(color: Color, surface_type: SurfaceType, albedo: f64, glossiness: f64) -> Material {
        Material {
            texture: Box::new(ConstantTexture::new(color)),
            albedo: albedo,
            glossiness: glossiness,
            surface_type: surface_type,
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
    pub fn get_surface_type(&self) -> &SurfaceType { &self.surface_type }
}

pub struct CheckeredPatternTexture {
    color: Color,
    width: u32,
    height: u32,
}

impl CheckeredPatternTexture {
    pub fn new(color: Color, width: u32, height: u32) -> CheckeredPatternTexture {
        CheckeredPatternTexture {
            color: color,
            width: width,
            height: height,
        }
    } }

impl Texture for CheckeredPatternTexture {

    fn get_color(&self, x: f64, y: f64) -> Color {
        let cell_x = x.round() as u32 / self.width as u32;
        let cell_y = y.round() as u32 / self.height as u32;

        if (cell_x + cell_y) % 2 == 0 {
            Color::new(20, 20, 20)
        } else {
            self.color.clone()
        }
    }
}
