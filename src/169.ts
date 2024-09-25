export function majorityElement(nums: number[]): number {
  let count = 1;
  let certain = nums[0];
  const length = nums.length;
  for (let i = 1; i < length; i++) {
    if (nums[i] === certain) {
      count++;
    } else {
      if (count >= 1) {
        count--;
      } else {
        certain = nums[i];
        count = 1;
      }
    }
  }
  return certain;
}
