use ab_glyph::FontArc;
use ::glam::{DVec2, dvec2};
use image::ImageReader;
use palette::Srgba;
use render_agnostic::{ImageRenderer, Renderer};

fn main() {
    let mut image_renderer = ImageRenderer::new(
        400,
        400,
        0.8,
        DVec2::splat(0.5),
        2,
        FontArc::try_from_slice(include_bytes!("roboto.ttf")).unwrap(),
    );

    image_renderer.register_image(
        String::from("beebo"),
        ImageReader::open("BeeboBall.png").unwrap().decode().unwrap().into_rgba8(),
    );

    image_renderer.render_rectangle(
        dvec2(0.0, 0.0),
        100.0,
        100.0,
        dvec2(0.0, 0.0),
        0.0,
        Srgba::new(1.0, 0.0, 0.0, 1.0),
    );

    image_renderer.render_image(
        "beebo",
        dvec2(0.0, 0.0),
        100.0,
        100.0,
        dvec2(0.0, 0.0),
        0.0,
    );

    image_renderer.render_rectangle(
        dvec2(100.0, 100.0),
        100.0,
        100.0,
        dvec2(0.0, 0.0),
        45.0f64.to_radians(),
        Srgba::new(1.0, 0.0, 0.0, 1.0),
    );

    image_renderer.render_image(
        "beebo",
        dvec2(100.0, 100.0),
        100.0,
        100.0,
        dvec2(0.0, 0.0),
        45.0f64.to_radians(),
    );

    image_renderer.render_rectangle(
        dvec2(200.0, 200.0),
        100.0,
        100.0,
        dvec2(0.5, 0.5),
        -45.0f64.to_radians(),
        Srgba::new(1.0, 0.0, 0.0, 1.0),
    );

    image_renderer.render_image(
        "beebo",
        dvec2(200.0, 200.0),
        100.0,
        100.0,
        dvec2(0.5, 0.5),
        -45.0f64.to_radians(),
    );

    image_renderer.render_rectangle(
        dvec2(300.0, 300.0),
        100.0,
        100.0,
        dvec2(1.0, 1.0),
        -90.0f64.to_radians(),
        Srgba::new(1.0, 0.0, 0.0, 1.0),
    );

    image_renderer.render_image(
        "beebo",
        dvec2(300.0, 300.0),
        100.0,
        100.0,
        dvec2(1.0, 1.0),
        -90.0f64.to_radians(),
    );

    image_renderer.render_circle(dvec2(100.0, 100.0), 10.0, Srgba::new(1.0, 1.0, 0.0, 1.0));
    image_renderer.render_circle(dvec2(200.0, 200.0), 10.0, Srgba::new(1.0, 1.0, 0.0, 1.0));
    image_renderer.render_circle(dvec2(300.0, 300.0), 10.0, Srgba::new(1.0, 1.0, 0.0, 1.0));


    image_renderer
        .render_image_onto(image_renderer.black())
        .save("images.png")
        .unwrap();
}
