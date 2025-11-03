use ::glam::dvec2;
use macroquad::{
    prelude::*,
    texture::{Texture2D, load_image},
};
use palette::Srgba;
use render_agnostic::{MacroquadRenderer, Renderer};

#[macroquad::main("Images")]
async fn main() {
    let mut macroquad_renderer = MacroquadRenderer::new(None);

    macroquad_renderer.register_image(
        String::from("beebo"),
        Texture2D::from_image(&load_image("BeeboBall.png").await.unwrap()),
    );

    loop {
        clear_background(BLACK);

        macroquad_renderer.render_rectangle(
            dvec2(0.0, 0.0),
            100.0,
            100.0,
            dvec2(0.0, 0.0),
            0.0,
            Srgba::new(1.0, 0.0, 0.0, 1.0),
        );

        macroquad_renderer.render_image(
            "beebo",
            dvec2(0.0, 0.0),
            100.0,
            100.0,
            dvec2(0.0, 0.0),
            0.0,
        );

        macroquad_renderer.render_rectangle(
            dvec2(100.0, 100.0),
            100.0,
            100.0,
            dvec2(0.0, 0.0),
            45.0f64.to_radians(),
            Srgba::new(1.0, 0.0, 0.0, 1.0),
        );

        macroquad_renderer.render_image(
            "beebo",
            dvec2(100.0, 100.0),
            100.0,
            100.0,
            dvec2(0.0, 0.0),
            45.0f64.to_radians(),
        );

        macroquad_renderer.render_rectangle(
            dvec2(200.0, 200.0),
            100.0,
            100.0,
            dvec2(0.5, 0.5),
            -45.0f64.to_radians(),
            Srgba::new(1.0, 0.0, 0.0, 1.0),
        );

        macroquad_renderer.render_image(
            "beebo",
            dvec2(200.0, 200.0),
            100.0,
            100.0,
            dvec2(0.5, 0.5),
            -45.0f64.to_radians(),
        );

        macroquad_renderer.render_rectangle(
            dvec2(300.0, 300.0),
            100.0,
            100.0,
            dvec2(1.0, 1.0),
            -90.0f64.to_radians(),
            Srgba::new(1.0, 0.0, 0.0, 1.0),
        );

        macroquad_renderer.render_image(
            "beebo",
            dvec2(300.0, 300.0),
            100.0,
            100.0,
            dvec2(1.0, 1.0),
            -90.0f64.to_radians(),
        );

        macroquad_renderer.render_circle(dvec2(100.0, 100.0), 10.0, Srgba::new(1.0, 1.0, 0.0, 1.0));
        macroquad_renderer.render_circle(dvec2(200.0, 200.0), 10.0, Srgba::new(1.0, 1.0, 0.0, 1.0));
        macroquad_renderer.render_circle(dvec2(300.0, 300.0), 10.0, Srgba::new(1.0, 1.0, 0.0, 1.0));

        next_frame().await
    }
}
