import { describe, expect, test } from '@jest/globals';
import { maxProfit } from '../../src/ts/121';

describe('121 Test', () => {
  test('Case 1', () => {
    const nums = [7, 1, 5, 3, 6, 4];
    expect(maxProfit(nums)).toBe(5);
  });

  test('Case 2', () => {
    const nums = [7, 6, 4, 3, 1];
    expect(maxProfit(nums)).toBe(0);
  });
});
