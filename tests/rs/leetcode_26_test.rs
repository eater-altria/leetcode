use leetcode::rs::leetcode_26;

#[test]
fn case_1() {
  let mut nums = vec![1, 1, 2];
  let result = leetcode_26::remove_duplicates(&mut nums);
  assert_eq!(result, 2);
  assert_eq!(nums, vec![1, 2, 2]);
}

#[test]
fn case_2() {
  let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
  let result = leetcode_26::remove_duplicates(&mut nums);
  assert_eq!(result, 5);
  assert_eq!(nums, vec![0, 1, 2, 3, 4, 2, 2, 3, 3, 4]);
}