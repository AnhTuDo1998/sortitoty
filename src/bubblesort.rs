// Bubble sort
use crate::Sorter;
pub struct BubbleSorter;

impl Sorter for BubbleSorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        // Keep swapping 2 adjacent elements until everything is sorted
        let mut sort_continue = true;
        while sort_continue {
            sort_continue = false;
            for i in 1..slice.len() {
                if slice[i] < slice[i - 1] {
                    slice.swap(i, i - 1);
                    sort_continue = true;
                }
            }
        }
    }
}

// Unit test
#[test]
fn test_i32() {
    let mut v = vec![5, 2, -1, 10];
    BubbleSorter::sort(&mut v);
    let v_expected = vec![-1, 2, 5, 10];

    assert_eq!(v, v_expected);
}

#[test]
fn test_empty() {
    let mut v: Vec<i32> = Vec::new();
    BubbleSorter::sort(&mut v);
    let v_expected: Vec<i32> = Vec::new();

    assert_eq!(v, v_expected);
}
