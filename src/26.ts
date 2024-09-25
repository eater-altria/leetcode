export function removeDuplicates(nums: number[]): number {
  const length = nums.length;
  if (length === 0) return 0;
  let slow = 1;
  let fast = 1;
  for (; fast < length; fast++) {
    if (nums[fast] !== nums[fast - 1]) {
      nums[slow] = nums[fast];
      slow++;
    }
  }
  return slow;
}
