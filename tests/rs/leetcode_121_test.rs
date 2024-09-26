use leetcode::rs::leetcode_121::max_profit;

#[test]
fn case_1() {
    let nums = vec![7, 1, 5, 3, 6, 4];
    let res = max_profit(nums);
    assert_eq!(res, 5);
}

#[test]
fn case_2() {
    let nums = vec![7, 6, 4, 3, 1];
    let res = max_profit(nums);
    assert_eq!(res, 0);
}
