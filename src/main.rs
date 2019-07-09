use gametest::base::{Scene, Sphere, Color};
use gametest::vector::Vector;

fn main() {
    let scene: Scene = Scene::new(
        600,
        400,
        50.0,
        Sphere::new(
            Vector::new(0.0, 0.0, -10.0),
            Color::new(128, 255, 0),5.0)
    );
    let img = scene.render();
    img.save("image.png");
}
