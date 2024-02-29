// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.



#[test]
fn main() {
    let mut x = 100;

    let y = &mut x;
    *y += 100;

    // 这里可以直接再次进行mut ref，rust会自动销毁y，因为后面的代码中没有再使用y
    let z = &mut x;
    *z += 1000;

    // 这里不能继续使用mutable ref了。因为z已经是第二次借用
    // println!("{:?}", y);

    assert_eq!(x, 1200);
}
