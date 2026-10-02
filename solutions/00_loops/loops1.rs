fn sum_list(list: &[u32]) -> u32 {
    let mut sum = 0;

    for element in list {
        sum += element;
    }

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
        assert_eq!(sum_list(&[]), 0);
    }

    #[test]
    fn sums_full_list() {
        assert_eq!(sum_list(&[1, 2, 3]), 6);
    }
}
