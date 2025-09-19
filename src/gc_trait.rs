use std::rc::Rc;

pub trait Gc {
    fn new() -> Self where Self: Sized;

    fn alloc<T: 'static>(&mut self, value: T) -> GcHandle<T>;

    fn collect(&mut self);

    fn stats(&self) -> GcStats;
}

#[derive(Clone)]
pub struct GcHandle<T> {
    pub(crate) id: usize,
    pub(crate) data: Rc<std::cell::RefCell<T>>,
}

impl<T> GcHandle<T> {
    pub fn borrow(&self) -> std::cell::Ref<'_, T> {
        self.data.borrow()
    }

    pub fn borrow_mut(&self) -> std::cell::RefMut<'_, T> {
        self.data.borrow_mut()
    }
}

#[derive(Debug, Clone, Default)]
pub struct GcStats {
    pub total_allocated: usize,
    pub total_freed: usize,
    pub current_heap_size: usize,
    pub num_collections: usize,
}