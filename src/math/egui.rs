use egui::Pos2;
use num_traits::{AsPrimitive, Float};
use crate::math::scalar::Vec2;

impl<T: AsPrimitive<f32>> Into<Pos2> for Vec2<T> {
    fn into(self) -> Pos2 {
        let [x, y]: [T; 2] = self.into();
        Pos2::new(x.as_(), y.as_())
    }
}

impl<T: 'static + Copy> From<Pos2> for Vec2<T> where f32: AsPrimitive<T> {
    fn from(pos: Pos2) -> Self {
        [
            pos.x.as_(),
            pos.y.as_()
        ].into()
    }
}

impl<T: AsPrimitive<f32>> Into<egui::Vec2> for Vec2<T> {
    fn into(self) -> egui::Vec2 {
        let [x, y]: [T; 2] = self.into();
        egui::Vec2::new(x.as_(), y.as_())
    }
}

impl<T: 'static + Copy> From<egui::Vec2> for Vec2<T> where f32: AsPrimitive<T> {
    fn from(pos: egui::Vec2) -> Self {
        [
            pos.x.as_(),
            pos.y.as_()
        ].into()
    }
}