use leetcode::rs::leetcode_58::length_of_last_word;

#[test]
fn case_1() {
    let str = String::from("Hello World");
    let res = length_of_last_word(str);
    assert_eq!(res, 5);
}

#[test]
fn case_2() {
    let str = String::from("   fly me   to   the moon  ");
    let res = length_of_last_word(str);
    assert_eq!(res, 4);
}

#[test]
fn case_3() {
    let str = String::from("luffy is still joyboy");
    let res = length_of_last_word(str);
    assert_eq!(res, 6);
}
