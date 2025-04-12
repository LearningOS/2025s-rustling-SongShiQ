// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);  // 正确：传递引用

    // 传递所有权给 string_uppercase
    string_uppercase(data);  // 正确：传递所有权
}

// get_char 接受引用，不需要修改
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// string_uppercase 需要获取所有权
fn string_uppercase(mut data: String) {
    data.to_uppercase();  // 转换为大写

    println!("{}", data);  // 打印转换后的字符串
}
