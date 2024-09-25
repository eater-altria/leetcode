import { describe, expect, test } from '@jest/globals';
import { majorityElement } from '../src/169';

describe('27 Test', () => {
  test('Case 1', () => {
    expect(majorityElement([6, 5, 5])).toBe(5);
  });

  test('Case 2', () => {
    expect(majorityElement([2, 2, 1, 1, 1, 2, 2])).toBe(2);
  });
  test('Case 3', () => {
    expect(majorityElement([3, 2, 3])).toBe(3);
  });
});
