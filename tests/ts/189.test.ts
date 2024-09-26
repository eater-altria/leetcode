import { describe, expect, test } from '@jest/globals';
import { rotate } from '../../src/ts/189';

describe('27 Test', () => {
  test('Case 1', () => {
    const nums = [1, 2, 3, 4];
    rotate(nums, 1);
    expect(nums).toEqual([4, 1, 2, 3]);
  });

  test('Case 2', () => {
    const nums = [1, 2, 3, 4];
    rotate(nums, 2);
    expect(nums).toEqual([3, 4, 1, 2]);
  });
});
