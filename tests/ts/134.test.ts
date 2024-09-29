import { describe, expect, test } from '@jest/globals';
import { canCompleteCircuit } from '../../src/ts/134';

describe('134 Test', () => {
  test('Case 1', () => {
    const gas = [1, 2, 3, 4, 5];
    const costs = [3, 4, 5, 1, 2];
    expect(canCompleteCircuit(gas, costs)).toBe(3);
  });

  test('Case 2', () => {
    const gas = [2, 3, 4];
    const costs = [3, 4, 3];
    expect(canCompleteCircuit(gas, costs)).toBe(-1);
  });
});
