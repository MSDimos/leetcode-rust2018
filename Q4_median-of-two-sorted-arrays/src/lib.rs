use std::mem::swap;

pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut A = nums1;
        let mut B = nums2;
        let m = A.len();
        let n = m;
        let (mut imin, mut imax): (usize, usize) = (0, m);
        let half = (m + n + 1) / 2;

        if m > n {
            swap(&mut A, &mut B);
        }

        while imin <= imax {
            let i = (imin + imax) / 2;
            let j = half - i;

            if i < imax && B[j - 1] > A[i] {
                imin = i + 1;
            } else if i > imin && A[i - 1] > B[j] {
                imax = i - 1;
            } else {
                let mut max_left = 0;
                let mut min_right = 0;

                if i == 0 {
                    max_left = B[j - 1];
                } else if j == 0 {
                    max_left = A[i - 1];
                } else {
                    max_left = A[i -1].max(B[j - 1]);
                }

                if (m + n) % 2 == 1 {
                    return max_left as f64;
                }

                if i == m {
                    min_right = B[j];
                } else if j == n {
                    min_right = A[i];
                } else {
                    min_right = A[i].max(B[j]);
                }

                return ((min_right + max_left) / 2) as f64;
            }
        }



        0.0
    }
}
