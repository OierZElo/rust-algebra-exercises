fn gauss_resolver_sistema(matriz: &mut [[f64; 6]; 5]) -> &mut [[f64; 6]; 5] {
    let filas = matriz.len();
    let columnas = matriz[0].len();

    for col in 0..filas {
        let mut fila_pivote = col;  // Inicializamos la fila pivote

        // Encontrar la fila con el pivote
        for fila in col..filas {
            if matriz[fila][col] != 0.0 {
                fila_pivote = fila;
                break;
            }
        }

        // Si no hay un pivote en la columna, la matriz no es invertible
        if matriz[fila_pivote][col] == 0.0 {
            println!("No existe solucion");
            return matriz;  // No se puede resolver el sistema
        }

        // Cambiar filas si es necesario
        if fila_pivote != col {
            matriz.swap(fila_pivote, col);
        }

        // Normalizar el pivote
        let pivote = matriz[col][col];
        for j in col..columnas {
            matriz[col][j] /= pivote;
        }

        // Hacer ceros tanto por debajo como por encima del pivote
        for fila in 0..filas {
            if fila != col {
                let factor = matriz[fila][col];
                for columna in col..columnas {
                    matriz[fila][columna] -= factor * matriz[col][columna];
                }
            }
        }
    }

    // Las últimas columnas contienen las soluciones
    matriz
}
fn redondear(valor: f64, decimales: usize) -> f64 {
    let factor_redondeo = 10f64.powi(decimales as i32);
    (valor * factor_redondeo).round() / factor_redondeo
}

fn main() {
    // Matriz de ejemplo: la última columna es el vector de constantes
    let mut matriz: [[f64; 6]; 5] = [
        [0.0, 0.0, 1.0, -1.0, 2.0, 3.0],
        [1.0, 1.0, 0.0, 1.0, -1.0, 1.0],
        [0.0, 2.0, 3.0, 0.0, 1.0, 5.0],
        [0.0, -1.0, 2.0, 1.0, 0.0, 2.0],
        [0.0, 1.0, -1.0, 1.0, 1.0, 0.0],
    ];

    let matriz_resuelta = gauss_resolver_sistema(&mut matriz);

    let ultima = matriz_resuelta.len();
    // Imprimir las soluciones del sistema, redondeadas solo al final
    println!("Soluciones del sistema (redondeadas a 3 decimales):");
    for fila in matriz_resuelta {
        println!("{:.3}", redondear(fila[ultima], 3));//Imprimir última columna redondeada
    }
}
