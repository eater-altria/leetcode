use leetcode::rs::leetcode_42::trap;

#[test]
fn case_1() {
    let input = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    let expected_output = 6;
    assert_eq!(trap(input), expected_output);
}

#[test]
fn case2() {
    let input = vec![4, 2, 0, 3, 2, 5];
    let expected_output = 9;
    assert_eq!(trap(input), expected_output);
}
