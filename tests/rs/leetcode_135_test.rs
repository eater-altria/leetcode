use leetcode::rs::leetcode_135::candy;

#[test]
fn test_candy_example_1() {
    let input = vec![1, 0, 2];
    let expected_output = 5;
    assert_eq!(candy(input), expected_output);
}

#[test]
fn test_candy_example_2() {
    let input = vec![1, 2, 2];
    let expected_output = 4;
    assert_eq!(candy(input), expected_output);
}
