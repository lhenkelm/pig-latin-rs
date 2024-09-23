pub fn ashay_igspay_atinlay(english : &str) -> String {
    english.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    // consonant examples
    #[test]
    fn test_first() {
        let result = ashay_igspay_atinlay("first");
        assert_eq!(result, "irstfay");
    }

    #[test]
    fn test_pigs() {
        let result = ashay_igspay_atinlay("pigs");
        assert_eq!(result, "igspay");
    }

    #[test]
    fn test_latin() {
        let result = ashay_igspay_atinlay("latin");
        assert_eq!(result, "atinlay");
    }
    
    #[test]
    fn test_banana() {
        let result = ashay_igspay_atinlay("banana");
        assert_eq!(result, "ananabay");
    }

    // vowel examples 
    #[test]
    fn test_apple() {
        let result = ashay_igspay_atinlay("apple");
        assert_eq!(result, "applehay");
    }
    
    #[test]
    fn test_ear() {
        let result = ashay_igspay_atinlay("ear");
        assert_eq!(result, "earhay");
    }
    
    #[test]
    fn test_omelet() {
        let result = ashay_igspay_atinlay("omelet");
        assert_eq!(result, "omelethay");
    }
    
    // sentence examples
    #[test]
    fn test_this_is_pigs_latin() {
        let result = ashay_igspay_atinlay("This is pigs latin.");
        assert_eq!(result, "Isthay ishay igspay atinlay.")
    }
    
    #[test]
    fn test_easy_innit() {
        let result = ashay_igspay_atinlay("This is all quite easy, is it not?");
        assert_eq!(result, "Isthay ishay allhay itequay easyhay, ishay ithay otnay?")
    }
}
