import { describe, expect, test } from '@jest/globals';
import { hIndex } from '../../src/ts/274';

describe('274 Test', () => {
  test('Case 1', () => {
    const nums = [3, 0, 6, 1, 5];
    const reslut = hIndex(nums);
    expect(reslut).toBe(3);
  });

  test('Case 2', () => {
    const nums = [1, 3, 1];
    const reslut = hIndex(nums);
    expect(reslut).toBe(1);
  });
});
