#[derive(Debug, Clone)]
pub struct Stack<T> {
    inner: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { inner: Vec::new() }
    }

    pub fn push(&mut self, i: T) {
        self.inner.push(i);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn peek(&self) -> Option<&T> {
        self.inner.last()
    }
}

impl<T> From<Vec<T>> for Stack<T> {
    fn from(i: Vec<T>) -> Self {
        Stack { inner: i }
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn it_works() {
        let mut stack = Stack::from(vec![1, 2, 3, 4, 5]);
        for i in 6..11 {
            stack.push(i);
        }

        stack.pop();
        let test_case: Vec<i32> = (1..10).collect();
        assert_eq!(stack.inner, test_case);
    }

    #[test]
    fn its_empty() {
        let stack: Stack<i32> = Stack::new();
        assert_eq!(
            (stack.is_empty(), stack.len(), stack.peek()),
            (true, 0, None)
        );
    }
}
