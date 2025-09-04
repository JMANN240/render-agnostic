use ab_glyph::FontArc;
use glam::{DVec2, dvec2};
use palette::Srgba;
use render_agnostic::{Renderer, renderers::image::ImageRenderer};

fn main() {
    let mut image_renderer = ImageRenderer::new(
        128,
        128,
        1.0,
        DVec2::ZERO,
        4,
        FontArc::try_from_slice(include_bytes!("roboto.ttf")).unwrap(),
    );

    image_renderer.render_text(
        "Top Left",
        dvec2(0.0, 0.0),
        anchor2d::LGT,
        24.0,
        Srgba::new(1.0, 1.0, 1.0, 1.0),
    );

    image_renderer.render_text_outline(
        "Center",
        dvec2(64.0, 64.0),
        anchor2d::CGC,
        24.0,
        1.0,
        Srgba::new(1.0, 1.0, 1.0, 1.0),
        Srgba::new(1.0, 0.0, 0.0, 1.0),
    );

    image_renderer.render_text(
        "Bottom Right",
        dvec2(128.0, 128.0),
        anchor2d::RGB,
        24.0,
        Srgba::new(1.0, 1.0, 1.0, 1.0),
    );

    image_renderer
        .render_image_onto(image_renderer.black())
        .save("text.png")
        .unwrap();
}
