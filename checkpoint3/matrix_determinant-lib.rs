

/*
pub fn matrix_determinant(matrix: [[isize; 3]; 3]) -> isize {
    // Extracting the first row variables (a, b, c)
    let a = matrix[0][0];
    let b = matrix[0][1];
    let c = matrix[0][2];

    // Extracting the second row variables (d, e, f)
    let d = matrix[1][0];
    let e = matrix[1][1];
    let f = matrix[1][2];

    // Extracting the third row variables (g, h, i)
    let g = matrix[2][0];
    let h = matrix[2][1];
    let i = matrix[2][2];

    // Calculating the determinants of the 2x2 submatrices
    let det_a = e * i - f * h;
    let det_b = d * i - f * g;
    let det_c = d * h - e * g;

    // Final calculation: a*(submatrix a) - b*(submatrix b) + c*(submatrix c)
    a * det_a - b * det_b + c * det_c
}
*/