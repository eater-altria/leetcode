use std::cmp::max;

pub fn jump(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut end:i32 = 0;
    let mut max_position = 0;
    let length = nums.len();
    for i in 0..length - 1 {
        max_position = max(max_position, nums[i] + i as i32);
        if i as i32 == end {
            end = max_position;
            count += 1;
        }
    }

    count
}
