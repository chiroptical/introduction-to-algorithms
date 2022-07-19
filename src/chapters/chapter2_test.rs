#[cfg(test)]
mod chapter2_test {
    use super::super::*;
    use quickcheck::quickcheck;

    #[test]
    fn insertion_sort_works() {
        let mut input = vec![5, 2, 4, 6, 1, 3];
        let output = vec![1, 2, 3, 4, 5, 6];
        insertion_sort(&mut input);
        assert_eq!(input, output);
    }

    #[test]
    fn insertion_sort_from_right_works() {
        let mut input = vec![5, 2, 4, 6, 1, 3];
        let output = vec![1, 2, 3, 4, 5, 6];
        insertion_sort_from_right(&mut input);
        assert_eq!(input, output);
    }

    #[test]
    fn sum_array_works() {
        let input = vec![1, 2, 3];
        let result = sum_array(&input);
        assert_eq!(result, 6);
    }

    #[test]
    fn insertion_sort_decreasing_works() {
        let mut input = vec![5, 2, 4, 6, 1, 3];
        let output = vec![6, 5, 4, 3, 2, 1];
        insertion_sort_decreasing(&mut input);
        assert_eq!(input, output);
    }

    #[test]
    fn linear_search_with_some() {
        let input = vec![5, 2, 4, 6, 1, 3];
        assert_eq!(linear_search(1, &input), Some(4));
    }

    #[test]
    fn linear_search_with_none() {
        let input = vec![5, 2, 4, 6, 1, 3];
        assert_eq!(linear_search(10, &input), None);
    }

    #[test]
    fn add_binary_integers_example_one() {
        use Bit::*;
        let a = vec![Zero, One, One, One];
        let b = vec![One, One, One, Zero];
        let c = vec![One, Zero, One, Zero, One];
        assert_eq!(add_binary_integers(c.len(), a, b), Some(c));
    }

    #[test]
    fn add_binary_integers_example_two() {
        use Bit::*;
        // 10101010 + 11001100 = 101110110
        let a = vec![One, Zero, One, Zero, One, Zero, One, Zero];
        let b = vec![One, One, Zero, Zero, One, One, Zero, Zero];
        let c = vec![One, Zero, One, One, One, Zero, One, One, Zero];
        assert_eq!(add_binary_integers(c.len(), a, b), Some(c));
    }

    #[test]
    fn minimum_index_with_empty_array_returns_none() {
        let a: Vec<u8> = vec![];
        assert_eq!(minimum_index(&a), None);
    }

    #[test]
    fn minimum_index_works_with_one_element() {
        let a: Vec<u8> = vec![1];
        assert_eq!(minimum_index(&a), Some(0));
    }

    #[test]
    fn minimum_index_works() {
        let a: Vec<u8> = vec![4, 3, 2, 1];
        assert_eq!(minimum_index(&a), Some(3));
    }

    #[test]
    fn selection_sort_with_empty_array_works() {
        let mut input: Vec<u8> = vec![];
        let output: Vec<u8> = vec![];
        selection_sort(&mut input);
        assert_eq!(input, output);
    }

    #[test]
    fn selection_sort_with_one_element_works() {
        let mut input = vec![1];
        let output = vec![1];
        selection_sort(&mut input);
        assert_eq!(input, output);
    }

    #[test]
    fn selection_sort_works() {
        let mut input = vec![5, 2, 4, 6, 1, 3];
        let output = vec![1, 2, 3, 4, 5, 6];
        selection_sort(&mut input);
        assert_eq!(input, output);
    }

    #[test]
    fn merge_with_zero_elements() {
        let input: Vec<u8> = vec![];
        assert_eq!(merge(&input, 0, 0, 0), Some(input));
    }

    #[test]
    fn merge_with_one_element() {
        let input = vec![1];
        assert_eq!(merge(&input, 0, 0, 0), Some(input));
    }

    #[test]
    fn merge_out_of_bounds_r_greater_than_length() {
        let input = vec![1, 2, 3];
        assert_eq!(merge(&input, 0, 0, 6), None);
    }

    #[test]
    fn merge_out_of_bounds_q_equals_r() {
        let input = vec![1, 2, 3];
        assert_eq!(merge(&input, 0, 2, 2), None);
    }

    #[test]
    fn merge_out_of_bounds_out_of_order() {
        let input = vec![1, 2, 3];
        assert_eq!(merge(&input, 2, 1, 0), None);
    }

    #[test]
    fn merge_basic() {
        let input = vec![1, 3, 5, 2, 4, 6];
        let output = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(merge(&input, 0, 2, 5), Some(output));
    }

    #[test]
    fn merge_non_ordered() {
        let input = vec![3, 2, 1, 6, 5, 4];
        assert_eq!(merge(&input, 0, 2, 5), Some(input));
    }

    #[test]
    fn merge_presorted() {
        let input = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(merge(&input, 0, 2, 5), Some(input));
    }

    #[test]
    fn merge_larger_test() {
        let input = vec![1, 2, 2, 2, 3, 3, 4, 1, 7, 7, 8, 8, 8, 9];
        let mut output = input.to_owned();
        output.sort();
        let r = input.len() - 1;
        assert_eq!(merge(&input, 0, 6, r), Some(output));
    }

    #[test]
    fn merge_sort_basic() {
        let mut input = vec![1, 7, 9, 8, 3, 4, 8, 2, 1, 2, 8, 2, 3, 7];
        let mut output = input.to_owned();
        output.sort();
        let r = input.len() - 1;
        assert_eq!(merge_sort(&mut input, 0, r), Ok(()));
        assert_eq!(input, output);
    }
    #[cfg(test)]
    quickcheck! {
        fn merge_prop(xs: Vec<u32>) -> bool {
            // Generate two sorted subarrays to merge
            let mut ys = xs.to_owned();
            let mut zs = xs.to_owned();
            ys.sort();
            zs.sort();

            // The input is just the sorted input using Rust
            let mut input = ys.to_owned();
            input.extend(zs.to_owned());
            println!("input: {:?}", input);

            // The reference is just the sorted input using Rust
            let mut reference = input.to_owned();
            reference.sort();
            println!("reference: {:?}", reference);

            // merge sort should work and produce a sorted list
            if input.len() != 0 {
                let r = input.len() - 1;
                merge(&mut input, 0, xs.len() - 1, r) == Some(reference)
            }
            // Unsure if we can throw away this test case more gracefully
            else {
                true
            }
        }
    }

    #[cfg(test)]
    quickcheck! {
        fn merge_sort_prop(xs: Vec<u32>) -> bool {
            // A mutable copy of our input to run through merge sort
            let mut input = xs.to_owned();
            // The reference is just the sorted input using Rust
            let mut reference = xs.to_owned();
            reference.sort();
            // merge sort should work and produce a sorted list
            let r = if input.len() == 0 {0} else {input.len() - 1};
            merge_sort(&mut input, 0, r).is_ok() && reference == input
        }
    }
}
