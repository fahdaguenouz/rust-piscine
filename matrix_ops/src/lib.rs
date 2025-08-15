use std::ops::{ Add, Sub, Mul, Div };
#[derive(Debug,PartialEq,Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![Vec::new()])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let mut parent=Vec::new();
        for _ in 0..row{
            let mut row= vec![];
            for _  in 0..col{
               row.push(T::zero());
            }
            parent.push(row);
        }
        Matrix(parent)
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut parent=Vec::new();
        for i in 0..n{
            let mut row= vec![];
            for j  in 0..n{
                if j==i{
                     row.push(T::one());
                }else{

                    row.push(T::zero());
                }
            }
            parent.push(row);
        }
        Matrix(parent)
    }
}

pub trait Scalar: Copy +
    PartialEq +
    Add<Output = Self> +
    Sub<Output = Self> +
    Mul<Output = Self> +
    Div<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
}

impl Scalar for u32 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Scalar for u64 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Scalar for i32 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Scalar for i64 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Scalar for f32 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
}

impl Scalar for f64 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
}


impl<T: Scalar> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }
        for i in 0..self.0.len() {
            if self.0[i].len() != rhs.0[i].len() {
                return None;
            }
        }

        let mut result = Vec::new();
        for i in 0..self.0.len() {
            let mut row = Vec::new();
            for j in 0..self.0[i].len() {
                row.push(self.0[i][j] + rhs.0[i][j]);
            }
            result.push(row);
        }
        Some(Matrix(result))
    }
}

impl<T: Scalar> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }
        for i in 0..self.0.len() {
            if self.0[i].len() != rhs.0[i].len() {
                return None;
            }
        }
        let mut result = Vec::new();
        for i in 0..self.0.len() {
            let mut row = Vec::new();
            for j in 0..self.0[i].len() {
                row.push(self.0[i][j] - rhs.0[i][j]);
            }
            result.push(row);
        }
        Some(Matrix(result))
    }
}
