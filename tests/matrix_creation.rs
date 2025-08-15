use matrix_rings::{Matrix, MatrixRef};

#[test]
fn test_matrix_creation() {
    let vals = [[1, 2], [3, 4]];
    let matrix = Matrix::new(vals);
    assert_eq!(matrix[(0, 0)], vals[0][0]);
    assert_eq!(matrix[(0, 1)], vals[0][1]);
    assert_eq!(matrix[(1, 0)], vals[1][0]);
    assert_eq!(matrix[(1, 1)], vals[1][1]);
}

#[test]
fn test_matrix_ref_creation() {
    let vals = [[1, 2], [3, 4]];
    let matrix_ref = MatrixRef::new(&vals);
    assert_eq!(matrix_ref[(0, 0)], vals[0][0]);
    assert_eq!(matrix_ref[(0, 1)], vals[0][1]);
    assert_eq!(matrix_ref[(1, 0)], vals[1][0]);
    assert_eq!(matrix_ref[(1, 1)], vals[1][1]);
}

#[test]
fn test_matrix_as_ref() {
    let vals = [[1, 2], [3, 4]];
    let matrix = Matrix::new(vals);
    let matrix_ref = matrix.as_ref();
    assert_eq!(matrix_ref[(0, 0)], vals[0][0]);
    assert_eq!(matrix_ref[(0, 1)], vals[0][1]);
    assert_eq!(matrix_ref[(1, 0)], vals[1][0]);
    assert_eq!(matrix_ref[(1, 1)], vals[1][1]);
}

#[test]
fn test_matrix_ref_to_matrix() {
    let vals = [[1, 2], [3, 4]];
    let matrix_ref = MatrixRef::new(&vals);
    let matrix = matrix_ref.to_matrix();
    assert_eq!(matrix[(0, 0)], vals[0][0]);
    assert_eq!(matrix[(0, 1)], vals[0][1]);
    assert_eq!(matrix[(1, 0)], vals[1][0]);
    assert_eq!(matrix[(1, 1)], vals[1][1]);
}

#[test]
fn test_different_sized_matrices() {
    // 1x3 matrix
    let vals_1x3 = [[1, 2, 3]];
    let matrix_1x3 = Matrix::new(vals_1x3);
    assert_eq!(matrix_1x3[(0, 0)], vals_1x3[0][0]);
    assert_eq!(matrix_1x3[(0, 1)], vals_1x3[0][1]);
    assert_eq!(matrix_1x3[(0, 2)], vals_1x3[0][2]);
    
    // 3x1 matrix  
    let vals_3x1 = [[1], [2], [3]];
    let matrix_3x1 = Matrix::new(vals_3x1);
    assert_eq!(matrix_3x1[(0, 0)], vals_3x1[0][0]);
    assert_eq!(matrix_3x1[(1, 0)], vals_3x1[1][0]);
    assert_eq!(matrix_3x1[(2, 0)], vals_3x1[2][0]);
    
    // Single element matrix
    let vals_1x1 = [[42]];
    let matrix_1x1 = Matrix::new(vals_1x1);
    assert_eq!(matrix_1x1[(0, 0)], vals_1x1[0][0]);
}