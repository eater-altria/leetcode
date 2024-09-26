export function jump(nums: number[]): number {
  let count = 0;
  let end = 0;
  let maxPosition = 0;
  const length = nums.length;
  for (let i = 0; i < length - 1; i++) {
    maxPosition = Math.max(maxPosition, i + nums[i]);
    if (i === end) {
      end = maxPosition;
      count++;
    }
  }
  return count;
}
