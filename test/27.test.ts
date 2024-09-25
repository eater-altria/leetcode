import { describe, expect, test } from '@jest/globals';
import { removeElement } from '../src/27';

describe('27 Test', () => {
  test('Case 1', () => {
    const nums = [3, 2, 2, 3];
    const val = 3;
    expect(removeElement(nums, val)).toBe(2);
    expect(nums).toEqual([2, 2, 3, 3]);
  });

  test('Case 2', () => {
    const nums = [0, 1, 2, 2, 3, 0, 4, 2];
    const val = 2;
    expect(removeElement(nums, val)).toBe(5);
    expect(nums).toEqual([0, 1, 4, 0, 3, 2, 2, 2]);
  });

  test('Case 3', () => {
    const nums = [3, 3];
    const val = 3;
    expect(removeElement(nums, val)).toBe(0);
    expect(nums).toEqual([3, 3]);
  });

  test('Case 4', () => {
    const nums = [1];
    const val = 1;
    expect(removeElement(nums, val)).toBe(0);
    expect(nums).toEqual([1]);
  });
});
