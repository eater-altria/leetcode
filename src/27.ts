export function removeElement(nums: number[], val: number): number {
  const length = nums.length;
  let k = 0;
  let right = length - 1;
  for (let left = 0; left <= right; left++) {
    if (nums[left] === val) {
      k = k + 1;
      for (; right > left; right--) {
        if (nums[right] !== val) {
          const temp = nums[left];
          nums[left] = nums[right];
          nums[right] = temp;
          right--;
          break;
        } else {
          k = k + 1;
        }
      }
    }
  }
  return length - k;
}
