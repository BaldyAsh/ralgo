pub mod fib_last_num;
pub mod fib_remainder;

pub fn calc_fib(num: u64) -> u64 {
    if num <= 1 { return num; }

    let mut a = 0;
    let mut b = 1;
    let mut c;

    for _ in 2 ..= num {
        c = b;
        b = a + b;
        a = c;
    }

    return b;
}

#[cfg(test)]
mod tests_calc_fib {
    use super::calc_fib;

    #[test]
    fn test0() {
        let result = calc_fib(0);

        assert_eq!(result, 0);
    }

    #[test]
    fn test1() {
        let result = calc_fib(1);

        assert_eq!(result, 1);
    }

    #[test]
    fn test2() {
        let result = calc_fib(2);

        assert_eq!(result, 1);
    }

    #[test]
    fn test3() {
        let result = calc_fib(3);

        assert_eq!(result, 2);
    }

    #[test]
    fn test4() {
        let result = calc_fib(4);

        assert_eq!(result, 3);
    }

    #[test]
    fn test5() {
        let result = calc_fib(5);

        assert_eq!(result, 5);
    }

    #[test]
    fn test6() {
        let result = calc_fib(15);

        assert_eq!(result, 610);
    }
}
