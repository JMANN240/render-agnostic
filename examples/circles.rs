use std::f64::consts::{PI, TAU};

use ab_glyph::FontArc;
use glam::DVec2;
use palette::Srgba;
use render_agnostic::{Renderer, renderers::image::ImageRenderer};

fn main() {
    let mut image_renderer = ImageRenderer::new(
        64,
        64,
        0.8,
        DVec2::splat(0.5),
        1,
        FontArc::try_from_slice(include_bytes!("roboto.ttf")).unwrap(),
    );

    image_renderer.render_circle(DVec2::splat(16.0), 16.0, Srgba::new(1.0, 1.0, 1.0, 1.0));

    image_renderer.render_circle_lines(
        DVec2::splat(48.0),
        16.0,
        2.0,
        Srgba::new(0.0, 0.0, 1.0, 1.0),
    );

    image_renderer.render_arc(
        DVec2::splat(32.0),
        16.0,
        0.0,
        64,
        PI,
        Srgba::new(1.0, 0.0, 0.0, 1.0),
    );

    image_renderer.render_arc_lines(
        DVec2::splat(32.0),
        16.0,
        PI,
        64,
        PI,
        2.0,
        Srgba::new(0.0, 1.0, 0.0, 1.0),
    );

    image_renderer.render_arc_lines(
        DVec2::splat(32.0),
        23.0 * 0.6,
        0.75 * TAU,
        64,
        0.0,
        23.0 * 0.2,
        Srgba::new(0.0, 1.0, 1.0, 1.0),
    );

    image_renderer
        .render_image_onto(image_renderer.black())
        .save("circles.png")
        .unwrap();
}
