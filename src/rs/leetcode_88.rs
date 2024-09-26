pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut i: isize = (m - 1).try_into().unwrap();
    let mut j: isize = (n - 1).try_into().unwrap();
    let mut k: isize = (m + n - 1).try_into().unwrap();
    while k >= 0 {
        let val1 = if i >= 0 {
            Some(nums1[i as usize])
        } else {
            None
        };
        let val2 = if j >= 0 {
            Some(nums2[j as usize])
        } else {
            None
        };
        if val2.is_none() || (val1.is_some() && val1.unwrap() > val2.unwrap()) {
            nums1[k as usize] = val1.unwrap();
            i -= 1;
        } else {
            nums1[k as usize] = val2.unwrap();
            j -= 1;
        }
        k -= 1;
    }
}
