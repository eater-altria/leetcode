use leetcode::rs::leetcode_13::roman_to_int;

#[test]
fn case_1() {
    let s = String::from("III");
    let result = roman_to_int(s);
    assert_eq!(result, 3)
}

#[test]
fn case_2() {
    let s = String::from("LVIII");
    let result = roman_to_int(s);
    assert_eq!(result, 58)
}

#[test]
fn case_3() {
    let s = String::from("MCMXCIV");
    let result = roman_to_int(s);
    assert_eq!(result, 1994)
}
