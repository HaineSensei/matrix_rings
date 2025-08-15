use std::{array::from_fn, ops::{Mul, Add}};
use num_traits::Zero;

use crate::matrix::{Matrix, MatrixRef};

// Multiplication implementations will go here

impl<'a, 'b, R1, R2, R3, const A: usize, const B: usize, const C: usize> Mul<MatrixRef<'b, R2, B, C>> for MatrixRef<'a, R1, A, B>
where 
    &'a R1 : Mul<&'b R2, Output = R3>,
    R3: Add<Output = R3> + Zero
{
    type Output = Matrix<R3, A, C>;

    fn mul(self, rhs: MatrixRef<'b, R2, B, C>) -> Self::Output {
        let MatrixRef { vals: lhs } = self;
        let MatrixRef { vals: rhs } = rhs;
        let vals = from_fn(|i|
            from_fn(|k|
                (0..B)
                .map(|j| &lhs[i][j] * &rhs[j][k]
                )
                .fold(R3::zero(), |x, y| x + y)
            )
        );
        Self::Output {
            vals
        }
    }
}

impl<'a, 'b, R1, R2, R3, const A: usize, const B: usize, const C: usize> Mul<&'b Matrix<R2, B, C>> for &'a Matrix<R1, A, B>
where 
    &'a R1: Mul<&'b R2, Output = R3>,
    R3: Add<Output = R3> + Zero
{
    type Output = Matrix<R3, A, C>;

    fn mul(self, rhs: &'b Matrix<R2, B, C>) -> Self::Output {
        self.as_ref() * rhs.as_ref()
    }
}

impl<R1, R2, R3, const A: usize, const B: usize, const C: usize> Mul<Matrix<R2, B, C>> for Matrix<R1, A, B>
where
    for<'a, 'b> &'a R1: Mul<&'b R2, Output = R3>,
    R3: Add<Output = R3> + Zero
{
    type Output = Matrix<R3, A, C>;

    fn mul(self, rhs: Matrix<R2, B, C>) -> Self::Output {
        &self * &rhs
    }
}