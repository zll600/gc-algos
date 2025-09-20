use crate::gc_trait::{Gc, GcHandle, GcStats};
use crate::heap::Heap;
use crate::object::GcObject;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct MarkSweepGc {
    heap: Heap,
    roots: Vec<Rc<GcObject>>,
    objects: Vec<Rc<GcObject>>,
    stats: GcStats,
}

impl MarkSweepGc {
    fn mark(&mut self) {
        let mut work_list = VecDeque::new();

        for root in &self.roots {
            if !root.is_marked() {
                root.mark();
                work_list.push_back(root.clone());
            }
        }

        for obj in &self.objects {
            if Rc::strong_count(obj) > 1 && !obj.is_marked() {
                obj.mark();
                work_list.push_back(obj.clone());
            }
        }

        while let Some(obj) = work_list.pop_front() {
            let refs = obj.references.borrow();
            for weak_ref in refs.iter() {
                if let Some(referenced) = weak_ref.upgrade() {
                    if !referenced.is_marked() {
                        referenced.mark();
                        work_list.push_back(referenced);
                    }
                }
            }
        }
    }

    fn sweep(&mut self) {
        let before_size = self.objects.len();

        self.objects.retain(|obj| {
            if obj.is_marked() {
                obj.unmark();
                true
            } else {
                false
            }
        });

        self.heap.sweep();

        let freed = before_size - self.objects.len();
        self.stats.total_freed += freed;
        self.stats.current_heap_size = self.objects.len();
    }
}

impl Gc for MarkSweepGc {
    fn new() -> Self {
        MarkSweepGc {
            heap: Heap::new(),
            roots: Vec::new(),
            objects: Vec::new(),
            stats: GcStats::default(),
        }
    }

    fn alloc<T: 'static>(&mut self, value: T) -> GcHandle<T> {
        let data = Rc::new(RefCell::new(value));
        let boxed: Box<dyn std::any::Any> = Box::new(data.clone());
        let obj = self.heap.allocate(boxed);

        self.objects.push(obj.clone());

        self.stats.total_allocated += 1;
        self.stats.current_heap_size += 1;

        if self.heap.should_collect() {
            self.collect();
        }

        GcHandle { data }
    }

    fn collect(&mut self) {
        self.mark();
        self.sweep();
        self.stats.num_collections += 1;
    }

    fn stats(&self) -> GcStats {
        self.stats.clone()
    }
}
