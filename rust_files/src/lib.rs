mod project_212_rovarspraket;
mod project_381_yahtzee;
use crate::project_212_rovarspraket::rovarspraket::{lang_to_rov, rov_to_lang};
use crate::project_381_yahtzee::yahtzee::yahtzee_upper;


pub fn qualify_rovarspraket(some_str: &str) -> bool {
    let rov = lang_to_rov(some_str);
    let reverse_lang = rov_to_lang(rov.as_str());
    return some_str == reverse_lang;
}

pub fn qualify_yahtzee() -> bool {
    let complex_dice_roll = vec![1654, 1654, 50995, 30864, 1654, 50995, 22747,
                                 1654, 1654, 1654, 1654, 1654, 30864, 4868, 1654, 4868, 1654,
                                 30864, 4868, 30864];

    return 123456 == yahtzee_upper(complex_dice_roll)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qualify_rovarspraket() {
        let sentence = "Hello, my name is Matt McCarthy";
        assert!(qualify_rovarspraket(sentence));
    }

    #[test]
    fn test_qualify_yahtzee() {
        assert!(qualify_yahtzee())
    }

}
