// O(nlogn) - worst, mid, best time, O(n) - best time for eq keys, O(n) - memory
use std::fmt::Debug;

pub fn heapsort<T: Debug +PartialOrd + Copy>(array: &mut [T], n: usize) {
    if n == 0 {
        return;
    }

    for i in (0 ..= n / 2 - 1).rev() {
        heapify(array, n, i);
    }

    for i in (1 ..= n - 1).rev() {
        swap(array, 0, i);

        heapify(array, i, 0);
    }
}

fn heapify<T: Debug + PartialOrd + Copy>(array: &mut [T], n: usize, i: usize) {
    let mut largest = i;
    let l = 2 * i + 1;
    let r = 2 * i + 2;
 
    if l < n && array[l] > array[largest] {
        largest = l;
    }
 
    if r < n && array[r] > array[largest] {
        largest = r;
    }
 
    if largest != i {
        swap(array, i, largest);

        heapify(array, n, largest);
    }
}

fn swap<T: Copy>(array: &mut [T], a: usize, b: usize) {
    let temp = array[a];
    array[a] = array[b];
    array[b] = temp;
}

#[cfg(test)]
mod tests {
    use super::heapsort;

    // #[test]
    // fn test0() {
    //     let mut vpoints: Vec<usize> = vec![];
    //     let points = vpoints.as_mut_slice();

    //     heapsort(points, 0);

    //     assert_eq!(points, vec![].as_slice());
    // }

    #[test]
    fn test1() {
        let mut vpoints: Vec<usize> = vec![1, 5, 9, 1221, 0, 12, 4, 5, 4];
        let points = vpoints.as_mut_slice();

        heapsort(points, 9);

        assert_eq!(points, vec![0, 1, 4, 4, 5, 5, 9, 12, 1221].as_slice());
    }

    // #[test]
    // fn test2() {
    //     let mut vpoints: Vec<usize> = vec![0, 0, 0, 0, 0];
    //     let points = vpoints.as_mut_slice();

    //     heapsort(points, 5);

    //     assert_eq!(points, vec![0, 0, 0, 0, 0].as_slice());
    // }

    // #[test]
    // fn test3() {
    //     let mut vpoints: Vec<usize> = vec![1, 1, 1, 0, 0, 0];
    //     let points = vpoints.as_mut_slice();

    //     heapsort(points, 6);

    //     assert_eq!(points, vec![0, 0, 0, 1, 1, 1].as_slice());
    // }

    // #[test]
    // fn test4() {
    //     let mut vpoints: Vec<usize> = vec![1, 5, 9, 1221, 0, 12, 4];
    //     let points = vpoints.as_mut_slice();

    //     heapsort(points, 7);

    //     assert_eq!(points, vec![0, 1, 4, 5, 9, 12, 1221].as_slice());
    // }

    // #[test]
    // fn test5() {
    //     let mut vpoints: Vec<usize> = vec![1, 2, 3, 4, 5];
    //     let points = vpoints.as_mut_slice();

    //     heapsort(points, 5);

    //     assert_eq!(points, vec![1, 2, 3, 4, 5].as_slice());
    // }

    // #[test]
    // fn test6() {
    //     let mut vpoints: Vec<usize> = vec![5, 4, 3, 2, 1];
    //     let points = vpoints.as_mut_slice();

    //     heapsort(points, 5);

    //     assert_eq!(points, vec![1, 2, 3, 4, 5].as_slice());
    // }
}
