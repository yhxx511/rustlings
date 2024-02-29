// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!



fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    // 这4个函数，底层都是调用的to_owned()
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());

    // 生成了新的String
    string(format!("Interpolation {}", "Station"));

    // from()生成了新的，但是马上做了切片
    string_slice(&String::from("abc")[0..1]);

    // trim返回str是因为返回值是原始string的一个切片，并未创建新的String对象
    string_slice("  hello there ".trim());
    string_slice("  hello there ".to_owned().trim());

    // replace创建了新String
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("Happy Monday!".replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
