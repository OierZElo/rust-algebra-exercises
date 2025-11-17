➗ Matrix Rank Calculation using Gaussian Elimination (Rust)

This Rust module implements a function to calculate the rank of a matrix using Gaussian elimination. The rank is determined by the number of non-zero rows (pivots) remaining after reducing the matrix to its row echelon form.

The current implementation operates on fixed-size 5×5 arrays ([[f64; 5]; 5]) and modifies the input matrix in-place.

🛠️ Implementation Details

1. fn gauss_calcular_rango(matriz: &mut [[f64; 5]; 5]) -> usize

This function performs Gaussian elimination to find the rank of the input matrix.

    Input: A mutable 5×5 array (&mut [[f64; 5]; 5]). The matrix is reduced in-place.

    Output: The rank of the matrix, returned as a usize.

    Methodology:

        Initialization: The rank is initialized to 0. The function iterates through the columns.

        Pivoting: For the current column, it searches for a non-zero pivot starting from the row equal to the current rank count.

        Row Swapping: If a pivot is found, the pivot row is swapped into the current rank position (rango).

        Normalization: The pivot is normalized to 1.0 by dividing the entire row by the pivot value.

        Elimination: Operations are performed to create 0.0s below the pivot.

        Rank Count: After successfully placing and reducing a pivot, the rank counter (rango) is incremented, and the process moves to the next column and the next row position.

        If no pivot is found in a column, the rank count remains the same, and the process moves to the next column (continue). The final value of rango is returned.
