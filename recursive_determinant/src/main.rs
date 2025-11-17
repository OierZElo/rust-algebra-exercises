fn calcular_determinante(matriz: &[[f64; 5]; 5], n: usize) -> f64 {
    if n == 1 {
        return matriz[0][0];
    }


    let mut det = 0.0;
    for col in 0..n {
        // Crear la submatriz eliminando la primera fila y la columna actual
        let mut submatriz: [[f64; 5]; 5] = [[0.0; 5]; 5];
        let mut sub_n = 0;


        for i in 1..n {
            let mut sub_m = 0;
            for j in 0..n {
                if j == col {
                    continue; // Saltamos la columna actual
                }
                submatriz[sub_n][sub_m] = matriz[i][j];
                sub_m += 1;
            }
            sub_n += 1;
        }


        // Calcular el determinante con el signo alternante
        det += (-1.0f64).powi(col as i32) * matriz[0][col] * calcular_determinante(&submatriz, n - 1);
    }
    det
}


fn main() {
    // Matriz de ejemplo (5x5)
    let matriz: [[f64; 5]; 5] = [
        [1.0, 2.0, 3.0, 4.0, 5.0],
        [2.0, 4.0, 5.0, 6.0, 7.0],
        [3.0, 5.0, 8.0, 9.0, 10.0],
        [4.0, 6.0, 7.0, 11.0, 13.0],
        [5.0, 7.0, 9.0, 14.0, 16.0],
    ];


    // Calcular el determinante de la matriz
    let determinante = calcular_determinante(&matriz, 5);


    // Imprimir el determinante
    println!("Determinante de la matriz ==> {}", determinante);
}
