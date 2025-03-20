// Function to add two numbers allocated on the heap
pub fn add_heap(a: Box<i32>, b: Box<i32>) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_heap() {
        let a = Box::new(5);
        let b = Box::new(10);
        assert_eq!(add_heap(a, b), 15);

        let a = Box::new(-5);
        let b = Box::new(5);
        assert_eq!(add_heap(a, b), 0);
    }
}
