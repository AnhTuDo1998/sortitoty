use super::Sorter;
pub struct InsertionSorter;

impl Sorter for InsertionSorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 1..slice.len(){
            for i in (0..unsorted).rev(){
                if slice[i+1] < slice[i]{
                    slice.swap(i+1, i);
                }
            }
        }
    }
}

// Unit test
#[test]
fn test_i32() {
    let mut v = vec![5, 2, -1, 10];
    InsertionSorter::sort(&mut v);
    let v_expected = vec![-1, 2, 5, 10];

    assert_eq!(v, v_expected);
}

#[test]
fn test_empty() {
    let mut v: Vec<i32> = Vec::new();
    InsertionSorter::sort(&mut v);
    let v_expected: Vec<i32> = Vec::new();

    assert_eq!(v, v_expected);
}
