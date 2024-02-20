pub fn exponential(mut base: u32, mut power: u32) -> u32 {
    let mut result = 1;
    while power > 0 {
        if power & 1 == 1 {
            result *= base;
        }
        base *= base;
        power >>= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(exponential(10, 0), 1);
        assert_eq!(exponential(10, 2), 100);
        assert_eq!(exponential(2, 10), 1024);
    }
}
