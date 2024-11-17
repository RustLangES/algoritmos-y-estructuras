use stack_node::StackNode;

pub mod stack;
pub mod stack_node;
type StackPointer<T> = Option<Box<StackNode<T>>>;
