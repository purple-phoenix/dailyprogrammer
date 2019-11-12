mod project_212_rovarspraket;
mod project_381_yahtzee;
use crate::project_212_rovarspraket::rovarspraket::{lang_to_rov, rov_to_lang};
use crate::project_381_yahtzee::yahtzee::yahtzee_upper;

use std::fs::File;
use std::io::{BufReader, BufRead};


pub fn qualify_rovarspraket(some_str: &str) -> bool {
    let rov = lang_to_rov(some_str);
    let reverse_lang = rov_to_lang(rov.as_str());
    return some_str == reverse_lang;
}

pub fn qualify_yahtzee() -> bool {
    let path = "src/project_381_yahtzee/yahtzee-upper-1.txt";
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    let mut complex_dice_roll: Vec<usize> = Vec::with_capacity(100000);

    for line in buffered.lines() {
        complex_dice_roll.push(line.unwrap().parse::<usize>().unwrap())
    }


    return 31415926535 == yahtzee_upper(complex_dice_roll)
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
