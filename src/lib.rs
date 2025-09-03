use anchor2d::Anchor2D;
use glam::DVec2;
use palette::Srgba;

#[cfg(feature = "macroquad")]
pub mod macroquad;

#[cfg(feature = "image")]
pub mod image;

pub trait Renderer {
    type Font;

    fn render_line(&mut self, start: DVec2, end: DVec2, thickness: f64, color: Srgba);
    fn render_circle(&mut self, position: DVec2, radius: f64, color: Srgba);
    fn render_circle_lines(&mut self, position: DVec2, radius: f64, thickness: f64, color: Srgba);

    fn render_arc(
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
        font: Self::Font,
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
