use exercises::read_services;

/// You can iterate many collections in Rust.
///
/// ToDo: Explain that array indexing works but is error prone, as seen in last
/// excercise, better to iterate the collection
///
/// To make for loops simpler to write the compiler will automatically add calls to [`.into_iter()`](https://doc.rust-lang.org/stable/std/iter/trait.IntoIterator.html#tymethod.into_iter).
/// The docs for that trait also list out all the implementers, so all of those
/// can be implicitly be iterated over.
///
/// Goal: Learn about iterating a collection and implicit `.into_iter()` in for
/// loops

fn sum_list(list: Vecu<32>) -> u32 {
    // TODO sum up all the elements in list in a for loop
    let mut sum = 0;

    // for ...

    sum
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_empty_list() {
        assert_eq!(sum_list(vec![]), 0);
    }

    #[test]
    fn sums_full_list() {
        assert_eq!(sum_list(vec![1, 2, 3]), 6);
    }
}
