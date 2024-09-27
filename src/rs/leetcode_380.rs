use rand::Rng;
use std::collections::HashMap;

pub struct RandomizedSet {
    nums: Vec<i32>,
    map: HashMap<i32, usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    pub fn new() -> Self {
        RandomizedSet {
            nums: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, val: i32) -> bool {
        let index = self.map.get(&val);
        match index {
            Some(_) => {
                return false;
            }
            None => {
                self.nums.push(val);
                self.map.insert(val, self.nums.len() - 1);
                return true;
            }
        }
    }

    pub fn remove(&mut self, val: i32) -> bool {
        let index = self.map.get(&val);
        match index {
            None => {
                return false;
            }
            Some(i) => {
                if self.nums.len() - 1 == *i {
                    self.nums.pop();
                    self.map.remove(&val);
                } else {
                    let last_val = self.nums.pop().unwrap();
                    self.nums[*i] = last_val;
                    self.map.insert(last_val, *i);
                    self.map.remove(&val);
                }
                return true;
            }
        }
    }

    pub fn get_random(&mut self) -> i32 {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.nums.len());
        self.nums[idx as usize]
    }
}
