// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.



macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // 宏的调用，需要显式添加！，以和普通的函数调用进行区别
    my_macro!();
}
