use crate::project_238_fallout_hacking::game_constants::{make_num_words,
                                                         make_word_length,
                                                         get_rand_num_x_to_y};

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::fmt::{Display, Formatter, Error};

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

impl Display for GameDifficulty {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            GameDifficulty::VeryEasy => write!(f, "Very Easy"),
            GameDifficulty::Easy => write!(f, "Easy"),
            GameDifficulty::Average => write!(f, "Average"),
            GameDifficulty::Hard => write!(f, "Hard"),
            GameDifficulty::VeryHard => write!(f, "Very Hard")
        };
        Ok(())
    }
}


impl FalloutHackingGame {

    fn make_game(difficulty: GameDifficulty) -> FalloutHackingGame {
        let num_words = make_num_words(&difficulty);
        let word_length = make_word_length(&difficulty);
        let (other_words, correct_word) =
            FalloutHackingGame::get_word_list_and_correct_word(word_length, num_words);
        FalloutHackingGame {
            word_length,
            other_words,
            correct_word,
            difficulty
        }
    }

    fn get_word_list_and_correct_word(word_len: usize, num_words: usize) -> (Vec<String>, String) {
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

    fn make_start_of_game_print(&self) -> String {
        let difficulty_str_format = format!("Difficulty: {}\n", self.difficulty);
        return difficulty_str_format
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
        let game =
            FalloutHackingGame::make_game(GameDifficulty::VeryEasy);
        assert_eq!(game.correct_word.len(), game.word_length);

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
        let (word_list, correct_word) =
            FalloutHackingGame::get_word_list_and_correct_word(word_len, num_words);
        assert_eq!(word_list.len(), num_words);
        for word in word_list {
            assert_eq!(word.len(), word_len);
        }
        assert_eq!(correct_word.len(), word_len);
    }

    #[test]
    fn test_make_start_of_game_print() {
        let game =
            FalloutHackingGame::make_game(GameDifficulty::VeryEasy);
        let game_print = game.make_start_of_game_print();
        println!("{}", game_print);
        assert!(game_print.contains("Difficulty: Very Easy\n"));

    }


}