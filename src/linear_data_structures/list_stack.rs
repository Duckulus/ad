use std::mem;

#[derive(Debug, PartialEq, Clone)]
struct Node<T> {
    value: Option<T>,
    next: Option<Box<Node<T>>>,
}

/// A Stack implementation that uses a linked list for holding its elements
pub struct ListStack<T> {
    head: Node<T>,
}

impl<T> ListStack<T> {

    /// Creates a new ListStack
    pub fn new() -> Self {
        ListStack { head: Node { value: None, next: None } }
    }

    /// Pushes a new value onto the stack
    pub fn push(&mut self, val: T) {
        if self.is_empty() {
            self.head.next = Some(Box::new(Node {
                value: Some(val),
                next: None,
            }));
        } else {
            let new_node = Node {
                value: Some(val),
                next: mem::replace(&mut self.head.next, None),
            };
            self.head.next = Some(Box::new(new_node));
        }
    }

    /// If the stack is not empty, the top element is removed and returned, otherwise None is returned
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let top = mem::replace(&mut self.head.next, None).expect("head should have next element if stack is not empty");
        self.head.next = top.next;
        top.value
    }

    /// returns true iff the stack is empty
    pub fn is_empty(&self) -> bool {
        self.head.next.is_none()
    }
}

#[test]
pub fn push_test() {
    let mut stack = ListStack::new();
    assert_eq!(stack.head.next, None);
    stack.push(13);
    assert_eq!(stack.head.next.clone().unwrap().value.unwrap(), 13);
    assert_eq!(stack.head.next.clone().unwrap().next, None);
    stack.push(14);
    assert_eq!(stack.head.next.clone().unwrap().value.unwrap(), 14);
    assert_eq!(stack.head.next.clone().unwrap().next.unwrap().value.unwrap(), 13);
    assert_eq!(stack.head.next.clone().unwrap().next.unwrap().next, None);
    stack.push(15);
    assert_eq!(stack.head.next.clone().unwrap().value.unwrap(), 15);
    assert_eq!(stack.head.next.clone().unwrap().next.unwrap().value.unwrap(), 14);
    assert_eq!(stack.head.next.clone().unwrap().next.unwrap().next.unwrap().value.unwrap(), 13);
    assert_eq!(stack.head.next.clone().unwrap().next.unwrap().next.unwrap().next, None);
}

#[test]
pub fn pop_test() {
    let mut stack = ListStack::new();
    assert_eq!(stack.pop(), None);
    stack.push(13);
    stack.push(14);
    stack.push(15);
    assert_eq!(stack.pop().unwrap(), 15);
    assert_eq!(stack.pop().unwrap(), 14);
    assert_eq!(stack.pop().unwrap(), 13);
    assert_eq!(stack.pop(), None);
}

#[test]
pub fn is_empty_test() {
    let mut stack = ListStack::new();
    assert!(stack.is_empty());
    stack.push(1);
    assert!(!stack.is_empty());
    stack.pop();
    assert!(stack.is_empty());
    stack.pop();
    assert!(stack.is_empty());
}
