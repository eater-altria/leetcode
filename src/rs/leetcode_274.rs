pub fn h_index(citations: Vec<i32>) -> i32 {
    let length = citations.len();
    let mut tot = 0;
    let mut counter: Vec<i32> = vec![0; length + 1];
    for i in 0..length {
        counter[if citations[i] > length as i32 {
            length
        } else {
            citations[i] as usize
        }] += 1;
    }
    for i in (0..length + 1).rev() {
        tot += counter[i];
        if tot >= i as i32 {
            return i as i32;
        }
    }
    0
}
