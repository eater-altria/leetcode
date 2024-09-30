export function romanToInt(s: string): number {
  const map: Map<string, number> = new Map();
  map.set('I', 1);
  map.set('V', 5);
  map.set('X', 10);
  map.set('L', 50);
  map.set('C', 100);
  map.set('D', 500);
  map.set('M', 1000);
  const arr = s.split('');
  const length = arr.length;
  let res = 0;
  for (let i = 0; i < length - 1; i++) {
    const currentValue = map.get(arr[i]) as number;
    const nextValue = map.get(arr[i + 1]) as number;
    if (currentValue >= nextValue) {
      res += currentValue;
    } else {
      res -= currentValue;
    }
  }
  res += map.get(arr[length - 1]) as number;
  return res;
}
