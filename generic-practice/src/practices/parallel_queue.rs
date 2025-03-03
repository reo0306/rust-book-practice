pub struct SharedQueue<T> {
    queue: Vec<T>
}

impl<T> SharedQueue<T> {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.queue.push(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        Some(self.queue.remove(0))
    }

    pub fn size(&self) -> usize {
        self.queue.len()
    }
}