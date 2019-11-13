mod project_212_rovarspraket;
mod project_381_yahtzee;
mod project_375_graph_of_thrones;
use crate::project_212_rovarspraket::rovarspraket::{lang_to_rov, rov_to_lang};
use crate::project_381_yahtzee::yahtzee::yahtzee_upper;
use crate::project_375_graph_of_thrones::graph_of_thrones::{make_graph_from_lines, UndirectedCompleteGraph};

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

}
