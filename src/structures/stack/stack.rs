// Importamos las traits y estructuras necesarias
use super::{stack_node::StackNode, StackPointer};
use crate::NodeItemTraits;

// Definimos la estructura de la pila con un atributo top que apunta a la cima de la pila
#[derive(Default)]
pub struct Stack<T: NodeItemTraits> {
    top: StackPointer<T>,
}

impl<T: NodeItemTraits> Stack<T> {
    // Método para crear una nueva pila
    pub fn new() -> Self {
        Stack::default()
    }

    // Método para empujar (agregar) un elemento a la pila
    pub fn push(&mut self, item: T) {
        // Creamos un nuevo nodo y lo colocamos en la cima de la pila
        let new_node = Box::new(StackNode::new(item, self.top.take()));
        self.top = Some(new_node)
    }

    // Método para sacar (eliminar) un elemento de la cima de la pila
    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.top.take() {
            // Actualizamos la cima de la pila al nodo siguiente
            self.top = node.previous;
            Some(node.item)
        } else {
            None
        }
    }

    // Método para verificar si la pila está vacía
    pub fn is_empty(&self) -> bool {
        self.top.is_none()
    }

    // Método que permite ver el elemento en la cima de la pila sin eliminarlo
    pub fn peek(&self) -> Option<T> {
        // Utilizamos el método `as_ref` para obtener una referencia a la `Option<Box<StackNode<T>>>`
        // y `map` para transformar esa referencia en `Option<T>` clonando el elemento `item` del nodo
        self.top.as_ref().map(|node| node.item.clone())
    }
}
