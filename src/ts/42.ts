export function trap(height: number[]): number {
  const length = height.length;
  const LeftMax: number[] = new Array(length).fill(0);
  const rightMax: number[] = new Array(length).fill(0);
  let max = 0;
  let res = 0;
  for (let i = 1; i < length; i++) {
    if (height[i - 1] > max) {
      max = height[i - 1];
      LeftMax[i] = max;
    } else {
      LeftMax[i] = max;
    }
  }
  max = 0;
  for (let i = length - 2; i >= 0; i--) {
    if (height[i + 1] > max) {
      max = height[i + 1];
      rightMax[i] = max;
    } else {
      rightMax[i] = max;
    }
  }
  for (let i = 1; i < length - 1; i++) {
    const min = Math.min(LeftMax[i], rightMax[i]);
    res += min - height[i] > 0 ? min - height[i] : 0;
  }
  return res;
}
