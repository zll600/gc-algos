pub mod algorithms;
pub mod gc_trait;
pub mod heap;
pub mod object;

pub use algorithms::MarkSweepGc;
pub use gc_trait::{Gc, GcHandle, GcStats};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_allocation() {
        let mut gc = MarkSweepGc::new();
        let handle = gc.alloc(42);
        assert_eq!(*handle.borrow(), 42);

        *handle.borrow_mut() = 100;
        assert_eq!(*handle.borrow(), 100);
    }

    #[test]
    fn test_gc_collection() {
        let mut gc = MarkSweepGc::new();

        for i in 0..150 {
            gc.alloc(i);
        }

        let stats = gc.stats();
        assert!(stats.num_collections > 0);
        assert_eq!(stats.total_allocated, 150);
    }

    #[test]
    fn test_manual_collection() {
        let mut gc = MarkSweepGc::new();

        let _h1 = gc.alloc(String::from("hello"));
        let _h2 = gc.alloc(vec![1, 2, 3]);
        let _h3 = gc.alloc(42);

        let stats_before = gc.stats();
        gc.collect();
        let stats_after = gc.stats();

        assert_eq!(
            stats_after.num_collections,
            stats_before.num_collections + 1
        );
    }

    #[test]
    fn test_handle_operations() {
        let mut gc = MarkSweepGc::new();

        let handle = gc.alloc(vec![1, 2, 3]);
        {
            let borrowed = handle.borrow();
            assert_eq!(borrowed.len(), 3);
            assert_eq!(borrowed[0], 1);
        }

        {
            let mut borrowed_mut = handle.borrow_mut();
            borrowed_mut.push(4);
            assert_eq!(borrowed_mut.len(), 4);
        }

        let borrowed = handle.borrow();
        assert_eq!(borrowed.len(), 4);
        assert_eq!(borrowed[3], 4);
    }

    #[test]
    fn test_stats_tracking() {
        let mut gc = MarkSweepGc::new();

        let initial_stats = gc.stats();
        assert_eq!(initial_stats.total_allocated, 0);
        assert_eq!(initial_stats.current_heap_size, 0);

        for i in 0..10 {
            gc.alloc(i);
        }

        let stats = gc.stats();
        assert_eq!(stats.total_allocated, 10);
        assert!(stats.current_heap_size > 0);
    }
}
