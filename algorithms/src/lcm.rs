pub fn lcm(x: u32, y: u32) -> u32 {
    x / crate::gcd(x, y) * y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(lcm(3, 1), 3);
        assert_eq!(lcm(1, 3), 3);
        assert_eq!(lcm(5, 5), 5);
        assert_eq!(lcm(20, 6), 60);
        assert_eq!(lcm(6, 20), 60);
        assert_eq!(lcm(100, 52), 1300);
        assert_eq!(lcm(52, 100), 1300);
    }
}
