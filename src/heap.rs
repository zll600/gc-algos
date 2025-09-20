use crate::object::GcObject;
use std::rc::Rc;

pub struct Heap {
    objects: Vec<Rc<GcObject>>,
    next_id: usize,
    threshold: usize,
}

impl Heap {
    pub fn new() -> Self {
        Heap {
            objects: Vec::new(),
            next_id: 0,
            threshold: 100,
        }
    }

    pub fn allocate(&mut self, obj: Box<dyn std::any::Any>) -> Rc<GcObject> {
        let id = self.next_id;
        self.next_id += 1;

        let gc_obj = Rc::new(GcObject::new(id, obj));
        self.objects.push(gc_obj.clone());

        gc_obj
    }

    pub fn should_collect(&self) -> bool {
        self.objects.len() >= self.threshold
    }

    pub fn sweep(&mut self) {
        self.objects.retain(|obj| {
            if obj.is_marked() {
                obj.unmark();
                true
            } else {
                false
            }
        });
    }

    pub fn roots(&self) -> Vec<Rc<GcObject>> {
        self.objects
            .iter()
            .filter(|obj| Rc::strong_count(obj) > 1)
            .cloned()
            .collect()
    }

    pub fn size(&self) -> usize {
        self.objects.len()
    }
}
