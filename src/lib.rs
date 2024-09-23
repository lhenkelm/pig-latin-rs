use std::iter::repeat;

fn is_vowel(c: &char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn apply_casing_like(text: &str, casing_of: &str) -> String {
    text
    .chars()
    .zip(casing_of.chars())
    .map(
        |(txt, csg)| {
            if csg.is_lowercase() {
                txt.to_lowercase().to_string()
            } else if csg.is_uppercase() {
                txt.to_uppercase().to_string()
            } else {
                txt.to_string()
            }
        }
    )
    .collect()
}

fn solve_single_word(english_word : &str) -> String {
    if english_word == "" {
        return "".to_string()
    }
    let starts_voweled = is_vowel(
        &english_word
        .chars()
        .next()
        .expect("got empty english_word")
    );
    if starts_voweled {
        return format!("{english_word}hay");
    }
    let (first_consonant_indices, first_consonants) : (Vec<usize>, String) = 
        english_word
        .char_indices().
        take_while(| (_, c)| !is_vowel(c))
        .unzip();
    let first_consonants_to = first_consonant_indices.last().expect("missing: last consonant");    
    let core : String = english_word.chars().skip(*first_consonants_to+1).collect();
    format!("{core}{first_consonants}ay")
}


pub fn ashay_igspay_atinlay(english : &str) -> String {
    english
        .split(|c: char| {c.is_ascii_punctuation() || c.is_whitespace()})
        .map(|ew: &str| solve_single_word(ew))
        .zip(
            english
            .split(
                |c: char| {(!c.is_ascii_punctuation()) && (!c.is_whitespace())}
            )
            .filter(|s| s.chars().count()> 0)
            .chain(repeat(""))
        )
        .map(|(word, delim)| word + delim)
        .collect()
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

    // edge cases and regressions
    #[test]
    fn test_empty() {
        assert_eq!(ashay_igspay_atinlay(""), "");
    }

    // units
    #[test]
    fn test_is_vowel() {
        assert_eq!(is_vowel(&'a'), true);
        assert_eq!(is_vowel(&'u'), true);
        assert_eq!(is_vowel(&'k'), false);
        assert_eq!(is_vowel(&'q'), false);
        assert_eq!(is_vowel(&'f'), false);
        assert_eq!(is_vowel(&' '), false);
        assert_eq!(is_vowel(&'.'), false);
        assert_eq!(is_vowel(&'7'), false);
    }

    #[test]
    fn test_copy_casing() {
        assert_eq!(apply_casing_like("foo", "bar"), String::from("foo"));
        assert_eq!(apply_casing_like("foo", "bAr"), String::from("fOo"));
        assert_eq!(apply_casing_like("foo", "BAR"), String::from("FOO"));
        assert_eq!(apply_casing_like("fOo", "Bar"), String::from("Foo"));
        assert_eq!(apply_casing_like("fOObar", "BarBaz"),String::from("FooBar"));
        assert_eq!(apply_casing_like("AB", "ABC"), "AB");
        assert_eq!(apply_casing_like("AbCd", "Ab"), "Abcd");
    }
}
