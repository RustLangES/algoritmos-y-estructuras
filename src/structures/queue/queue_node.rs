use crate::NodeItemTraits;

use super::QueuePointer;

// Derive el rasgo Debug para poder formatear la estructura para depuración
#[derive(Debug)]
pub struct QueueNode<T: NodeItemTraits> {
    pub item: T,               // El valor almacenado en el nodo
    pub next: QueuePointer<T>, // Un puntero al siguiente nodo en la cola
}

// Implementación de métodos para la estructura QueueNode
impl<T: NodeItemTraits> QueueNode<T> {
    // Constructor para crear un nuevo nodo de la cola
    pub fn new(item: T, next: QueuePointer<T>) -> Self {
        Self { item, next }
    }
}
