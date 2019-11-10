use std::str::Chars;

fn lang_to_rov(lang_str: &str) -> String {
    let mut accum = "".to_string();
    for a_char in lang_str.chars() {
        let next_rov_chars = lang_char_to_rov_chars(&a_char.to_string());
        accum.push_str(next_rov_chars.as_str());
    }
    return accum;
}


fn lang_char_to_rov_chars(lang_char: &String) -> String {
    if is_swedish_vowel(lang_char) || is_punctuation(lang_char) {
        return (*lang_char.clone()).parse().unwrap();
    }
    else {
        // is consonant
        let new_chars = &mut String::new();
        let local_lang_char_copy = lang_char.clone();
        new_chars.push_str(lang_char.as_str());
        new_chars.push_str("o");
        new_chars.push_str(local_lang_char_copy.to_ascii_lowercase().as_str());
        return new_chars.clone();
    }
    return String::new()
}

fn rov_to_eng(rov_str: &str) -> String {
    return String::new();
}

fn is_swedish_vowel(maybe_vowel: &String) -> bool {
    return maybe_vowel.eq_ignore_ascii_case("a")
        || maybe_vowel.eq_ignore_ascii_case("e")
        || maybe_vowel.eq_ignore_ascii_case("i")
        || maybe_vowel.eq_ignore_ascii_case("o")
        || maybe_vowel.eq_ignore_ascii_case("u")
        || maybe_vowel.eq_ignore_ascii_case("y")
        || maybe_vowel.eq_ignore_ascii_case("å")
        || maybe_vowel.eq_ignore_ascii_case("ä")
        || maybe_vowel.eq_ignore_ascii_case("ö")
}

fn is_punctuation(maybe_punc: &String) -> bool {
    return maybe_punc.eq_ignore_ascii_case(".")
        || maybe_punc.eq_ignore_ascii_case(",")
        || maybe_punc.eq_ignore_ascii_case("!")
        || maybe_punc.eq_ignore_ascii_case("?")
        || maybe_punc.eq_ignore_ascii_case(" ")
        || maybe_punc.eq_ignore_ascii_case("'")
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lang_char_to_rov_chars() {
        let lang_char1 = &"a".to_string();
        let lang_char2 = &"r".to_string();
        let lang_char3 = &".".to_string();
        let space = &" ".to_string();

        assert_eq!("a", lang_char_to_rov_chars(lang_char1));
        assert_eq!("ror", lang_char_to_rov_chars(lang_char2));
        assert_eq!(".", lang_char_to_rov_chars(lang_char3));
        assert_eq!(" ", lang_char_to_rov_chars(space));
    }

    #[test]
    fn test_eng_to_rov() {
        let lang1 = "Jag talar Rövarspråket!";
        let lang2 = "I'm speaking Robber's language!";
        let lang3 = "Tre Kronor är världens bästa ishockeylag.";
        let lang4 = "Vår kung är coolare än er kung.";

        let rov1 = "Jojagog totalolaror Rorövovarorsospoproråkoketot!".to_string();
        let rov2 = "I'mom sospopeakokinongog Rorobobboberor'sos \
        lolanongoguagoge!".to_string();
        let rov3 = "Totrore Kokrorononoror äror vovärorloldodenonsos \
        bobäsostota isoshohocockokeylolagog.".to_string();
        let rov4 = "Vovåror kokunongog äror cocoololarore änon eror kokunongog.".to_string();

        assert_eq!(lang_to_rov(lang1), rov1);
        assert_eq!(lang_to_rov(lang2), rov2);
        assert_eq!(lang_to_rov(lang3), rov3);
        assert_eq!(lang_to_rov(lang4), rov4);

    }

    #[test]
    fn test_rov_to_eng() {
        let lang1 = "Jag talar Rövarspråket!".to_string();
        let lang2 = "I'm speaking Robber's language!".to_string();
        let lang3 = "Tre Kronor är världens bästa ishockeylag.".to_string();
        let lang4 = "Vår kung är coolare än er kung.".to_string();

        let rov1 = "Jojagog totalolaror Rorövovarorsospoproråkoketot!";
        let rov2 = "I'mom sospopeakokinongog Rorobobboberor'sos lolanongoguagoge!";
        let rov3 = "Totrore Kokrorononoror äror vovärorloldodenons \
        bobäsostota isoshohocockokeylolagog.";
        let rov4 = "Vovåror kokunongog äror cocoololarore änon eror kokunongog.";

        assert_eq!(rov_to_eng(rov1), lang1);
        assert_eq!(rov_to_eng(rov2), lang2);
        assert_eq!(rov_to_eng(rov3), lang3);
        assert_eq!(rov_to_eng(rov4), lang4);
    }

    #[test]
    fn test_is_swedish_vowel() {
        assert!(is_swedish_vowel(&"a".to_string()));
        assert!(is_swedish_vowel(&"e".to_string()));
        assert!(is_swedish_vowel(&"i".to_string()));
        assert!(is_swedish_vowel(&"o".to_string()));
        assert!(is_swedish_vowel(&"u".to_string()));
        assert!(is_swedish_vowel(&"y".to_string()));
        assert!(is_swedish_vowel(&"å".to_string()));
        assert!(is_swedish_vowel(&"ä".to_string()));
        assert!(is_swedish_vowel(&"ö".to_string()));

        assert!(!is_swedish_vowel(&"p".to_string()));
        assert!(!is_swedish_vowel(&".".to_string()));
    }

    #[test]
    fn test_is_punctuation() {
        assert!(is_punctuation(&"!".to_string()));
        assert!(is_punctuation(&"?".to_string()));
        assert!(is_punctuation(&".".to_string()));
        assert!(is_punctuation(&",".to_string()));
        assert!(is_punctuation(&" ".to_string()));
        assert!(is_punctuation(&"'".to_string()));

        assert!(!is_punctuation(&"p".to_string()));
        assert!(!is_punctuation(&"a".to_string()));
    }
}
