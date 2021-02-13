// O(n^2) - worst time, O(n^2) - common time, O(n) - best time for compare, O(1) - best time for swap, O(n) main memory + 0(1) additional memory

pub fn insertionsort<T: PartialOrd + Copy>(array: &mut [T], len: usize) {
    for i in 1 .. len {
        let x = array[i];
        let mut j = i;
        
        while j > 0 && array[j - 1] > x {
            array[j] = array[j - 1];
            j -= 1;
        }
        array[j] = x;
    }
}

#[cfg(test)]
mod tests_insertionsort {
    use super::insertionsort;

    #[test]
    fn test0() {
        let mut vpoints: Vec<usize> = vec![];
        let points = vpoints.as_mut_slice();

        insertionsort(points, 0);

        assert_eq!(points, vec![].as_slice());
    }

    #[test]
    fn test1() {
        let mut vpoints: Vec<usize> = vec![1, 5, 9, 1221, 0, 12, 4, 5, 4];
        let points = vpoints.as_mut_slice();

        insertionsort(points, 9);

        assert_eq!(points, vec![0, 1, 4, 4, 5, 5, 9, 12, 1221].as_slice());
    }

    #[test]
    fn test2() {
        let mut vpoints: Vec<usize> = vec![0, 0, 0, 0, 0];
        let points = vpoints.as_mut_slice();

        insertionsort(points, 5);

        assert_eq!(points, vec![0, 0, 0, 0, 0].as_slice());
    }

    #[test]
    fn test3() {
        let mut vpoints: Vec<usize> = vec![1, 1, 1, 0, 0, 0];
        let points = vpoints.as_mut_slice();

        insertionsort(points, 6);

        assert_eq!(points, vec![0, 0, 0, 1, 1, 1].as_slice());
    }

    #[test]
    fn test4() {
        let mut vpoints: Vec<usize> = vec![1, 5, 9, 1221, 0, 12, 4];
        let points = vpoints.as_mut_slice();

        insertionsort(points, 7);

        assert_eq!(points, vec![0, 1, 4, 5, 9, 12, 1221].as_slice());
    }

    #[test]
    fn test5() {
        let mut vpoints: Vec<usize> = vec![1, 2, 3, 4, 5];
        let points = vpoints.as_mut_slice();

        insertionsort(points, 5);

        assert_eq!(points, vec![1, 2, 3, 4, 5].as_slice());
    }

    #[test]
    fn test6() {
        let mut vpoints: Vec<usize> = vec![5, 4, 3, 2, 1];
        let points = vpoints.as_mut_slice();

        insertionsort(points, 5);

        assert_eq!(points, vec![1, 2, 3, 4, 5].as_slice());
    }
}
