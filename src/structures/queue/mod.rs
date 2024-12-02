use std::{cell::RefCell, rc::Rc};

use queue_node::QueueNode;

pub mod queue;
pub mod queue_node;

type QueuePointer<T> = Option<Rc<RefCell<QueueNode<T>>>>;
