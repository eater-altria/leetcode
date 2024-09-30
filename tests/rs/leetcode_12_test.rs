use leetcode::rs::leetcode_12::int_to_roman;

#[test]
fn case_1() {
    let num = 3749;
    let result = int_to_roman(num);
    assert_eq!(result, String::from("MMMDCCXLIX"))
}

#[test]
fn case_2() {
    let num = 58;
    let result = int_to_roman(num);
    assert_eq!(result, String::from("LVIII"))
}

#[test]
fn case_3() {
    let num = 1994;
    let result = int_to_roman(num);
    assert_eq!(result, String::from("MCMXCIV"))
}
