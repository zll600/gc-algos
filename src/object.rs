use std::any::Any;
use std::cell::RefCell;
use std::rc::Weak;

pub struct GcObject {
    pub id: usize,
    pub marked: RefCell<bool>,
    pub data: Box<dyn Any>,
    pub references: RefCell<Vec<Weak<GcObject>>>,
}

impl GcObject {
    pub fn new(id: usize, data: Box<dyn Any>) -> Self {
        GcObject {
            id,
            marked: RefCell::new(false),
            data,
            references: RefCell::new(Vec::new()),
        }
    }

    pub fn mark(&self) {
        *self.marked.borrow_mut() = true;
    }

    pub fn unmark(&self) {
        *self.marked.borrow_mut() = false;
    }

    pub fn is_marked(&self) -> bool {
        *self.marked.borrow()
    }

    pub fn add_reference(&self, obj: Weak<GcObject>) {
        self.references.borrow_mut().push(obj);
    }
}
