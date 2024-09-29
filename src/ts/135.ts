export function candy(ratings: number[]): number {
  const length = ratings.length;
  let res = 0;
  let left: number[] = [1];
  for (let i = 1; i < length; i++) {
    if (ratings[i] > ratings[i - 1]) {
      left[i] = left[i - 1] + 1;
    } else {
      left[i] = 1;
    }
  }
  let right = 1;
  for (let i = length - 1; i >= 0; i--) {
    if (i !== length - 1 && ratings[i] > ratings[i + 1]) {
      right++;
    } else {
      right = 1;
    }
    res += Math.max(left[i], right);
  }
  return res;
}
