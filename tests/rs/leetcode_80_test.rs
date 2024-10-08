use leetcode::rs::leetcode_80::remove_duplicates;

#[test]
fn case_1() {
    let mut nums = vec![1, 1, 1, 2, 2, 3];
    let result = remove_duplicates(&mut nums);
    assert_eq!(result, 5);
    assert_eq!(nums, vec![1, 1, 2, 2, 3, 3]);
}

#[test]
fn case_2() {
    let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
    let result = remove_duplicates(&mut nums);
    assert_eq!(result, 7);
    assert_eq!(nums, vec![0, 0, 1, 1, 2, 3, 3, 3, 3]);
}
