pub fn length_of_last_word(s: String) -> i32 {
    let words: Vec<&str> = s.split_whitespace().collect(); // 使用 split_whitespace 处理空白字符
    if let Some(last_word) = words.last() {
        last_word.len() as i32
    } else {
        0
    }
}
