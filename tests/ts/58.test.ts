import { lengthOfLastWord } from '../../src/ts/58';

describe('58 Test', () => {
  test('Case 1', () => {
    const nums = 'Hello world';
    expect(lengthOfLastWord(nums)).toBe(5);
  });

  test('Case 2', () => {
    const nums = '   fly me   to   the moon  ';
    expect(lengthOfLastWord(nums)).toBe(4);
  });

  test('Case 3', () => {
    const nums = 'luffy is still joyboy';
    expect(lengthOfLastWord(nums)).toBe(6);
  });
});
