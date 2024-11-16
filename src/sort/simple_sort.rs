
pub fn selection_sort<T: PartialOrd>(array: &mut [T]) {
    for i in 0..array.len()-1 {
        let mut min = i;
        for j in i + 1..array.len() {
            if array[j] < array[min]{
                min = j;
            }
        }
        array.swap(i, min);
    }
}

#[test]
pub fn simple_sort_test() {
    let start = [1, 5, 7, 3, 2, 4, 9, 10, 8, 6];
    let goal = [1,2,3,4,5,6,7,8,9,10];

    let mut array = start.clone();
    selection_sort(&mut array);
    assert_eq!(array, goal);
}