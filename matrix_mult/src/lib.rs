use std::ops::{ Add, Sub, Mul, Div };


impl<T:Scalar + Clone> Mul for Matrix<T> {
     type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.number_of_cols() != rhs.number_of_rows() {
            return None;
        }

        let rows = self.number_of_rows();
        let cols = rhs.number_of_cols();
        let mut result = vec![vec![T::zero(); cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                let mut sum = T::zero();
                for k in 0..self.number_of_cols() {
                    sum = sum + (self.0[i][k] * rhs.0[k][j]);
                }
                result[i][j] = sum;
            }
        }

        Some(Matrix(result))
    }
}





#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar+Clone> Matrix<T> {
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
    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        if n < self.0.len() {
            self.0[n].clone()
        } else {
            vec![]
        }
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut col_vec = Vec::new();
        for row in &self.0 {
            if n < row.len() {
                col_vec.push(row[n].clone());
            }
        }
        col_vec
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
