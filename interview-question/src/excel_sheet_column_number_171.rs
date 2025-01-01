fn title_to_number(column_title: String) -> i32 {
    let alphabets = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let base:i32 = 26;
    let mut result = 0;
    for (i, c) in column_title.chars().enumerate() {
        let pos = alphabets.chars().position(|x| x == c).unwrap() as i32 + 1;
        result += pos * base.pow((column_title.len() -1 - i) as u32);
    }
    result
}

pub fn run() {
    let column_title = "ZY".to_string();
    let result = title_to_number(column_title);
    println!("result: {}", result);
}
