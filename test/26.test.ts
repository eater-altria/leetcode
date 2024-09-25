import { describe, expect, test } from '@jest/globals';
import { removeDuplicates } from '../src/26';

describe('27 Test', () => {
  test('Case 1', () => {
    const nums = [1, 1, 2];
    expect(removeDuplicates(nums)).toBe(2);
    expect(nums).toEqual([1, 2, 2]);
  });

  test('Case 2', () => {
    const nums = [0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    expect(removeDuplicates(nums)).toBe(5);
    expect(nums).toEqual([0, 1, 2, 3, 4, 2, 2, 3, 3, 4]);
  });
});
