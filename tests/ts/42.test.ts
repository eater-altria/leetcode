import { trap } from '../../src/ts/42';

describe('42 Test', () => {
  test('Case 1', () => {
    const input = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    const expectedOutput = 6;
    expect(trap(input)).toBe(expectedOutput);
  });

  test('Case2', () => {
    const input = [4, 2, 0, 3, 2, 5];
    const expectedOutput = 9;
    expect(trap(input)).toBe(expectedOutput);
  });
});
