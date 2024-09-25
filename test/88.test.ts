import { describe, expect, test } from '@jest/globals';
import { merge } from '../src/88';

describe('88 Test', () => {
  test('Case 1', () => {
    const nums1 = [1, 3, 5, 0, 0, 0];
    const m = 3;
    const nums2 = [2, 4, 6];
    const n = 3;
    merge(nums1, m, nums2, n);
    expect(nums1).toEqual([1, 2, 3, 4, 5, 6]);
  });

  test('Case 2', () => {
    const nums1 = [1, 2, 3, 0, 0, 0];
    const m = 3;
    const nums2 = [2, 5, 6];
    const n = 3;
    merge(nums1, m, nums2, n);
    expect(nums1).toEqual([1, 2, 2, 3, 5, 6]);
  });

  test('Case 3', () => {
    const nums1 = [1];
    const m = 1;
    const nums2: number[] = [];
    const n = 0;
    merge(nums1, m, nums2, n);
    expect(nums1).toEqual([1]);
  });

  test('Case 4', () => {
    const nums1 = [0];
    const m = 0;
    const nums2: number[] = [1];
    const n = 1;
    merge(nums1, m, nums2, n);
    expect(nums1).toEqual([1]);
  });
});
