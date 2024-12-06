pub fn mergesort<T: PartialOrd>(array: &mut [T]) {
    let mut temp = vec![0; array.len()];
    mergesort_internal(array, &mut temp, 0, array.len());
}

fn mergesort_internal<T: PartialOrd>(
    array: &mut [T],
    temp: &mut Vec<usize>,
    left: usize,
    right: usize,
) {
    if right - left <= 1 {
        return;
    }
    let m = (right + left) / 2;
    mergesort_internal(array, temp, left, m);
    mergesort_internal(array, temp, m, right);

    let mut idx = left;
    let mut lp = left;
    let mut rp = m;
    while idx < right {
        if rp >= right || (lp < m && array[lp] < array[rp]) {
            temp[idx] = lp;
            lp += 1;
        } else {
            temp[idx] = rp;
            rp += 1;
        }
        idx += 1;
    }

    for i in left..right {
        let pointed_to = temp[i];
        array.swap(i, pointed_to);
        // since the order in the array changed, we need to update the pointers
        let orig_pos = temp.iter().position(|n| *n == i).unwrap();
        let orig_pos2 = temp.iter().position(|n| *n == pointed_to).unwrap();
        temp.swap(orig_pos, orig_pos2);
    }
}

#[test]
pub fn mergesort_test() {
    let start = [1, 5, 7, 3, 2, 4, 9, 10, 8, 6];
    let mut goal = start.clone();
    goal.sort();

    let mut array = start.clone();
    mergesort(&mut array);
    assert_eq!(array, goal);
}
