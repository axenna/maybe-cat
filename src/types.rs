use std::cell::RefCell;
use std::rc::Rc;

pub type Share<T> = Rc<RefCell<T>>;
