use std::collections::VecDeque;

#[derive(Debug)]
struct BinarySearchTreeNode<T: PartialOrd> {
    value: T,
    left: Option<Box<BinarySearchTreeNode<T>>>,
    right: Option<Box<BinarySearchTreeNode<T>>>,
}

impl<T: PartialOrd> BinarySearchTreeNode<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        if value < self.value {
            if let Some(left) = &mut self.left {
                left.insert(value);
            } else {
                self.left = Some(Box::new(BinarySearchTreeNode::new(value)));
            }
        } else {
            if let Some(right) = &mut self.right {
                right.insert(value);
            } else {
                self.right = Some(Box::new(BinarySearchTreeNode::new(value)));
            }
        }
    }

    fn traverse_inorder<'a>(&'a self, nodes: &mut Vec<&'a T>) {
        if let Some(left) = &self.left {
            left.traverse_inorder(nodes);
        }
        nodes.push(&self.value);
        if let Some(right) = &self.right {
            right.traverse_inorder(nodes);
        }
    }

    fn traverse_preorder<'a>(&'a self, nodes: &mut Vec<&'a T>) {
        nodes.push(&self.value);
        if let Some(left) = &self.left {
            left.traverse_preorder(nodes);
        }
        if let Some(right) = &self.right {
            right.traverse_preorder(nodes);
        }
    }

    fn traverse_postorder<'a>(&'a self, nodes: &mut Vec<&'a T>) {
        if let Some(left) = &self.left {
            left.traverse_postorder(nodes);
        }
        if let Some(right) = &self.right {
            right.traverse_postorder(nodes);
        }
        nodes.push(&self.value);
    }
}

pub struct BinarySearchTree<T: PartialOrd> {
    root: Option<BinarySearchTreeNode<T>>,
}

impl<T: PartialOrd> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self {
            root: None
        }
    }

    pub fn insert(&mut self, value: T) {
        if let Some(root) = &mut self.root {
            root.insert(value)
        } else {
            self.root = Some(BinarySearchTreeNode::new(value));
        }
    }

    pub fn traverse_inorder(&self) -> Vec<&T> {
        let mut nodes = Vec::new();
        if let Some(root) = &self.root {
            root.traverse_inorder(&mut nodes);
        }
        nodes
    }

    pub fn traverse_preorder(&self) -> Vec<&T> {
        let mut nodes = Vec::new();
        if let Some(root) = &self.root {
            root.traverse_preorder(&mut nodes);
        }
        nodes
    }

    pub fn traverse_postorder(&self) -> Vec<&T> {
        let mut nodes = Vec::new();
        if let Some(root) = &self.root {
            root.traverse_postorder(&mut nodes);
        }
        nodes
    }

    pub fn traverse_levelorder(&self) -> Vec<&T> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        if let Some(root) = &self.root {
            queue.push_back(root);
        }

        while let Some(front) = queue.pop_front() {
            result.push(&front.value);
            if let Some(left) = &front.left {
                queue.push_back(left);
            }
            if let Some(right) = &front.right {
                queue.push_back(right);
            }
        }

        result
    }
}

#[test]
pub fn insert_test() {
    let mut tree = BinarySearchTree::new();
    assert!(matches!(tree.root, None));

    tree.insert(3);
    assert_eq!(tree.root.as_ref().unwrap().value, 3);
    assert!(matches!(tree.root.as_ref().unwrap().left, None));
    assert!(matches!(tree.root.as_ref().unwrap().right, None));

    tree.insert(6);
    assert_eq!(tree.root.as_ref().unwrap().right.as_ref().unwrap().value, 6);
}

#[test]
pub fn traverse_test() {
    let mut tree = BinarySearchTree::new();
    tree.insert(40);
    tree.insert(30);
    tree.insert(50);
    tree.insert(25);
    tree.insert(35);
    tree.insert(45);
    tree.insert(60);

    assert_eq!(tree.traverse_inorder().into_iter().copied().collect::<Vec<i32>>(), vec![25, 30, 35, 40, 45, 50, 60]);
    assert_eq!(tree.traverse_preorder().into_iter().copied().collect::<Vec<i32>>(), vec![40, 30, 25, 35, 50, 45, 60]);
    assert_eq!(tree.traverse_postorder().into_iter().copied().collect::<Vec<i32>>(), vec![25, 35, 30, 45, 60, 50, 40]);
    assert_eq!(tree.traverse_levelorder().into_iter().copied().collect::<Vec<i32>>(), vec![40, 30, 50, 25, 35, 45, 60]);
}
