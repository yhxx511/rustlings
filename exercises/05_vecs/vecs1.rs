// vecs1.rs
//
// Your task is to create a `Vec` which holds the exact same elements as in the
// array `a`.
//
// Make me compile and pass the test!
//
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.



fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    // let v: Vec<i32> = vec!(a); // TODO: declare your vector here with the macro for vectors
    // 只能自己写一个循环来进行复制吗？有没有现成的库函数来做这个事情？
    let mut v: Vec<i32> = Vec::new();
    for element in a {
        v.push(element)
    }

    (a, v)
}

fn array_and_vec2() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    // 看from的实现，其实是让a生成一个vec，直接将结果作为from的返回值
    let mut v: Vec<i32> = Vec::from(&a);

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);

        let (a, v) = array_and_vec2();
        assert_eq!(a, v[..]);
    }
}
