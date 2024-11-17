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

#[cfg(test)]
mod tests {
    use super::*;

    // Implementamos NodeItemTraits para i32 directamente
    impl NodeItemTraits for i32 {}

    #[test]
    fn test_new_stack() {
        // Test para verificar que una nueva pila está vacía
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_push() {
        // Test para verificar que al empujar un elemento, la pila ya no está vacía
        let mut stack: Stack<i32> = Stack::new();
        stack.push(1);
        assert!(!stack.is_empty());
        // Verificamos que el elemento en la cima de la pila sea el correcto
        assert_eq!(stack.peek(), Some(1));
    }

    #[test]
    fn test_pop() {
        // Test para verificar que los elementos se sacan en el orden correcto (LIFO)
        let mut stack: Stack<i32> = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        // Verificamos que la pila esté vacía después de sacar todos los elementos
        assert!(stack.is_empty());
    }

    #[test]
    fn test_peek() {
        // Test para verificar que peek devuelve el elemento correcto sin eliminarlo
        let mut stack: Stack<i32> = Stack::new();
        assert_eq!(stack.peek(), None); // La pila está vacía
        stack.push(1);
        assert_eq!(stack.peek(), Some(1)); // Cima es 1
        stack.push(2);
        assert_eq!(stack.peek(), Some(2)); // Cima es 2
        stack.pop();
        assert_eq!(stack.peek(), Some(1)); // Cima es 1 después de pop
    }

    #[test]
    fn test_is_empty() {
        // Test para verificar que is_empty funciona correctamente
        let mut stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        stack.push(1);
        assert!(!stack.is_empty());
        stack.pop();
        assert!(stack.is_empty());
    }
}
