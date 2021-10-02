#[derive(Debug, Clone)]
pub struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    
    pub fn new() -> Stack<T> {
        Self(Vec::new())
    }

    pub fn push(&mut self, value: T) {
        self.0.push(value)
    }

    pub fn pop(&mut self) -> T {
        self.0.pop().expect("Unable to pop from empty stack.")
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_not_empty(&self) -> bool {
        ! self.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let stack: Stack<usize> = Stack::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn push() {
        let mut stack: Stack<usize> = Stack::new();
        stack.push(1);
        assert!(stack.is_not_empty())
    }

    #[test]
    fn pop() {
        let mut stack: Stack<usize> = Stack::new();
        stack.push(1);
        stack.pop();
        assert!(stack.is_empty())
    }

    #[test]
    #[should_panic(expected = "Unable to pop from empty stack.")]
    fn pop_when_empty() {
        let mut stack: Stack<usize> = Stack::new();
        stack.pop();
    }
}