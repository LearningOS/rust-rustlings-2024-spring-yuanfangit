// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.


struct Wrapper {
    value1: u32,
    value2: String
}

impl Wrapper {
    pub fn new_u32(value1: u32) -> Self {
        Wrapper { value1, value2: String::new() }
    }
    pub fn new_str(value2: &str) -> Self {
        Wrapper { value1: 0, value2: value2.to_string() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new_u32(42).value1, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new_str("Foo").value2, "Foo");
    }
}
