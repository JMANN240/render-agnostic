use anchor2d::{Anchor2D, HorizontalAnchor, VerticalAnchorContext, VerticalAnchorValue};
use macroquad::prelude::*;
use palette::Srgba;

use crate::Renderer;

fn srgba_to_color(srgba: Srgba) -> Color {
    Color {
        r: srgba.red,
        g: srgba.green,
        b: srgba.blue,
        a: srgba.alpha,
    }
}

#[derive(Debug, Default, Clone)]
pub struct MacroquadRenderer {
    font: Option<Font>,
}

impl MacroquadRenderer {
    pub fn new(font: Option<Font>) -> Self {
        Self { font }
    }

    pub fn get_font(&self) -> Option<&Font> {
        self.font.as_ref()
    }

    pub fn set_font(&mut self, font: Option<Font>) {
        self.font = font;
    }
}

impl Renderer for MacroquadRenderer {
    fn render_point(&mut self, position: ::glam::DVec2, color: Srgba) {
        draw_rectangle(position.x as f32, position.y as f32, 1.0, 1.0, srgba_to_color(color));
    }

    fn render_line(
        &mut self,
        start: ::glam::DVec2,
        end: ::glam::DVec2,
        thickness: f64,
        color: Srgba,
    ) {
        draw_line(
            start.x as f32,
            start.y as f32,
            end.x as f32,
            end.y as f32,
            thickness as f32,
            srgba_to_color(color),
        );
    }

    fn render_circle(&mut self, position: ::glam::DVec2, radius: f64, color: Srgba) {
        draw_circle(
            position.x as f32,
            position.y as f32,
            radius as f32,
            srgba_to_color(color),
        );
    }

    fn render_circle_lines(
        &mut self,
        position: ::glam::DVec2,
        radius: f64,
        thickness: f64,
        color: Srgba,
    ) {
        draw_circle_lines(
            position.x as f32,
            position.y as f32,
            radius as f32,
            thickness as f32,
            srgba_to_color(color),
        );
    }

    fn render_arc(
        &mut self,
        position: ::glam::DVec2,
        radius: f64,
        rotation: f64,
        arc: f64,
        thickness: f64,
        color: Srgba,
    ) {
        draw_arc(
            position.x as f32,
            position.y as f32,
            64,
            radius as f32,
            rotation as f32,
            thickness as f32,
            arc as f32,
            srgba_to_color(color),
        );
    }

    fn render_text(
        &mut self,
        text: &str,
        position: ::glam::DVec2,
        anchor: Anchor2D,
        size: f64,
        color: Srgba,
    ) {
        let measurement = measure_text(text, self.font.as_ref(), size as u16, 1.0);

        let x = match anchor.get_horizontal() {
            HorizontalAnchor::Left => position.x,
            HorizontalAnchor::Center => position.x - measurement.width as f64 / 2.0,
            HorizontalAnchor::Right => position.x - measurement.width as f64,
        };

        let vertical_anchor = anchor.get_vertical();

        let y = match (vertical_anchor.get_context(), vertical_anchor.get_value()) {
            (VerticalAnchorContext::Graphics, VerticalAnchorValue::Bottom) => position.y,
            (VerticalAnchorContext::Math, VerticalAnchorValue::Bottom) => {
                position.y + measurement.offset_y as f64
            }
            (_, VerticalAnchorValue::Center) => position.y + measurement.offset_y as f64 / 2.0,
            (VerticalAnchorContext::Graphics, VerticalAnchorValue::Top) => {
                position.y + measurement.offset_y as f64
            }
            (VerticalAnchorContext::Math, VerticalAnchorValue::Top) => position.y,
        };

        draw_text_ex(
            text,
            x as f32,
            y as f32,
            TextParams {
                font: self.font.as_ref(),
                font_size: size as u16,
                color: srgba_to_color(color),
                ..TextParams::default()
            },
        );
    }

    fn render_rectangle(
        &mut self,
        position: ::glam::DVec2,
        width: f64,
        height: f64,
        offset: ::glam::DVec2,
        rotation: f64,
        color: Srgba,
    ) {
        draw_rectangle_ex(
            position.x as f32,
            position.y as f32,
            width as f32,
            height as f32,
            DrawRectangleParams {
                offset: vec2(offset.x as f32, offset.y as f32),
                rotation: rotation as f32,
                color: srgba_to_color(color),
            },
        );
    }

    fn render_rectangle_lines(
        &mut self,
        position: ::glam::DVec2,
        width: f64,
        height: f64,
        offset: ::glam::DVec2,
        rotation: f64,
        thickness: f64,
        color: Srgba,
    ) {
        draw_rectangle_lines_ex(
            position.x as f32,
            position.y as f32,
            width as f32,
            height as f32,
            thickness as f32,
            DrawRectangleParams {
                offset: vec2(offset.x as f32, offset.y as f32),
                rotation: rotation as f32,
                color: srgba_to_color(color),
            },
        );
    }

    fn render_equilateral_triangle(
        &mut self,
        position: ::glam::DVec2,
        radius: f64,
        rotation: f64,
        color: Srgba,
    ) {
        draw_poly(
            position.x as f32,
            position.y as f32,
            3,
            radius as f32,
            rotation.to_degrees() as f32,
            srgba_to_color(color),
        );
    }

    fn render_equilateral_triangle_lines(
        &mut self,
        position: ::glam::DVec2,
        radius: f64,
        rotation: f64,
        thickness: f64,
        color: Srgba,
    ) {
        draw_poly_lines(
            position.x as f32,
            position.y as f32,
            3,
            radius as f32,
            rotation.to_degrees() as f32,
            thickness as f32,
            srgba_to_color(color),
        );
    }
}
