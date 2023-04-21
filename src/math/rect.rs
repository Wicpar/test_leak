use num_traits::Num;
use serde::{Deserialize, Serialize};
use crate::math::scalar::Vec2;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Rect<T> {
    top_left: Vec2<T>,
    bottom_right: Vec2<T>
}

impl<T> Rect<T> {
    pub fn new(top_left: Vec2<T>, bottom_right: Vec2<T>) -> Self {
        Self {top_left, bottom_right}
    }
}

impl<T: Num + Copy> Rect<T> {
    pub fn from_center_and_radius(center: Vec2<T>, radius: Vec2<T>) -> Self {
        Self {
            top_left: center - radius,
            bottom_right: center + radius
        }
    }
}