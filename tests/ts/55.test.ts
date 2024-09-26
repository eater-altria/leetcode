import { describe, expect, test } from '@jest/globals';
import { canJump } from '../../src/ts/55';

describe('55 Test', () => {
  test('Case 1', () => {
    const nums = [2, 3, 1, 1, 4];
    expect(canJump(nums)).toBe(true);
  });

  test('Case 2', () => {
    const nums = [3, 2, 1, 0, 4];
    expect(canJump(nums)).toBe(false);
  });
});
