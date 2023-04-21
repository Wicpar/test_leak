use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};
use std::process::Output;
use std::ptr::write;
use num_traits::Zero;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{SeqAccess, Visitor};

#[derive(Debug, Copy, Clone)]
pub struct Scalar<T, const N: usize>([T;N]);

impl<T: Serialize, const N: usize> Serialize for Scalar<T, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        self.0.as_slice().serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>, const N: usize> Deserialize<'de> for Scalar<T, N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        Ok(Self(serde_arrays::deserialize(deserializer)?))
    }
}

pub type Vec2<T> = Scalar<T, 2>;

impl<T> Vec2<T> {
    pub fn x(&self) -> &T {
        &self.0[0]
    }
    pub fn y(&self) -> &T {
        &self.0[1]
    }
}

impl<T, const N: usize> Scalar<T, N> {
    pub fn new(arr: [T; N]) -> Self {
        arr.into()
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        self.0.get(idx)
    }
}

impl<T: Default + Copy, const N: usize> Default for Scalar<T, N> {
    fn default() -> Self {
        Self([T::default();N])
    }
}

impl<T, const N: usize> From<[T;N]> for Scalar<T, N> {
    fn from(value: [T; N]) -> Self {
        Self(value)
    }
}

impl<T, const N: usize> Into<[T;N]> for Scalar<T, N> {
    fn into(self) -> [T; N] {
        self.0
    }
}

impl<T, const N: usize> AsRef<[T;N]> for Scalar<T, N> {
    fn as_ref(&self) -> &[T; N] {
        &self.0
    }
}

impl<T: Add<Output=T>, const N: usize> Add for Scalar<T, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.zip(rhs.0).map(|(a, b)| a + b))
    }
}

impl<T: Add<Output=T> + Copy, const N: usize> AddAssign for Scalar<T, N> {
    fn add_assign(&mut self, rhs: Self) {
        self.0.iter_mut().zip(rhs.0).for_each(|(a, b)| *a = *a + b);
    }
}

impl<T: Sub<Output=T>, const N: usize> Sub for Scalar<T, N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.zip(rhs.0).map(|(a, b)| a - b))
    }
}

impl<T: Sub<Output=T> + Copy, const N: usize> SubAssign for Scalar<T, N> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0.iter_mut().zip(rhs.0).for_each(|(a, b)| *a = *a - b);
    }
}

impl<T: Mul<Output=T> + Copy, const N: usize> Mul<T> for Scalar<T, N> {
    type Output = Self;

    fn mul(self, b: T) -> Self::Output {
        Self(self.0.map(|a| a * b))
    }
}

impl<T: Div<Output=T> + Copy, const N: usize> Div<T> for Scalar<T, N> {
    type Output = Self;

    fn div(self, b: T) -> Self::Output {
        Self(self.0.map(|a| a / b))
    }
}
