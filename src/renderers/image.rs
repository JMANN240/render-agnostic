use std::f64::consts::PI;

use ab_glyph::FontArc;
use anchor2d::{Anchor2D, HorizontalAnchor, VerticalAnchorContext, VerticalAnchorValue};
use glam::{DVec2, IVec2, dvec2, ivec2};
use image::{
    Rgba, RgbaImage,
    imageops::{FilterType, overlay, resize},
};
use imageproc::{
    drawing::{draw_filled_circle_mut, draw_polygon_mut, draw_text_mut, text_size},
    point::Point,
};
use palette::Srgba;

use crate::Renderer;

fn srgba_to_rgba8(color: Srgba) -> Rgba<u8> {
    let red = (color.red * 255.0).round().clamp(0.0, 255.0) as u8;
    let green = (color.green * 255.0).round().clamp(0.0, 255.0) as u8;
    let blue = (color.blue * 255.0).round().clamp(0.0, 255.0) as u8;
    let alpha = (color.alpha * 255.0).round().clamp(0.0, 255.0) as u8;
    Rgba([red, green, blue, alpha])
}

#[derive(Clone)]
pub struct ImageRenderer {
    width: u32,
    height: u32,
    image: RgbaImage,
    scale: f64,
    scaling_target: DVec2,
    supersampling: u32,
    font: FontArc,
}

impl ImageRenderer {
    pub fn new(
        width: u32,
        height: u32,
        scale: f64,
        scaling_target: DVec2,
        supersampling: u32,
        font: FontArc,
    ) -> Self {
        Self {
            width,
            height,
            image: RgbaImage::new(width * supersampling, height * supersampling),
            scale,
            scaling_target,
            supersampling,
            font,
        }
    }

    pub fn get_font(&self) -> &FontArc {
        &self.font
    }

    pub fn set_font(&mut self, font: FontArc) {
        self.font = font;
    }

    fn get_supersampled_width(&self) -> u32 {
        self.width * self.supersampling
    }

    fn get_supersampled_height(&self) -> u32 {
        self.height * self.supersampling
    }

    fn map_x(&self, x: f64) -> f64 {
        let target_x = self.get_supersampled_width() as f64 * self.scaling_target.x;
        (x * self.supersampling as f64 - target_x) * self.scale + target_x
    }

    fn map_y(&self, y: f64) -> f64 {
        let target_y = self.get_supersampled_height() as f64 * self.scaling_target.y;
        (y * self.supersampling as f64 - target_y) * self.scale + target_y
    }

    fn map_dvec2(&self, v: DVec2) -> DVec2 {
        dvec2(self.map_x(v.x), self.map_y(v.y))
    }

    pub fn reset(&mut self) {
        self.image = self.transparent();
    }

    pub fn get_image(&self) -> &RgbaImage {
        &self.image
    }

    pub fn render_image_onto(&self, mut image: RgbaImage) -> RgbaImage {
        overlay(&mut image, &self.image, 0, 0);

        resize(&image, self.width, self.height, FilterType::Lanczos3)
    }

    pub fn transparent(&self) -> RgbaImage {
        RgbaImage::new(
            self.get_supersampled_width(),
            self.get_supersampled_height(),
        )
    }

    pub fn black(&self) -> RgbaImage {
        RgbaImage::from_par_fn(
            self.get_supersampled_width(),
            self.get_supersampled_height(),
            |_, _| Rgba([0, 0, 0, 255]),
        )
    }
}

impl Renderer for ImageRenderer {
    fn render_line(&mut self, start: DVec2, end: DVec2, thickness: f64, color: Srgba) {
        let thickness = thickness * self.scale * self.supersampling as f64;

        let offset = (thickness / 2.0).round();

        let normal = DVec2::from_angle((end - start).to_angle() + PI / 2.0);

        let mapped_start = self.map_dvec2(start);
        let mapped_end = self.map_dvec2(end);

        let p1 = mapped_start + normal * offset;
        let p2 = mapped_start - normal * offset;
        let p3 = mapped_end - normal * offset;
        let p4 = mapped_end + normal * offset;

        let points = vec![
            Point::new(p1.x.round() as i32, p1.y.round() as i32),
            Point::new(p2.x.round() as i32, p2.y.round() as i32),
            Point::new(p3.x.round() as i32, p3.y.round() as i32),
            Point::new(p4.x.round() as i32, p4.y.round() as i32),
        ];

        draw_polygon_mut(&mut self.image, &points, srgba_to_rgba8(color));
    }

    fn render_circle(&mut self, position: DVec2, radius: f64, color: Srgba) {
        let position = self.map_dvec2(position).round().as_ivec2();
        let radius = (radius * self.scale * self.supersampling as f64).round() as u32;

        draw_filled_circle_mut(
            &mut self.image,
            position.into(),
            radius as i32,
            srgba_to_rgba8(color),
        );
    }

    fn render_circle_lines(&mut self, position: DVec2, radius: f64, thickness: f64, color: Srgba) {
        let position = self.map_dvec2(position).round().as_ivec2();
        let radius = (radius * self.scale * self.supersampling as f64).round() as u32;
        let thickness = (thickness * self.scale * self.supersampling as f64).round() as u32;

        let mut circle_image = RgbaImage::new(2 * radius + 1, 2 * radius + 1);

        draw_filled_circle_mut(
            &mut circle_image,
            (radius as i32, radius as i32),
            radius as i32,
            srgba_to_rgba8(color),
        );

        draw_filled_circle_mut(
            &mut circle_image,
            (radius as i32, radius as i32),
            radius as i32 - thickness as i32,
            Rgba([0, 0, 0, 0]),
        );

        overlay(
            &mut self.image,
            &circle_image,
            (position.x - radius as i32) as i64,
            (position.y - radius as i32) as i64,
        );
    }

    fn render_arc(
        &mut self,
        position: DVec2,
        radius: f64,
        _rotation: f64,
        _arc: f64,
        thickness: f64,
        color: Srgba,
    ) {
        self.render_circle_lines(position, radius, thickness, color); //TODO
    }

    fn render_text(
        &mut self,
        text: &str,
        position: DVec2,
        anchor: Anchor2D,
        size: f64,
        color: Srgba,
    ) {
        let position = self.map_dvec2(position);
        let size = size * self.scale * self.supersampling as f64;

        let (text_width, _) = text_size(size as f32, &self.font, text);

        let x = match anchor.get_horizontal() {
            HorizontalAnchor::Left => position.x,
            HorizontalAnchor::Center => position.x - text_width as f64 / 2.0,
            HorizontalAnchor::Right => position.x - text_width as f64,
        };

        let vertical_anchor = anchor.get_vertical();

        let y = match (vertical_anchor.get_context(), vertical_anchor.get_value()) {
            (VerticalAnchorContext::Graphics, VerticalAnchorValue::Bottom) => {
                position.y - size / 1.25
            }
            (VerticalAnchorContext::Math, VerticalAnchorValue::Bottom) => position.y,
            (_, VerticalAnchorValue::Center) => position.y - size / 1.25 / 2.0,
            (VerticalAnchorContext::Graphics, VerticalAnchorValue::Top) => position.y,
            (VerticalAnchorContext::Math, VerticalAnchorValue::Top) => position.y - size / 1.25,
        };

        draw_text_mut(
            &mut self.image,
            srgba_to_rgba8(color),
            x as i32,
            y as i32,
            size as f32,
            &self.font,
            text,
        );
    }

    fn render_rectangle(
        &mut self,
        position: DVec2,
        width: f64,
        height: f64,
        offset: DVec2,
        rotation: f64,
        color: Srgba,
    ) {
        let position = self.map_dvec2(position);
        let width = width * self.scale * self.supersampling as f64;
        let height = height * self.scale * self.supersampling as f64;

        let absolute_offset_x = width * offset.x;
        let absolute_offset_y = height * offset.y;

        let absolute_offset = dvec2(absolute_offset_x, absolute_offset_y);

        let p1 = -absolute_offset;
        let p2 = p1 + DVec2::X * width;
        let p3 = p2 + DVec2::Y * height;
        let p4 = p1 + DVec2::Y * height;

        let theta1 = p1.to_angle();
        let theta2 = p2.to_angle();
        let theta3 = p3.to_angle();
        let theta4 = p4.to_angle();

        let q1 = position + DVec2::from_angle(theta1 + rotation) * p1.length();
        let q2 = position + DVec2::from_angle(theta2 + rotation) * p2.length();
        let q3 = position + DVec2::from_angle(theta3 + rotation) * p3.length();
        let q4 = position + DVec2::from_angle(theta4 + rotation) * p4.length();

        draw_polygon_mut(
            &mut self.image,
            &[
                Point::new(q1.x.round() as i32, q1.y.round() as i32),
                Point::new(q2.x.round() as i32, q2.y.round() as i32),
                Point::new(q3.x.round() as i32, q3.y.round() as i32),
                Point::new(q4.x.round() as i32, q4.y.round() as i32),
            ],
            srgba_to_rgba8(color),
        );
    }

    fn render_rectangle_lines(
        &mut self,
        position: DVec2,
        width: f64,
        height: f64,
        offset: DVec2,
        rotation: f64,
        thickness: f64,
        color: Srgba,
    ) {
        let position = self.map_dvec2(position);
        let width = (width * self.scale * self.supersampling as f64).round();
        let height = (height * self.scale * self.supersampling as f64).round();
        let thickness = (thickness * self.scale * self.supersampling as f64).round() as u32;

        let absolute_offset_x = width * offset.x;
        let absolute_offset_y = height * offset.y;

        let absolute_offset = dvec2(absolute_offset_x, absolute_offset_y);

        let p1 = -absolute_offset;
        let p2 = p1 + DVec2::X * width;
        let p3 = p2 + DVec2::Y * height;
        let p4 = p1 + DVec2::Y * height;

        let theta1 = p1.to_angle();
        let theta2 = p2.to_angle();
        let theta3 = p3.to_angle();
        let theta4 = p4.to_angle();

        let q1 = position + DVec2::from_angle(theta1 + rotation) * p1.length();
        let q2 = position + DVec2::from_angle(theta2 + rotation) * p2.length();
        let q3 = position + DVec2::from_angle(theta3 + rotation) * p3.length();
        let q4 = position + DVec2::from_angle(theta4 + rotation) * p4.length();

        let r1 = q1.round().as_ivec2();
        let r2 = q2.round().as_ivec2();
        let r3 = q3.round().as_ivec2();
        let r4 = q4.round().as_ivec2();

        let min_x = r1.x.min(r2.x).min(r3.x).min(r4.x);
        let max_x = r1.x.max(r2.x).max(r3.x).max(r4.x);

        let min_y = r1.y.min(r2.y).min(r3.y).min(r4.y);
        let max_y = r1.y.max(r2.y).max(r3.y).max(r4.y);

        let renderer_width = (max_x - min_x + 1) as u32;
        let renderer_height = (max_y - min_y + 1) as u32;

        let mut rectangle_renderer = ImageRenderer::new(
            renderer_width,
            renderer_height,
            self.scale,
            self.scaling_target,
            self.supersampling,
            self.font.clone(),
        );

        rectangle_renderer.render_rectangle(
            dvec2(
                (renderer_width as f64 / 2.0).floor(),
                (renderer_height as f64 / 2.0).floor(),
            ),
            width,
            height,
            offset,
            rotation,
            color,
        );

        rectangle_renderer.render_rectangle(
            dvec2(
                (renderer_width as f64 / 2.0).floor(),
                (renderer_height as f64 / 2.0).floor(),
            ),
            width - 2.0 * thickness as f64,
            height - 2.0 * thickness as f64,
            offset,
            rotation,
            Srgba::new(0.0, 0.0, 0.0, 0.0),
        );

        overlay(
            &mut self.image,
            &rectangle_renderer.image,
            (position.x - renderer_width as f64 / 2.0).floor() as i64,
            (position.y - renderer_height as f64 / 2.0).floor() as i64,
        );
    }

    fn render_equilateral_triangle(
        &mut self,
        position: DVec2,
        radius: f64,
        rotation: f64,
        color: Srgba,
    ) {
        let points = (0..3)
            .map(|i| position + radius * DVec2::from_angle(i as f64 * 2.0 * PI / 3.0 + rotation))
            .collect::<Vec<DVec2>>();

        draw_polygon_mut(
            &mut self.image,
            &points
                .iter()
                .map(|point| Point::new(point.x.round() as i32, point.y.round() as i32))
                .collect::<Vec<Point<i32>>>(),
            srgba_to_rgba8(color),
        );
    }

    fn render_equilateral_triangle_lines(
        &mut self,
        position: DVec2,
        radius: f64,
        rotation: f64,
        thickness: f64,
        color: Srgba,
    ) {
        let points = (0..3)
            .map(|i| position + radius * DVec2::from_angle(i as f64 * 2.0 * PI / 3.0 + rotation))
            .collect::<Vec<DVec2>>();

        let integer_points = points
            .iter()
            .map(|point| point.floor().as_ivec2())
            .collect::<Vec<IVec2>>();

        let min_x = integer_points
            .iter()
            .map(|integer_point| integer_point.x)
            .min()
            .expect("triangles have more than 0 points");
        let max_x = integer_points
            .iter()
            .map(|integer_point| integer_point.x)
            .max()
            .expect("triangles have more than 0 points");
        let min_y = integer_points
            .iter()
            .map(|integer_point| integer_point.y)
            .min()
            .expect("triangles have more than 0 points");
        let max_y = integer_points
            .iter()
            .map(|integer_point| integer_point.y)
            .max()
            .expect("triangles have more than 0 points");

        let min_point = ivec2(min_x, min_y);

        let renderer_width = (max_x - min_x + 1) as u32;
        let renderer_height = (max_y - min_y + 1) as u32;

        let mut triangle_renderer = ImageRenderer::new(
            renderer_width,
            renderer_height,
            self.scale,
            self.scaling_target,
            self.supersampling,
            self.font.clone(),
        );

        triangle_renderer.render_equilateral_triangle(
            (position - min_point.as_dvec2()).floor(),
            radius,
            rotation,
            color,
        );

        triangle_renderer.render_equilateral_triangle(
            (position - min_point.as_dvec2()).floor(),
            radius - thickness,
            rotation,
            Srgba::new(0.0, 0.0, 0.0, 0.0),
        );

        overlay(
            &mut self.image,
            &triangle_renderer.image,
            min_x as i64,
            min_y as i64,
        );
    }
}
