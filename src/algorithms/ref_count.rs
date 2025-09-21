use crate::gc_trait::{Gc, GcHandle, GcStats};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

struct RefCountNode {
    strong_count: Cell<usize>,
}

impl RefCountNode {
    fn new() -> Self {
        RefCountNode {
            strong_count: Cell::new(1),
        }
    }

    fn is_alive(&self) -> bool {
        self.strong_count.get() > 0
    }
}

pub struct RefCountGc {
    stats: RefCell<GcStats>,
    nodes: RefCell<Vec<Rc<RefCountNode>>>,
}

impl RefCountGc {
    fn track_allocation(&self) {
        let mut stats = self.stats.borrow_mut();
        stats.total_allocated += 1;
        stats.current_heap_size += 1;
    }

    fn cleanup_deallocated(&self) {
        let before_count = self.nodes.borrow().len();

        self.nodes.borrow_mut().retain(|node| node.is_alive());

        let after_count = self.nodes.borrow().len();
        let freed = before_count - after_count;

        if freed > 0 {
            let mut stats = self.stats.borrow_mut();
            stats.total_freed += freed;
            stats.current_heap_size = after_count;
        }
    }
}

impl Gc for RefCountGc {
    fn new() -> Self {
        RefCountGc {
            stats: RefCell::new(GcStats::default()),
            nodes: RefCell::new(Vec::new()),
        }
    }

    fn alloc<T: 'static>(&mut self, value: T) -> GcHandle<T> {
        let data = Rc::new(RefCell::new(value));
        let node = Rc::new(RefCountNode::new());

        self.nodes.borrow_mut().push(node);
        self.track_allocation();

        GcHandle { data }
    }

    fn collect(&mut self) {
        self.cleanup_deallocated();

        let mut stats = self.stats.borrow_mut();
        stats.num_collections += 1;
    }

    fn stats(&self) -> GcStats {
        self.stats.borrow().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ref_count_node() {
        let node = RefCountNode::new();
        assert!(node.is_alive());
        assert_eq!(node.strong_count.get(), 1);
    }

    #[test]
    fn test_allocation() {
        let mut gc = RefCountGc::new();
        let handle1 = gc.alloc(42);
        let handle2 = gc.alloc(String::from("hello"));

        assert_eq!(*handle1.borrow(), 42);
        assert_eq!(*handle2.borrow(), String::from("hello"));

        let stats = gc.stats();
        assert_eq!(stats.total_allocated, 2);
        assert_eq!(stats.current_heap_size, 2);
    }

    #[test]
    fn test_modification() {
        let mut gc = RefCountGc::new();
        let handle = gc.alloc(vec![1, 2, 3]);

        {
            let mut borrowed = handle.borrow_mut();
            borrowed.push(4);
        }

        assert_eq!(handle.borrow().len(), 4);
        assert_eq!(handle.borrow()[3], 4);
    }

    #[test]
    fn test_collection() {
        let mut gc = RefCountGc::new();

        for i in 0..5 {
            gc.alloc(i);
        }

        let stats_before = gc.stats();
        assert_eq!(stats_before.total_allocated, 5);

        gc.collect();

        let stats_after = gc.stats();
        assert_eq!(stats_after.num_collections, 1);
        assert_eq!(stats_after.total_allocated, 5);
    }

    #[test]
    fn test_multiple_references() {
        let mut gc = RefCountGc::new();

        let handle1 = gc.alloc(100);
        let handle2 = handle1.clone();
        let handle3 = handle1.clone();

        assert_eq!(*handle1.borrow(), 100);
        assert_eq!(*handle2.borrow(), 100);
        assert_eq!(*handle3.borrow(), 100);

        *handle1.borrow_mut() = 200;
        assert_eq!(*handle2.borrow(), 200);
        assert_eq!(*handle3.borrow(), 200);
    }

    #[test]
    fn test_drop_behavior() {
        let mut gc = RefCountGc::new();

        {
            let _handle = gc.alloc(vec![1, 2, 3, 4, 5]);
            let stats = gc.stats();
            assert_eq!(stats.total_allocated, 1);
            assert_eq!(stats.current_heap_size, 1);
        }

        gc.collect();
        let stats = gc.stats();
        assert_eq!(stats.total_allocated, 1);
    }
}
