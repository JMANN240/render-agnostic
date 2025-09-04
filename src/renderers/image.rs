use std::{
    f64::consts::{FRAC_PI_2, PI},
    iter::once,
};

use ab_glyph::FontArc;
use anchor2d::{Anchor2D, HorizontalAnchor, VerticalAnchorContext, VerticalAnchorValue};
use glam::{DVec2, IVec2, dvec2, ivec2};
use image::{
    Rgba, RgbaImage,
    imageops::{FilterType, overlay, resize},
};
use imageproc::{
    drawing::{
        draw_filled_circle_mut, draw_filled_rect_mut, draw_polygon_mut, draw_text_mut, text_size,
    },
    point::Point,
    rect::Rect,
};
use itertools::Itertools;
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
    virtual_width: u32,
    virtual_height: u32,
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
            virtual_width: width,
            virtual_height: height,
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
        self.virtual_width * self.supersampling
    }

    fn get_supersampled_height(&self) -> u32 {
        self.virtual_height * self.supersampling
    }

    fn map_value(&self, value: f64) -> f64 {
        value * self.scale * self.supersampling as f64
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

        resize(
            &image,
            self.virtual_width,
            self.virtual_height,
            FilterType::Lanczos3,
        )
    }

    pub fn transparent(&self) -> RgbaImage {
        RgbaImage::new(
            self.get_supersampled_width(),
            self.get_supersampled_height(),
        )
    }

    pub fn black(&self) -> RgbaImage {
        RgbaImage::from_pixel(
            self.get_supersampled_width(),
            self.get_supersampled_height(),
            Rgba([0, 0, 0, 255]),
        )
    }

    fn get_base_points(&self, position: DVec2, width: f64, height: f64) -> Vec<DVec2> {
        vec![
            position,
            position + DVec2::X * width,
            position + DVec2::X * width + DVec2::Y * height,
            position + DVec2::Y * height,
        ]
    }

    fn get_offset_vec(&self, width: f64, height: f64, offset: DVec2) -> DVec2 {
        let offset_width = width * offset.x;
        let offset_height = height * offset.y;

        dvec2(offset_width, offset_height)
    }

    fn get_offset_points(&self, points: &[DVec2], offset_vec: DVec2) -> Vec<DVec2> {
        points
            .iter()
            .copied()
            .map(|base_point| base_point - offset_vec)
            .collect::<Vec<DVec2>>()
    }

    fn get_rotated_points(&self, points: &[DVec2], axis: DVec2, rotation: f64) -> Vec<DVec2> {
        points
            .iter()
            .copied()
            .map(|point| rotate_point_around(point, axis, rotation))
            .collect::<Vec<DVec2>>()
    }

    fn get_unique_integer_points(&self, points: &[DVec2]) -> Vec<IVec2> {
        points
            .iter()
            .map(|point| point.round().as_ivec2())
            .unique()
            .collect::<Vec<IVec2>>()
    }
}

impl Renderer for ImageRenderer {
    fn render_point(&mut self, position: DVec2, color: Srgba) {
        let position = self.map_dvec2(position);
        let width = self.map_value(1.0);
        let height = self.map_value(1.0);

        let integer_position = position.round().as_ivec2();

        draw_filled_rect_mut(
            &mut self.image,
            Rect::at(integer_position.x, integer_position.y)
                .of_size(width.round() as u32, height.round() as u32),
            srgba_to_rgba8(color),
        );
    }

    fn render_line(&mut self, start: DVec2, end: DVec2, thickness: f64, color: Srgba) {
        let start = self.map_dvec2(start);
        let end = self.map_dvec2(end);

        let thickness = self.map_value(thickness);
        let offset = thickness / 2.0;
        let normal = DVec2::from_angle((end - start).to_angle() + FRAC_PI_2);

        let points = vec![
            start + normal * offset,
            start - normal * offset,
            end - normal * offset,
            end + normal * offset,
        ];

        let integer_points = self
            .get_unique_integer_points(&points)
            .iter()
            .map(|integer_point| Point::new(integer_point.x, integer_point.y))
            .collect::<Vec<Point<i32>>>();

        if integer_points.len() == 1 {
            let integer_point = integer_points.first().unwrap();

            self.render_point(dvec2(integer_point.x as f64, integer_point.y as f64), color);
        } else {
            draw_polygon_mut(&mut self.image, &integer_points, srgba_to_rgba8(color));
        }
    }

    fn render_circle(&mut self, position: DVec2, radius: f64, color: Srgba) {
        let position = self.map_dvec2(position).round().as_ivec2();
        let radius = self.map_value(radius).round() as u32;

        draw_filled_circle_mut(
            &mut self.image,
            position.into(),
            radius as i32,
            srgba_to_rgba8(color),
        );
    }

    fn render_circle_lines(&mut self, position: DVec2, radius: f64, thickness: f64, color: Srgba) {
        let position = self.map_dvec2(position).round().as_ivec2();
        let radius = self.map_value(radius).round();
        let thickness = self.map_value(thickness).round();

        let mut circle_renderer = ImageRenderer::new(
            2 * radius as u32 + 1,
            2 * radius as u32 + 1,
            self.scale,
            self.scaling_target,
            self.supersampling,
            self.font.clone(),
        );

        circle_renderer.render_circle(dvec2(radius, radius), radius, color);

        circle_renderer.render_circle(
            dvec2(radius, radius),
            radius - thickness,
            Srgba::new(0.0, 0.0, 0.0, 0.0),
        );

        overlay(
            &mut self.image,
            &circle_renderer.render_image_onto(circle_renderer.transparent()),
            (position.x - radius as i32) as i64,
            (position.y - radius as i32) as i64,
        );
    }

    fn render_arc(&mut self, position: DVec2, radius: f64, rotation: f64, arc: f64, color: Srgba) {
        if arc == 0.0 {
            return;
        }

        let position = self.map_dvec2(position);
        let radius = self.map_value(radius);

        let points =
            once(position)
                .chain((0..32).map(|i| {
                    position + radius * DVec2::from_angle(rotation + arc * i as f64 / 31.0)
                }))
                .collect::<Vec<DVec2>>();

        let integer_points = self
            .get_unique_integer_points(&points)
            .iter()
            .map(|integer_point| Point::new(integer_point.x, integer_point.y))
            .collect::<Vec<Point<i32>>>();

        if integer_points.len() == 1 {
            let integer_point = integer_points.first().unwrap();

            self.render_point(dvec2(integer_point.x as f64, integer_point.y as f64), color);
        } else {
            draw_polygon_mut(&mut self.image, &integer_points, srgba_to_rgba8(color));
        }
    }

    fn render_arc_lines(
        &mut self,
        position: DVec2,
        radius: f64,
        rotation: f64,
        arc: f64,
        thickness: f64,
        color: Srgba,
    ) {
        if arc == 0.0 {
            return;
        }

        let position = self.map_dvec2(position).round().as_ivec2();
        let radius = self.map_value(radius).round();
        let thickness = self.map_value(thickness).round();

        let mut circle_renderer = ImageRenderer::new(
            2 * radius as u32 + 1,
            2 * radius as u32 + 1,
            self.scale,
            self.scaling_target,
            self.supersampling,
            self.font.clone(),
        );

        circle_renderer.render_arc(dvec2(radius, radius), radius, rotation, arc, color);

        circle_renderer.render_circle(
            dvec2(radius, radius),
            radius - thickness,
            Srgba::new(0.0, 0.0, 0.0, 0.0),
        );

        overlay(
            &mut self.image,
            &circle_renderer.render_image_onto(circle_renderer.transparent()),
            (position.x - radius as i32) as i64,
            (position.y - radius as i32) as i64,
        );
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
        let size = self.map_value(size);

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

    fn render_text_outline(
        &mut self,
        text: &str,
        position: DVec2,
        anchor: Anchor2D,
        size: f64,
        outline_thickness: f64,
        color: Srgba,
        outline_color: Srgba,
    ) {
        let position = self.map_dvec2(position);
        let size = self.map_value(size);
        let outline_thickness = self.map_value(outline_thickness);

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

        for i in -1..=1 {
            for j in -1..=1 {
                if i != 0 || j != 0 {
                    draw_text_mut(
                        &mut self.image,
                        srgba_to_rgba8(outline_color),
                        (x - i as f64 * outline_thickness).round() as i32,
                        (y - j as f64 * outline_thickness).round() as i32,
                        size as f32,
                        &self.font,
                        text,
                    );
                }
            }
        }

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
        let width = self.map_value(width) - 1.0;
        let height = self.map_value(height) - 1.0;

        let base_points = self.get_base_points(position, width, height);
        let offset_vec = self.get_offset_vec(width, height, offset);
        let offset_points = self.get_offset_points(&base_points, offset_vec);
        let rotated_points = self.get_rotated_points(&offset_points, position, rotation);

        let integer_points = self
            .get_unique_integer_points(&rotated_points)
            .iter()
            .map(|integer_point| Point::new(integer_point.x, integer_point.y))
            .collect::<Vec<Point<i32>>>();

        if integer_points.len() == 1 {
            let integer_point = integer_points.first().unwrap();

            self.render_point(dvec2(integer_point.x as f64, integer_point.y as f64), color);
        } else {
            draw_polygon_mut(&mut self.image, &integer_points, srgba_to_rgba8(color));
        }
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
        let width = self.map_value(width) - 1.0;
        let height = self.map_value(height) - 1.0;
        let thickness = self.map_value(thickness);

        let base_points = self.get_base_points(position, width, height);
        let offset_vec = self.get_offset_vec(width, height, offset);
        let offset_points = self.get_offset_points(&base_points, offset_vec);
        let rotated_points = self.get_rotated_points(&offset_points, position, rotation);

        let integer_points = self
            .get_unique_integer_points(&rotated_points)
            .iter()
            .map(|integer_point| Point::new(integer_point.x, integer_point.y))
            .collect::<Vec<Point<i32>>>();

        let min_x = integer_points
            .iter()
            .map(|integer_point| integer_point.x)
            .min()
            .unwrap();
        let max_x = integer_points
            .iter()
            .map(|integer_point| integer_point.x)
            .max()
            .unwrap();

        let min_y = integer_points
            .iter()
            .map(|integer_point| integer_point.y)
            .min()
            .unwrap();
        let max_y = integer_points
            .iter()
            .map(|integer_point| integer_point.y)
            .max()
            .unwrap();

        let min_vec = ivec2(min_x, min_y).as_dvec2();

        let renderer_width = max_x - min_x + 1;
        let renderer_height = max_y - min_y + 1;

        let mut rectangle_renderer = ImageRenderer::new(
            renderer_width as u32,
            renderer_height as u32,
            1.0,
            DVec2::ZERO,
            1,
            self.font.clone(),
        );

        rectangle_renderer.render_rectangle(
            position - min_vec,
            width + 1.0,
            height + 1.0,
            offset,
            rotation,
            color,
        );

        let midpoint = rotated_points
            .iter()
            .copied()
            .map(|rotated_point| rotated_point - min_vec)
            .sum::<DVec2>()
            / 4.0;

        rectangle_renderer.render_rectangle(
            midpoint,
            width + 1.0 - 2.0 * thickness,
            height + 1.0 - 2.0 * thickness,
            DVec2::splat(0.5),
            rotation,
            Srgba::new(0.0, 0.0, 0.0, 0.0),
        );

        overlay(
            &mut self.image,
            &rectangle_renderer.render_image_onto(rectangle_renderer.transparent()),
            min_x as i64,
            min_y as i64,
        );
    }

    fn render_equilateral_triangle(
        &mut self,
        position: DVec2,
        radius: f64,
        rotation: f64,
        color: Srgba,
    ) {
        let position = self.map_dvec2(position);
        let radius = self.map_value(radius);

        let points = (0..3)
            .map(|i| position + radius * DVec2::from_angle(i as f64 * 2.0 * PI / 3.0 + rotation))
            .collect::<Vec<DVec2>>();

        let integer_points = self
            .get_unique_integer_points(&points)
            .iter()
            .map(|integer_point| Point::new(integer_point.x, integer_point.y))
            .collect::<Vec<Point<i32>>>();

        if integer_points.len() == 1 {
            let integer_point = integer_points.first().unwrap();

            self.render_point(dvec2(integer_point.x as f64, integer_point.y as f64), color);
        } else {
            draw_polygon_mut(&mut self.image, &integer_points, srgba_to_rgba8(color));
        }
    }

    fn render_equilateral_triangle_lines(
        &mut self,
        position: DVec2,
        radius: f64,
        rotation: f64,
        thickness: f64,
        color: Srgba,
    ) {
        let position = self.map_dvec2(position);
        let radius = self.map_value(radius);
        let thickness = self.map_value(thickness);

        let points = (0..3)
            .map(|i| position + radius * DVec2::from_angle(i as f64 * 2.0 * PI / 3.0 + rotation))
            .collect::<Vec<DVec2>>();

        let integer_points = self
            .get_unique_integer_points(&points)
            .iter()
            .map(|integer_point| Point::new(integer_point.x, integer_point.y))
            .collect::<Vec<Point<i32>>>();

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
            1.0,
            DVec2::ZERO,
            1,
            self.font.clone(),
        );

        triangle_renderer.render_equilateral_triangle(
            (position - min_point.as_dvec2()).round(),
            radius,
            rotation,
            color,
        );

        triangle_renderer.render_equilateral_triangle(
            (position - min_point.as_dvec2()).round(),
            radius - thickness,
            rotation,
            Srgba::new(0.0, 0.0, 0.0, 0.0),
        );

        overlay(
            &mut self.image,
            &triangle_renderer.render_image_onto(triangle_renderer.transparent()),
            min_x as i64,
            min_y as i64,
        );
    }
}

fn rotate_point_around(point: DVec2, axis: DVec2, theta: f64) -> DVec2 {
    if theta == 0.0 {
        return point;
    }

    let relative = point - axis;
    let relative_theta = relative.to_angle();
    let new_relative_theta = relative_theta + theta;
    let new_relative = DVec2::from_angle(new_relative_theta) * relative.length();
    new_relative + axis
}
