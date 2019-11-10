use std::str::Chars;

fn lang_to_rov(lang_str: &str) -> String {
    return lang_to_rov_helper(&lang_str.to_string(), &String::new());
}

fn lang_to_rov_helper(lang_str: &String, accum: &String) -> String {
    if lang_str.is_empty() {
        return accum.clone();
    }
    else {
        let next_char = &lang_str[0..0].to_string();
        let rest_lang_chars = &lang_str[1..].to_string();
        let next_rov_chars = lang_char_to_rov_chars(next_char);
        let mut updated_accum = accum.clone();
        updated_accum.push_str(next_rov_chars.as_str());
        return lang_to_rov_helper(rest_lang_chars, &updated_accum);
    }
}

fn lang_char_to_rov_chars(lang_char: &String) -> String {
    return String::new()
}

fn rov_to_eng(rov_str: &str) -> String {
    return String::new();
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lang_char_to_rov_chars() {
        let lang_char1 = &"a".to_string();
        let lang_char2 = &"r".to_string();
        let lang_char3 = &".!?".to_string();

        assert_eq!("a", lang_char_to_rov_chars(lang_char1));
        assert_eq!("ror", lang_char_to_rov_chars(lang_char2));
        assert_eq!(".!?", lang_char_to_rov_chars(lang_char3));
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
        let rov3 = "Totrore Kokrorononoror äror vovärorloldodenons \
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
}
