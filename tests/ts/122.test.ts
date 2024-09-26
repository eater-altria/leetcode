import { describe, expect, test } from '@jest/globals';
import { maxProfit } from '../../src/ts/122';

describe('122 Test', () => {
  test('Case 1', () => {
    const nums = [7, 1, 5, 3, 6, 4];
    expect(maxProfit(nums)).toBe(7);
  });

  test('Case 2', () => {
    const nums = [1, 2, 3, 4, 5];
    expect(maxProfit(nums)).toBe(4);
  });

  test('Case 3', () => {
    const nums = [7, 6, 4, 3, 1];
    expect(maxProfit(nums)).toBe(0);
  });
});
