use leetcode::rs::leetcode_14::longest_common_prefix;

#[test]
fn case_1() {
    let strs = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    let res = longest_common_prefix(strs);
    assert_eq!(res, "fl".to_string());
}

#[test]
fn case_2() {
    let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
    let res = longest_common_prefix(strs);
    assert_eq!(res, "".to_string());
}

#[test]
fn case_3() {
    let strs = vec![
        "flower".to_string(),
        "flower".to_string(),
        "flower".to_string(),
        "flower".to_string(),
        "flower".to_string(),
        "flower".to_string(),
    ];
    let res = longest_common_prefix(strs);
    assert_eq!(res, "flower".to_string());
}

#[test]
fn case_4() {
    let strs = vec!["a".to_string()];
    let res = longest_common_prefix(strs);
    assert_eq!(res, "a".to_string());
}
