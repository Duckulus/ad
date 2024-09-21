/// A Stack implementation that uses a vec for holding its elements
pub struct ArrayStack<T> {
    array: Vec<T>,
    stack_pointer: usize,
}

impl<T> ArrayStack<T> {
    /// Creates a new ArrayStack
    pub fn new() -> Self {
        ArrayStack {
            array: Vec::new(),
            stack_pointer: 0,
        }
    }

    /// Pushes a new value onto the stack
    pub fn push(&mut self, val: T) {
        if self.array.len() > self.stack_pointer {
            self.array[self.stack_pointer] = val;
        } else {
            self.array.push(val);
        }
        self.stack_pointer += 1;
    }

    /// If the stack is not empty, the top element is removed and returned, otherwise None is returned
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.stack_pointer -= 1;
        Some(self.array.remove(self.stack_pointer))
    }

    /// returns true iff the stack is empty
    pub fn is_empty(&self) -> bool {
        self.stack_pointer == 0
    }
}

#[test]
pub fn push_test() {
    let mut stack = ArrayStack::new();
    assert_eq!(stack.stack_pointer, 0);
    stack.push(1);
    assert_eq!(stack.stack_pointer, 1);
    assert_eq!(stack.array[0], 1);
    stack.push(2);
    assert_eq!(stack.stack_pointer, 2);
    assert_eq!(stack.array[0], 1);
    assert_eq!(stack.array[1], 2);
}

#[test]
pub fn pop_test() {
    let mut stack = ArrayStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    assert_eq!(stack.stack_pointer, 3);
    assert_eq!(stack.pop().unwrap(), 3);
    assert_eq!(stack.stack_pointer, 2);
    assert_eq!(stack.pop().unwrap(), 2);
    assert_eq!(stack.stack_pointer, 1);
    assert_eq!(stack.pop().unwrap(), 1);
    assert_eq!(stack.stack_pointer, 0);
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.stack_pointer, 0);
}

#[test]
pub fn is_empty_test() {
    let mut stack = ArrayStack::new();
    assert!(stack.is_empty());
    stack.push(1);
    assert!(!stack.is_empty());
    stack.pop();
    assert!(stack.is_empty());
    stack.pop();
    assert!(stack.is_empty());
}