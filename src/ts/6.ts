export function convert(s: string, numRows: number): string {
  if (numRows === 1) return s;
  const rowArr: string[] = new Array(numRows).fill('');
  const length = s.length;
  enum DIRECTION {
    'down',
    'up',
  }
  let currentRow = 0;
  let direction = DIRECTION.down;
  for (let i = 0; i < length; i++) {
    if (currentRow === 0) {
      if (direction === DIRECTION.up) {
        direction = DIRECTION.down;
      }
      rowArr[currentRow] += s[i];
      currentRow += 1;
    } else if (currentRow === numRows - 1) {
      // 到达第numRow 行
      if (direction === DIRECTION.down) {
        direction = DIRECTION.up;
      }
      rowArr[currentRow] += s[i];
      currentRow -= 1;
    } else {
      rowArr[currentRow] += s[i];
      currentRow =
        direction === DIRECTION.down ? currentRow + 1 : currentRow - 1;
    }
  }
  return rowArr.join('');
}
