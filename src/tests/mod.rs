use super::*;

#[test]
fn test_matrix_indexing() {
    let vals = [[1, 2], [3, 4]];
    let matrix = Matrix::new(vals);
    assert_eq!(matrix[(0, 0)], 1);
    assert_eq!(matrix[(0, 1)], 2);
    assert_eq!(matrix[(1, 0)], 3);
    assert_eq!(matrix[(1, 1)], 4);
}

#[test]
fn test_matrix_ref_indexing() {
    let vals = [[1, 2], [3, 4]];
    let matrix_ref = MatrixRef::new(&vals);
    assert_eq!(matrix_ref[(0, 0)], 1);
    assert_eq!(matrix_ref[(0, 1)], 2);
    assert_eq!(matrix_ref[(1, 0)], 3);
    assert_eq!(matrix_ref[(1, 1)], 4);
}

#[test]
fn test_matrix_vals_field_access() {
    let vals = [[1, 2], [3, 4]];
    let matrix = Matrix::new(vals);
    assert_eq!(matrix.vals[0][0], 1);
    assert_eq!(matrix.vals[0][1], 2);
    assert_eq!(matrix.vals[1][0], 3);
    assert_eq!(matrix.vals[1][1], 4);
}

#[test]
fn test_vals_vs_index_consistency() {
    let vals = [[1, 2], [3, 4]];
    let matrix = Matrix::new(vals);
    
    // Test that direct vals access matches indexing
    assert_eq!(matrix.vals[0][0], matrix[(0, 0)]);
    assert_eq!(matrix.vals[0][1], matrix[(0, 1)]);
    assert_eq!(matrix.vals[1][0], matrix[(1, 0)]);
    assert_eq!(matrix.vals[1][1], matrix[(1, 1)]);
}

#[test]
fn test_matrix_ref_vals_vs_index() {
    let vals = [[5, 6], [7, 8]];
    let matrix_ref = MatrixRef::new(&vals);
    
    // Test that direct vals access matches indexing for MatrixRef
    assert_eq!(matrix_ref.vals[0][0], matrix_ref[(0, 0)]);
    assert_eq!(matrix_ref.vals[0][1], matrix_ref[(0, 1)]);
    assert_eq!(matrix_ref.vals[1][0], matrix_ref[(1, 0)]);
    assert_eq!(matrix_ref.vals[1][1], matrix_ref[(1, 1)]);
}