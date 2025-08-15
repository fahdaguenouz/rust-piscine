impl Matrix<T> {
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
                col_vec.push(row[n]);
            }
        }
        col_vec
    }
}

impl Mul for Matrix<T> {
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