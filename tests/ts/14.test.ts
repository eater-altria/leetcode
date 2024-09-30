import { longestCommonPrefix } from '../../src/ts/14';

describe('14 Test', () => {
  test('Case 1', () => {
    const strs = ['flower', 'flow', 'flight'];
    expect(longestCommonPrefix(strs)).toBe('fl');
  });

  test('Case 2', () => {
    const strs = ['dog', 'racecar', 'car'];
    expect(longestCommonPrefix(strs)).toBe('');
  });

  test('Case 3', () => {
    const strs = ['flower', 'flower', 'flower', 'flower', 'flower', 'flower'];
    expect(longestCommonPrefix(strs)).toBe('flower');
  });

  test('Case 4', () => {
    const strs = ['a'];
    expect(longestCommonPrefix(strs)).toBe('a');
  });
});
