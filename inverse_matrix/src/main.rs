fn gauss_calcular_inversa(matriz: &mut [[f64; 5]; 5]) -> [[f64; 5]; 5] {
    let filas = matriz.len();
    let columnas = matriz[0].len();
    let mut matriz_identidad = [[0.0; 5]; 5];


    // Generar la matriz identidad
    for fila in 0..filas {
        matriz_identidad[fila][fila] = 1.0;
    }


    for col in 0..filas {
        let mut fila_pivote = filas;  // filas es un valor fuera del rango válido


        // Encontrar la fila con el pivote
        for fila in col..filas {
            if matriz[fila][col] != 0.0 {
                fila_pivote = fila;
                break;
            }
        }


        // Si no hay un pivote en la columna, la matriz no es invertible
        if fila_pivote == filas {
            println!("La matriz no es invertible");
            return matriz_identidad;  // Devolver la matriz identidad en lugar de Option
        }


        // Cambiar filas si es necesario
        if fila_pivote != col {
            matriz.swap(fila_pivote, col);
            matriz_identidad.swap(fila_pivote, col);
        }


        // Normalizar el pivote
        let pivote = matriz[col][col];
        if pivote == 0.0 {
            println!("La matriz no es invertible");
            return matriz_identidad;  // Devolver la matriz identidad
        }


        for j in 0..columnas {
            matriz[col][j] /= pivote;
            matriz_identidad[col][j] /= pivote;
        }


        // Hacer ceros tanto por debajo como por encima del pivote
        for fila in 0..filas {
            if fila != col {
                let factor = matriz[fila][col];
                for columna in 0..columnas {
                    matriz[fila][columna] -= factor * matriz[col][columna];
                    matriz_identidad[fila][columna] -= factor * matriz_identidad[col][columna];
                }
            }
        }
    }
    matriz_identidad
}


fn redondear_matriz(matriz: &[[f64; 5]; 5], decimales: usize) -> [[f64; 5]; 5] {
    let mut matriz_redondeada = [[0.0; 5]; 5];
    let factor_redondeo = 10f64.powi(decimales as i32);


    for fila in 0..matriz.len() {
        for columna in 0..matriz[0].len() {
            matriz_redondeada[fila][columna] = (matriz[fila][columna] * factor_redondeo).round() / factor_redondeo;
        }
    }
    matriz_redondeada
}

fn main() {
    // Matriz de ejemplo
    let mut matriz: [[f64; 5]; 5] = [
        [1.0, 0.0, 1.0, -1.0, 2.0],
        [2.0, 1.0, 0.0, 1.0, -1.0],
        [-1.0, 2.0, 1.0, 0.0, 1.0],
        [3.0, -1.0, 2.0, 1.0, 0.0],
        [1.0, 1.0, -1.0, 1.0, 1.0],
    ];

    let matriz_inversa = gauss_calcular_inversa(&mut matriz);
    let matriz_inversa_redondeada = redondear_matriz(&matriz_inversa, 3);


    println!("Matriz inversa redondeada a 3 decimales:");
    for fila in &matriz_inversa_redondeada {
        println!("{:?}", fila);
    }
}
