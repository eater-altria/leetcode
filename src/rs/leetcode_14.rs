pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let length = strs.len();
    if length == 1 {
        return strs[0].clone();
    }
    let str_length = strs[0].len();
    for i in 0..str_length {
        let first_str: Vec<char> = strs[0].chars().collect();
        let first_str_char = first_str[i];
        for j in 1..length {
            let str: Vec<char> = strs[j].chars().collect();
            if i == str.len() {
                let res: String = first_str[..i].to_vec().into_iter().collect();
                return res;
            } else {
                let char = str[i];
                if char != first_str_char {
                    let res: String = first_str[..i].to_vec().into_iter().collect();
                    return res;
                }
            }
        }
    }
    return strs[0].clone();
}
