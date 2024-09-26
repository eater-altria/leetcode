pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let length = nums.len();
    if length <= 2 {
        return length.try_into().unwrap();
    }
    let mut slow = 2;
    let mut fast = 2;
    while fast < length {
        if nums[slow - 2] != nums[fast] {
            nums[slow] = nums[fast];
            slow += 1;
        }

        fast += 1;
    }
    slow.try_into().unwrap()
}
