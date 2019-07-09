use gametest::base::{Color};
use gametest::vector::Vector;
use gametest::objects::sphere::Sphere;
use gametest::scene::Scene;

fn main() {
    let scene: Scene = Scene::new(
        600,
        400,
        90.0,
        Sphere::new(
            Vector::new(0.0, 0.0, -10.0),
            Color::new(128, 255, 0),5.0)
    );
    let img = scene.render();
    img.save("image.png");
}
