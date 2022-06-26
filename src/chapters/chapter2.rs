use std::fmt::Debug;

#[cfg(test)]
#[path = "chapter2_test.rs"]
mod chapter2_test;

//
// - You can't use slices to mutate arrays, you should use `&mut Vec<T>`
// - `T` is almost always going to go need `Ord + Copy` for sorting
pub fn insertion_sort<T: Ord + Copy>(a: &mut [T]) {
    for i in 1..a.len() {
        let key = a[i];
        let mut j = i;
        while j != 0 && a[j - 1] > key {
            a[j] = a[j - 1];
            j = j - 1;
        }
        a[j] = key
    }
}

// let mut input = vec![5, 2, 4, 6, 1, 3];
// i = len - 1
// j = i
// key = a[j] = 3
// Initialization/Maintenance: a[i:a.len()] is sorted
pub fn insertion_sort_from_right<T: Ord + Copy>(a: &mut [T]) {
    for i in (0..a.len() - 1).rev() {
        let key = a[i]; // i = 3, 6
        let mut j = i;
        while j != a.len() - 1 && key > a[j + 1] {
            a[j] = a[j + 1];
            j = j + 1;
        }
        a[j] = key;
    }
}

// Similar-ish to Exercise 2.1-1, illustrating how insertion sort works
// i = 4 from the left
// key = 1
// vec![2, 4, 5, 6, _, 3] -- Initialization condition applies here
// vec![2, 4, 5, _, 6, 3]
// vec![2, 4, _, 5, 6, 3]
// vec![2, _, 4, 5, 6, 3]
// vec![_, 2, 4, 5, 6, 3]
// vec![1, 2, 4, 5, 6, 3] -- Maintenance condition applies here

// i = 3 from the right
// vec![5, 2, 4, _, 1, 3]
// vec![5, 2, 4, 1, _, 3]
// vec![5, 2, 4, 1, 3, _]
// vec![5, 2, 4, 1, 3, 6]

// Exercise 2.1-2
// Loop invariant: sum_array(a[0:i]) = sum
// Initialization: sum_array([]) ~ 0 :check:
// Maintenance: could test this, but trivial
// Termination: sum_array([1, 2, 3]) ~ 6 :check:, see sum_array_works
pub fn sum_array(a: &[i32]) -> i32 {
    let mut sum = 0;
    for i in 0..a.len() {
        sum += a[i];
    }
    sum
}

pub fn sum_array_functional(a: &[i32]) -> i32 {
    a.iter().fold(0, |acc, x| acc + *x)
}

// Exercise 2.1-3
// i = 3
// j = 3
// key = 6
// [5, 4, 3, _, 1]
// [5, 4, _, 3, 1]
// [5, _, 4, 3, 1]
// [_, 5, 4, 3, 1]
// [6, 5, 4, 3, 1] :check:
pub fn insertion_sort_decreasing<T: Ord + Copy>(a: &mut [T]) {
    for i in 1..a.len() {
        let key = a[i];
        let mut j = i;
        while j != 0 && a[j - 1] < key {
            a[j] = a[j - 1];
            j = j - 1;
        }
        a[j] = key
    }
}

// Exercise 2.1-4
pub fn linear_search<T: Eq>(x: T, a: &[T]) -> Option<usize> {
    // loop invariant: linear_search(x, a[0:i]) = None
    for i in 0..a.len() {
        if x == a[i] {
            return Some(i);
        }
    }
    None
}

// Exercise 2.1-5
#[derive(Clone, Copy)]
pub enum Bit {
    Zero,
    One,
}

#[derive(Clone, Copy)]
pub struct BitAdd {
    carry: Bit,
    // TODO: better name
    result: Bit,
}

fn add_bit(x: BitAdd, y: Bit) -> BitAdd {
    use Bit::*;
    let BitAdd { carry, result } = x;
    match (carry, result, y) {
        // y is Zero
        (Zero, Zero, Zero) => BitAdd {
            carry: Zero,
            result: Zero,
        },
        (Zero, Zero, One) => BitAdd {
            carry: Zero,
            result: One,
        },
        (Zero, One, Zero) => BitAdd {
            carry: Zero,
            result: One,
        },
        (Zero, One, One) => BitAdd {
            carry: One,
            result: Zero,
        },

        // y is One
        (One, Zero, Zero) => BitAdd {
            carry: Zero,
            result: One,
        },
        (One, Zero, One) => BitAdd {
            carry: One,
            result: Zero,
        },
        (One, One, Zero) => BitAdd {
            carry: One,
            result: Zero,
        },
        (One, One, One) => BitAdd {
            carry: One,
            result: One,
        },
    }
}

// TODO: Test this
pub fn add_binary_integers(n: usize, a: Vec<Bit>, b: Vec<Bit>) -> Option<Vec<Bit>> {
    use Bit::*;
    if n == 0 || a.len() != n || b.len() != n {
        return None;
    }
    let mut ret: Vec<Bit> = Vec::with_capacity(n);
    let mut _carry = Zero;
    for i in (0..n - 1).rev() {
        let BitAdd { carry, result } = add_bit(
            BitAdd {
                carry: _carry,
                result: a[i],
            },
            b[i],
        );
        ret[i + 1] = result;
        _carry = carry;
    }
    ret[0] = _carry;
    Some(ret)
}
