pub fn ashay_igspay(english : &str) -> String {
    english.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let result = ashay_igspay("first");
        assert_eq!(result, "irstfay");
    }
    
    #[test]
    fn test_apple() {
        let result = ashay_igspay("apple");
        assert_eq!(result, "applehay");
    }
}
