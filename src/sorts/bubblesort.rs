// O(n^2) - worst time, O(n^2) - common time, O(n) - best time, O(1) - memory

pub fn bubblesort<T: PartialOrd + Copy>(array: &mut [T], len: usize) {
    for i in 0 .. len {
        for j in (i + 1 .. len).rev() {
            if array[j - 1] > array[j] {
                swap(array, j - 1, j);
            }
        }
    }
}

fn swap<T: Copy>(array: &mut [T], a: usize, b: usize) {
    let temp = array[a];
    array[a] = array[b];
    array[b] = temp;
}

#[cfg(test)]
mod tests_bubblesort {
    use super::bubblesort;

    #[test]
    fn test0() {
        let mut vpoints: Vec<usize> = vec![];
        let points = vpoints.as_mut_slice();

        bubblesort(points, 0);

        assert_eq!(points, vec![].as_slice());
    }

    #[test]
    fn test1() {
        let mut vpoints: Vec<usize> = vec![1, 5, 9, 1221, 0, 12, 4, 5, 4];
        let points = vpoints.as_mut_slice();

        bubblesort(points, 9);

        assert_eq!(points, vec![0, 1, 4, 4, 5, 5, 9, 12, 1221].as_slice());
    }

    #[test]
    fn test2() {
        let mut vpoints: Vec<usize> = vec![0, 0, 0, 0, 0];
        let points = vpoints.as_mut_slice();

        bubblesort(points, 5);

        assert_eq!(points, vec![0, 0, 0, 0, 0].as_slice());
    }

    #[test]
    fn test3() {
        let mut vpoints: Vec<usize> = vec![1, 1, 1, 0, 0, 0];
        let points = vpoints.as_mut_slice();

        bubblesort(points, 6);

        assert_eq!(points, vec![0, 0, 0, 1, 1, 1].as_slice());
    }

    #[test]
    fn test4() {
        let mut vpoints: Vec<usize> = vec![1, 5, 9, 1221, 0, 12, 4];
        let points = vpoints.as_mut_slice();

        bubblesort(points, 7);

        assert_eq!(points, vec![0, 1, 4, 5, 9, 12, 1221].as_slice());
    }

    #[test]
    fn test5() {
        let mut vpoints: Vec<usize> = vec![1, 2, 3, 4, 5];
        let points = vpoints.as_mut_slice();

        bubblesort(points, 5);

        assert_eq!(points, vec![1, 2, 3, 4, 5].as_slice());
    }

    #[test]
    fn test6() {
        let mut vpoints: Vec<usize> = vec![5, 4, 3, 2, 1];
        let points = vpoints.as_mut_slice();

        bubblesort(points, 5);

        assert_eq!(points, vec![1, 2, 3, 4, 5].as_slice());
    }
}
