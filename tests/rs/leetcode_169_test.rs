use leetcode::rs::leetcode_169::majority_element;

#[test]
fn case_1() {
    let nums = vec![6, 5, 5];
    let result = majority_element(nums);
    assert_eq!(result, 5);
}

#[test]
fn case_2() {
    let nums = vec![2, 2, 1, 1, 1, 2, 2];
    let result = majority_element(nums);
    assert_eq!(result, 2);
}

#[test]
fn case_3() {
    let nums = vec![3, 2, 3];
    let result = majority_element(nums);
    assert_eq!(result, 3);
}
