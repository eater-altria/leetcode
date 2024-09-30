export function lengthOfLastWord(s: string): number {
  const words = s.split(' ').filter(item => item !== '');
  const length = words.length;
  return words[length - 1].length;
}
