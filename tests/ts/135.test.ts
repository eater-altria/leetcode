import { candy } from '../../src/ts/135';

describe('candy', () => {
  test('should return 5 for input [1, 0, 2]', () => {
    const input = [1, 0, 2];
    const expectedOutput = 5;
    expect(candy(input)).toBe(expectedOutput);
  });

  test('should return 4 for input [1, 2, 2]', () => {
    const input = [1, 2, 2];
    const expectedOutput = 4;
    expect(candy(input)).toBe(expectedOutput);
  });
});
