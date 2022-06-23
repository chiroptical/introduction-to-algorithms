pub mod chapters;

// The `Copy` trait allows you to copy values that would otherwise be moved. In this case, we want
// a new `T` not a reference to one (for whatever reason)
#[allow(dead_code)]
fn head<T: Copy>(xs: &[T]) -> Option<T> {
    match &xs {
        [x, ..] => Some(*x),
        _ => None,
    }
}

pub fn main() {
    println!("Introduction to Algorithms!");
}
