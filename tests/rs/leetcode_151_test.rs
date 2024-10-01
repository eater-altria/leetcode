use leetcode::rs::leetcode_151::reverse_words;

#[test]
fn case_1() {
    let input = String::from("the sky is blue");
    let res = reverse_words(input);
    assert_eq!(res, String::from("blue is sky the"))
}

#[test]
fn case_2() {
    let input = String::from("  hello world  ");
    let res = reverse_words(input);
    assert_eq!(res, String::from("world hello"))
}

#[test]
fn case_3() {
    let input = String::from("a good   example");
    let res = reverse_words(input);
    assert_eq!(res, String::from("example good a"))
}

#[test]
fn case_4() {
    let input = String::from("");
    let res = reverse_words(input);
    assert_eq!(res, String::from(""))
}

#[test]
fn case_5() {
    let input = String::from("word");
    let res = reverse_words(input);
    assert_eq!(res, String::from("word"))
}
