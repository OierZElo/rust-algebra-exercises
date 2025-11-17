➗ Recursive Determinant Calculation (Rust)

This Rust module implements a function to calculate the determinant of a square matrix using the recursive cofactor expansion method (also known as Laplace expansion).

The current implementation uses fixed-size arrays ([[f64; 5]; 5]) and is optimized for demonstration, particularly for matrices where the size is passed as an argument (n).

🛠️ Implementation Details

1. fn calcular_determinante(matriz: &[[f64; 5]; 5], n: usize) -> f64

This function recursively calculates the determinant by expanding across the first row.

    Input:

        matriz: The square matrix (as a fixed 5×5 array of f64).

        n: The actual size of the submatrix being processed (e.g., 5 for the initial call, 4 for the first recursive step).

    Output: The determinant value (f64).

    Methodology (Cofactor Expansion):

        Base Case: If the matrix size n is 1, the determinant is simply the single element matriz[0][0].

        Recursion: The function iterates through the columns (col) of the current matrix (usually the first row).

        Submatrix Creation: For each element matriz[0][col], a submatrix is constructed by excluding the first row (0) and the current column (col).

        Alternating Sign: The cofactor term is calculated with an alternating sign, (−1)col.

        Accumulation: The total determinant (det) is the sum of:
        det+=(−1)col⋅matriz[0][col]⋅calcular_determinante(submatriz,n−1)
