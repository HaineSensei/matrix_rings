use matrix_rings::{Matrix, MatrixRef};

#[test]
fn test_matrix_addition_owned() {
    let vals1 = [[1, 2], [3, 4]];
    let vals2 = [[5, 6], [7, 8]];
    let expected = [[6, 8], [10, 12]];
    
    let matrix1 = Matrix::new(vals1);
    let matrix2 = Matrix::new(vals2);
    let result = matrix1 + matrix2;
    
    assert_eq!(result[(0, 0)], expected[0][0]);
    assert_eq!(result[(0, 1)], expected[0][1]);
    assert_eq!(result[(1, 0)], expected[1][0]);
    assert_eq!(result[(1, 1)], expected[1][1]);
}

#[test]
fn test_matrix_addition_borrowed() {
    let vals1 = [[1, 2], [3, 4]];
    let vals2 = [[5, 6], [7, 8]];
    let expected = [[6, 8], [10, 12]];
    
    let matrix1 = Matrix::new(vals1);
    let matrix2 = Matrix::new(vals2);
    let result = &matrix1 + &matrix2;
    
    assert_eq!(result[(0, 0)], expected[0][0]);
    assert_eq!(result[(0, 1)], expected[0][1]);
    assert_eq!(result[(1, 0)], expected[1][0]);
    assert_eq!(result[(1, 1)], expected[1][1]);
}

#[test]
fn test_matrix_ref_addition() {
    let vals1 = [[1, 2], [3, 4]];
    let vals2 = [[5, 6], [7, 8]];
    let expected = [[6, 8], [10, 12]];
    
    let matrix_ref1 = MatrixRef::new(&vals1);
    let matrix_ref2 = MatrixRef::new(&vals2);
    let result = matrix_ref1 + matrix_ref2;
    
    assert_eq!(result[(0, 0)], expected[0][0]);
    assert_eq!(result[(0, 1)], expected[0][1]);
    assert_eq!(result[(1, 0)], expected[1][0]);
    assert_eq!(result[(1, 1)], expected[1][1]);
}

#[test]
fn test_matrix_addition_different_sizes() {
    // 1x3 matrices
    let vals1 = [[1, 2, 3]];
    let vals2 = [[4, 5, 6]];
    let expected = [[5, 7, 9]];
    
    let matrix1 = Matrix::new(vals1);
    let matrix2 = Matrix::new(vals2);
    let result = matrix1 + matrix2;
    
    assert_eq!(result[(0, 0)], expected[0][0]);
    assert_eq!(result[(0, 1)], expected[0][1]);
    assert_eq!(result[(0, 2)], expected[0][2]);
}

#[test]
fn test_matrix_addition_single_element() {
    let vals1 = [[42]];
    let vals2 = [[8]];
    let expected = [[50]];
    
    let matrix1 = Matrix::new(vals1);
    let matrix2 = Matrix::new(vals2);
    let result = matrix1 + matrix2;
    
    assert_eq!(result[(0, 0)], expected[0][0]);
}

#[test]
fn test_matrix_addition_3x3() {
    let vals1 = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let vals2 = [[9, 8, 7], [6, 5, 4], [3, 2, 1]];
    let expected = [[10, 10, 10], [10, 10, 10], [10, 10, 10]];
    
    let matrix1 = Matrix::new(vals1);
    let matrix2 = Matrix::new(vals2);
    let result = matrix1 + matrix2;
    
    for i in 0..3 {
        for j in 0..3 {
            assert_eq!(result[(i, j)], expected[i][j]);
        }
    }
}

#[test]
fn test_matrix_subtraction_owned() {
    let vals1 = [[10, 8], [12, 6]];
    let vals2 = [[5, 6], [7, 8]];
    let expected = [[5, 2], [5, -2]];
    
    let matrix1 = Matrix::new(vals1);
    let matrix2 = Matrix::new(vals2);
    let result = matrix1 - matrix2;
    
    assert_eq!(result[(0, 0)], expected[0][0]);
    assert_eq!(result[(0, 1)], expected[0][1]);
    assert_eq!(result[(1, 0)], expected[1][0]);
    assert_eq!(result[(1, 1)], expected[1][1]);
}

#[test]
fn test_matrix_subtraction_borrowed() {
    let vals1 = [[10, 8], [12, 6]];
    let vals2 = [[5, 6], [7, 8]];
    let expected = [[5, 2], [5, -2]];
    
    let matrix1 = Matrix::new(vals1);
    let matrix2 = Matrix::new(vals2);
    let result = &matrix1 - &matrix2;
    
    assert_eq!(result[(0, 0)], expected[0][0]);
    assert_eq!(result[(0, 1)], expected[0][1]);
    assert_eq!(result[(1, 0)], expected[1][0]);
    assert_eq!(result[(1, 1)], expected[1][1]);
    
    // Matrices should still be usable after borrowing
    assert_eq!(matrix1[(0, 0)], 10);
    assert_eq!(matrix2[(0, 0)], 5);
}

#[test]
fn test_matrix_ref_subtraction() {
    let vals1 = [[10, 8], [12, 6]];
    let vals2 = [[5, 6], [7, 8]];
    let expected = [[5, 2], [5, -2]];
    
    let matrix_ref1 = MatrixRef::new(&vals1);
    let matrix_ref2 = MatrixRef::new(&vals2);
    let result = matrix_ref1 - matrix_ref2;
    
    assert_eq!(result[(0, 0)], expected[0][0]);
    assert_eq!(result[(0, 1)], expected[0][1]);
    assert_eq!(result[(1, 0)], expected[1][0]);
    assert_eq!(result[(1, 1)], expected[1][1]);
}

#[test]
fn test_matrix_subtraction_f64() {
    let vals1 = [[10.5, 8.0], [12.5, 6.0]];
    let vals2 = [[5.0, 6.5], [7.0, 8.5]];
    let expected = [[5.5, 1.5], [5.5, -2.5]];
    
    let matrix1: Matrix<f64, 2, 2> = Matrix::new(vals1);
    let matrix2: Matrix<f64, 2, 2> = Matrix::new(vals2);
    let result = matrix1 - matrix2;
    
    for i in 0..2 {
        for j in 0..2 {
            assert!((result[(i, j)] - expected[i][j]).abs() < f64::EPSILON);
        }
    }
}