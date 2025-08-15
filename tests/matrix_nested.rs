use matrix_rings::Matrix;
use num_traits::{Zero, One};

#[test]
fn test_matrix_of_matrix_addition() {
    // Create 2x2 matrices to use as elements
    let a11 = Matrix::new([[1, 2], [3, 4]]);
    let a12 = Matrix::new([[5, 6], [7, 8]]);
    let a21 = Matrix::new([[9, 10], [11, 12]]);
    let a22 = Matrix::new([[13, 14], [15, 16]]);
    
    let b11 = Matrix::new([[1, 1], [1, 1]]);
    let b12 = Matrix::new([[2, 2], [2, 2]]);
    let b21 = Matrix::new([[3, 3], [3, 3]]);
    let b22 = Matrix::new([[4, 4], [4, 4]]);
    
    // Create 2x2 matrices of 2x2 matrices
    let a: Matrix<Matrix<i32, 2, 2>, 2, 2> = Matrix::new([[a11, a12], [a21, a22]]);
    let b: Matrix<Matrix<i32, 2, 2>, 2, 2> = Matrix::new([[b11, b12], [b21, b22]]);
    
    let result = a + b;
    
    // Check that each sub-matrix was added correctly
    assert_eq!(result[(0, 0)][(0, 0)], 2);  // a11[0,0] + b11[0,0] = 1 + 1
    assert_eq!(result[(0, 0)][(0, 1)], 3);  // a11[0,1] + b11[0,1] = 2 + 1
    assert_eq!(result[(0, 1)][(0, 0)], 7);  // a12[0,0] + b12[0,0] = 5 + 2
    assert_eq!(result[(1, 1)][(1, 1)], 20); // a22[1,1] + b22[1,1] = 16 + 4
}

#[test]
fn test_matrix_of_matrix_multiplication() {
    // Create simple 2x2 matrices for block multiplication
    let a11 = Matrix::new([[1, 0], [0, 1]]);  // Identity
    let a12 = Matrix::new([[2, 0], [0, 2]]);  // 2 * Identity
    let a21: Matrix<i32, 2, 2> = Matrix::zero(); // Zero matrix
    let a22 = Matrix::new([[1, 0], [0, 1]]);  // Identity
    
    let b11 = Matrix::new([[3, 0], [0, 3]]);  // 3 * Identity
    let b12: Matrix<i32, 2, 2> = Matrix::zero(); // Zero matrix
    let b21 = Matrix::new([[1, 1], [1, 1]]);  // All ones
    let b22 = Matrix::new([[2, 0], [0, 2]]);  // 2 * Identity
    
    // Create block matrices
    let a: Matrix<Matrix<i32, 2, 2>, 2, 2> = Matrix::new([[a11, a12], [a21, a22]]);
    let b: Matrix<Matrix<i32, 2, 2>, 2, 2> = Matrix::new([[b11, b12], [b21, b22]]);
    
    let result = a * b;
    
    // result[0,0] = a11*b11 + a12*b21 = I*3I + 2I*ones = 3I + 2*ones
    assert_eq!(result[(0, 0)][(0, 0)], 5);  // 3 + 2*1
    assert_eq!(result[(0, 0)][(0, 1)], 2);  // 0 + 2*1
    assert_eq!(result[(0, 0)][(1, 0)], 2);  // 0 + 2*1
    assert_eq!(result[(0, 0)][(1, 1)], 5);  // 3 + 2*1
    
    // result[0,1] = a11*b12 + a12*b22 = I*0 + 2I*2I = 0 + 4I = 4I
    assert_eq!(result[(0, 1)][(0, 0)], 4);
    assert_eq!(result[(0, 1)][(0, 1)], 0);
    assert_eq!(result[(0, 1)][(1, 0)], 0);
    assert_eq!(result[(0, 1)][(1, 1)], 4);
    
    // result[1,0] = a21*b11 + a22*b21 = 0*3I + I*ones = 0 + ones = ones
    assert_eq!(result[(1, 0)][(0, 0)], 1);
    assert_eq!(result[(1, 0)][(0, 1)], 1);
    assert_eq!(result[(1, 0)][(1, 0)], 1);
    assert_eq!(result[(1, 0)][(1, 1)], 1);
    
    // result[1,1] = a21*b12 + a22*b22 = 0*0 + I*2I = 0 + 2I = 2I
    assert_eq!(result[(1, 1)][(0, 0)], 2);
    assert_eq!(result[(1, 1)][(0, 1)], 0);
    assert_eq!(result[(1, 1)][(1, 0)], 0);
    assert_eq!(result[(1, 1)][(1, 1)], 2);
}

#[test]
fn test_matrix_of_matrix_rectangular() {
    // Test 2x3 matrix of 2x2 matrices times 3x1 matrix of 2x2 matrices
    // This will give us a 2x1 matrix of 2x2 matrices
    
    // Create some simple 2x2 matrices to work with
    let ones = Matrix::new([[1, 1], [1, 1]]);
    let twos = Matrix::new([[2, 2], [2, 2]]);
    let threes = Matrix::new([[3, 3], [3, 3]]);
    let fours = Matrix::new([[4, 4], [4, 4]]);
    let fives = Matrix::new([[5, 5], [5, 5]]);
    let sixes = Matrix::new([[6, 6], [6, 6]]);
    
    // Create 2x3 matrix of matrices
    let a: Matrix<Matrix<i32, 2, 2>, 2, 3> = Matrix::new([
        [ones.clone(), twos.clone(), threes.clone()],
        [fours.clone(), fives.clone(), sixes.clone()]
    ]);
    
    // Create 3x1 matrix of matrices  
    let b: Matrix<Matrix<i32, 2, 2>, 3, 1> = Matrix::new([
        [ones.clone()],
        [twos.clone()],
        [threes.clone()]
    ]);
    
    let result = a * b;
    
    // result[0,0] = a[0,0]*b[0,0] + a[0,1]*b[1,0] + a[0,2]*b[2,0]
    //             = ones*ones + twos*twos + threes*threes  
    //             = [[2,2],[2,2]] + [[8,8],[8,8]] + [[18,18],[18,18]]
    //             = [[28,28],[28,28]]
    assert_eq!(result[(0, 0)][(0, 0)], 28);
    assert_eq!(result[(0, 0)][(0, 1)], 28);
    assert_eq!(result[(0, 0)][(1, 0)], 28);
    assert_eq!(result[(0, 0)][(1, 1)], 28);
    
    // result[1,0] = a[1,0]*b[0,0] + a[1,1]*b[1,0] + a[1,2]*b[2,0]
    //             = fours*ones + fives*twos + sixes*threes
    //             = [[8,8],[8,8]] + [[20,20],[20,20]] + [[36,36],[36,36]]
    //             = [[64,64],[64,64]]
    assert_eq!(result[(1, 0)][(0, 0)], 64);
    assert_eq!(result[(1, 0)][(0, 1)], 64);
    assert_eq!(result[(1, 0)][(1, 0)], 64);
    assert_eq!(result[(1, 0)][(1, 1)], 64);
}

#[test]
fn test_matrix_of_matrix_non_square_internal() {
    // Test with non-square internal matrices: 2x2 matrix of (2x3 matrices) times 2x1 matrix of (3x2 matrices)
    // This will give us a 2x1 matrix of (2x2 matrices)
    
    // Create some 2x3 matrices for first block matrix
    let a_mat = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    let b_mat = Matrix::new([[2, 0, 1], [1, 3, 2]]);
    let c_mat = Matrix::new([[1, 1, 1], [0, 1, 0]]);
    let d_mat = Matrix::new([[3, 2, 1], [1, 1, 1]]);
    
    // Create some 3x2 matrices for second block matrix
    let e_mat = Matrix::new([[1, 0], [2, 1], [1, 2]]);
    let f_mat = Matrix::new([[2, 3], [0, 1], [1, 0]]);
    
    // Create 2x2 matrix of (2x3 matrices)
    let outer_a: Matrix<Matrix<i32, 2, 3>, 2, 2> = Matrix::new([
        [a_mat.clone(), b_mat.clone()],
        [c_mat.clone(), d_mat.clone()]
    ]);
    
    // Create 2x1 matrix of (3x2 matrices)
    let outer_b: Matrix<Matrix<i32, 3, 2>, 2, 1> = Matrix::new([
        [e_mat.clone()],
        [f_mat.clone()]
    ]);
    
    let result = outer_a * outer_b;
    
    // result[0,0] = a_mat*e_mat + b_mat*f_mat
    // a_mat*e_mat = [[1,2,3],[4,5,6]] * [[1,0],[2,1],[1,2]] = [[8,8],[20,17]]
    // b_mat*f_mat = [[2,0,1],[1,3,2]] * [[2,3],[0,1],[1,0]] = [[5,6],[4,6]]
    // sum = [[13,14],[24,23]]
    assert_eq!(result[(0, 0)][(0, 0)], 13);
    assert_eq!(result[(0, 0)][(0, 1)], 14);
    assert_eq!(result[(0, 0)][(1, 0)], 24);
    assert_eq!(result[(0, 0)][(1, 1)], 23);
    
    // result[1,0] = c_mat*e_mat + d_mat*f_mat  
    // c_mat*e_mat = [[1,1,1],[0,1,0]] * [[1,0],[2,1],[1,2]] = [[4,3],[2,1]]
    // d_mat*f_mat = [[3,2,1],[1,1,1]] * [[2,3],[0,1],[1,0]] = [[7,11],[3,4]]
    // sum = [[11,14],[5,5]]
    assert_eq!(result[(1, 0)][(0, 0)], 11);
    assert_eq!(result[(1, 0)][(0, 1)], 14);
    assert_eq!(result[(1, 0)][(1, 0)], 5);
    assert_eq!(result[(1, 0)][(1, 1)], 5);
}

#[test]
fn test_matrix_of_matrix_zero() {
    // Test that we can create zero matrices of matrices
    let zero: Matrix<Matrix<i32, 2, 2>, 3, 3> = Matrix::zero();
    
    // Each element should be a zero 2x2 matrix
    for i in 0..3 {
        for j in 0..3 {
            for x in 0..2 {
                for y in 0..2 {
                    assert_eq!(zero[(i, j)][(x, y)], 0);
                }
            }
        }
    }
}

#[test]
fn test_matrix_of_matrix_identity() {
    // Test that we can create identity matrices of matrices
    let identity: Matrix<Matrix<i32, 2, 2>, 2, 2> = Matrix::one();
    
    // Diagonal elements should be identity matrices
    assert_eq!(identity[(0, 0)][(0, 0)], 1);
    assert_eq!(identity[(0, 0)][(0, 1)], 0);
    assert_eq!(identity[(0, 0)][(1, 0)], 0);
    assert_eq!(identity[(0, 0)][(1, 1)], 1);
    
    assert_eq!(identity[(1, 1)][(0, 0)], 1);
    assert_eq!(identity[(1, 1)][(0, 1)], 0);
    assert_eq!(identity[(1, 1)][(1, 0)], 0);
    assert_eq!(identity[(1, 1)][(1, 1)], 1);
    
    // Off-diagonal elements should be zero matrices
    for x in 0..2 {
        for y in 0..2 {
            assert_eq!(identity[(0, 1)][(x, y)], 0);
            assert_eq!(identity[(1, 0)][(x, y)], 0);
        }
    }
}