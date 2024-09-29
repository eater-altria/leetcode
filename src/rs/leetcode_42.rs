pub fn trap(height: Vec<i32>) -> i32 {
    let length = height.len();
    let mut left_max: Vec<i32> = vec![0; length];
    let mut right_max: Vec<i32> = vec![0; length];
    let mut res = 0;
    let mut max = 0;
    for i in 1..length {
        if height[i - 1] > max {
            max = height[i - 1]
        }
        left_max[i] = max
    }
    max = 0;
    for i in (0..(length - 1)).rev() {
        if height[i + 1] > max {
            max = height[i + 1];
        }
        right_max[i] = max;
    }
    for i in 0..length {
        let min = if left_max[i] > right_max[i] {
            right_max[i]
        } else {
            left_max[i]
        };
        if min > height[i] {
            res += min - height[i];
        }
    }

    return res;
}
