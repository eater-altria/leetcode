export function merge(
  nums1: number[],
  m: number,
  nums2: number[],
  n: number,
): void {
  let i = m - 1;
  let j = n - 1;
  let k = m + n - 1;
  while (k >= 0) {
    const val1 = nums1[i];
    const val2 = nums2[j];
    if (val1 > val2 || val2 === undefined) {
      nums1[k] = val1;
      i--;
    } else if (val1 <= val2 || val1 === undefined) {
      nums1[k] = val2;
      j--;
    }
    k--;
  }
}
