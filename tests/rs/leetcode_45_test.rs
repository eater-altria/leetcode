use leetcode::rs::leetcode_45::jump;

#[test]
fn case_1() {
    let nums = vec![5, 9, 3, 2, 1, 0, 2, 3, 3, 1, 0, 0];
    let result = jump(nums);
    assert_eq!(result, 3);
}

#[test]
fn case_2() {
    let nums = vec![2, 3, 1, 1, 4];
    let result = jump(nums);
    assert_eq!(result, 2);
}

#[test]
fn case_3() {
    let nums = vec![2, 3, 0, 1, 4];
    let result = jump(nums);
    assert_eq!(result, 2);
}