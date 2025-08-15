use std::{array::from_fn, ops::Add};
use crate::matrix::{Matrix, MatrixRef};

impl<'a, 'b, R, const ROWS: usize, const COLS: usize> Add<MatrixRef<'b, R, ROWS, COLS>> for MatrixRef<'a, R, ROWS, COLS>
where
    &'a R: Add<&'b R, Output = R>
{
    type Output = Matrix<R, ROWS, COLS>;

    fn add(self, rhs: MatrixRef<'b, R, ROWS, COLS>) -> Self::Output {
        let MatrixRef { vals: lhs } = self;
        let MatrixRef { vals: rhs } = rhs;
        let vals = from_fn(|i|
        from_fn(|j| &lhs[i][j] + &rhs[i][j]));
        Matrix {
            vals
        }
    }
}

impl<'a, 'b, R, const ROWS: usize, const COLS: usize> Add<&'b Matrix<R, ROWS, COLS>> for &'a Matrix<R, ROWS, COLS>
where 
    &'a R: Add<&'b R, Output = R>
{
    type Output = Matrix<R, ROWS, COLS>;

    fn add(self, rhs: &'b Matrix<R, ROWS, COLS>) -> Self::Output {
        self.as_ref() + rhs.as_ref()
    }
}

impl <R, const ROWS: usize, const COLS: usize> Add for Matrix<R, ROWS, COLS>
where
    for<'a, 'b> &'a R: Add<&'b R, Output = R>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}