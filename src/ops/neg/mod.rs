use std::{array::from_fn, ops::Neg};

use crate::{Matrix, MatrixRef};

impl<'a, R,const ROWS: usize, const COLS: usize> Neg for MatrixRef<'a,R,ROWS,COLS> 
where 
    &'a R: Neg<Output = R>
{
    type Output = Matrix<R,ROWS,COLS>;

    fn neg(self) -> Self::Output {
        let vals = from_fn(|i|
            from_fn(|j| 
                -&self.vals[i][j]
            )
        );
        Self::Output {
            vals
        }
    }
}

impl<R,const ROWS: usize, const COLS: usize> Neg for &Matrix<R,ROWS,COLS> 
where 
    for<'a> &'a R: Neg<Output = R>
{
    type Output = Matrix<R,ROWS,COLS>;

    fn neg(self) -> Self::Output {
        -self.as_ref()
    }
}

impl<R,const ROWS: usize, const COLS: usize> Neg for Matrix<R,ROWS,COLS> 
where 
    for<'a> &'a R: Neg<Output = R>
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        -&self
    }
}