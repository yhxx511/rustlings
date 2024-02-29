// if2.rs
//
// Step 1: Make me compile!
// Step 2: Get the bar_for_fuzz and default_to_baz tests passing!
//
// Execute `rustlings hint if2` or use the `hint` watch subcommand for a hint.



pub fn convert(txt: &str) -> &str {
    match txt {
        "fizz" => "foo",
        "fuzz" => "bar",
        _ => "baz"
    }
}

pub fn convert_with_if(txt: &str) -> &str {
    if txt == "fizz" {
        "foo"
    } else if txt == "fuzz" {
        "bar"
    } else {
        "baz"
    }
}

// No test changes needed!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        assert_eq!(convert("fizz"), "foo")
    }

    #[test]
    fn foo_for_fizz2() {
        assert_eq!(convert_with_if("fizz"), "foo")
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(convert("fuzz"), "bar")
    }

    #[test]
    fn bar_for_fuzz2() {
        assert_eq!(convert_with_if("fuzz"), "bar")
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(convert("literally anything"), "baz")
    }

    #[test]
    fn default_to_baz2() {
        assert_eq!(convert_with_if("literally anything"), "baz")
    }
}
