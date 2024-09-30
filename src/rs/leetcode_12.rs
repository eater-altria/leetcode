pub fn int_to_roman(num: i32) -> String {
    let mut num = num;
    let mut res = String::new();
    let symbols: Vec<(i32, &str)> = vec![
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    for (value, symbol) in &symbols {
        while num >= *value {
            num -= *value;
            res += *symbol
        }
    }

    res
}
