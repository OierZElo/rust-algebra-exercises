➗ Matrix Inversion using Gauss-Jordan (Rust)

This Rust module implements a function to calculate the inverse of a square matrix using the Gauss-Jordan elimination method. It operates on fixed-size 5×5 arrays ([[f64; 5]; 5]) for demonstration purposes, typical in academic projects where matrix size might be constrained.

🛠️ Implementation Details

1. fn gauss_calcular_inversa(matriz: &mut [[f64; 5]; 5]) -> [[f64; 5]; 5]

This function performs the core Gauss-Jordan elimination process on the input matrix (matriz) while simultaneously applying the same operations to an augmented identity matrix.

    Input: A mutable 5×5 array (&mut [[f64; 5]; 5]). The matrix is modified in-place.

    Output: The resulting inverse matrix as a 5×5 array.

    Methodology:

        Identity Initialization: Creates a 5×5 identity matrix.

        Pivoting: Iterates through columns, finding a non-zero pivot and swapping rows if necessary (partial pivoting).

        Invertibility Check: If a zero pivot is encountered and no row swap can fix it, the function returns the identity matrix and prints a message indicating the matrix is non-invertible.

        Normalization: Divides the pivot row by the pivot value to make the pivot 1.0.

        Elimination: Subtracts multiples of the pivot row from all other rows to make all other entries in the pivot column 0.0.

        The operations are applied identically to both the input matrix (which becomes the identity) and the identity matrix (which becomes the inverse).

2. fn redondear_matriz(matriz: &[[f64; 5]; 5], decimales: usize) -> [[f64; 5]; 5]

A utility function used to round the results of the floating-point calculations to a specified number of decimal places for cleaner output.

    Input: The matrix to round and the number of desired decimal places (usize).

    Output: The rounded matrix.
