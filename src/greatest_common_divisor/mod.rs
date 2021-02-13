pub fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 { return b; }
    if b == 0 { return a; }

    if a >= b {
        gcd(a % b, b)
    } else {
        gcd(a, b % a)
    }
}

#[cfg(test)]
mod greatest_common_divisor_tests {
    use super::gcd;

    #[test]
    fn test0() {
        let result = gcd(0, 5);

        assert_eq!(result, 5);
    }

    #[test]
    fn test1() {
        let result = gcd(37, 0);

        assert_eq!(result, 37);
    }

    #[test]
    fn test2() {
        let result = gcd(18, 35);

        assert_eq!(result, 1);
    }

    #[test]
    fn test3() {
        let result = gcd(14159572, 63967072);

        assert_eq!(result, 4);
    }
}