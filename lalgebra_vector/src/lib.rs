use std::fmt::Debug;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            panic!("Vectors must have the same length");
        }
        let mut result = Vec::new();
        for i in 0..self.0.len() {
            result.push(self.0[i] + rhs.0[i]);
        }
        Vector(result)
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut sum = T::zero();
        for i in 0..self.0.len() {
            sum = sum + (self.0[i] * other.0[i]);
        }
        Some(sum)
    }
}





pub trait Scalar:
    Copy
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
}


impl Scalar for u32 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for u64 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for i32 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for i64 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}


impl Scalar for f32 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
}

impl Scalar for f64 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
}
