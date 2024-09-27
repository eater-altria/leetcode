import { describe, expect, test } from '@jest/globals';
import { productExceptSelf } from '../../src/ts/238';

describe('238 Test', () => {
  test('Case 1', () => {
    const nums = [1, 2, 3, 4];
    const res = productExceptSelf(nums);
    expect(res).toEqual([24, 12, 8, 6]);
  });

  test('Case 2', () => {
    const nums = [-1, 1, 0, -3, 3];
    const res = productExceptSelf(nums);
    expect(res).toEqual([-0, 0, 9, -0, 0]);
  });
});
