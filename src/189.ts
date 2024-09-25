function clacTargetKey(len: number, key: number, k: number) {
  return (key + k) % len;
}

const gcd = (x: number, y: number): number => (y ? gcd(y, x % y) : x);

function rotateOnce(
  nums: number[],
  k: number,
  start: number,
  count: number,
): void {
  const len = nums.length;
  let key = start;
  let temp = nums[start];
  let flag = 0;
  // let targetKey = clacTargetKey(len, key, k);
  while (flag < count) {
    const targetKey = clacTargetKey(len, key, k);
    const val = nums[targetKey];
    nums[targetKey] = temp;
    temp = val;
    key = targetKey;
    flag++;
  }
}

export function rotate(nums: number[], k: number): void {
  const len = nums.length;
  const circleCount = gcd(k, len);
  const itemCount = len / circleCount;
  let start = 0;
  while (start < circleCount) {
    rotateOnce(nums, k, start, itemCount);
    start++;
  }
}
