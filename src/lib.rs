pub fn add(a: u64, b: u64) -> u64 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_simple() {
        assert_eq!(add(2, 3), 5);
    }
}
