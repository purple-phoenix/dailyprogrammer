mod project_212_rovarspraket;
mod project_381_yahtzee;
use crate::project_212_rovarspraket::rovarspraket::{lang_to_rov, rov_to_lang};

pub fn qualify_rovarspraket(some_str: &str) -> bool {
    let rov = lang_to_rov(some_str);
    let reverse_lang = rov_to_lang(rov.as_str());
    return some_str == reverse_lang;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rovarspraket() {
        let sentence = "Hello, my name is Matt McCarthy";
        assert!(qualify_rovarspraket(sentence));
    }
}
