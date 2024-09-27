import { describe, expect, test } from '@jest/globals';
import { RandomizedSet } from '../../src/ts/380';

describe('380 Test', () => {
  test('Case 1', () => {
    const r = new RandomizedSet();
    expect(r.insert(1)).toBe(true);
    expect(r.remove(2)).toBe(false);
    expect(r.insert(2)).toBe(true);
    expect([1, 2]).toContain(r.getRandom());
    expect(r.remove(1)).toBe(true);
    expect(r.insert(2)).toBe(false);
    expect([2]).toContain(r.getRandom());
  });

  test('Case 2', () => {
    const r = new RandomizedSet();
    expect(r.remove(0)).toBe(false);
    expect(r.remove(0)).toBe(false);
    expect(r.insert(0)).toBe(true);
    expect([0]).toContain(r.getRandom());
    expect(r.remove(0)).toBe(true);
    expect(r.insert(0)).toBe(true);
  });
  test('Case 3', () => {
    const r = new RandomizedSet();
    expect(r.insert(1)).toBe(true);
    expect(r.insert(10)).toBe(true);
    expect(r.insert(20)).toBe(true);
    expect(r.insert(30)).toBe(true);
    expect([1, 10, 20, 30]).toContain(r.getRandom());
    expect([1, 10, 20, 30]).toContain(r.getRandom());
    expect([1, 10, 20, 30]).toContain(r.getRandom());
    expect([1, 10, 20, 30]).toContain(r.getRandom());
    expect([1, 10, 20, 30]).toContain(r.getRandom());
  });
});
