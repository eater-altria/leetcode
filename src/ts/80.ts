export function removeDuplicates(nums: number[]): number {
  const length = nums.length;
  if (length <= 2) return length;
  let slow = 2;
  let fast = 2;
  for (; fast < length; fast++) {
    if (nums[slow - 2] != nums[fast]) {
      nums[slow] = nums[fast];
      slow++;
    }
  }
  return slow;
}
