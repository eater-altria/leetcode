import { convert } from '../../src/ts/6';

describe('6 Test', () => {
  test('Case 1', () => {
    const input = 'PAYPALISHIRING';
    const expectedOutput = 'PAHNAPLSIIGYIR';
    expect(convert(input, 3)).toBe(expectedOutput);
  });

  test('Case 2', () => {
    const input = 'PAYPALISHIRING';
    const expectedOutput = 'PINALSIGYAHRPI';
    expect(convert(input, 4)).toBe(expectedOutput);
  });

  test('Case 3', () => {
    const input = 'A';
    const expectedOutput = 'A';
    expect(convert(input, 1)).toBe(expectedOutput);
  });
});
