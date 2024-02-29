// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let data = "Rust is great!".to_string();
    let mut data2 = data.clone();

    get_char(&data);

    // data的所有权传递给了string_uppercase
    string_uppercase(data);
    // 不传递所有权的版本
    data2 = string_uppercase2(&data2);
    println!("{}", data2);
    println!("{}", data2);

    data2 = string_uppercase3(data2);
    println!("{}", data2);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}

fn string_uppercase2(data: &String) -> String {
    // 通过let，做shadow。此时，作为左值的data已经是String类型了，而不是&String
    let data = data.to_uppercase();
    println!("{}", data);
    data
}

fn string_uppercase3(mut data: String) -> String {
    data = data.to_uppercase();
    println!("{}", data);
    data
}
