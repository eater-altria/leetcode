pub fn candy(ratings: Vec<i32>) -> i32 {
    let length = ratings.len();
    let mut res = 0;
    let mut left = vec![1; length];
    for i in 1..length {
        left[i] = if ratings[i] > ratings[i - 1] {
            left[i - 1] + 1
        } else {
            1
        };
    }
    let mut right = 1;
    for i in (0..length).rev() {
        right = if i != length - 1 && ratings[i] > ratings[i + 1] {
            right + 1
        } else {
            1
        };
        res += if left[i] > right { left[i] } else { right };
    }
    return res;
}
