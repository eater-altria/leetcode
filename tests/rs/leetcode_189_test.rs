use leetcode::rs::leetcode_189::rotate;

#[test]
fn case_1() {
    let mut nums = vec![1, 2, 3, 4];
    rotate(&mut nums, 1);
    assert_eq!(nums, vec![4, 1, 2, 3]);
}

#[test]
fn case_2() {
    let mut nums = vec![1, 2, 3, 4];
    rotate(&mut nums, 2);
    assert_eq!(nums, vec![3, 4, 1, 2]);
}