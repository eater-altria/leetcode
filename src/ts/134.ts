export function canCompleteCircuit(gas: number[], cost: number[]): number {
  const length = gas.length;
  let i = 0;

  // 原理，从 0 开始检查，看看能不能走 length 个节点
  // 如果 从 0 开始，只能走i个，走不到 length 个
  // 那么 [0, i]在内的其他节点开始，不会好于现在，不如从 i + 1开始
  while (i < length) {
    let sumGas = 0;
    let sumCost = 0;
    let current = 0;
    while (current < length) {
      const j = (i + current) % length; // 下标
      sumGas += gas[j];
      sumCost += cost[j];
      if (sumCost > sumGas) {
        break;
      }
      current++;
    }
    if (current === length) {
      return i;
    } else {
      i = i + current + 1;
    }
  }
  return -1;
}
