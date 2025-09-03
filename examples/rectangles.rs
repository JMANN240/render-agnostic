use std::f64::consts::TAU;

use glam::DVec2;
use palette::Srgba;
use render_agnostic::{Renderer, renderers::image::ImageRenderer};

#[macroquad::main("")]
async fn main() {
    let mut image_renderer = ImageRenderer::new(128, 128, 1.0, DVec2::ZERO, 1);

    image_renderer.render_rectangle_lines(
        DVec2::splat(64.0),
        32.0,
        64.0,
        DVec2::splat(0.5),
        0.0,
        4.0,
        Srgba::new(1.0, 1.0, 1.0, 1.0),
    );
    image_renderer.render_rectangle_lines(
        DVec2::splat(64.0),
        32.0,
        64.0,
        DVec2::splat(0.5),
        TAU / 16.0,
        4.0,
        Srgba::new(0.5, 0.5, 0.5, 1.0),
    );

    image_renderer.get_image().save("rectangles.png").unwrap();
}
