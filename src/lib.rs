//! # Pig-Latin
//! 
//! This crate provides functions for translating English into Pig-Latin.
//! 
//! The advantage of [Pig-Latin](https://en.wikipedia.org/wiki/Pig_Latin) 
//! is its extreme suitability to machine translation, without requiring
//! any kind of machine learning (so long as you translate from English).
//!
//! The general purpose entry point is [`pig_latin::translate`](crate::translate). 
//! In the special case of single-word inputs, 
//! [`pig_latin::translate_word`](crate::translate_word) may be slightly faster
//! -- but may provide wrong results on non-single-word inputs, and behavior
//! on such inputs may change without warning.
//!  
//! ## One True Dialect
//! 
//!   > Ash nazg durbatulûk, ash nazg gimbatul, ash nazg thrakatulûk, agh burzum-ishi krimpatul  
//!     -- Inspiration, by [one most humble and demure](https://google.gprivate.com/search.php?search?q=Sauron).
//! 
//! There exists more than one dialect of Pig-Latin. Because all other dialects
//! are *stinky*, __incorrect__, ***and annoying***, `pig_latin` implements only the One True
//! Dialect of Pig-Latin (OTDoPL). OTDoPL is love, OTDoPL is life, and these are
//! its rules: 
//!  - The general suffix is "ay". 
//!  - The suffix for English words starting with a vowel is "hay".
//!  - The vowel "u", if preceded by the consonant "q", is treated
//!    as "part of" the consonant as far as translation is concerned.
//!     - This is done to preserve pronouncability according to English
//!       phonetics.
//! 
//! ## Example
//! 
//! ```rust
//! use pig_latin;
//! 
//! let english_input = "This crate provides functions for translating English into Pig-Latin.
//! 
//! The advantage of Pig-Latin is its extreme suitability to machine translation,
//! without requiring any kind of machine learning (so long as you translate from English).";
//! 
//! 
//!let expected_pig_latin = "Isthay atecray ovidespray unctionsfay orfay anslatingtray Englishhay intohay Igpay-Atinlay.
//! 
//! Ethay advantagehay ofhay Igpay-Atinlay ishay itshay extremehay uitabilitysay otay achinemay anslationtray,
//! ithoutway equiringray anyhay indkay ofhay achinemay earninglay (osay onglay ashay ouyay anslatetray omfray Englishhay)."; 
//! 
//! assert_eq!(pig_latin::translate(&english_input), expected_pig_latin);
//! ```

use std::iter::repeat;

/// # Translate English into Pig-Latin.
/// 
/// This function translates arbitrary English text into [OTDoPL](crate#one-true-dialect) Pig-Latin.
/// 
/// This is done by tokenizing the text into words (contiguous, non-whitespace, non-punctuation
/// substrings), translating the words (cf. [`translate_word`]), and re-inserting the non-word 
/// characters. Thus, whitespace, layout, structure, and punctuation should be preserved in
/// translation. 
/// 
/// ## Examples
/// 
/// Translate a word:
/// ```rust
/// # use pig_latin::translate;
/// assert_eq!(translate("Hello"), String::from("Ellohay"));
/// ```
/// 
/// Translate a sentence:
/// ```rust
/// # use pig_latin::translate;
/// assert_eq!(translate("Hello world!"), String::from("Ellohay orldway!"));
/// ```
/// 
/// Translate words containing "Q":
/// 
/// ```rust
/// # use pig_latin::translate;
/// assert_eq!(translate("Question:"), String::from("Estionquay:"));
/// ```
/// 
/// Translate words starting with vowels:
/// ```rust
/// # use pig_latin::translate;
/// assert_eq!(
///     translate("Early-Adopters are ecstatic?"), 
///     String::from("Earlyhay-Adoptershay arehay ecstatichay?")
/// );
/// ```
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

/// implementation details go here, and exposed function's implementations
/// that are not intended as default entry points
mod details {
    /// Return `true` if `c` is an ASCII-vowel, else `false` (uncased).
    fn is_vowel(c: &char) -> bool {
        let c = c.to_ascii_lowercase();
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        }
    }

    #[derive(PartialEq,Debug,Copy, Clone)]
    enum CharCase {
        Lower,
        Upper,
        Eh,
    }

    impl CharCase {
        fn from_char(c: &char) -> CharCase {
            if c.is_lowercase() {
                CharCase::Lower
            } else if c.is_uppercase() {
                CharCase::Upper
            } else {
                CharCase::Eh
            }
        }
    }


    /// Transfer the sequence of upper/lower casing from one string to another.
    /// 
    /// Identifies the sequence of UPPER/lower casing of characters
    /// in `casing_of`, then apply the same casing to `text`.
    /// Apart from the casing, the content of `text` remains unchanged.
    fn apply_casing_like(text: &str, casing_of: &str) -> String {
        let mut substrings = Vec::with_capacity(1);
        let mut text_byte_idx = 0;
        let mut last_edit = 0;
        let mut target_case = CharCase::Eh;
        let mut casing_of_chars = casing_of.chars();
        let mut exhausted = false;
        for text_char in text.chars() {
            let text_case = CharCase::from_char(&text_char);
            if !exhausted {
                match casing_of_chars.next() {
                    Some(casing_of_char) => {
                        target_case = CharCase::from_char(&casing_of_char);
                    }
                    None => {
                        exhausted = true;
                    }
                }
            }
            if text_case != target_case && target_case != CharCase::Eh {
                if text_byte_idx > last_edit {
                    substrings.push(text[last_edit..text_byte_idx].to_owned());
                }
                let end_edit = text_byte_idx+text_char.len_utf8();
                substrings.push(
                    match target_case {
                        CharCase::Upper => text_char.to_uppercase().to_string(),
                        CharCase::Lower => text_char.to_lowercase().to_string(),
                        CharCase::Eh => panic!("{target_case:?} should be unreachable here"),
                    }
                );
                last_edit = end_edit;
            } else if text_byte_idx+1 == text.len() {
                substrings.push(text[last_edit..text_byte_idx+1].to_owned());
                 
            }
            text_byte_idx += text_char.len_utf8();
        }
        // NB: I already tried with_capacity + push_str here,
        // but it slows the benchmark down
        substrings.into_iter().collect()
    }

    /// # Translate a single english word into Pig-Latin.
    /// 
    /// Translate a single word into [OTDoPL](crate#one-true-dialect) Pig Latin.
    /// 
    /// The input is assumed to be a single word, and this is not checked.
    /// "Single word" means that there are no special characters, no whitespace,
    /// and no punctuation anywhere in the string. In other words, each character
    /// should be able to be either uppercase or lowercase, and either a vowel or
    /// a consonant.
    ///  
    /// **Hint: If you are not fully able to guarantee single-word inputs,
    /// use [`crate::translate`] instead.**
    /// 
    ///  ## Examples
    /// 
    /// Works fine for words. Don't use it for non-word strings!
    /// ```rust
    /// # use pig_latin::translate_word;
    /// assert_eq!(translate_word("Rar"), String::from("Array"));
    /// ```
    /// 
    /// The empty string is, technically, a word in our above technical sense:
    /// ```rust
    /// # use pig_latin::translate_word;
    /// assert_eq!(translate_word(""), String::new());
    /// ```
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
        let mut byte_idx_cut_at =0;
        for char in english_word.chars() {
            if is_vowel(&char) {
                break
            }
            byte_idx_cut_at += char.len_utf8();
        }

        if english_word.len() > byte_idx_cut_at { 
            let mut chars  = english_word[..byte_idx_cut_at+1].chars();
            if 
                   chars.next().unwrap().to_ascii_lowercase() == 'q' 
                && chars.next().unwrap().to_ascii_lowercase() == 'u'
            {
                byte_idx_cut_at += 'u'.len_utf8();
            };
        }
        let mut translated = String::with_capacity(english_word.len() + "ay".len());
        translated.push_str(&english_word[byte_idx_cut_at..]);
        translated.push_str(&english_word[..byte_idx_cut_at]);
        translated.push_str("ay");
        apply_casing_like(&translated,english_word)
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
        fn copy_casing_ligature() {
            assert_eq!(apply_casing_like("ﬁre", "HELLO"), "FIRE");
        }

        #[test]
        fn copy_casing_empty() {
            assert_eq!(apply_casing_like("", "Hello"), "");
            assert_eq!(apply_casing_like("Hello", ""), "Hello");
        }

        #[test]
        fn translate_word_just_q_or_qu(){
            assert_eq!(translate_word("q"), "qay");
            assert_eq!(translate_word("qu"), "quay");
            assert_eq!(translate_word("quaint"), "aintquay");
        }

        #[test]
        fn translate_word_upper_qu(){
            assert_eq!(translate_word("QUERY"), "ERYQUAY");
            assert_eq!(translate_word("query"), "eryquay");
            assert_eq!(translate_word("qUeRy"), "eRyQuay");
            assert_eq!(translate_word("Query"), "Eryquay");
        }
    }
}

