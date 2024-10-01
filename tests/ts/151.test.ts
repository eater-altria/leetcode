import { reverseWords } from '../../src/ts/151';

describe('151 Test', () => {
  test('Case 1', () => {
    const input = 'the sky is blue';
    const expectedOutput = 'blue is sky the';
    expect(reverseWords(input)).toBe(expectedOutput);
  });

  test('Case 2', () => {
    const input = '  hello world  ';
    const expectedOutput = 'world hello';
    expect(reverseWords(input)).toBe(expectedOutput);
  });

  test('Case 3', () => {
    const input = 'a good   example';
    const expectedOutput = 'example good a';
    expect(reverseWords(input)).toBe(expectedOutput);
  });

  test('Case 4', () => {
    const input = '';
    const expectedOutput = '';
    expect(reverseWords(input)).toBe(expectedOutput);
  });

  test('Case 5', () => {
    const input = 'word';
    const expectedOutput = 'word';
    expect(reverseWords(input)).toBe(expectedOutput);
  });
});
