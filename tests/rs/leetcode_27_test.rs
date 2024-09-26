use leetcode::rs::leetcode_27;

#[test]
fn case_1() {
  let mut nums = vec![3, 2, 2, 3];
  let result = leetcode_27::remove_element(&mut nums, 3);
  assert_eq!(result, 2);
  assert_eq!(nums, vec![2, 2, 3, 3]);
}

#[test]
fn case_2() {
  let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
  let result = leetcode_27::remove_element(&mut nums, 2);
  assert_eq!(result, 5);
  assert_eq!(nums, vec![0, 1, 4, 0, 3, 2, 2, 2]);
}

#[test]
fn case_3() {
  let mut nums = vec![3, 3];
  let result = leetcode_27::remove_element(&mut nums, 3);
  assert_eq!(result, 0);
  assert_eq!(nums, vec![3, 3]);
}

#[test]
fn case_4() {
  let mut nums = vec![1];
  let result = leetcode_27::remove_element(&mut nums, 1);
  assert_eq!(result, 0);
  assert_eq!(nums, vec![1]);
}
