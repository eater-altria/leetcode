export function hIndex(citations: number[]): number {
  const length = citations.length;
  let counter: number[] = new Array(length + 1).fill(0);
  let tot: number = 0;
  for (let i = 0; i < length; i++) {
    if (citations[i] > length) {
      counter[length]++;
    } else {
      counter[citations[i]]++;
    }
  }
  //设 tot 为引用次数 ≥ i 的论文数，我们需要算出满足 tot ≥ i 的最大的 i。
  // counter[i] 为引用此时为 i 的沦为数，
  // counter[i]的累加即为 tot
  for (let i = length; i >= 0; i--) {
    tot += counter[i];
    if (tot >= i) {
      return i;
    }
  }
  return 0;
}
