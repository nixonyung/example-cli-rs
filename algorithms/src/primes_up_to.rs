pub fn primes_up_to(upper_bound /* inclusive */: usize) -> Vec<usize> {
    if upper_bound < 2 {
        return vec![];
    }

    let mut is_primes = {
        let mut val = vec![true; upper_bound + 1];
        val[0] = false;
        val[1] = false;
        val
    };

    let upper_bound_sqrt = (upper_bound as f64).sqrt() as usize;
    for i in 2..=upper_bound_sqrt {
        if is_primes[i] {
            for j in ((i * i)..=upper_bound).step_by(i) {
                is_primes[j] = false;
            }
        }
    }

    is_primes
        .iter()
        .enumerate()
        .filter_map(|x| match x {
            (idx, true) => Some(idx),
            (_, false) => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(primes_up_to(0), vec![]);
        assert_eq!(primes_up_to(1), vec![]);
        assert_eq!(primes_up_to(2), vec![2]);
        assert_eq!(primes_up_to(10), vec![2, 3, 5, 7]);
        assert_eq!(
            primes_up_to(100),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ]
        );
        assert_eq!(
            primes_up_to(101),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97, 101
            ]
        );
        assert_eq!(
            primes_up_to(120),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97, 101, 103, 107, 109, 113
            ]
        );
    }
}
