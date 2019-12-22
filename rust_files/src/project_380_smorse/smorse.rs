
fn smorse(word: String) -> String {
    return String::new();
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
}