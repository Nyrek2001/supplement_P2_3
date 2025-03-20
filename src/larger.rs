// Function to return the larger of two variables
pub fn larger<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_larger() {
        assert_eq!(larger(5, 10), 10);
        assert_eq!(larger(10.5, 5.2), 10.5);
        assert_eq!(larger('a', 'b'), 'b');
    }
}
