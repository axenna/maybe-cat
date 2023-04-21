use std::cell::RefCell;
use std::rc::Rc;

//multiple mutable refs using interior mutability
pub type Share<T> = Rc<RefCell<T>>;

//above but in a vector
pub type ShareVec<T> = Vec<Share<T>>;
