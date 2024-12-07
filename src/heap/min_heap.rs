pub struct MinHeap<T: PartialOrd> {
    size: usize,
    elements: Vec<T>,
}

impl<T: PartialOrd> MinHeap<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            elements: Vec::new(),
        }
    }

    pub fn from_elements(elements: &mut Vec<T>) -> Self {
        let mut heap = Self::new();
        for _ in 0..elements.len() {
            heap.insert(elements.remove(0));
        }
        heap
    }

    pub fn insert(&mut self, value: T) {
        self.elements.push(value);
        self.size += 1;
        self.upheap();
    }

    fn upheap(&mut self) {
        let mut index = self.size - 1;
        while index != 0 && self.elements[parent(index)] > self.elements[index] {
            self.elements.swap(parent(index), index);
            index = parent(index);
        }
    }

    pub fn remove_root(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        self.elements.swap(0, self.size - 1);
        let root = self.elements.remove(self.size - 1);
        self.size -= 1;
        self.downheap();
        Some(root)
    }

    fn downheap(&mut self) {
        let mut index = 0;
        loop {
            let left_child = left_child(index);
            let right_child = right_child(index);
            let mut min_index = index;
            if left_child < self.size && self.elements[left_child] < self.elements[min_index] {
                min_index = left_child;
            }
            if right_child < self.size && self.elements[right_child] < self.elements[min_index] {
                min_index = right_child;
            }
            if min_index != index {
                self.elements.swap(index, min_index);
                index = min_index;
            } else {
                break;
            }
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

fn parent(index: usize) -> usize {
    (index - 1) / 2
}

fn left_child(index: usize) -> usize {
    2 * index + 1
}

fn right_child(index: usize) -> usize {
    2 * index + 2
}

#[test]
pub fn min_heap_test() {
    let mut heap = MinHeap::new();
    assert_eq!(heap.remove_root(), None);

    heap.insert(5);
    heap.insert(42);
    heap.insert(26);
    heap.insert(99);
    heap.insert(1);

    assert_eq!(heap.remove_root(), Some(1));
    assert_eq!(heap.remove_root(), Some(5));
    assert_eq!(heap.remove_root(), Some(26));
    assert_eq!(heap.remove_root(), Some(42));
    assert_eq!(heap.remove_root(), Some(99));
    assert_eq!(heap.remove_root(), None);
}
