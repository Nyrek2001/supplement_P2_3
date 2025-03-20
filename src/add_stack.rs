// Function to add two numbers allocated on the stack
pub fn add_stack(a: i32, b: i32) -> i32 {
    a + b
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_stack() {
        assert_eq!(add_stack(5, 10), 15);
        assert_eq!(add_stack(-5, 5), 0);
    }
}
