// Importamos las dependencias necesarias
use super::StackPointer;
use crate::NodeItemTraits;
use std::fmt::Display;

// Definimos la estructura del nodo de la pila con derivación de los traits Debug y Default
#[derive(Debug, Default)]
pub struct StackNode<T: NodeItemTraits> {
    pub item: T,                   // El valor almacenado en el nodo
    pub previous: StackPointer<T>, // Un puntero al nodo anterior en la pila
}

impl<T: NodeItemTraits> StackNode<T> {
    // Método para crear un nuevo nodo de la pila
    pub fn new(item: T, previous: StackPointer<T>) -> Self {
        Self { item, previous }
    }
}

// Implementación del trait Display para StackNode, que permite formatear la salida
impl<T: NodeItemTraits> Display for StackNode<T> {
    // Método que define cómo se debe mostrar el nodo
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.item)
    }
}
