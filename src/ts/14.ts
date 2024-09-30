export function longestCommonPrefix(strs: string[]): string {
  const length = strs.length;
  if (length == 1) {
    return strs[0];
  }
  const strLength = strs[0].length;
  for (let i = 0; i < strLength; i++) {
    const firstStr = strs[0];
    const firstStrChar = firstStr[i];
    for (let j = 1; j < length; j++) {
      let str = strs[j];
      if (i === str.length) {
        return firstStr.substring(0, i);
      } else {
        let char = str[i];
        if (char !== firstStrChar) {
          return firstStr.substring(0, i);
        }
      }
    }
  }
  return strs[0];
}
