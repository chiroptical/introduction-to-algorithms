#[cfg(test)]
#[path = "example_tests.rs"]
mod example_tests;

#[allow(dead_code)]
pub fn public_add_two(x: i32) -> i32 {
    x + 2
}

#[allow(dead_code)]
fn private_add_two(x: i32) -> i32 {
    x + 2
}
