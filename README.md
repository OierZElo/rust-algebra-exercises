➗ Linear Algebra in Rust: Core Functions

This repository contains the implementation of four fundamental linear algebra functions written in the Rust programming language. This project was developed for the Algebra course at the University of Deusto.

The primary goal is to translate complex mathematical concepts, such as matrix rank and the solution of linear systems, into robust and efficient algorithms using Rust.

🚀 Implemented Functionalities

The project focuses on programming the following four functions, which are essential in the study of linear algebra:

1. inverse_matrix

    Description: Calculates the inverse matrix (A−1) of a square matrix A.

    Methodology: Typically uses the Gauss-Jordan elimination method applied to the augmented matrix [A∣I].

2. matrix_rank

    Description: Computes the rank of a matrix (the number of linearly independent rows or columns).

    Methodology: Implements Gaussian elimination to reduce the matrix to its row echelon form and counts the number of pivots.

3. recursive_determinant

    Description: Calculates the determinant of a square matrix.

    Methodology: Employs a recursive approach based on cofactor expansion (or Laplace's method).

4. solve_equations_gauss_jordan

    Description: Solves a system of linear equations Ax=b.

    Methodology: Implements the Gauss-Jordan elimination algorithm to bring the augmented matrix [A∣b] to its reduced row echelon form and find the solutions.

🛠️ Project Requirements

Language

    Rust

🎓 University of Deusto

This project forms part of the practical coursework for the Algebra module at the School of Engineering.
