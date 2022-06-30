#[cfg(test)]
mod chapter2_test {
    use super::super::*;

    // tests go here...
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
}
