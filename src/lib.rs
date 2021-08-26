trait Sorter {
    fn sort<T>(_slice: &mut [T])
    where
        T: Ord,
    {
    }
}

// freestanding sort function that is generics over the Sorter-implemented
// algorithm and the slice
fn sort_freestanding<T, S>(slice: &mut [T])
where
    T: Ord,
    S: Sorter,
{
    S::sort(slice);
}

mod bubblesort;
mod insertionsort;
mod selectionsort;

#[cfg(test)]
mod integrate_tests {
    use super::sort_freestanding;
    use crate::{bubblesort::BubbleSorter,insertionsort::InsertionSorter, selectionsort::SelectionSorter};

    // Integrate test
    #[test]
    fn integrate_test_i32_bubble() {
        let mut v = vec![5, 2, -1, 10];
        sort_freestanding::<_, BubbleSorter>(&mut v);
        let v_expected = vec![-1, 2, 5, 10];

        assert_eq!(v, v_expected);
    }

    #[test]
    fn integrate_test_i32_selection() {
        let mut v = vec![5, 2, -1, 10];
        sort_freestanding::<_, SelectionSorter>(&mut v);
        let v_expected = vec![-1, 2, 5, 10];

        assert_eq!(v, v_expected);
    }

    #[test]
    fn integrate_test_i32_insertion() {
        let mut v = vec![5, 2, -1, 10];
        sort_freestanding::<_, InsertionSorter>(&mut v);
        let v_expected = vec![-1, 2, 5, 10];

        assert_eq!(v, v_expected);
    }
}
