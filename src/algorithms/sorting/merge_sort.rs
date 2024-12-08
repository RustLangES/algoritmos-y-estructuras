/// # Merge Sort
///
/// La función `merge` implementa el algoritmo de Merge Sort para ordenar un vector de enteros en Rust.
///
/// ## Funcionamiento
///
/// Merge Sort es un algoritmo de ordenamiento que sigue el paradigma de divide y vencerás. El algoritmo divide el array en subarrays más pequeños hasta que cada subarray tiene un solo elemento, y luego fusiona esos subarrays de una manera ordenada para producir el array final ordenado.
///
/// ## Implementación
///
/// Esta implementación utiliza dos vectores (`arr` y `arr_2`) para alternar entre la lectura y la escritura durante las fusiones. Aquí están los pasos detallados:
///
/// 1. **Inicialización**: Se inicializan variables como el vector auxiliar `arr_2`, y un indicador `using_flag` para alternar entre los vectores.
/// 2. **Ciclo Principal**: Mientras el tamaño de los elementos a fusionar sea menor que la longitud del array, se ejecuta un ciclo para fusionar subarrays:
///     - Se alterna entre leer y escribir en `arr` y `arr_2` utilizando `using_flag`.
///     - Se divide el array en bloques de tamaño `elements` y se fusionan dos bloques adyacentes.
///     - La fusión se realiza comparando elementos de los dos bloques y escribiéndolos en el vector de escritura.
/// 3. **Incremento del Tamaño de Bloque**: Después de cada paso de fusión, se duplica el tamaño de los bloques a fusionar.
/// 4. **Resultado Final**: Al terminar el ciclo, se retorna el vector ordenado.
///
/// ## Ejemplo de Uso
///
/// ```rust
/// pub fn merge(mut arr: Vec<i32>) -> Vec<i32> {
///     let arr_len = arr.len();
///     let mut arr_2: Vec<i32> = vec![0; arr_len];
///     let mut reading: &Vec<i32>;
///     let mut writing: &mut Vec<i32>;
///     let mut using_flag: i8 = -1;
///     let mut elements = 1;
///
///     while elements < arr_len {
///         using_flag *= -1;
///         if using_flag == -1 {
///             writing = &mut arr;
///             reading = &mut arr_2;
///         } else {
///             writing = &mut arr_2;
///             reading = &mut arr;
///         }
///
///         let mut pos = 0;
///
///         while pos < arr_len {
///             let mut index_1 = pos;
///             let mut index_2 = pos + elements;
///             let slice_1 = &reading[index_1..index_2];
///             let slice_2 = if (index_2 + elements) > arr_len {
///                 &reading[index_2..arr_len]
///             } else {
///                 &reading[index_2..(index_2 + elements)]
///             };
///             index_1 = 0;
///             index_2 = 0;
///
///             for _ in 0..(elements * 2) {
///                 match (slice_1.get(index_1), slice_2.get(index_2)) {
///                     (Some(value1), Some(value2)) => {
///                         if value1 < value2 {
///                             writing[pos + index_1 + index_2] = *value1;
///                             index_1 += 1;
///                         } else {
///                             writing[pos + index_1 + index_2] = *value2;
///                             index_2 += 1
///                         }
///                     }
///                     (Some(value1), None) => {
///                         writing[pos + index_1 + index_2] = *value1;
///                         index_1 += 1;
///                     }
///                     (None, Some(value2)) => {
///                         writing[pos + index_1 + index_2] = *value2;
///                         index_2 += 1;
///                     }
///                     (None, None) => {
///                         break;
///                     }
///                 }
///             }
///
///             pos += elements * 2;
///         }
///
///         elements *= 2;
///     }
///
///     if using_flag == -1 {
///         arr
///     } else {
///         arr_2
///     }
/// }
/// ```
///
/// Esta función se utiliza en pruebas para validar su correcto funcionamiento:
///
/// ```rust
/// #[cfg(test)]
/// mod tests {
///     use crate::algorithms::sorting::merge_sort::merge;
///
///     #[test]
///     fn test_sorting() {
///         let vec_1 = vec![6, 2, 3, 9, 1, 0, 5, 8, 7, 1, 0, 4, 3, 1];
///         assert_eq!(vec![0, 0, 1, 1, 1, 2, 3, 3, 4, 5, 6, 7, 8, 9], merge(vec_1));
///     }
/// }
/// ```

// Define la función merge que toma un vector de enteros como parámetro y devuelve un vector de enteros
pub fn merge(mut arr: Vec<i32>) -> Vec<i32> {
    // Obtener la longitud del array
    let arr_len = arr.len();
    // Crear un vector auxiliar arr_2 del mismo tamaño que arr, inicializado con ceros
    let mut arr_2: Vec<i32> = vec![0; arr_len];
    // Declarar punteros para leer y escribir en los vectores
    let mut reading: &Vec<i32>;
    let mut writing: &mut Vec<i32>;
    // Flag que indica qué vector se usa para leer y cuál para escribir
    let mut using_flag: i8 = -1;
    // Número de elementos a ser fusionados, empieza en 1
    let mut elements = 1;

    // Mientras el número de elementos a fusionar sea menor que la longitud del array
    while elements < arr_len {
        // Alternar el valor de using_flag entre -1 y 1
        using_flag *= -1;
        if using_flag == -1 {
            // Si using_flag es -1, escribir en arr y leer de arr_2
            writing = &mut arr;
            reading = &mut arr_2;
        } else {
            // Si using_flag es 1, escribir en arr_2 y leer de arr
            writing = &mut arr_2;
            reading = &mut arr;
        }

        // Inicializar la posición a 0
        let mut pos = 0;

        // Mientras la posición sea menor que la longitud del array
        while pos < arr_len {
            // Índices para fusionar dos mitades
            let mut index_1 = pos;
            let mut index_2 = pos + elements;
            // Obtener los slices de los subarrays a fusionar
            let slice_1 = &reading[index_1..index_2];
            let slice_2 = if (index_2 + elements) > arr_len {
                &reading[index_2..arr_len]
            } else {
                &reading[index_2..(index_2 + elements)]
            };
            // Reinicializar los índices de los slices
            index_1 = 0;
            index_2 = 0;

            // Fusionar los dos slices en el array de escritura
            for _ in 0..(elements * 2) {
                match (slice_1.get(index_1), slice_2.get(index_2)) {
                    (Some(value1), Some(value2)) => {
                        if value1 < value2 {
                            writing[pos + index_1 + index_2] = *value1;
                            index_1 += 1;
                        } else {
                            writing[pos + index_1 + index_2] = *value2;
                            index_2 += 1
                        }
                    }
                    (Some(value1), None) => {
                        writing[pos + index_1 + index_2] = *value1;
                        index_1 += 1;
                    }
                    (None, Some(value2)) => {
                        writing[pos + index_1 + index_2] = *value2;
                        index_2 += 1;
                    }
                    (None, None) => {
                        break;
                    }
                }
            }

            // Avanzar la posición al siguiente bloque de elementos
            pos += elements * 2;
        }

        // Duplicar el tamaño de los elementos a fusionar
        elements *= 2;
    }

    // Devolver el array ordenado
    if using_flag == -1 {
        arr
    } else {
        arr_2
    }
}

#[cfg(test)]
mod tests {
    // Importar la función merge
    use crate::algorithms::sorting::merge_sort::merge;

    #[test]
    fn test_sorting() {
        // Definir un vector de prueba
        let vec_1 = vec![6, 2, 3, 9, 1, 0, 5, 8, 7, 1, 0, 4, 3, 1];
        // Imprimir el vector ordenado usando la función merge
        assert_eq!(vec![0, 0, 1, 1, 1, 2, 3, 3, 4, 5, 6, 7, 8, 9], merge(vec_1));
    }
}
