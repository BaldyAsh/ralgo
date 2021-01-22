pub fn quicksort<T: PartialOrd + Copy>(array: &mut [T], lo: isize, hi: isize) {
    if lo < hi {
        let midpoint = partition(array, lo, hi);

        quicksort(array, lo, midpoint);
        quicksort(array, midpoint + 1, hi);
    }
}

fn partition<T: PartialOrd + Copy>(array: &mut [T], lo: isize, hi: isize) -> isize {
    let pivot = array[((lo + hi) / 2) as usize];

    let mut i = lo;
    let mut j = hi;
    
    loop {
        while array[i as usize] < pivot {
            i += 1;
        }
        while array[j as usize] > pivot {
            j -= 1;
        }

        if i >= j {
            return j;
        }

        swap(array, i, j);
        
        // in case of duplicates
        i += 1;
        j -= 1;
    }
}

fn swap<T: Copy>(array: &mut [T], a: isize, b: isize) {
    let temp = array[a as usize];
    array[a as usize] = array[b as usize];
    array[b as usize] = temp;
}

#[cfg(test)]
mod tests {
    use super::quicksort;

    #[test]
    fn test0() {
        let mut vpoints: Vec<usize> = vec![];
        let points = vpoints.as_mut_slice();

        quicksort(points, 0, 0);

        assert_eq!(points, vec![].as_slice());
    }

    #[test]
    fn test1() {
        let mut vpoints: Vec<usize> = vec![1, 5, 9, 1221, 0, 12, 4, 5, 4];
        let points = vpoints.as_mut_slice();

        quicksort(points, 0, 8);

        assert_eq!(points, vec![0, 1, 4, 4, 5, 5, 9, 12, 1221].as_slice());
    }

    #[test]
    fn test2() {
        let mut vpoints: Vec<usize> = vec![0, 0, 0, 0, 0];
        let points = vpoints.as_mut_slice();

        quicksort(points, 0, 4);

        assert_eq!(points, vec![0, 0, 0, 0, 0].as_slice());
    }

    #[test]
    fn test3() {
        let mut vpoints: Vec<usize> = vec![1, 1, 1, 0, 0, 0];
        let points = vpoints.as_mut_slice();

        quicksort(points, 0, 5);

        assert_eq!(points, vec![0, 0, 0, 1, 1, 1].as_slice());
    }

    #[test]
    fn test4() {
        let mut vpoints: Vec<usize> = vec![1, 5, 9, 1221, 0, 12, 4];
        let points = vpoints.as_mut_slice();

        quicksort(points, 0, 6);

        assert_eq!(points, vec![0, 1, 4, 5, 9, 12, 1221].as_slice());
    }

    #[test]
    fn test5() {
        let mut vpoints: Vec<usize> = vec![1, 2, 3, 4, 5];
        let points = vpoints.as_mut_slice();

        quicksort(points, 0, 4);

        assert_eq!(points, vec![1, 2, 3, 4, 5].as_slice());
    }

    #[test]
    fn test6() {
        let mut vpoints: Vec<usize> = vec![5, 4, 3, 2, 1];
        let points = vpoints.as_mut_slice();

        quicksort(points, 0, 4);

        assert_eq!(points, vec![1, 2, 3, 4, 5].as_slice());
    }
}
