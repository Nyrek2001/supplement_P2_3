// Generic function to swap two variables
pub fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap() {
        let (a, b) = swap(1, 2);
        assert_eq!(a, 2);
        assert_eq!(b, 1);
        
        let (x, y) = swap("hello", "world");
        assert_eq!(x, "world");
        assert_eq!(y, "hello");
    }
}
