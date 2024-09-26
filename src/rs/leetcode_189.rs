fn gcd(x: i32, y: i32) -> i32 {
    if y != 0 {
        gcd(y, x % y)
    } else {
        x
    }
}
fn calc_target_key(length: usize, key: usize, k: i32) -> usize {
    (key + k as usize) % length
}

fn rotate_once(nums: &mut Vec<i32>, k: i32, start: usize, count: i32) {
    let length = nums.len();
    let mut key = start;
    let mut temp = nums[start];
    let mut flag = 0;
    while flag < count {
        let target_key = calc_target_key(length, key, k);
        let val = nums[target_key];
        nums[target_key] = temp;
        temp = val;
        key = target_key;
        flag += 1;
    }
}

pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let length: usize = nums.len();
    let circle_count = gcd(k, length as i32);
    let item_count = length as i32 / circle_count;
    let mut start: usize = 0;
    while start < circle_count as usize {
        rotate_once(nums, k, start, item_count);
        start += 1;
    }
}
