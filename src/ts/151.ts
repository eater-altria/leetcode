export function reverseWords(s: string): string {
  const arr = s.split(' ').filter(i => i !== '');
  const wordCount = arr.length;
  for (let i = 0; i < Math.floor(wordCount / 2); i++) {
    const j = wordCount - 1 - i;
    const word = arr[i];
    arr[i] = arr[j];
    arr[j] = word;
  }
  return arr.join(' ');
}
