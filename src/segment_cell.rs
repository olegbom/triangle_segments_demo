use macroquad::prelude::*;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_6, PI, SQRT_2};

use crate::raw_miniquad::VertexQ;

pub const SQRT_3: f32 = 1.73205080757;
pub struct SegmentCell {
    pub vertices: Vec<VertexQ>,
    pub indices: Vec<u16>,
}

impl SegmentCell {
    const HEX_NUMBER_OF_VERTICES: usize = 6 * 3 * 2 * 3;
    pub const HEX_NUMBER_OF_INDICES: usize = 3 * 4 * 3 * 2 * 3;
    pub const TRIANGLE_NUMBER_OF_POINTS: usize = 3 * 6 * 2;

    pub fn new(a: f32, b: f32, k: f32) -> SegmentCell {
        let mut cell = SegmentCell {
            vertices: Vec::<VertexQ>::with_capacity(
                Self::HEX_NUMBER_OF_VERTICES + Self::TRIANGLE_NUMBER_OF_POINTS,
            ),
            indices: Vec::<u16>::with_capacity(
                Self::HEX_NUMBER_OF_INDICES + Self::TRIANGLE_NUMBER_OF_POINTS,
            ),
        };
        cell.generate_vertices(a, b, k);
        cell.use_segments(std::u32::MAX);
        cell
    }

    pub fn generate_vertices(&mut self, a: f32, b: f32, k: f32) {
        let vec2_to_vertex = |v: Vec2| VertexQ { pos: v, index: 0. };

        let center_arrow = [
            vec2(a * 0.5, a * SQRT_3 * 0.5 + b),
            vec2(0., b),
            vec2(-a * 0.5, a * SQRT_3 * 0.5 + b),
        ];

        let vertex_arrow = [
            vec2(
                a * 0.5,
                -b * SQRT_2 / (SQRT_3 - 1.) - a * 0.5 / (2. - SQRT_3),
            ),
            vec2(0., -b * SQRT_2 / (SQRT_3 - 1.)),
            vec2(
                -a * 0.5,
                -b * SQRT_2 / (SQRT_3 - 1.) - a * 0.5 / (2. - SQRT_3),
            ),
        ];

        let edge_arrow = [
            vec2(a * 0.5, -b * SQRT_2 * 0.5 - a * 0.5),
            vec2(0., -b * SQRT_2 * 0.5),
            vec2(-a * 0.5, -b * SQRT_2 * 0.5 - a * 0.5),
        ];

        let c = a * 0.5 + b;
        let filling_triangle = [
            vec2(-c, c * SQRT_3),
            vec2(-c, k - c / (2.0 - SQRT_3)),
            vec2(c, k * 0.5 - c).rotate(Vec2::from_angle(FRAC_PI_3)),
        ];

        self.vertices.clear();

        for i in 0..3 {
            for j in [0., 1.] {
                let sign = -j * 2.0 + 1.;
                let dv: Vec2 = j * Vec2::from_angle(FRAC_PI_6) * k;
                let rot = Vec2::from_angle(i as f32 * 2.0 * PI / 3.);
                self.vertices.extend(
                    center_arrow
                        .into_iter()
                        .map(|v| v.rotate(rot))
                        .map(|v| v * sign + dv)
                        .map(vec2_to_vertex),
                );
                self.vertices.extend(
                    vertex_arrow
                        .into_iter()
                        .rev()
                        .map(|v| v.with_y(v.y + k).rotate(rot))
                        .map(|v| v * sign + dv)
                        .map(vec2_to_vertex),
                );

                let rot = Vec2::from_angle(i as f32 * 2.0 * PI / 3. + PI / 3.);
                self.vertices.extend(
                    center_arrow
                        .into_iter()
                        .map(|v| rot.rotate(v))
                        .map(|v| v * sign + dv)
                        .map(vec2_to_vertex),
                );
                self.vertices.extend(
                    edge_arrow
                        .into_iter()
                        .rev()
                        .map(|v| v.with_y(v.y + k * 0.5).rotate(rot))
                        .map(|v| v * sign + dv)
                        .map(vec2_to_vertex),
                );
            }

            let rot = i as f32 * 2.0 * PI / 3.;
            for sign in [-1., 1.] {
                self.vertices.extend(
                    vertex_arrow
                        .into_iter()
                        .map(|v| v.rotate(Vec2::from_angle(FRAC_PI_6 * sign)))
                        .map(|v| v.with_y(v.y + k).rotate(Vec2::from_angle(rot)))
                        .map(vec2_to_vertex),
                );
                self.vertices.extend(
                    edge_arrow
                        .into_iter()
                        .map(|v| v.rotate(Vec2::from_angle(-FRAC_PI_2 * sign)))
                        .map(|v| {
                            v.with_y(v.y + k * 0.5)
                                .rotate(Vec2::from_angle(-FRAC_PI_3 * sign + rot))
                        })
                        .map(vec2_to_vertex),
                );
            }
        }
        for i in 0..3 {
            let rot = Vec2::from_angle(i as f32 * 2.0 * PI / 3.0);
            for j in [0., 1.] {
                let sign = -j * 2.0 + 1.;
                let dv: Vec2 = j * Vec2::from_angle(FRAC_PI_6) * k;
                self.vertices.extend(
                    filling_triangle
                        .into_iter()
                        .map(|v| v.rotate(rot))
                        .map(|v| v * sign + dv)
                        .map(vec2_to_vertex),
                );
                self.vertices.extend(
                    filling_triangle
                        .into_iter()
                        .map(|v| v.with_x(-v.x))
                        .map(|v| v.rotate(rot))
                        .map(|v| v * sign + dv)
                        .map(vec2_to_vertex),
                );
            }
        }
        let mut index = 0;
        for value in self.vertices.iter_mut() {
            value.index = index as f32;
            index += 1;
        }
    }

    pub fn use_segments(&mut self, bits: u32) {
        self.indices.clear();

        let mut counter = 0;
        for j in (0..Self::HEX_NUMBER_OF_VERTICES as u16).step_by(6) {
            if (bits & (1 << counter)) != 0 {
                for i in j + 1..j + 5 {
                    self.indices.extend_from_slice(&[j, i, i + 1]);
                }
            }
            counter += 1;
        }
        for i in (0..Self::TRIANGLE_NUMBER_OF_POINTS as u16).step_by(3) {
            if (bits & (1 << counter)) != 0 {
                let l = Self::HEX_NUMBER_OF_VERTICES as u16 + i;
                self.indices.extend_from_slice(&[l, l + 1, l + 2]);
            }
            counter += 1;
        }
    }

    pub fn is_point_in_triangle(a: Vec2, b: Vec2, c: Vec2, o: Vec2) -> bool {
        let p = Self::vect(a - o, b - a);
        let q = Self::vect(b - o, c - b);
        let r = Self::vect(c - o, a - c);

        (p <= 0. && q <= 0. && r <= 0.) || (p >= 0. && q >= 0. && r >= 0.)
    }

    fn vect(a: Vec2, b: Vec2) -> f32 {
        a.x * b.y - a.y * b.x
    }

    pub fn get_segment_index(&self, p: Vec2) -> i32 {
        let mut result = -1;
        for i in (0..self.indices.len()).step_by(3) {
            if Self::is_point_in_triangle(
                self.vertices.get(self.indices[i] as usize).unwrap().pos,
                self.vertices.get(self.indices[i + 1] as usize).unwrap().pos,
                self.vertices.get(self.indices[i + 2] as usize).unwrap().pos,
                p,
            ) {
                let index = if i < Self::HEX_NUMBER_OF_INDICES {
                    i / 3 / 4
                } else {
                    Self::HEX_NUMBER_OF_INDICES / 3 / 4 + (i - Self::HEX_NUMBER_OF_INDICES) / 3
                };

                result = index as i32;
                break;
            }
        }
        result
    }

    pub fn get_dx(i: i32, j: i32) -> f32 {
        SQRT_3 * 0.5 + (i as f32 + (i32::abs(j) % 2) as f32 * 0.5) * SQRT_3
    }

    pub fn get_dy(j: i32) -> f32 {
        0.5 + 1.5 * j as f32
    }

    pub fn get_index_i(x: f32, y: f32) -> i32 {
        (x / SQRT_3
            - 0.5
            - (Self::get_index_j(y).abs() % 2) as f32 * 0.5
            - (y - 0.5 - ((y - 0.5) / 1.5).floor() * 1.5) / 4.5)
            .floor() as i32
    }

    pub fn get_index_j(y: f32) -> i32 {
        ((2.0 * y - 1.0) / 3.0).floor() as i32
    }

    pub fn modify_segments_bit(
        &self,
        seg_bits: &mut Vec<Vec2>,
        x: f32,
        y: f32,
        scale: f32,
        is_set: bool,
    ) {
        let mut mx = x;
        let mut my = y;
        mx = mx / screen_width() * 2.0 - 1.0;
        mx = mx / scale - SQRT_3 * 0.5;
        my = my / screen_height() * 2.0 - 1.0;
        my = my / (scale * screen_width() / screen_height()) + 0.5;
        my = -my;
        let i = SegmentCell::get_index_i(mx + SQRT_3, my + 1.0);
        let j = SegmentCell::get_index_j(my + 1.0);
        mx -= SegmentCell::get_dx(i, j) - SQRT_3 * 0.5;
        my -= SegmentCell::get_dy(j) - 0.5;

        let index = self.get_segment_index(vec2(mx, my));
        let coord_index = ((i + 12) + (j + 6) * 24) as usize;
        if index >= 0 && coord_index < seg_bits.len() {
            let old = seg_bits[coord_index];
            if is_set {
                if index < 16 {
                    seg_bits[coord_index] = old.with_x((old.x as u16 | (1 << index)) as f32);
                } else {
                    seg_bits[coord_index] = old.with_y((old.y as u16 | (1 << (index - 16))) as f32);
                }
            } else {
                if index < 16 {
                    seg_bits[coord_index] = old.with_x((old.x as u16 & !(1 << index)) as f32);
                } else {
                    seg_bits[coord_index] =
                        old.with_y((old.y as u16 & !(1 << (index - 16))) as f32);
                }
            }
        }
    }

    pub fn toggle_segments_bit(&self, seg_bits: &mut Vec<Vec2>, x: f32, y: f32, scale: f32) {
        let mut mx = x;
        let mut my = y;
        mx = mx / screen_width() * 2.0 - 1.0;
        mx = mx / scale - SQRT_3 * 0.5;
        my = my / screen_height() * 2.0 - 1.0;
        my = my / (scale * screen_width() / screen_height()) + 0.5;
        my = -my;
        let i = SegmentCell::get_index_i(mx + SQRT_3, my + 1.0);
        let j = SegmentCell::get_index_j(my + 1.0);
        mx -= SegmentCell::get_dx(i, j) - SQRT_3 * 0.5;
        my -= SegmentCell::get_dy(j) - 0.5;

        let index = self.get_segment_index(vec2(mx, my));
        let coord_index = ((i + 12) + (j + 6) * 24) as usize;
        if index >= 0 && coord_index < seg_bits.len() {
            let old = seg_bits[coord_index];
            if index < 16 {
                seg_bits[coord_index] = old.with_x((old.x as u16 ^ (1 << index)) as f32);
            } else {
                seg_bits[coord_index] = old.with_y((old.y as u16 ^ (1 << (index - 16))) as f32);
            }
        }
    }
}
