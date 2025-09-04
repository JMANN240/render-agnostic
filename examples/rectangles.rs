use std::f64::consts::TAU;

use ab_glyph::FontArc;
use glam::DVec2;
use macroquad::{time::get_time, window::next_frame};
use palette::Srgba;
use render_agnostic::{
    Renderer,
    renderers::{image::ImageRenderer, macroquad::MacroquadRenderer},
};

#[macroquad::main("test")]
async fn main() {
    let mut image_renderer = ImageRenderer::new(
        128,
        128,
        1.0,
        DVec2::ZERO,
        1,
        FontArc::try_from_slice(include_bytes!("roboto.ttf")).unwrap(),
    );

    image_renderer.render_rectangle_lines(
        DVec2::splat(100.0),
        15.0,
        15.0,
        DVec2::splat(0.5),
        0.0,
        2.0,
        Srgba::new(1.0, 0.0, 0.0, 1.0),
    );

    image_renderer.render_rectangle_lines(
        DVec2::splat(100.0),
        15.0,
        15.0,
        DVec2::splat(0.5),
        TAU / 8.0,
        2.0,
        Srgba::new(0.0, 0.0, 1.0, 1.0),
    );

    image_renderer.render_rectangle(
        DVec2::splat(500.0),
        1.0,
        1.0,
        DVec2::splat(0.5),
        TAU / 8.0,
        Srgba::new(0.0, 0.0, 1.0, 1.0),
    );

    // image_renderer.render_rectangle_lines(
    //     DVec2::splat(10.0),
    //     9.0,
    //     9.0,
    //     DVec2::splat(0.0),
    //     TAU / 8.0,
    //     1.0,
    //     Srgba::new(1.0, 1.0, 1.0, 1.0),
    // );

    // image_renderer.render_rectangle_lines(
    //     DVec2::splat(2.0),
    //     5.0,
    //     5.0,
    //     DVec2::splat(0.5),
    //     TAU / 16.0,
    //     4.0,
    //     Srgba::new(0.5, 0.5, 0.5, 1.0),
    // );

    image_renderer.render_circle_lines(DVec2::splat(100.0), 4.0, 1.0, Srgba::new(0.0, 1.0, 0.0, 1.0));

    image_renderer
        .render_image_onto(image_renderer.black())
        .save("rectangles.png")
        .unwrap();

    let mut macroquad_renderer = MacroquadRenderer::new(None);

    {
        for i in 1..=100 {
            macroquad_renderer.render_rectangle(
                DVec2::splat(100.0),
                101.0 - i as f64,
                101.0 - i as f64,
                DVec2::splat(0.5),
                0.0,
                Srgba::new((i % 2) as f32, ((i + 1) % 2) as f32, ((i + 1) % 2) as f32, 1.0),
            );
        }

        next_frame().await;
    }
}
