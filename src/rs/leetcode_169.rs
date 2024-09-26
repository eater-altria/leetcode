pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut count = 1;
    let mut certain = nums[0];
    let lenght = nums.len();

    for i in 1..lenght {
        if nums[i] == certain {
            count += 1;
        } else {
            if count >= 1 {
                count -= 1;
            } else {
                certain = nums[i];
                count = 1;
            }
        }
    }

    certain
}
