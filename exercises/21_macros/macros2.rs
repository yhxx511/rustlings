// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.




// TODO 需要先定义宏，再使用宏。这个顺序不能错。这与其他东西的规则是不同的
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
