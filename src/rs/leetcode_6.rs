pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }
    let mut row_arr: Vec<String> = vec![String::from(""); num_rows as usize];
    #[derive(PartialEq)]
    enum DIRECTION {
        Down,
        Up,
    }

    let mut direction = DIRECTION::Down;
    let mut current_row = 0;
    let chars: Vec<char> = s.chars().collect();
    let length = chars.len();
    for i in 0..length {
        row_arr[current_row] = row_arr[current_row].clone() + &chars[i].to_string();
        if current_row == 0 {
            if direction == DIRECTION::Up {
                direction = DIRECTION::Down;
            }
            current_row += 1;
        } else if current_row == (num_rows as usize - 1){
            if direction == DIRECTION::Down {
                direction = DIRECTION::Up;
            }
            current_row -= 1;
        } else {
            current_row = match direction {
                DIRECTION::Down => current_row + 1,
                DIRECTION::Up => current_row - 1,
            }
        }
    }
    return row_arr.join("");
}
