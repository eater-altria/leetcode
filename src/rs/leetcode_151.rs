pub fn reverse_words(s: String) -> String {
    let mut arr: Vec<&str> = s.split_whitespace().collect();
    let word_count = arr.len();

    for i in 0..word_count / 2 {
        let j = word_count - 1 - i;
        arr.swap(i, j);
    }

    arr.join(" ")
}
