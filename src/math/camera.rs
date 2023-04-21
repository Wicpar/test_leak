use num_traits::{AsPrimitive, Float};
use serde::{Deserialize, Serialize};
use crate::math::rect::Rect;
use crate::math::scalar::Vec2;

/// Camera optimized to keep the screen stable as it is resized or scaled.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Camera2D<T: Float> {
    /// half the size of the screen in the world
    size: Vec2<T>,
    /// the center of the screen in the world
    pos: Vec2<T>,
    /// the scale of the screen
    scale: T,
}

impl<T: Float + Default> Default for Camera2D<T> {
    fn default() -> Self {
        Self::new(Default::default(), Default::default(), T::one())
    }
}

impl<T: Float + Copy> Camera2D<T> {
    pub fn new(size: Vec2<T>, pos: Vec2<T>, scale: T) -> Self {
        Self {
            size,
            pos,
            scale,
        }
    }

    pub fn set_size(&mut self, size: impl Into<Vec2<T>>) where T: 'static, f64: AsPrimitive<T> {
        self.size = size.into() * 0.5.as_();
    }

    pub fn translate_screen(&mut self, delta: impl Into<Vec2<T>>) {
        self.pos -= delta.into() / self.scale;
    }

    pub fn translate_world(&mut self, delta: impl Into<Vec2<T>>) {
        self.pos -= delta.into();
    }

    pub fn scale_by_on_screen(&mut self, position: impl Into<Vec2<T>>, delta: impl Into<T>) {
        let screen_pos = position.into() - self.size;
        self.pos += screen_pos / self.scale;
        self.scale = self.scale * delta.into();
        self.pos -= screen_pos / self.scale;
    }

    pub fn transform_point(&self, point: Vec2<T>) -> Vec2<T> {
        (point - self.pos) * self.scale + self.size
    }

    pub fn transform_length(&self, length: T) -> T {
        length * self.scale
    }

    fn world_rect(&self) -> Rect<T> {
        Rect::from_center_and_radius(self.pos, self.size / self.scale)
    }
}