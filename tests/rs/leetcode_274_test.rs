use leetcode::rs::leetcode_274::h_index;

#[test]
fn case_1() {
    let nums = vec![3, 0, 6, 1, 5];
    let result = h_index(nums);
    assert_eq!(result, 3);
}

#[test]
fn case_2() {
    let nums = vec![1, 3, 1];
    let result = h_index(nums);
    assert_eq!(result, 1);
}
