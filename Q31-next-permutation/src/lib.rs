pub struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {

        if nums.len() < 2 {
            return;
        }

        let mut i = nums.len() - 2;

        while nums[i + 1] <= nums[i] {
             // 防止溢出，这是Rust的限制，usize类型的值不能为负数
            // 如果i - 1为负数，说明nums已经严格的单调递减了
            // 根据题意，直接反转即可
             if i == 0 {
                 return nums.reverse();
             } else {
                 i -= 1;
             }
        }

        {
            let mut j = nums.len() - 1;
            // 找到比nums[i]大的数中最小的数
            while nums[j] <= nums[i] {
                j -= 1;
            }

            let c = nums[i];

            nums[i] = nums[j];
            nums[j] = c;
        }

        // 这里可以优化，不使用这个api
        // 手动对其进行排序
        let (_, b) = nums.split_at_mut(i + 1);

        b.reverse();
    }
}
