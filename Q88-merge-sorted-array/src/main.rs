struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, mut m: i32, nums2: &mut Vec<i32>, mut n: i32) {
        let mut p = (m + n - 1) as usize;
        let mut m = m - 1;
        let mut n = n - 1;

        // move n elements from nums2 to nums1
        while n >= 0 {
            // if nums1[m] > nums2[n], move nums1[m] to nums1[p]
            nums1[p] = if m >= 0 && nums1[m as usize] > nums2[n as usize] {
                let v = nums1[m as usize];
                m -= 1;
                v
            } else {
                let v = nums2[n as usize];
                n -= 1;
                v
            };

            p -= 1;
        }
    }
}

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    let mut output = vec![1, 2, 2, 3, 5, 6];
    Solution::merge(&mut nums1, 3, &mut nums2, 3);
    assert_eq!(output, nums1);
}
