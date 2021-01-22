// O(n^2) - worst time, OMG(n^2 / 2^p) - common time where p is increments num, O(n log n) - best time, O(1) - memory

use super::bubblesort::bubblesort;

pub fn combsort<T: PartialOrd + Copy>(array: &mut [T], len: usize) {
    if len == 0 {
        return;
    }

    let factor = 1.247;
    let mut step: f32 = (len - 1) as f32;
    
    while step >= 1.0 {
        let mut i = 0;

        let usize_step = step as usize;

        while i + usize_step < len {
            if array[i] > array[i + usize_step] {
                swap(array, i, i + usize_step);
            }

            i += 1;
        }

        step /= factor;
    }

    bubblesort(array, len);
}

fn swap<T: Copy>(array: &mut [T], a: usize, b: usize) {
    let temp = array[a];
    array[a] = array[b];
    array[b] = temp;
}

#[cfg(test)]
mod tests {
    use super::combsort;

    #[test]
    fn test0() {
        let mut vpoints: Vec<usize> = vec![];
        let points = vpoints.as_mut_slice();

        combsort(points, 0);

        assert_eq!(points, vec![].as_slice());
    }

    #[test]
    fn test1() {
        let mut vpoints: Vec<usize> = vec![1, 5, 9, 1221, 0, 12, 4, 5, 4];
        let points = vpoints.as_mut_slice();

        combsort(points, 9);

        assert_eq!(points, vec![0, 1, 4, 4, 5, 5, 9, 12, 1221].as_slice());
    }

    #[test]
    fn test2() {
        let mut vpoints: Vec<usize> = vec![0, 0, 0, 0, 0];
        let points = vpoints.as_mut_slice();

        combsort(points, 5);

        assert_eq!(points, vec![0, 0, 0, 0, 0].as_slice());
    }

    #[test]
    fn test3() {
        let mut vpoints: Vec<usize> = vec![1, 1, 1, 0, 0, 0];
        let points = vpoints.as_mut_slice();

        combsort(points, 6);

        assert_eq!(points, vec![0, 0, 0, 1, 1, 1].as_slice());
    }

    #[test]
    fn test4() {
        let mut vpoints: Vec<usize> = vec![1, 5, 9, 1221, 0, 12, 4];
        let points = vpoints.as_mut_slice();

        combsort(points, 7);

        assert_eq!(points, vec![0, 1, 4, 5, 9, 12, 1221].as_slice());
    }

    #[test]
    fn test5() {
        let mut vpoints: Vec<usize> = vec![1, 2, 3, 4, 5];
        let points = vpoints.as_mut_slice();

        combsort(points, 5);

        assert_eq!(points, vec![1, 2, 3, 4, 5].as_slice());
    }

    #[test]
    fn test6() {
        let mut vpoints: Vec<usize> = vec![5, 4, 3, 2, 1];
        let points = vpoints.as_mut_slice();

        combsort(points, 5);

        assert_eq!(points, vec![1, 2, 3, 4, 5].as_slice());
    }
}
