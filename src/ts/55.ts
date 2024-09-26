export function canJump(nums: number[]): boolean {
  const dp: number[] = [];
  const length = nums.length;
  dp[0] = nums[0];
  for (let i = 1; i < length; i++) {
    const pre = dp[i - 1];
    if (pre <= -1) {
      return false;
    } else {
      dp[i] = Math.max(pre - 1, nums[i - 1] - 1);
    }
  }
  return dp[length - 1] >= 0;
}
