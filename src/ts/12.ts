export function intToRoman(num: number): string {
  let res = '';
  const symbols: [number, string][] = [
    [1000, 'M'],
    [900, 'CM'],
    [500, 'D'],
    [400, 'CD'],
    [100, 'C'],
    [90, 'XC'],
    [50, 'L'],
    [40, 'XL'],
    [10, 'X'],
    [9, 'IX'],
    [5, 'V'],
    [4, 'IV'],
    [1, 'I'],
  ];
  symbols.forEach(symbol => {
    const val = symbol[0];
    const str = symbol[1];
    while (num >= val) {
      res += str;
      num -= val;
    }
  });
  return res;
}
