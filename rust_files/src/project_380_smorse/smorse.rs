use std::collections::HashMap;

fn smorse(word: String) -> String {
    return String::new();
}

fn make_smorse_letter_map() -> HashMap<char, String> {
    let mut map = HashMap::new();

    map.insert('a', ".-".to_string());
    map.insert('b', "-...".to_string());
    map.insert('c', "-.-.".to_string());
    map.insert('d', "-..".to_string());
    map.insert('e', ".".to_string());
    map.insert('f', "..-.".to_string());
    map.insert('g', "--.".to_string());
    map.insert('h', "....".to_string());
    map.insert('i', "..".to_string());
    map.insert('j', ".---".to_string());
    map.insert('k', "-.-".to_string());
    map.insert('l', ".-..".to_string());
    map.insert('m', "--".to_string());
    map.insert('n', "-.".to_string());
    map.insert('o', "---".to_string());
    map.insert('p', ".--.".to_string());
    map.insert('q', "--.-".to_string());
    map.insert('r', ".-.".to_string());
    map.insert('s', "...".to_string());
    map.insert('t', "-".to_string());
    map.insert('u', "..-".to_string());
    map.insert('v', "...-".to_string());
    map.insert('w', ".--".to_string());
    map.insert('x', "-..-".to_string());
    map.insert('y', "-.--".to_string());
    map.insert('z', "--..".to_string());

    return map
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