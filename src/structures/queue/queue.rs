// Importamos los módulos necesarios
use std::{cell::RefCell, rc::Rc};

// Importamos el rasgo NodeItemTraits de otro módulo
use crate::NodeItemTraits;

// Importamos QueueNode y QueuePointer de un módulo superior
use super::{queue_node::QueueNode, QueuePointer};

/// # Estructura de Datos: Cola (Queue)
///
/// La cola (Queue) es una estructura de datos FIFO (First In, First Out), donde el primer elemento
/// en entrar es el primero en salir. Es útil en escenarios donde se necesita procesar elementos en
/// el orden en el que llegan.
///
/// La implementación de la cola en Rust utiliza punteros inteligentes (`Rc` y `RefCell`) para manejar
/// referencias compartidas y mutabilidad interior. A continuación, se describen los métodos principales
/// que proporciona esta estructura de datos:
///
/// ## Métodos
///
/// ### `new`
///
/// ```rust
/// pub fn new() -> Self
/// ```
///
/// Crea una nueva cola vacía.
///
/// ### `is_empty`
///
/// ```rust
/// pub fn is_empty(&self) -> bool
/// ```
///
/// Verifica si la cola está vacía. Retorna `true` si la cola no contiene elementos.
///
/// ### `has_one_element`
///
/// ```rust
/// pub fn has_one_element(&self) -> bool
/// ```
///
/// Verifica si la cola contiene exactamente un elemento. Retorna `true` si hay solo un elemento en la cola.
///
/// ### `enqueue`
///
/// ```rust
/// pub fn enqueue(&mut self, item: T)
/// ```
///
/// Agrega un nuevo elemento al final de la cola.
///
/// ### `dequeue`
///
/// ```rust
/// pub fn dequeue(&mut self) -> Option<T>
///
/// ```
/// Remueve y retorna el primer elemento de la cola. Retorna `None` si la cola está vacía.
///
/// ## Implementación
///
/// La estructura `Queue` está implementada de tal manera que maneja nodos (`QueueNode`) que contienen
/// el valor del elemento y un puntero al siguiente nodo en la cola.

// Definimos la estructura Queue que será una cola genérica que requiere que el tipo T implemente NodeItemTraits
#[derive(Debug)]
pub struct Queue<T: NodeItemTraits> {
    head: QueuePointer<T>,
    tail: QueuePointer<T>,
}

// Implementación de métodos para la estructura Queue
impl<T: NodeItemTraits> Queue<T> {
    // Método para crear una nueva instancia de la cola
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    // Método para verificar si la cola está vacía
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // Método para verificar si la cola tiene solo un elemento
    pub fn has_one_element(&self) -> bool {
        // si no nos aseguramos de que la cola esté vacia puede generar panics
        if self.is_empty() {
            false
        } else {
            let head = &self.head.clone().unwrap();
            let tail = &self.tail.clone().unwrap();

            // Comparamos los punteros de cabeza y cola para determinar si son el mismo nodo
            std::ptr::eq(Rc::as_ptr(head), Rc::as_ptr(tail))
        }
    }

    // Método para agregar un elemento a la cola
    pub fn enqueue(&mut self, item: T) {
        if self.is_empty() {
            // Si la cola está vacía, creamos un nuevo nodo y lo asignamos a la cabeza y la cola
            let new_node = Some(Rc::new(RefCell::new(QueueNode::new(item, None))));

            self.head = new_node.clone();
            self.tail = new_node.clone();
        } else {
            // Si la cola no está vacía, creamos un nuevo nodo y lo enlazamos al final de la cola
            let new_node = Some(Rc::new(RefCell::new(QueueNode::new(item, None))));
            self.tail.clone().unwrap().as_ref().borrow_mut().next = new_node.clone();
            self.tail = new_node.clone();
        }
    }

    // Método para eliminar y devolver un elemento de la cola
    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else if self.has_one_element() {
            // Si la cola tiene un solo elemento, lo eliminamos y dejamos la cola vacía
            let res = self.head.clone().unwrap().as_ref().borrow().item.clone();
            self.head = None;
            self.tail = None;
            Some(res)
        } else {
            // Si la cola tiene más de un elemento, eliminamos el primer nodo y ajustamos la cabeza
            let res = self.head.clone().unwrap().as_ref().borrow().item.clone();

            self.head = self.head.clone().unwrap().as_ref().borrow().next.clone();
            Some(res)
        }
    }
}

// Implementamos el rasgo Default para Queue, que simplemente crea una nueva instancia de Queue
impl<T: NodeItemTraits> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub mod tests {
    use super::Queue;

    // Test para crear una cola y llenarla con 10 elementos
    #[test]
    fn create_queue() {
        // Crear una cola de enteros usando el valor por defecto
        let mut my_queue = Queue::<i32>::default();

        // Encolar números del 0 al 9
        for item in 0..10 {
            my_queue.enqueue(item);
        }

        // Imprimir la cola para inspección visual
        println!("{:?}", my_queue)
    }

    // Test para encolar y desencolar elementos en una cola
    #[test]
    fn enqueue_and_dequeue() {
        // Crear una cola de enteros usando el valor por defecto
        let mut my_queue = Queue::<i32>::default();

        // Encolar dos elementos
        for item in 0..2 {
            my_queue.enqueue(item);
        }

        // Desencolar los elementos y verificar que se desencolan en orden correcto
        assert_eq!(Some(0), my_queue.dequeue());
        assert_eq!(Some(1), my_queue.dequeue());
        assert_eq!(None, my_queue.dequeue());

        // Volver a encolar dos elementos
        for item in 0..2 {
            my_queue.enqueue(item);
        }

        // Desencolar nuevamente y verificar el orden y que la cola esté vacía al final
        assert_eq!(Some(0), my_queue.dequeue());
        assert_eq!(Some(1), my_queue.dequeue());
        assert_eq!(None, my_queue.dequeue());
    }
}
