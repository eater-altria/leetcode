use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let mut map: HashMap<char, i32> = HashMap::new();
    map.insert('I', 1);
    map.insert('V', 5);
    map.insert('X', 10);
    map.insert('L', 50);
    map.insert('C', 100);
    map.insert('D', 500);
    map.insert('M', 1000);

    let chars: Vec<char> = s.chars().collect();
    let length = chars.len();
    let mut res = 0;

    for i in 0..(length - 1) {
        let current_value = *map.get(&chars[i]).unwrap();
        let next_value = *map.get(&chars[i + 1]).unwrap();
        if current_value >= next_value {
            res += current_value;
        } else {
            res -= current_value;
        }
    }

    // Add the value of the last Roman numeral
    res += *map.get(&chars[length - 1]).unwrap();

    res
}
