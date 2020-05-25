use std::collections::HashMap;

pub struct Solution {}

struct GCD {
    inner: HashMap<(i32, i32), i32>,
}

impl GCD {
    fn new() -> Self {
        GCD {
            inner: HashMap::new(),
        }
    }

    fn gcd(&mut self, a: i32, b: i32) -> i32 {
        if let Some(v) = self.inner.get(&(a, b)) {
            *v
        } else {
            if b == 0 {
                return a;
            }

            let r = self.gcd(b, a % b);
            self.inner.insert((a, b), r);
            r
        }
    }
}

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() < 3 {
            return points.len() as i32;
        }

        let mut res = 0;
        let mut gcd = GCD::new();
        let mut slope: HashMap<(i32, i32), i32> = HashMap::new();

        for i in 0..(points.len() - 1) {
            let mut repeat = 0;
            let mut tmp_max = 0;
            slope.clear();

            for j in (i + 1)..points.len() {
                let mut dy = points[i][1] - points[j][1];
                let mut dx = points[i][0] - points[j][0];

                if dx == 0 && dy == 0 {
                    repeat += 1;
                    continue;
                }

                let g = gcd.gcd(dy, dx);

                if g != 0 {
                    dy /= g;
                    dx /= g;
                }

                let tmp = (dy, dx);

                if let Some(v) = slope.get_mut(&tmp) {
                    *v += 1;
                    tmp_max = tmp_max.max(*v);
                } else {
                    slope.insert(tmp, 1);
                    tmp_max = tmp_max.max(1);
                }
            }

            res = res.max(repeat + tmp_max + 1);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let input = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        assert_eq!(Solution::max_points(input), 3);

        let input = vec![
            vec![1, 1],
            vec![3, 2],
            vec![5, 3],
            vec![4, 1],
            vec![2, 3],
            vec![1, 4],
        ];
        assert_eq!(Solution::max_points(input), 4);
    }
}
