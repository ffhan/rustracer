use gametest::base::{Color};
use gametest::vector::Vector;
use gametest::objects::sphere::Sphere;
use gametest::scene::Scene;
use gametest::objects::plane::Plane;
use gametest::lighting::directional::DirectionalLight;
use gametest::lighting::spherical::SphericalLight;

fn main() {
    let mut scene: Scene = Scene::new(
        600,
        400,
        90.0,
    );
    scene.add_object(Box::new(Plane::new(
        Vector::new(0.0, 1.0, 0.0),
    Vector::new(0.0, -2.5, 0.0),
        Color::new(100, 100, 100))));

    scene.add_object(Box::new(Sphere::new(
        Vector::new(-3.0, 0.0, -10.0),
        Color::new(255, 0, 0),
        4.0
    )));
    scene.add_object(Box::new(Sphere::new(
        Vector::new(4.0, 0.0, -8.0),
        Color::new(0, 255, 0),
        2.0
    )));
    scene.add_object(Box::new(Sphere::new(
        Vector::new(-2.0, 4.0, -6.0),
        Color::new(128, 128, 128),
        1.5
    )));

    scene.add_light(Box::new(
        DirectionalLight::new(
            Vector::new(1.0, -1.0, -1.0),
            Color::new(255,0, 0),
            1.0
        )
    ));
    scene.add_light(Box::new(
        DirectionalLight::new(
            Vector::new(-1.0, -1.0, -1.0),
            Color::new(0,255, 255),
            1.0
        )
    ));
    scene.add_light(Box::new(
       SphericalLight::new(
           Vector::new(0.0, 9.0, -6.0),
           Color::new(255, 255, 255),
           0.5
       )
    ));
    let img = scene.render();
    let result = img.save("image.png");
    if result.is_err() {
        eprintln!("Image save failed!");
    }
}
