use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Debug, Default, Clone)]
pub struct Counter {
    count: Arc<AtomicUsize>,
    stop: Option<usize>,
}

impl Counter {
    /// create the default counter starting at zero and no stop value
    pub fn create() -> Counter {
        Counter {
            count: Arc::new(AtomicUsize::new(0)),
            stop: None,
        }
    }

    /// start at the specified number
    pub fn from(start_at: usize) -> Counter {
        Counter {
            count: Arc::new(AtomicUsize::new(start_at)),
            stop: None,
        }
    }

    /// return the current count
    pub fn count(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }

    /// return the max value that this counter will count up to
    pub fn stop_at(&self) -> Option<usize> {
        self.stop
    }

    /// increment the counter
    pub fn incr(&mut self) -> usize {
        self.count.fetch_add(1, Ordering::SeqCst);
        self.count.load(Ordering::SeqCst)
    }

    /// decrement the counter
    pub fn decr(&mut self) -> usize {
        if self.count.load(Ordering::SeqCst) > 0 {
            self.count.fetch_sub(1, Ordering::SeqCst);
        }

        self.count.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter() {
        let counter = Counter::create();
        assert_eq!(counter.count(), 0);
        assert_eq!(counter.stop_at(), None);
    }

    #[test]
    fn from() {
        let counter = Counter::from(10);
        assert_eq!(counter.count(), 10);
    }

    #[test]
    fn incr() {
        let mut counter = Counter::create();
        let count = counter.incr();
        assert_eq!(count, 1);
        assert_eq!(count, counter.count());
    }

    #[test]
    fn decr() {
        let mut counter = Counter::from(10);
        let count = counter.decr();
        assert_eq!(count, 9);
        assert_eq!(count, counter.count());
    }
}
