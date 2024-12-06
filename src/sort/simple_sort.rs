pub fn selection_sort<T: PartialOrd>(array: &mut [T]) {
    for i in 0..array.len() - 1 {
        let mut min = i;
        for j in i + 1..array.len() {
            if array[j] < array[min] {
                min = j;
            }
        }
        array.swap(i, min);
    }
}

pub fn insertion_sort<T: PartialOrd>(array: &mut [T]) {
    for i in 1..array.len() {
        for j in (1..=i).rev() {
            if array[j] < array[j - 1] {
                array.swap(j - 1, j);
            } else {
                break;
            }
        }
    }
}

pub fn bubble_sort<T: PartialOrd>(array: &mut [T]) {
    let mut swapped;
    for i in 0..array.len() {
        swapped = false;
        for j in 0..array.len() - i - 1{
            if array[j] > array[j+1] {
                array.swap(j, j+1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

#[test]
pub fn simple_sort_test() {
    let start = [1, 5, 7, 3, 2, 4, 9, 10, 8, 6];
    let mut goal = start.clone();
    goal.sort();

    let mut array = start.clone();
    selection_sort(&mut array);
    assert_eq!(array, goal);

    array = start.clone();
    insertion_sort(&mut array);
    assert_eq!(array, goal);

    array = start.clone();
    bubble_sort(&mut array);
    assert_eq!(array, goal);
}
