pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
  let length = nums.len();
  if length == 0 {
    return 0;
  }
  let mut slow = 1;
  for fast in 1..length {
    if nums[fast] != nums[fast - 1] {
        nums[slow] = nums[fast];
        slow += 1;
    }
  }
  slow.try_into().unwrap()
}