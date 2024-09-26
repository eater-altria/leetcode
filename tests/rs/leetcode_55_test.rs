use leetcode::rs::leetcode_55::can_jump;

#[test]
fn case_1() {
    let nums = vec![2, 3, 1, 1, 4];
    let result = can_jump(nums);
    assert_eq!(result, true);
}

#[test]
fn case_2() {
    let nums = vec![3, 2, 1, 0, 4];
    let result = can_jump(nums);
    assert_eq!(result, false);
}
