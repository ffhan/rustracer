use gametest::base::{Color};
use gametest::vector::Vector;
use gametest::objects::sphere::Sphere;
use gametest::scene::Scene;

fn main() {
    let mut scene: Scene = Scene::new(
        600,
        400,
        90.0,
    );
    scene.add(Box::new(Sphere::new(
        Vector::new(0.0, 0.0, -15.0),
        Color::new(255, 0, 0),
        4.0
    )));
    scene.add(Box::new(Sphere::new(
        Vector::new(4.0, 0.0, -10.0),
        Color::new(0, 255, 0),
        2.0
    )));
    let img = scene.render();
    let result = img.save("image.png");
    if result.is_err() {
        eprintln!("Image save failed!");
    }
}
