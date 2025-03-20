// Function to add a stack-allocated number to a heap-allocated number
pub fn add_stack_heap(a: i32, b: Box<i32>) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_stack_heap() {
        let b = Box::new(10);
        assert_eq!(add_stack_heap(5, b), 15);

        let b = Box::new(-5);
        assert_eq!(add_stack_heap(5, b), 0);
    }
}
