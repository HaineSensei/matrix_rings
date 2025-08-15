use std::{fmt::Debug, ops::Index};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Matrix<R, const ROWS: usize, const COLS: usize> {
    pub(crate) vals: [[R;COLS];ROWS]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MatrixRef<'a, R, const ROWS: usize, const COLS: usize> {
    pub(crate) vals: &'a [[R;COLS];ROWS]
}

impl<R, const ROWS: usize, const COLS: usize> Matrix<R,ROWS,COLS> {
    pub fn new(vals: [[R;COLS];ROWS]) -> Self {
        Self {
            vals
        }
    }

    pub fn as_ref<'a>(&'a self) -> MatrixRef<'a, R, ROWS, COLS> {
        MatrixRef {
            vals: &self.vals
        }
    }
}

impl<'a, R, const ROWS: usize, const COLS: usize> MatrixRef<'a, R, ROWS, COLS> {
    pub fn new(vals: &'a [[R;COLS];ROWS]) -> Self {
        Self {
            vals
        }
    }

    pub fn to_matrix(&self) -> Matrix<R,ROWS,COLS> 
    where
        R: Clone
    {
        Matrix {
            vals: self.vals.clone()
        }
    }
}

impl<R, const ROWS: usize, const COLS: usize> Index<(usize, usize)> for Matrix<R, ROWS, COLS> {
    type Output = R;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.vals[row][col]
    }
}

impl<'a, R, const ROWS: usize, const COLS: usize> Index<(usize, usize)> for MatrixRef<'a, R, ROWS, COLS> {
    type Output = R;

    fn index(&self, (row, col): (usize, usize)) -> &'a R {
        &self.vals[row][col]
    }
}