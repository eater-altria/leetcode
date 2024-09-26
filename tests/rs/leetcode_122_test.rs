use leetcode::rs::leetcode_122::max_profit;

#[test]
fn case_1() {
    let nums = vec![7, 1, 5, 3, 6, 4];
    let res = max_profit(nums);
    assert_eq!(res, 7);
}

#[test]
fn case_2() {
    let nums = vec![1, 2, 3, 4, 5];
    let res = max_profit(nums);
    assert_eq!(res, 4);
}

#[test]
fn case_3() {
    let nums = vec![7, 6, 4, 3, 1];
    let res = max_profit(nums);
    assert_eq!(res, 0);
}
