use leetcode::rs::leetcode_6::convert;

#[test]
fn case_1() {
    let s = String::from("PAYPALISHIRING");
    let res = convert(s, 3);
    assert_eq!(res, String::from("PAHNAPLSIIGYIR"));
}

#[test]
fn case_2() {
    let s = String::from("PAYPALISHIRING");
    let res = convert(s, 4);
    assert_eq!(res, String::from("PINALSIGYAHRPI"));
}

#[test]
fn case_3() {
    let s = String::from("A");
    let res = convert(s, 1);
    assert_eq!(res, String::from("A"));
}
