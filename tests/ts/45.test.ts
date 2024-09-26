import { describe, expect, test } from '@jest/globals';
import { jump } from '../../src/ts/45';

describe('45 Test', () => {
  test('Case 1', () => {
    const nums = [5, 9, 3, 2, 1, 0, 2, 3, 3, 1, 0, 0];
    expect(jump(nums)).toBe(3);
  });

  test('Case 2', () => {
    const nums = [2, 3, 1, 1, 4];
    expect(jump(nums)).toBe(2);
  });

  test('Case 3', () => {
    const nums = [2, 3, 0, 1, 4];
    expect(jump(nums)).toBe(2);
  });
});
