use std::f64::consts::TAU;

use glam::{dvec2, DVec2};
use macroquad::window::next_frame;
use palette::Srgba;
use render_agnostic::{image::ImageRenderer, macroquad::MacroquadRenderer, Renderer};

#[macroquad::main("")]
async fn main() {
    let mut image_renderer = ImageRenderer::new(128, 128, 1.0, DVec2::ZERO, 1);

    image_renderer.render_rectangle_lines(DVec2::splat(64.0), 32.0, 64.0, DVec2::splat(0.5), 0.0, 4.0, Srgba::new(1.0, 1.0, 1.0, 1.0));
    image_renderer.render_rectangle_lines(DVec2::splat(64.0), 32.0, 64.0, DVec2::splat(0.5), TAU / 16.0, 4.0, Srgba::new(0.5, 0.5, 0.5, 1.0));

    image_renderer.render_equilateral_triangle_lines(dvec2(64.0, 64.0), 32.0, 0.2, 16.0, Srgba::new(1.0, 0.0, 0.0, 1.0));

    image_renderer.get_image().save("rectangles.png").unwrap();

    let mut macroquad_renderer = MacroquadRenderer::default();

    loop {
        macroquad_renderer.render_rectangle_lines(DVec2::splat(64.0), 32.0, 64.0, DVec2::splat(0.5), 0.0, 4.0, Srgba::new(1.0, 1.0, 1.0, 1.0));
        macroquad_renderer.render_rectangle_lines(DVec2::splat(64.0), 32.0, 64.0, DVec2::splat(0.5), TAU / 16.0, 4.0, Srgba::new(0.5, 0.5, 0.5, 1.0));

        macroquad_renderer.render_equilateral_triangle_lines(dvec2(64.0, 64.0), 32.0, 0.2, 16.0, Srgba::new(1.0, 0.0, 0.0, 1.0));
        next_frame().await;
    }
}
