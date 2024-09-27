use leetcode::rs::leetcode_238::product_except_self;

#[test]
fn case_1() {
    let nums = vec![1, 2, 3, 4];
    let res = product_except_self(nums);
    assert_eq!(res, vec![24, 12, 8, 6])
}

#[test]
fn case_2() {
    let nums = vec![-1, 1, 0, -3, 3];
    let res = product_except_self(nums);
    assert_eq!(res, vec![0, 0, 9, 0, 0])
}
