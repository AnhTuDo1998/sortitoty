use crate::Sorter;

pub struct SelectionSorter;

impl Sorter for SelectionSorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted_start_idx in 0..slice.len() {
            let mut min_index = unsorted_start_idx;
            // find min element's index
            for i in unsorted_start_idx..slice.len() {
                if slice[i] < slice[min_index] {
                    min_index = i;
                }
            }
            // Place the min element at the front of the unsorted
            if unsorted_start_idx != min_index {
                slice.swap(unsorted_start_idx, min_index);
            }
        }
    }
}
// Unit test
#[test]
fn test_i32() {
    let mut v = vec![5, 2, -1, 10];
    SelectionSorter::sort(&mut v);
    let v_expected = vec![-1, 2, 5, 10];

    assert_eq!(v, v_expected);
}

#[test]
fn test_empty() {
    let mut v: Vec<i32> = Vec::new();
    SelectionSorter::sort(&mut v);
    let v_expected: Vec<i32> = Vec::new();

    assert_eq!(v, v_expected);
}
