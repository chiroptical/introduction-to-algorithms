#[cfg(test)]
mod example_tests {
    use super::super::*;

    #[test]
    fn public_works() {
        assert_eq!(public_add_two(2), 4);
    }

    #[test]
    fn private_works() {
        assert_eq!(private_add_two(2), 4)
    }
}
