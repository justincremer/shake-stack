#[derive(Debug, Clone)]
pub struct Queue<T> {
    inner: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { inner: Vec::new() }
    }

    pub fn push(&mut self, i: T) {
        self.inner.push(i);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.inner.remove(0))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn peek(&self) -> Option<&T> {
        self.inner.first()
    }
}

impl<T> From<Vec<T>> for Queue<T> {
    fn from(i: Vec<T>) -> Self {
        Queue { inner: i }
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn it_works() {
        let mut stack = Queue::from(vec![1, 2, 3, 4, 5]);
        for i in 6..11 {
            stack.push(i);
        }

        stack.pop();
        let test_case: Vec<i32> = (2..11).collect();
        assert_eq!(stack.inner, test_case);
    }

    #[test]
    fn its_empty() {
        let stack: Queue<i32> = Queue::new();
        assert_eq!(
            (stack.is_empty(), stack.len(), stack.peek()),
            (true, 0, None)
        );
    }
}
