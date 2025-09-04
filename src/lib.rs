use anchor2d::Anchor2D;
use glam::DVec2;
use palette::Srgba;

pub mod renderers;

#[cfg(feature = "image")]
pub use renderers::image::ImageRenderer;

#[cfg(feature = "macroquad")]
pub use renderers::macroquad::MacroquadRenderer;

pub trait Renderer {
    fn render_point(&mut self, position: DVec2, color: Srgba);
    fn render_line(&mut self, start: DVec2, end: DVec2, thickness: f64, color: Srgba);
    fn render_circle(&mut self, position: DVec2, radius: f64, color: Srgba);
    fn render_circle_lines(&mut self, position: DVec2, radius: f64, thickness: f64, color: Srgba);

    fn render_arc(&mut self, position: DVec2, radius: f64, rotation: f64, arc: f64, color: Srgba);

    fn render_arc_lines(
        &mut self,
        position: DVec2,
        radius: f64,
        rotation: f64,
        arc: f64,
        thickness: f64,
        color: Srgba,
    );

    fn render_text(
        &mut self,
        text: &str,
        position: DVec2,
        anchor: Anchor2D,
        size: f64,
        color: Srgba,
    );

    fn render_rectangle(
        &mut self,
        position: DVec2,
        width: f64,
        height: f64,
        offset: DVec2,
        rotation: f64,
        color: Srgba,
    );

    fn render_rectangle_lines(
        &mut self,
        position: DVec2,
        width: f64,
        height: f64,
        offset: DVec2,
        rotation: f64,
        thickness: f64,
        color: Srgba,
    );

    fn render_equilateral_triangle(
        &mut self,
        position: DVec2,
        radius: f64,
        rotation: f64,
        color: Srgba,
    );

    fn render_equilateral_triangle_lines(
        &mut self,
        position: DVec2,
        radius: f64,
        rotation: f64,
        thickness: f64,
        color: Srgba,
    );
}
