use std::f64::consts::{PI, TAU};

use ab_glyph::FontArc;
use glam::DVec2;
use palette::Srgba;
use render_agnostic::{Renderer, renderers::image::ImageRenderer};

fn main() {
    let mut image_renderer = ImageRenderer::new(
        64,
        64,
        1.0,
        DVec2::ZERO,
        4,
        FontArc::try_from_slice(include_bytes!("roboto.ttf")).unwrap(),
    );

    image_renderer.render_equilateral_triangle(
        DVec2::splat(16.0),
        16.0,
        0.0,
        Srgba::new(1.0, 1.0, 1.0, 1.0),
    );

    image_renderer.render_equilateral_triangle(
        DVec2::splat(16.0),
        16.0,
        PI,
        Srgba::new(1.0, 0.0, 0.0, 1.0),
    );

    image_renderer.render_equilateral_triangle_lines(
        DVec2::splat(48.0),
        16.0,
        0.0,
        2.0,
        Srgba::new(0.0, 0.0, 1.0, 1.0),
    );

    image_renderer.render_equilateral_triangle_lines(
        DVec2::splat(48.0),
        16.0,
        PI,
        2.0,
        Srgba::new(0.0, 1.0, 0.0, 1.0),
    );

    image_renderer
        .render_image_onto(image_renderer.black())
        .save("triangles.png")
        .unwrap();
}
