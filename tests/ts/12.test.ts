import { intToRoman } from '../../src/ts/12';

describe('13 Test', () => {
  test('Case 1', () => {
    const num = 3749;
    expect(intToRoman(num)).toBe('MMMDCCXLIX');
  });

  test('Case 2', () => {
    const num = 58;
    expect(intToRoman(num)).toBe('LVIII');
  });

  test('Case 3', () => {
    const num = 1994;
    expect(intToRoman(num)).toBe('MCMXCIV');
  });
});
