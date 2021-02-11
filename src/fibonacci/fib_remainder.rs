pub fn fib_remainder(num: u64, div: u64) -> u64 {
    if num <= 1 { return num; }

    let mut a = 0;
    let mut b = 1;
    let mut c;

    for _ in 2 ..= num {
        c = b;
        b = (a + b) % div;
        a = c;
    }

    return b;
}

#[cfg(test)]
mod tests {
    use super::fib_remainder;

    #[test]
    fn test0() {
        let result = fib_remainder(9, 2);

        assert_eq!(result, 0);
    }

    #[test]
    fn test1() {
        let result = fib_remainder(10, 2);

        assert_eq!(result, 1);
    }

    #[test]
    fn test2() {
        let result = fib_remainder(1025, 55);

        assert_eq!(result, 5);
    }

    #[test]
    fn test3() {
        let result = fib_remainder(12589, 369);

        assert_eq!(result, 89);
    }

    #[test]
    fn test4() {
        let result = fib_remainder(1598753, 25897);

        assert_eq!(result, 20305);
    }

    #[test]
    fn test5() {
        let result = fib_remainder(5134413, 123);

        assert_eq!(result, 110);
    }
}
