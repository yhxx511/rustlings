// cow1.rs
//
// This exercise explores the Cow, or Clone-On-Write type. Cow is a
// clone-on-write smart pointer. It can enclose and provide immutable access to
// borrowed data, and clone the data lazily when mutation or ownership is
// required. The type is designed to work with general borrowed data via the
// Borrow trait.
//
// This exercise is meant to show you what to expect when passing data to Cow.
// Fix the unit tests by checking for Cow::Owned(_) and Cow::Borrowed(_) at the
// TODO markers.
//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.



use std::borrow::Cow;

// 给输入的数组取绝对值，如果需要，则会修改数组本身；但也可能不会修改。
// 因此，使用Cow来解决“按需修改”时的所有权问题
fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            // TODO 这里assert_eq，需要类型匹配 Vec::<i32>::from  这种方式使用，而不是Vec<i32>::from()
            Cow::Owned(v) => {assert_eq!(Vec::<i32>::from([1, 0, 1]), *v); Ok(())},
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // No clone occurs because `input` doesn't need to be mutated.
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Borrowed(_) => Ok(()),
            _ => Err("Expected borrowed value"),
        }
    }

    #[test]
    fn reference_no_mutation2() -> Result<(), &'static str> {
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(&slice);
        match abs_all(&mut input) {
            Cow::Borrowed(v) => {assert_eq!(Vec::<i32>::from([0, 1, 2]), *v); Ok(())},
            _ => Err("Expected borrowed value")
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // We can also pass `slice` without `&` so Cow owns it directly. In this
        // case no mutation occurs and thus also no clone, but the result is
        // still owned because it was never borrowed or mutated.
        let slice = vec![0, 1, 2];
        // TODO 传给from的是slice本身，而不是他的引用。前面几个test都是直接传的引用。所以，会以borrowed方式初始化
        //      当以moved方式传递时，就直接是owned value
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(v) => {
                assert_eq!(Vec::<i32>::from([0, 1, 2]), *v);
                Ok(())
            },
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // Of course this is also the case if a mutation does occur. In this
        // case the call to `to_mut()` in the abs_all() function returns a
        // reference to the same data as before.
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(v) => { assert_eq!(Vec::<i32>::from([1, 0, 1]), *v); Ok(())},
            _ => Err("Expected owned value")
        }
    }
}
