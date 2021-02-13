// O(n^2) - worst, mid, best time, O(n) - main memory, O(1) - additional memory

pub fn selectsort<T: PartialOrd + Copy>(array: &mut [T], len: usize) {
    for i in 0 .. len {
        let mut k = i;
        let mut x = array[i];

        for j in i + 1 .. len {
            if array[j] < x {
                k = j;
                x = array[j];
            }
        }

        swap(array, i, k);
    }
}

fn swap<T: Copy>(array: &mut [T], a: usize, b: usize) {
    let temp = array[a];
    array[a] = array[b];
    array[b] = temp;
}

#[cfg(test)]
mod tests_selectsort {
    use super::selectsort;

    #[test]
    fn test0() {
        let mut vpoints: Vec<usize> = vec![];
        let points = vpoints.as_mut_slice();

        selectsort(points, 0);

        assert_eq!(points, vec![].as_slice());
    }

    #[test]
    fn test1() {
        let mut vpoints: Vec<usize> = vec![1, 5, 9, 1221, 0, 12, 4, 5, 4];
        let points = vpoints.as_mut_slice();

        selectsort(points, 9);

        assert_eq!(points, vec![0, 1, 4, 4, 5, 5, 9, 12, 1221].as_slice());
    }

    #[test]
    fn test2() {
        let mut vpoints: Vec<usize> = vec![0, 0, 0, 0, 0];
        let points = vpoints.as_mut_slice();

        selectsort(points, 5);

        assert_eq!(points, vec![0, 0, 0, 0, 0].as_slice());
    }

    #[test]
    fn test3() {
        let mut vpoints: Vec<usize> = vec![1, 1, 1, 0, 0, 0];
        let points = vpoints.as_mut_slice();

        selectsort(points, 6);

        assert_eq!(points, vec![0, 0, 0, 1, 1, 1].as_slice());
    }

    #[test]
    fn test4() {
        let mut vpoints: Vec<usize> = vec![1, 5, 9, 1221, 0, 12, 4];
        let points = vpoints.as_mut_slice();

        selectsort(points, 7);

        assert_eq!(points, vec![0, 1, 4, 5, 9, 12, 1221].as_slice());
    }

    #[test]
    fn test5() {
        let mut vpoints: Vec<usize> = vec![1, 2, 3, 4, 5];
        let points = vpoints.as_mut_slice();

        selectsort(points, 5);

        assert_eq!(points, vec![1, 2, 3, 4, 5].as_slice());
    }

    #[test]
    fn test6() {
        let mut vpoints: Vec<usize> = vec![5, 4, 3, 2, 1];
        let points = vpoints.as_mut_slice();

        selectsort(points, 5);

        assert_eq!(points, vec![1, 2, 3, 4, 5].as_slice());
    }
}
