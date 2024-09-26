use std::cmp::max;

pub fn can_jump(nums: Vec<i32>) -> bool {
    let length = nums.len();
    let mut dp: Vec<i32> = vec![0; length];
    dp[0] = nums[0];
    for i in 1..length {
        let pre = dp[i - 1];
        if pre <= -1 {
            return false;
        } else {
            dp[i] = max(pre - 1, nums[i - 1] - 1)
        }
    }
    dp[length - 1] >= 0
}
