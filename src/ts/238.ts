export function productExceptSelf(nums: number[]): number[] {
  const length = nums.length;
  let temp = 1;
  let res: number[] = new Array(length).fill(1);
  for (let i = 1; i < length; i++) {
    res[i] = res[i - 1] * nums[i - 1];
  }
  for (let i = length - 2; i > -1; i--) {
    temp *= nums[i + 1];
    res[i] *= temp;
  }
  return res;
}
