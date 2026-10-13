/// TODO: Write intro, "you have used iterators before, for example `for i in
/// 0..5`...", `0..5` iterates over the numbers 0, 1, 2, 3, 4
///
///  GOAL recognize previous for loop usage

fn sum_vector(vec: Vec<u32>) -> u32 {
    let mut sum = 0;

    // TODO: change the range to cover all elements of `vec`
    for i in 0..3 {
        sum += vec[i];
        // Or: sum += vec.get(i).unwrap_or(0);
    }

    sum
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_short_list() {
        assert_eq!(sum_list(vec![0, 1, 2]), 3);
    }

    #[test]
    fn sums_full_list() {
        assert_eq!(sum_list(vec![1, 2, 3, 4, 5, 6]), 1 + 2 + 3 + 4 + 5 + 6);
    }
}
