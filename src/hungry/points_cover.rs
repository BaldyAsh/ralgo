use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LineSide {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Eq)]
pub struct Point {
    pub num_line: usize,
    pub point_val: usize,
    pub line_side: LineSide,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.point_val.eq(&other.point_val) {
            match self.line_side {
                LineSide::Left => return Ordering::Less,
                LineSide::Right => return Ordering::Greater
            }
        }

        self.point_val.cmp(&other.point_val)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.point_val == other.point_val && self.line_side == other.line_side
    }
}

pub fn points_cover(lines: &Vec<(usize, usize)>) -> Vec<usize> {
    let lines = lines.clone();

    let len = lines.len();

    let mut list_points: Vec<Point> = vec![];

    for i in 0 .. len {
        list_points.push(Point {
            num_line: i + 1,
            point_val: lines[i].0,
            line_side: LineSide::Left,
        });

        list_points.push(Point {
            num_line: i + 1,
            point_val: lines[i].1,
            line_side: LineSide::Right,
        });
    }

    list_points.sort();

    let len = list_points.len();

    let mut stack: Vec<usize> = vec![];
    let mut result: Vec<usize> = vec![];

    for i in 0 .. len {
        if list_points[i].line_side == LineSide::Left {
            stack.push(list_points[i].num_line);
        } else if stack.contains(&list_points[i].num_line) {
            result.push(list_points[i].point_val);
            stack.clear();
        }
    }

    result
}


#[cfg(test)]
mod tests_points_cover {
    use super::points_cover;

    #[test]
    fn test0() {
        let lines: Vec<(usize, usize)> = vec![
            (4, 7),
            (1, 3),
            (2, 5),
            (5, 6),
            (6, 8),
            (3, 4)
        ];
        let points = points_cover(&lines);

        assert_eq!(points, vec![3, 6]);
    }

    #[test]
    fn test1() {
        let lines: Vec<(usize, usize)> = vec![
            (1, 3),
            (2, 5),
            (3, 6)
        ];
        let points = points_cover(&lines);

        assert_eq!(points, vec![3]);
    }

    #[test]
    fn test2() {
        let lines: Vec<(usize, usize)> = vec![
            (5, 6),
            (4, 7),
            (3, 8),
            (2, 9),
            (1, 10)
        ];
        let points = points_cover(&lines);

        assert_eq!(points, vec![6]);
    }

    #[test]
    fn test3() {
        let lines: Vec<(usize, usize)> = vec![
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 5),
            (5, 6)
        ];
        let points = points_cover(&lines);

        assert_eq!(points, vec![2, 4, 6]);
    }

    #[test]
    fn test4() {
        let lines: Vec<(usize, usize)> = vec![
            (1, 2),
            (3, 4),
            (5, 6),
            (7, 8),
            (9, 10)
        ];
        let points = points_cover(&lines);

        assert_eq!(points, vec![2, 4, 6, 8, 10]);
    }
}
