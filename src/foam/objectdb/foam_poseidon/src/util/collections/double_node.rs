use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct DNode<T> {
    value: T,
    next: Option<Rc<RefCell<DNode<T>>>>,
    prev: Option<Weak<RefCell<DNode<T>>>>,
}