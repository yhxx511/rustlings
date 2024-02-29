// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.



mod macros {
    // TODO  export之后，使用的地方就不用显式添加mod名称前缀了。如果是有重名的export怎么办？如果想要跨模块应用怎么办？
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
