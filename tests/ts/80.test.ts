import { describe, expect, test } from '@jest/globals';
import { removeDuplicates } from '../../src/ts/80';

describe('80 Test', () => {
  test('Case 1', () => {
    const nums = [1, 1, 1, 2, 2, 3];
    expect(removeDuplicates(nums)).toBe(5);
    expect(nums).toEqual([1, 1, 2, 2, 3, 3]);
  });

  test('Case 2', () => {
    const nums = [0, 0, 1, 1, 1, 1, 2, 3, 3];
    expect(removeDuplicates(nums)).toBe(7);
    expect(nums).toEqual([0, 0, 1, 1, 2, 3, 3, 3, 3]);
  });
});
