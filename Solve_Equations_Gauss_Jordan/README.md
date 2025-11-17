➗ Solving Linear Equations with Gauss-Jordan (Rust)

This Rust module implements the Gauss-Jordan elimination method to solve a system of linear equations, represented by an augmented matrix.

The current implementation uses a fixed-size 5×6 augmented array ([[f64; 6]; 5]), representing a system of 5 equations with 5 variables, plus the constant vector. The solution is obtained by reducing the coefficient matrix to the identity matrix.

🛠️ Implementation Details

1. fn gauss_resolver_sistema(matriz: &mut [[f64; 6]; 5]) -> &mut [[f64; 6]; 5]

This function performs Gauss-Jordan elimination on the augmented matrix.

    Input: A mutable 5×6 augmented array (&mut [[f64; 6]; 5]). The matrix is modified in-place.

    Output: A mutable reference to the solved augmented matrix. The solution vector resides in the last column.

    Methodology:

        Pivot Search: Iterates through columns, searching for a non-zero pivot starting from the current row index (col).

        Solvability Check: If no non-zero pivot can be found in a column, the system might have no unique solution (or no solution at all). The function prints an error and returns early.

        Row Swapping: Swaps rows to place the pivot in the correct position.

        Normalization: Divides the pivot row by the pivot value to make the pivot 1.0.

        Elimination (Above and Below): Operations are performed to create 0.0s both below and above the pivot.

        After the loop completes, the first 5 columns of the matrix form the identity matrix, and the last column contains the solutions (x1​,x2​,…,x5​).

2. fn redondear(valor: f64, decimales: usize) -> f64

A helper function to round floating-point values to a specified number of decimal places for clean output.
