// O(nlogn) - time, O(n) - additional memory

pub fn mergesort<T: PartialOrd + Copy>(array: Vec<T>, mut buffer: Vec<T>, left: usize, right: usize) -> Vec<T> {
    if left == right {
        if array.len() > 0 {
            buffer[left] = array[left];
        }
        return buffer;
    }

    let middle = (left + right) / 2;

    let l_buff = mergesort(array.clone(), buffer.clone(), left, middle);
    let r_buff = mergesort(array.clone(), buffer.clone(), middle + 1, right);

    let mut target = if l_buff[0] == array[0] {
        buffer
    } else {
        array
    };

    let mut l_cur = left;
    let mut r_cur = middle + 1;

    for i in left ..= right {
        if l_cur <= middle && r_cur <= right {
            if l_buff[l_cur] < r_buff[r_cur] {
                target[i] = l_buff[l_cur];
                l_cur += 1;
            } else {
                target[i] = r_buff[r_cur];
                r_cur += 1;
            }
        } else if l_cur <= middle {
            target[i] = l_buff[l_cur];
            l_cur += 1;
        } else {
            target[i] = r_buff[r_cur];
            r_cur += 1;
        }
    }

    target
}

#[cfg(test)]
mod tests {
    use super::mergesort;

    #[test]
    fn test0() {
        let array: Vec<usize> = vec![];
        let buffer: Vec<usize> = vec![];

        let result = mergesort(array, buffer, 0, 0);

        assert_eq!(result, vec![]);
    }

    #[test]
    fn test1() {
        let array: Vec<usize> = vec![1, 5, 9, 1221, 0, 12, 4, 5, 4];
        let buffer: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];

        let result = mergesort(array, buffer, 0, 8);

        assert_eq!(result, vec![0, 1, 4, 4, 5, 5, 9, 12, 1221]);
    }

    #[test]
    fn test2() {
        let array: Vec<usize> = vec![0, 0, 0, 0, 0];
        let buffer: Vec<usize> = vec![0, 0, 0, 0, 0];

        let result = mergesort(array, buffer, 0, 4);

        assert_eq!(result, vec![0, 0, 0, 0, 0]);
    }

    #[test]
    fn test3() {
        let array: Vec<usize> = vec![1, 1, 1, 0, 0, 0];
        let buffer: Vec<usize> = vec![0, 0, 0, 0, 0, 0];

        let result = mergesort(array, buffer, 0, 5);

        assert_eq!(result, vec![0, 0, 0, 1, 1, 1]);
    }

    #[test]
    fn test4() {
        let array: Vec<usize> = vec![1, 5, 9, 1221, 0, 12, 4];
        let buffer: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0];

        let result = mergesort(array, buffer, 0, 6);

        assert_eq!(result, vec![0, 1, 4, 5, 9, 12, 1221]);
    }

    #[test]
    fn test5() {
        let array: Vec<usize> = vec![1, 2, 3, 4, 5];
        let buffer: Vec<usize> = vec![0, 0, 0, 0, 0];

        let result = mergesort(array, buffer, 0, 4);

        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test6() {
        let array: Vec<usize> = vec![5, 4, 3, 2, 1];
        let buffer: Vec<usize> = vec![0, 0, 0, 0, 0];

        let result = mergesort(array, buffer, 0, 4);

        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }
}
