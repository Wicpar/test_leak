use serde::{Deserialize, Serialize};
use crate::math::scalar::Vec2;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Circle<T> {
    center: Vec2<T>,
    radius: T
}

