export class RandomizedSet {
  nums: number[] = [];
  map: Map<number, number> = new Map();
  constructor() {}

  insert(val: number): boolean {
    const index = this.map.get(val);
    if (index !== undefined) {
      return false;
    } else {
      this.nums.push(val);
      this.map.set(val, this.nums.length - 1);
      return true;
    }
  }

  remove(val: number): boolean {
    const index = this.map.get(val);
    if (index === undefined) {
      return false;
    } else {
      const lastVal = this.nums[this.nums.length - 1];
      this.nums[index] = lastVal;
      this.map.set(lastVal, index);
      this.map.delete(val);
      this.nums.pop();
      return true;
    }
  }

  getRandom(): number {
    const randomIndex = Math.floor(Math.random() * this.nums.length);
    return this.nums[randomIndex];
  }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * var obj = new RandomizedSet()
 * var param_1 = obj.insert(val)
 * var param_2 = obj.remove(val)
 * var param_3 = obj.getRandom()
 */
