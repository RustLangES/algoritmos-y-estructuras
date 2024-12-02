// Define una constante que representa la longitud del buffer
const LEN: usize = 50;
// Declara un buffer estático mutable con tamaño LEN inicializado a ceros
static mut BUFFER_SLICE: [i64; LEN] = [0; LEN];

// Función para escribir un valor en el buffer en un índice específico
pub fn write_on_buff_slice(item: i64, index: usize) {
    unsafe {
        BUFFER_SLICE[index] = item; // Escribe el valor en el índice dado de forma insegura
    }
}

// Función que determina si se debe cambiar de índice
pub fn should_change_indexes(current_index: usize, index_b: usize) -> bool {
    (current_index % LEN) + 1 == index_b // Comprueba si el índice actual más uno es igual a index_b
}

// Función que ejecuta el algoritmo principal
pub fn run(data: Vec<i64>) {
    let mut index_1: usize = 0; // Inicializa el primer índice
    let mut index_2: usize = 0; // Inicializa el segundo índice

    let mut current_index: bool = true; // true -> index_1, false -> index_2

    for item in data {
        // Determina cuál índice se está utilizando actualmente
        let working_index = match current_index {
            true => {
                // Si current_index es true, se usa index_1
                if should_change_indexes(index_1, index_2) {
                    current_index = false; // Cambia al otro índice
                    &mut index_2
                } else {
                    &mut index_1 // Mantiene el índice actual
                }
            }
            false => {
                // Si current_index es false, se usa index_2
                if should_change_indexes(index_2, index_1) {
                    current_index = true; // Cambia al otro índice
                    &mut index_2
                } else {
                    &mut index_1 // Mantiene el índice actual
                }
            }
        };

        unsafe {
            BUFFER_SLICE[*working_index] = item; // Escribe el valor en el índice de trabajo de forma insegura
        }

        *working_index = (*working_index + 1) % LEN; // Actualiza el índice de trabajo
    }
}

#[cfg(test)]
pub mod tests {
    use super::{run, write_on_buff_slice};
    use crate::structures::circular_buffer::buffer_static_slice::BUFFER_SLICE;

    #[test]
    fn test_write_on_buffer() {
        write_on_buff_slice(10, 0); // Escribe el valor 10 en el índice 0

        unsafe {
            assert_eq!(BUFFER_SLICE[0], 10); // Verifica que el valor en el índice 0 sea 10
        }
    }

    #[test]
    fn test_200_elements() {
        let mut v: Vec<i64> = vec![];

        for l in 0..230 {
            v.push(l); // Llena el vector con valores de 0 a 229
        }

        run(v); // Ejecuta la función principal con el vector

        // Este código es extremadamente inseguro, genera comportamiento inesperado si no se administra correctamente
        unsafe { println!("Buffer: {:?}", BUFFER_SLICE) }; // Imprime el estado final del buffer
    }
}
