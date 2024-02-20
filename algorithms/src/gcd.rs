pub fn gcd(mut x: u32, mut y: u32) -> u32 {
    while y > 0 {
        (x, y) = (y, x % y);
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(gcd(3, 1), 1);
        assert_eq!(gcd(1, 3), 1);
        assert_eq!(gcd(5, 5), 5);
        assert_eq!(gcd(20, 6), 2);
        assert_eq!(gcd(6, 20), 2);
        assert_eq!(gcd(100, 52), 4);
        assert_eq!(gcd(52, 100), 4);
    }
}
