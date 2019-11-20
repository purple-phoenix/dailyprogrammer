mod project_212_rovarspraket;
mod project_381_yahtzee;
mod project_375_graph_of_thrones;
pub mod project_238_fallout_hacking;
mod project_375_print_number_digit;
mod utility;

use crate::project_212_rovarspraket::rovarspraket::{lang_to_rov, rov_to_lang};
use crate::project_381_yahtzee::yahtzee::yahtzee_upper;
use crate::project_375_graph_of_thrones::graph_of_thrones::{make_graph_from_lines};
use crate::utility::avltree::{make_tree_from_list};
use crate::project_375_print_number_digit::print_number_digit::increment_digits;

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

pub fn qualify_graph_of_thrones() -> bool {
    let path = "src/project_375_graph_of_thrones/graph_of_thrones.txt";
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    let mut line_vec = Vec::with_capacity(121);

    for buf_line in buffered.lines() {
        let buf_line_to_string = buf_line.unwrap();
        line_vec.push(buf_line_to_string);
    }

    let graph = make_graph_from_lines(&line_vec);
    return !graph.is_balanced();
}

pub fn qualify_avl() -> bool {
    let num_elms_in_bst = 1000;
    let mut bst_init_vector = Vec::with_capacity(num_elms_in_bst);

    for x in 0..num_elms_in_bst {
        bst_init_vector.push(x);
    }

    let bst = make_tree_from_list(&bst_init_vector);

    match bst {
        None => return false,
        Some(_bst) => return true
    }
}

pub fn qualify_print_number_digit() -> bool {
    let positive_works = increment_digits(1193304) == 22104415;
    println!("{}", increment_digits(-384039));
    /*
    Thinking about what the expected behavior should be for negative numbers now that
    I've considered a negative number with a 0 digit like -100 should this be -11(-011)
    or -211? Instructions unclear :) So it is what I've implemented
    */
    let negative_works = increment_digits(-384139) == -273028;
    let zero_works = increment_digits(0) == 1;
    return positive_works && negative_works && zero_works;
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

    #[test]
    fn test_qualify_graph_of_thrones() {
        assert!(qualify_graph_of_thrones())
    }

    #[test]
    fn test_qualify_avl() {
        assert!(qualify_avl())
    }

    #[test]
    fn test_qualify_print_number_digit() {
        assert!(qualify_print_number_digit())
    }

}
