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
}
