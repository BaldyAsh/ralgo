// O(n^2) - worst time, O(n^2) - common time, O(n) - best time, O(1) - memory

pub fn shakersort<T: PartialOrd + Copy>(array: &mut [T], len: usize) {
    if len == 0 {
        return;
    }

    let mut left = 0;
    let mut right = len - 1;

    while left <= right {
        for i in (left + 1 ..= right).rev() {
            if array[i - 1] > array[i] {
                swap(array, i - 1, i);
            }
        }
        left += 1;

        for i in left ..= right - 1 {
            if array[i] > array[i + 1] {
                swap(array, i, i + 1);
            }
        }
        right -= 1;
    }
}

fn swap<T: Copy>(array: &mut [T], a: usize, b: usize) {
    let temp = array[a];
    array[a] = array[b];
    array[b] = temp;
}

#[cfg(test)]
mod tests_shakersort {
    use super::shakersort;

    #[test]
    fn test0() {
        let mut vpoints: Vec<usize> = vec![];
        let points = vpoints.as_mut_slice();

        shakersort(points, 0);

        assert_eq!(points, vec![].as_slice());
    }

    #[test]
    fn test1() {
        let mut vpoints: Vec<usize> = vec![1, 5, 9, 1221, 0, 12, 4, 5, 4];
        let points = vpoints.as_mut_slice();

        shakersort(points, 9);

        assert_eq!(points, vec![0, 1, 4, 4, 5, 5, 9, 12, 1221].as_slice());
    }

    #[test]
    fn test2() {
        let mut vpoints: Vec<usize> = vec![0, 0, 0, 0, 0];
        let points = vpoints.as_mut_slice();

        shakersort(points, 5);

        assert_eq!(points, vec![0, 0, 0, 0, 0].as_slice());
    }

    #[test]
    fn test3() {
        let mut vpoints: Vec<usize> = vec![1, 1, 1, 0, 0, 0];
        let points = vpoints.as_mut_slice();

        shakersort(points, 6);

        assert_eq!(points, vec![0, 0, 0, 1, 1, 1].as_slice());
    }

    #[test]
    fn test4() {
        let mut vpoints: Vec<usize> = vec![1, 5, 9, 1221, 0, 12, 4];
        let points = vpoints.as_mut_slice();

        shakersort(points, 7);

        assert_eq!(points, vec![0, 1, 4, 5, 9, 12, 1221].as_slice());
    }

    #[test]
    fn test5() {
        let mut vpoints: Vec<usize> = vec![1, 2, 3, 4, 5];
        let points = vpoints.as_mut_slice();

        shakersort(points, 5);

        assert_eq!(points, vec![1, 2, 3, 4, 5].as_slice());
    }

    #[test]
    fn test6() {
        let mut vpoints: Vec<usize> = vec![5, 4, 3, 2, 1];
        let points = vpoints.as_mut_slice();

        shakersort(points, 5);

        assert_eq!(points, vec![1, 2, 3, 4, 5].as_slice());
    }
}
