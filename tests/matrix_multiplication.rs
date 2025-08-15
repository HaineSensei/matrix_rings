use matrix_rings::Matrix;

#[test]
fn test_matrix_multiplication_2x2() {
    // [1 2] * [5 6] = [19 22]
    // [3 4]   [7 8]   [43 50]
    let a: Matrix<i32, 2, 2> = Matrix::new([[1, 2], [3, 4]]);
    let b: Matrix<i32, 2, 2> = Matrix::new([[5, 6], [7, 8]]);
    let expected = [[19, 22], [43, 50]];

    let result = a.as_ref() * b.as_ref(); 
    
    for i in 0..2 {
        for j in 0..2 {
            assert_eq!(result[(i, j)], expected[i][j]);
        }
    }
}

#[test]
fn test_matrix_multiplication_2x3_times_3x2() {
    // [1 2 3] * [7  8 ] = [58  64 ]
    // [4 5 6]   [9  10]   [139 154]
    //           [11 12]
    let a = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    let b = Matrix::new([[7, 8], [9, 10], [11, 12]]);
    let expected = [[58, 64], [139, 154]];
    
    let result = a.as_ref() * b.as_ref();
    
    for i in 0..2 {
        for j in 0..2 {
            assert_eq!(result[(i, j)], expected[i][j]);
        }
    }
}

#[test]
fn test_matrix_multiplication_identity() {
    // Any matrix times identity should equal itself
    let a = Matrix::new([[1, 2], [3, 4]]);
    let identity = Matrix::new([[1, 0], [0, 1]]);
    
    let result = a.as_ref() * identity.as_ref();
    
    for i in 0..2 {
        for j in 0..2 {
            assert_eq!(result[(i, j)], a[(i, j)]);
        }
    }
}

#[test]
fn test_matrix_multiplication_single_element() {
    // 1x1 matrices
    let a = Matrix::new([[5]]);
    let b = Matrix::new([[3]]);
    let expected = [[15]];
    
    let result = a.as_ref() * b.as_ref();
    assert_eq!(result[(0, 0)], expected[0][0]);
}

#[test]
fn test_matrix_multiplication_vector_like() {
    // 3x1 * 1x3 = 3x3 outer product
    let a = Matrix::new([[1], [2], [3]]);
    let b = Matrix::new([[4, 5, 6]]);
    let expected = [[4, 5, 6], [8, 10, 12], [12, 15, 18]];
    
    let result = a.as_ref() * b.as_ref();
    
    for i in 0..3 {
        for j in 0..3 {
            assert_eq!(result[(i, j)], expected[i][j]);
        }
    }
}

#[test]
fn test_matrix_multiplication_owned() {
    // Test consuming Matrix * Matrix (not references)
    let a = Matrix::new([[1, 2], [3, 4]]);
    let b = Matrix::new([[5, 6], [7, 8]]);
    let expected = [[19, 22], [43, 50]];
    
    let result = a * b; // consuming both matrices
    
    for i in 0..2 {
        for j in 0..2 {
            assert_eq!(result[(i, j)], expected[i][j]);
        }
    }
}

#[test]
fn test_matrix_multiplication_borrowed() {
    // Test &Matrix * &Matrix (borrowed references)
    let a = Matrix::new([[1, 2], [3, 4]]);
    let b = Matrix::new([[5, 6], [7, 8]]);
    let expected = [[19, 22], [43, 50]];
    
    let result = &a * &b; // borrowing both matrices
    
    for i in 0..2 {
        for j in 0..2 {
            assert_eq!(result[(i, j)], expected[i][j]);
        }
    }
    
    // Matrices should still be usable after borrowing
    assert_eq!(a[(0, 0)], 1);
    assert_eq!(b[(0, 0)], 5);
}

#[test]
fn test_matrix_multiplication_f64() {
    // Test with floating point numbers
    let a: Matrix<f64, 2, 2> = Matrix::new([[1.5, 2.0], [3.5, 4.0]]);
    let b: Matrix<f64, 2, 2> = Matrix::new([[2.0, 1.0], [0.5, 3.0]]);
    let expected = [[4.0, 7.5], [9.0, 15.5]];
    
    let result = a.as_ref() * b.as_ref();
    
    for i in 0..2 {
        for j in 0..2 {
            assert!((result[(i, j)] - expected[i][j]).abs() < f64::EPSILON);
        }
    }
}

#[test]
fn test_matrix_multiplication_dot_product() {
    // 1×N * N×1 = 1×1 (dot product style)
    let a = Matrix::new([[1, 2, 3, 4]]);  // 1×4
    let b = Matrix::new([[5], [6], [7], [8]]);  // 4×1
    let expected = [[70]];  // 1*5 + 2*6 + 3*7 + 4*8 = 5 + 12 + 21 + 32 = 70
    
    let result = a.as_ref() * b.as_ref();
    assert_eq!(result[(0, 0)], expected[0][0]);
}

#[test]
fn test_matrix_multiplication_tall_skinny() {
    // N×1 * 1×M = N×M (outer product style, but different dimensions)
    let a = Matrix::new([[2], [3]]);  // 2×1
    let b = Matrix::new([[4, 5, 6]]);  // 1×3
    let expected = [[8, 10, 12], [12, 15, 18]];  // 2×3
    
    let result = a.as_ref() * b.as_ref();
    
    for i in 0..2 {
        for j in 0..3 {
            assert_eq!(result[(i, j)], expected[i][j]);
        }
    }
}

#[test]
fn test_matrix_multiplication_zero_dimension() {
    // 3×0 * 0×2 = 3×2 zero matrix
    let a: Matrix<i32, 3, 0> = Matrix::new([[], [], []]);
    let b: Matrix<i32, 0, 2> = Matrix::new([]);
    
    let result = a.as_ref() * b.as_ref();
    
    // Result should be 3×2 zero matrix
    for i in 0..3 {
        for j in 0..2 {
            assert_eq!(result[(i, j)], 0);
        }
    }
}