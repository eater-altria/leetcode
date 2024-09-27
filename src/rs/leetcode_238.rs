pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let length = nums.len();
    let mut res: Vec<i32> = vec![1; length];
    let mut temp = 1;
    for i in 1..length {
        res[i] = res[i - 1] * nums[i - 1];
    }
    for i in (0..length - 1).rev() {
        temp *= nums[i + 1];
        res[i] *= temp;
    }

    return res;
}
