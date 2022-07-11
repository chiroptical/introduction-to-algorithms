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
#[derive(Clone, Copy, PartialEq, Debug)]
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
    if n == 0 || a.len() + 1 != n || b.len() + 1 != n {
        return None;
    }
    let mut ret: Vec<Bit> = vec![Zero; n];
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

pub fn minimum_index<T: Copy + Ord>(a: &[T]) -> Option<usize> {
    let mut minimum = None;
    let mut index = None;
    for i in 0..a.len() {
        if i == 0 {
            minimum = Some(a[i]);
            index = Some(i);
        } else {
            if Some(a[i]) < minimum {
                minimum = Some(a[i]);
                index = Some(i);
            }
        }
    }
    index
}

// Exercise 2.2-2
// Loop invariant: selection_sort(a[0..i]) ~ a[0..i]
pub fn selection_sort<T: Ord + Copy>(a: &mut [T]) {
    // This allows us to write `a.len() - 1` below and the algorithm still works for an empty list
    if a.len() == 0 {
        return;
    }
    for i in 0..a.len() - 1 {
        // n - 1, linear in n
        let min_index = minimum_index(&a[i..]); // n - i, linear in n
        match min_index {
            Some(x) => {
                let sub_to_arr_index = x + i;
                if sub_to_arr_index != i {
                    let swap = a[i];
                    a[i] = a[sub_to_arr_index];
                    a[sub_to_arr_index] = swap;
                }
            }
            None => {
                panic!("This probably shouldn't happen...")
            }
        }
    }
}

pub fn merge<T: Copy + Ord>(a: &[T], p: usize, q: usize, r: usize) -> Option<Vec<T>> {
    // An element with 0 or 1 elements is already merged
    if a.len() == 0 || a.len() == 1 {
        return Some(a.to_owned());
    }
    // Bounds checking
    if !(a.len() >= r + 1 && a.len() > p && p <= q && q < r) {
        return None;
    }
    let mut result = Vec::with_capacity(r - p);

    fn go<T: Copy + Ord>(res: &mut Vec<T>, l: &[T], r: &[T]) {
        match (l, r) {
            ([], []) => (),
            ([x, xs @ ..], []) => {
                res.push(*x);
                go(res, &xs, &[]);
            }
            ([], [y, ys @ ..]) => {
                res.push(*y);
                go(res, &[], &ys);
            }
            ([x, xs @ ..], [y, ys @ ..]) => {
                if x <= y {
                    res.push(*x);
                    go(res, &xs, &r);
                } else {
                    res.push(*y);
                    go(res, &l, &ys);
                }
            }
        }
    }

    go(&mut result, &a[p..q + 1], &a[q + 1..r + 1]);
    Some(result)
}

#[derive(Debug, PartialEq)]
pub enum MergeSortFailure {
    LengthZeroOrOne,
    BoundsChecks(usize, usize, usize),
    Merge,
}

// TODO: See `merge_check`
pub fn merge_sort<T: Copy + Ord>(a: &mut [T], p: usize, r: usize) -> Result<(), MergeSortFailure> {
    // termination
    if p >= r {
        return Ok(());
    }

    // An element with 0 or 1 elements is already merged
    if a.len() == 0 || a.len() == 1 {
        return Err(MergeSortFailure::LengthZeroOrOne);
    }

    // Bounds checking
    if !(a.len() >= r + 1 && a.len() > p && p < r) {
        return Err(MergeSortFailure::BoundsChecks(a.len(), p, r));
    }

    // in theory, this is floor division
    let q: usize = (p + r) / 2;

    merge_sort(a, p, q)?;
    merge_sort(a, p + 1, r)?;
    match merge(a, p, q, r) {
        Some(x) => {
            for (i, v) in x.iter().enumerate() {
                a[p + i] = *v;
            }
        }
        None => {
            return Err(MergeSortFailure::Merge);
        }
    }
    Ok(())
}

// TODO: Write merge using the book's style
