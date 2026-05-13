use std::collections::VecDeque;

#[derive(Debug)]
pub(crate) struct Ringbuffer<T> {
    inner: VecDeque<T>,
    capacity: usize,
}

impl<T> Ringbuffer<T> {
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            inner: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub(crate) fn push(&mut self, item: T) {
        if self.capacity == 0 {
            return;
        }

        if self.inner.len() >= self.capacity {
            self.inner.pop_front();
        }

        self.inner.push_back(item);
    }

    pub(crate) fn extend(&mut self, items: impl IntoIterator<Item = T>) {
        for item in items {
            self.push(item);
        }
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }

    pub(crate) fn len(&self) -> usize {
        self.inner.len()
    }

    #[allow(dead_code)] // Nice-to-have, remove when used.
    pub(crate) fn is_full(&self) -> bool {
        self.inner.len() >= self.capacity
    }

    pub(crate) fn drain(&mut self) -> Vec<T> {
        self.inner.drain(..).collect()
    }

    pub(crate) fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod push {
        use super::*;

        #[test]
        fn should_add_item_to_buffer() {
            let mut ring = Ringbuffer::new(3);

            ring.push(1);

            let items: Vec<_> = ring.iter().copied().collect();
            assert_eq!(items, vec![1]);
        }

        #[test]
        fn should_evict_oldest_when_at_capacity() {
            let mut ring = Ringbuffer::new(3);
            ring.push(1);
            ring.push(2);
            ring.push(3);

            ring.push(4);

            let items: Vec<_> = ring.iter().copied().collect();
            assert_eq!(items, vec![2, 3, 4]);
        }

        #[test]
        fn should_noop_when_capacity_is_zero() {
            let mut ring = Ringbuffer::new(0);

            ring.push(1);

            let items: Vec<_> = ring.iter().copied().collect();
            assert!(items.is_empty());
        }
    }

    mod extend {
        use super::*;

        #[test]
        fn should_add_all_items() {
            let mut ring = Ringbuffer::new(5);

            ring.extend(vec![1, 2, 3]);

            let items: Vec<_> = ring.iter().copied().collect();
            assert_eq!(items, vec![1, 2, 3]);
        }

        #[test]
        fn should_evict_oldest_when_exceeds_capacity() {
            let mut ring = Ringbuffer::new(3);

            ring.extend(vec![1, 2, 3, 4, 5]);

            let items: Vec<_> = ring.iter().copied().collect();
            assert_eq!(items, vec![3, 4, 5]);
        }
    }

    mod iter {
        use super::*;

        #[test]
        fn should_iterate_in_insertion_order() {
            let mut ring = Ringbuffer::new(5);
            ring.push(1);
            ring.push(2);
            ring.push(3);

            let items: Vec<_> = ring.iter().copied().collect();

            assert_eq!(items, vec![1, 2, 3]);
        }

        #[test]
        fn should_return_empty_iterator_for_empty_buffer() {
            let ring: Ringbuffer<i32> = Ringbuffer::new(5);

            let items: Vec<_> = ring.iter().copied().collect();

            assert!(items.is_empty());
        }
    }

    mod len {
        use super::*;

        #[test]
        fn should_return_zero_for_empty_buffer() {
            let ring: Ringbuffer<i32> = Ringbuffer::new(5);
            assert_eq!(ring.len(), 0);
        }

        #[test]
        fn should_return_actual_count_not_capacity() {
            let mut ring = Ringbuffer::new(10);
            ring.push(1);
            ring.push(2);
            ring.push(3);

            assert_eq!(ring.len(), 3);
        }

        #[test]
        fn should_not_exceed_capacity_after_eviction() {
            let mut ring = Ringbuffer::new(3);
            ring.extend(vec![1, 2, 3, 4, 5]);

            assert_eq!(ring.len(), 3);
        }
    }

    mod is_full {
        use super::*;

        #[test]
        fn should_return_false_when_empty() {
            let ring: Ringbuffer<i32> = Ringbuffer::new(5);
            assert!(!ring.is_full());
        }

        #[test]
        fn should_return_false_when_partially_filled() {
            let mut ring = Ringbuffer::new(5);
            ring.push(1);
            ring.push(2);

            assert!(!ring.is_full());
        }

        #[test]
        fn should_return_true_when_at_capacity() {
            let mut ring = Ringbuffer::new(3);
            ring.push(1);
            ring.push(2);
            ring.push(3);

            assert!(ring.is_full());
        }
    }

    mod drain {
        use super::*;

        #[test]
        fn should_return_all_items_in_insertion_order() {
            let mut ring = Ringbuffer::new(5);
            ring.extend(vec![1, 2, 3]);

            let items = ring.drain();

            assert_eq!(items, vec![1, 2, 3]);
        }

        #[test]
        fn should_leave_buffer_empty_after_drain() {
            let mut ring = Ringbuffer::new(5);
            ring.extend(vec![1, 2, 3]);

            ring.drain();

            assert_eq!(ring.len(), 0);
            assert!(!ring.is_full());
        }

        #[test]
        fn should_return_empty_vec_for_empty_buffer() {
            let mut ring: Ringbuffer<i32> = Ringbuffer::new(5);

            let items = ring.drain();

            assert!(items.is_empty());
        }

        #[test]
        fn should_return_surviving_items_after_eviction() {
            let mut ring = Ringbuffer::new(3);
            ring.extend(vec![1, 2, 3, 4, 5]);

            let items = ring.drain();

            assert_eq!(items, vec![3, 4, 5]);
        }
    }
}
