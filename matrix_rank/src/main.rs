fn gauss_calcular_rango(matriz: &mut [[f64; 5]; 5]) -> usize {
    let filas = matriz.len();
    let columnas = matriz[0].len();
    let mut rango = 0;


    for col in 0..columnas {
        let mut fila_pivote = filas;  // filas es un valor fuera del rango válido


        for fila in rango..filas {
            if matriz[fila][col] != 0.0 {
                fila_pivote = fila;
                break;
            }
        }


        // Si no se encontró un pivote en esta columna, continuar con la siguiente
        if fila_pivote == filas {
            continue;
        }


        // Intercambiar filas si es necesario
        if fila_pivote != rango {
            matriz.swap(fila_pivote, rango);
        }


        // Normalizar el pivote
        let pivote = matriz[rango][col];
        for j in 0..columnas {
            matriz[rango][j] /= pivote;
        }


        // Hacer 0s debajo del pivote
        for fila in (rango + 1)..filas {
            let factor = matriz[fila][col];
            for j in 0..columnas {
                matriz[fila][j] -= factor * matriz[rango][j];
            }
        }


        // Incrementar el rango
        rango += 1;
    }
    rango
}


fn main() {
    // Matriz de ejemplo
    let mut matriz: [[f64; 5]; 5] = [
        [1.0, 2.0, 3.0, 4.0, 5.0],
        [6.0, 7.0, 8.0, 9.0, 10.0],
        [11.0, 12.0, 13.0, 14.0, 15.0],
        [16.0, 17.0, 18.0, 19.0, 20.0],
        [21.0, 23.0, 25.0, 28.0, 30.0],
    ];

    // Calcular el rango
    let rango = gauss_calcular_rango(&mut matriz);


    println!("Matriz transformada:");
    for fila in &matriz {
        println!("{:?}", fila);
    }

    // Imprimir el rango
    println!("========================================================");
    println!("El rango de la matriz es: {}", rango);
}
