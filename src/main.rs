use gametest::base::{Color};
use gametest::vector::Vector;
use gametest::objects::sphere::Sphere;
use gametest::scene::Scene;

fn main() {
    let scene: Scene = Scene::new(
        600,
        400,
        90.0,
    );
    let img = scene.render();
    let result = img.save("image.png");
    if result.is_err() {
        eprintln!("Image save failed!");
    }
}
