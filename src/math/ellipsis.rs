use egui::{Color32, Pos2, Shape, Stroke};
use egui::epaint::{PathShape};
use num_traits::{AsPrimitive, Float, FloatConst};
use crate::math::camera::Camera2D;
use crate::math::scalar::Vec2;

pub struct Ellipsis<T> {
    pos: Vec2<T>,
    axis: Vec2<T>,
    rotation: T,
}


impl<T: Float + Copy> Ellipsis<T> {
    #[inline]
    pub fn a(&self) -> &T {
        self.axis.x()
    }

    #[inline]
    pub fn b(&self) -> &T {
        self.axis.y()
    }

    #[inline]
    pub fn new(pos: Vec2<T>, axis: Vec2<T>, rotation: T) -> Self {
        let [a, b]: [T; 2] = axis.into();
        Self {
            pos,
            axis: [
                a.max(b),
                a.min(b)
            ].into(),
            rotation,
        }
    }

    #[inline]
    pub fn new_at_focal_pos(pos: Vec2<T>, axis: Vec2<T>, rotation: T) -> Self {
        let [a, b]: [T; 2] = axis.into();
        let axis = [
            a.max(b),
            a.min(b)
        ].into();
        let pos = Self::calc_focal_pos(pos, axis, rotation);
        Self {
            pos,
            axis,
            rotation,
        }
    }

    #[inline]
    pub fn draw(&self, shapes: &mut impl Extend<Shape>, camera: &Camera2D<T>, fill: impl Into<Color32>, stroke: Stroke) where f64: AsPrimitive<T>, usize: AsPrimitive<T>, T: AsPrimitive<usize>, T: AsPrimitive<f32>  {
        let pi: T = f64::PI().as_();
        let displacement = camera.transform_length(*self.a());

        let points = (T::one() + displacement).sqrt().ceil().max(200.as_());
        if points < 5.as_() { return; }
        let amount = (pi + pi) / points;
        shapes.extend([Shape::convex_polygon((0..points.as_()).map(|point| {
            camera.transform_point(self.position_at(amount * point.as_())).into()
        }).collect(), fill, stroke).into()]);
    }

    #[inline]
    pub fn draw_line(&self, shapes: &mut impl Extend<Shape>, camera: &Camera2D<T>, stroke: Stroke) where f64: AsPrimitive<T>, usize: AsPrimitive<T>, T: AsPrimitive<usize>, T: AsPrimitive<f32>  {
        let pi: T = f64::PI().as_();
        let displacement = camera.transform_length(*self.a());

        let points = (T::one() + displacement).sqrt().ceil().max(200.as_());

        if points < 5.as_() { return; }

        let amount = (pi + pi) / points;
        shapes.extend([Shape::closed_line((0..points.as_()).map(|point| {
            camera.transform_point(self.position_at(amount * point.as_())).into()
        }).collect(),  stroke).into()]);
    }

    #[inline]
    pub fn position_at(&self, pos: T) -> Vec2<T> {
        let [x, y] = [
            *self.a() * pos.cos(),
            *self.b() * pos.sin()
        ];
        let cos = self.rotation.cos();
        let sin = self.rotation.sin();
        self.pos + [
            x * cos - y * sin,
            x * sin + y * cos
        ].into()
    }

    #[inline]
    fn calc_focal_pos(pos: Vec2<T>, axis: Vec2<T>, rotation: T) -> Vec2<T> {
        let [x, y]: [T;2] = axis.into();
        let c = (x * x - y * y).sqrt();
        pos + Vec2::new([c * rotation.cos(), c * rotation.sin()])
    }
}