use crate::project_238_fallout_hacking::game_constants::*;

use std::fs::File;
use std::io::{BufReader, BufRead};

struct FalloutHackingGame {

    word_length: usize,
    other_words: Vec<String>,
    correct_word: String,
    difficulty: GameDifficulty
}

pub enum GameDifficulty {
    VeryEasy,
    Easy,
    Average,
    Hard,
    VeryHard
}

impl FalloutHackingGame {

    fn make_game(difficulty: GameDifficulty) -> FalloutHackingGame {
        let other_words = Vec::with_capacity(make_num_words(&difficulty));
        let word_length = make_word_length(&difficulty);
        FalloutHackingGame {
            word_length,
            other_words,
            correct_word: "".to_owned(),
            difficulty
        }
    }

    fn get_word_list(word_len: usize, num_words: usize) -> (Vec<String>, String) {
        let all_words_of_len = collect_words_of_size_n(word_len);
        let total_words = all_words_of_len.len();
        let mut word_list = Vec::with_capacity(num_words);
        while word_list.len() < num_words {
            let random_index = get_rand_num_x_to_y(0, total_words);
            let potential_new_word = all_words_of_len.get(random_index).unwrap();
            if !word_list.contains(potential_new_word) {
                word_list.push(potential_new_word.parse().unwrap());
            }
        }

        loop {
            let random_index = get_rand_num_x_to_y(0, total_words);
            let potential_new_word = all_words_of_len.get(random_index).unwrap();
            if !word_list.contains(potential_new_word) {
                return (word_list, potential_new_word.parse().unwrap())
            }
        }

    }



}

fn collect_words_of_size_n(n: usize) -> Vec<String>{
    let mut words = Vec::with_capacity(1000);
    let path_to_dictionary = "src/project_238_fallout_hacking/enable1.txt";
    let file = File::open(path_to_dictionary).unwrap();
    let buffered = BufReader::new(file);

    for maybe_line in buffered.lines() {
        let line = maybe_line.unwrap();
        if line.len() == n {
            words.push(line);
        }
    }
    return words;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_make_game_model() {


        assert!(false);
    }

    #[test]
    fn test_collect_words_of_size_n() {
        let six_letter_words = collect_words_of_size_n(6);
        assert!(six_letter_words.len() > 10000)
    }

    #[test]
    fn test_get_word_list_and_correct_word() {
        let word_len = 6;
        let num_words = 14;
        let (word_list, correct_word) = FalloutHackingGame::get_word_list(word_len, num_words);
        assert_eq!(word_list.len(), num_words);
        for word in word_list {
            assert_eq!(word.len(), word_len);
        }
        assert_eq!(correct_word.len(), word_len);
    }

}