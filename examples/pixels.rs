use ab_glyph::FontArc;
use glam::{DVec2, dvec2};
use palette::Srgba;
use render_agnostic::{Renderer, renderers::image::ImageRenderer};

fn main() {
    let mut image_renderer = ImageRenderer::new(
        7,
        7,
        1.0,
        DVec2::ZERO,
        4,
        FontArc::try_from_slice(include_bytes!("roboto.ttf")).unwrap(),
    );

    image_renderer.render_point(dvec2(1.0, 1.0), Srgba::new(1.0, 0.0, 0.0, 1.0));

    image_renderer.render_point(dvec2(3.0, 3.0), Srgba::new(0.0, 1.0, 0.0, 1.0));

    image_renderer.render_point(dvec2(5.0, 5.0), Srgba::new(0.0, 0.0, 1.0, 1.0));

    image_renderer
        .render_image_onto(image_renderer.black())
        .save("pixels.png")
        .unwrap();
}
