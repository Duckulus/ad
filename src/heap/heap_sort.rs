use crate::heap::min_heap::MinHeap;

pub fn heap_sort<T: PartialOrd>(array: &mut Vec<T>) {
    let mut heap = MinHeap::from_elements(array);
    for _ in 0..heap.size() {
        array.push(heap.remove_root().unwrap());
    }
}

#[test]
pub fn heap_sort_test() {
    let mut start = vec![1, 5, 7, 3, 2, 4, 9, 10, 8, 6];
    let mut goal = start.clone();
    goal.sort();

    heap_sort(&mut start);
    assert_eq!(start, goal);
}
