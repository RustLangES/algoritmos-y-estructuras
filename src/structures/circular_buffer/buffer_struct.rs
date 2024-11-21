// Estructura que representa un buffer circular
#[derive(Debug)]
pub struct BufferStruct {
    buff: Vec<u8>,     // Vector que almacena los elementos del buffer
    buff_size: usize,  // Tamaño del buffer
    index_1: usize,    // Índice de escritura 1
    index_2: usize,    // Índice de escritura 2
    current_index: i8, // Índice actual de escritura (1 o -1)
}

impl BufferStruct {
    // Función para crear una nueva instancia de BufferStruct
    pub fn new(buff_size: usize) -> Self {
        // Crear un vector con capacidad inicial buff_size
        let buff = Vec::<u8>::with_capacity(buff_size);

        Self {
            buff,
            buff_size,
            index_1: 0,             // Inicializar index_1 a 0
            index_2: buff_size - 1, // Inicializar index_2 al último índice
            current_index: 1,       // Inicializar current_index a 1
        }
    }

    // Función para obtener el puntero al índice actual de escritura
    pub fn get_index_pointer(&mut self) -> &mut usize {
        match self.current_index {
            -1 => {
                // Si current_index es -1, usar index_2
                if (self.index_2 % self.buff_size) + 1 == self.index_1 {
                    self.current_index *= -1; // Cambiar current_index a 1
                    &mut self.index_1
                } else {
                    &mut self.index_2
                }
            }
            _ => {
                // Si current_index es 1, usar index_1
                if (self.index_1 % self.buff_size) + 1 == self.index_2 {
                    self.current_index *= -1; // Cambiar current_index a -1
                    &mut self.index_2
                } else {
                    &mut self.index_1
                }
            }
        }
    }

    // Función para añadir un elemento al buffer
    pub fn push_element(&mut self, element: u8) {
        if self.buff.len() < self.buff_size {
            self.buff.push(element); // Añadir elemento si el buffer no está lleno
        } else {
            let buff_size = self.buff_size;
            let index_ptr = self.get_index_pointer(); // Obtener el índice actual de escritura
            let current_index_value = *index_ptr;
            *index_ptr = (*index_ptr + 1) % buff_size; // Actualizar el índice de escritura
            self.buff[current_index_value] = element; // Reemplazar el elemento en el índice actual
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BufferStruct;

    #[test]
    fn test_buffer() {
        let mut my_buffer = BufferStruct::new(10); // Crear un buffer de tamaño 10

        for l in 0..15 {
            my_buffer.push_element(l); // Añadir 15 elementos al buffer
        }

        println!("{:?}", my_buffer.buff); // Imprimir el contenido del buffer
    }
}
