pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
  let length = nums.len();
  if length == 0 {
    return 0;
  }
  let mut k = 0;
  let mut right = length - 1;
  let mut left = 0;
  while left <= right {
    if nums[left] == val {
      k = k + 1;
      while right > left {
        if nums[right] != val {
          let temp = nums[left];
          nums[left] = nums[right];
          nums[right] = temp;
          right -= 1;
          break;
        } else {
          k += 1;
          right -= 1;
        }
      }
    }
    left += 1;
  }

  (length - k).try_into().unwrap()
}