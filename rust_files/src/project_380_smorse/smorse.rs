use std::collections::HashMap;

fn smorse(word: String) -> String {
    return String::new();
}

fn make_smorse_letter_map() -> HashMap<char, String> {
    return HashMap::new()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smorse() {
        assert_eq!(smorse("sos".to_string()), "...---...".to_string());
        assert_eq!(smorse("daily".to_string()), "-...-...-..-.--".to_string());
        assert_eq!(smorse("programmer".to_string()), ".--..-.-----..-..-----..-.");
        assert_eq!(smorse("bits".to_string()), "-.....-...".to_string());
        assert_eq!(smorse("three".to_string()), "-.....-...".to_string());
    }

    #[test]
    fn test_smorse_letter_map() {
        let map = make_smorse_letter_map();

        verify_smorse_char(&map, 'a', ".-".to_string());
        verify_smorse_char(&map, 'b', "-...".to_string());
        verify_smorse_char(&map, 'c', "-.-.".to_string());
        verify_smorse_char(&map, 'd', "-..".to_string());
        verify_smorse_char(&map, 'e', ".".to_string());
        verify_smorse_char(&map, 'f', "..-.".to_string());
        verify_smorse_char(&map, 'g', "--.".to_string());
        verify_smorse_char(&map, 'h', "....".to_string());
        verify_smorse_char(&map, 'i', "..".to_string());
        verify_smorse_char(&map, 'j', ".---".to_string());
        verify_smorse_char(&map, 'k', "-.-".to_string());
        verify_smorse_char(&map, 'l', ".-..".to_string());
        verify_smorse_char(&map, 'm', "--".to_string());
        verify_smorse_char(&map, 'n', "-.".to_string());
        verify_smorse_char(&map, 'o', "---".to_string());
        verify_smorse_char(&map, 'p', ".--.".to_string());
        verify_smorse_char(&map, 'q', "--.-".to_string());
        verify_smorse_char(&map, 'r', ".-.".to_string());
        verify_smorse_char(&map, 's', "...".to_string());
        verify_smorse_char(&map, 't', "-".to_string());
        verify_smorse_char(&map, 'u', "..-".to_string());
        verify_smorse_char(&map, 'v', "...-".to_string());
        verify_smorse_char(&map, 'w', ".--".to_string());
        verify_smorse_char(&map, 'x', "-..-".to_string());
        verify_smorse_char(&map, 'y', "-.--".to_string());
        verify_smorse_char(&map, 'z', "--..".to_string());




    }

    fn verify_smorse_char(map: &HashMap<char, String>, smorse_char: char, expected_smorse: String) {
        match map.get(&smorse_char) {
            Some(smorse) => assert_eq!(smorse, &expected_smorse),
            None => {
                println!("Smorse map does not contain character {}", smorse_char);
                assert!(false);
            }
        }
    }
}