use leetcode::rs::leetcode_88::merge;

#[test]
fn case_1() {
    let mut nums1 = vec![1, 3, 5, 0, 0, 0];
    let m = 3;
    let mut nums2 = vec![2, 4, 6];
    let n = 3;
    merge(&mut nums1,m,  &mut nums2, n);
    assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn case_2() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let mut nums2 = vec![2, 5, 6];
    let n = 3;
    merge(&mut nums1,m,  &mut nums2, n);
    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
}

#[test]
fn case_3() {
    let mut nums1 = vec![1];
    let m = 1;
    let mut nums2 = vec![];
    let n = 0;
    merge(&mut nums1,m,  &mut nums2, n);
    assert_eq!(nums1, vec![1]);
}

#[test]
fn case_4() {
    let mut nums1 = vec![0];
    let m = 0;
    let mut nums2 = vec![1];
    let n = 1;
    merge(&mut nums1,m,  &mut nums2, n);
    assert_eq!(nums1, vec![1]);
}