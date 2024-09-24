use std::iter::repeat;

pub fn translate(english : &str) -> String {
    english
        .split(|c: char| {c.is_ascii_punctuation() || c.is_whitespace()})
        .filter(|s| s.chars().count()> 0)
        .map(|ew: &str| translate_word(ew))
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

pub use crate::details::translate_word;

#[cfg(test)]
mod tests {
    use super::*;

    // consonant examples
    #[test]
    fn first() {
        let result = translate("first");
        assert_eq!(result, "irstfay");
    }

    #[test]
    fn pigs() {
        let result = translate("pigs");
        assert_eq!(result, "igspay");
    }

    #[test]
    fn latin() {
        let result = translate("latin");
        assert_eq!(result, "atinlay");
    }
    
    #[test]
    fn banana() {
        let result = translate("banana");
        assert_eq!(result, "ananabay");
    }

    // vowel examples 
    #[test]
    fn apple() {
        let result = translate("apple");
        assert_eq!(result, "applehay");
    }
    
    #[test]
    fn ear() {
        let result = translate("ear");
        assert_eq!(result, "earhay");
    }
    
    #[test]
    fn omelet() {
        let result = translate("omelet");
        assert_eq!(result, "omelethay");
    }

    #[test]
    fn words_is_sentence_if_word_input() {
        for example in ["first", "pigs", "latin", "apple", "banana", "ear", "omelet"] {
            assert_eq!(translate_word(example), translate(example))
        }
    }
    
    // sentence examples
    #[test]
    fn this_is_pigs_latin() {
        let result = translate("This is pigs latin.");
        assert_eq!(result, "Isthay ishay igspay atinlay.")
    }
    
    #[test]
    fn easy_innit() {
        let result = translate("This is all quite easy, is it not?");
        assert_eq!(result, "Isthay ishay allhay itequay easyhay, ishay ithay otnay?")
    }

    // edge cases and regressions
    #[test]
    fn empty() {
        assert_eq!(translate(""), "");
    }

}

mod details {
    use std::iter;
    fn is_vowel(c: &char) -> bool {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        }
    }

    fn apply_casing_like(text: &str, casing_of: &str) -> String {
        text
        .chars()
        .zip(
            casing_of
            .chars()
            .chain(
                iter::repeat(
                    casing_of
                    .chars()
                    .last()
                    .unwrap_or(' ') // default in an uncased char
                )
            )
        )
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

    pub fn translate_word(english_word : &str) -> String {
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
        let mut first_consonants_to = *first_consonant_indices.last().expect("missing: last consonant");
        let first_consonants = if 
               first_consonants.chars().last().unwrap() == 'q'
            && english_word.chars().skip(first_consonants_to+1).next().expect("missing: vowels") == 'u'
        {
            first_consonants_to+=1;
            first_consonants + "u"
        } else {
            first_consonants
        };
        let core : String = english_word.chars().skip(first_consonants_to+1).collect();
        apply_casing_like(&format!("{core}{first_consonants}ay"), english_word)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        // units
        #[test]
        fn _is_vowel() {
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
        fn copy_casing() {
            assert_eq!(apply_casing_like("foo", "bar"), String::from("foo"));
            assert_eq!(apply_casing_like("foo", "bAr"), String::from("fOo"));
            assert_eq!(apply_casing_like("foo", "BAR"), String::from("FOO"));
            assert_eq!(apply_casing_like("fOo", "Bar"), String::from("Foo"));
            assert_eq!(apply_casing_like("fOObar", "BarBaz"),String::from("FooBar"));
            assert_eq!(apply_casing_like("AB", "ABC"), "AB");
            assert_eq!(apply_casing_like("AbCd", "Ab"), "Abcd");
        }

        #[test]
        fn copy_casing_empty() {
            assert_eq!(apply_casing_like("", "Hello"), "");
            assert_eq!(apply_casing_like("Hello", ""), "Hello");
        }
    }
}

