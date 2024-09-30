import { romanToInt } from '../../src/ts/13';

describe('13 Test', () => {
  test('Case 1', () => {
    const s = 'III';
    expect(romanToInt(s)).toBe(3);
  });

  test('Case 2', () => {
    const s = 'LVIII';
    expect(romanToInt(s)).toBe(58);
  });

  test('Case 3', () => {
    const s = 'MCMXCIV';
    expect(romanToInt(s)).toBe(1994);
  });
});
