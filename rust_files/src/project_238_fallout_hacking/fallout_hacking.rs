use crate::project_238_fallout_hacking::game_constants::{make_num_words,
                                                         make_word_length,
                                                         get_rand_num_x_to_y,
                                                         NUM_GUESSES_PER_GAME};

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::fmt::{Display, Formatter, Error};

pub struct FalloutHackingGame {

    word_length: usize,
    other_words: Vec<String>,
    correct_word: String,
    difficulty: GameDifficulty,
    current_guesses: usize
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

    pub fn make_game(difficulty: GameDifficulty) -> FalloutHackingGame {
        let num_words = make_num_words(&difficulty);
        let word_length = make_word_length(&difficulty);
        let (other_words, correct_word) =
            FalloutHackingGame::get_word_list_and_correct_word(word_length, num_words);
        FalloutHackingGame {
            word_length,
            other_words,
            correct_word,
            difficulty,
            current_guesses: 0
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

    fn make_game_print(&self) -> String {
        let mut game_print = format!("Difficulty: {}\n", self.difficulty);
        game_print += self.correct_word.to_ascii_uppercase().as_ref();
        game_print += "\n";
        for word in &self.other_words {
            game_print += word.to_ascii_uppercase().as_str();
            game_print += "\n";
        }

        return game_print
    }

    pub fn print_game(&self) -> () {
        println!("{}", self.make_game_print());
        println!("{}", self.make_guess_print());
    }

    fn make_guess_print(&self) -> String {
        let guesses_remaining = NUM_GUESSES_PER_GAME - self.current_guesses;
        return "Guess (".to_owned() +
            guesses_remaining.to_string().as_str() +
            " left)?".to_owned().as_ref()

    }



    fn increment_guess(self) -> FalloutHackingGame {
        if self.current_guesses == 3 {
            return FalloutHackingGame::make_game(self.difficulty);
        }
        return FalloutHackingGame {
            word_length: self.word_length,
            other_words: self.other_words,
            correct_word: self.correct_word,
            difficulty: self.difficulty,
            current_guesses: self.current_guesses + 1
        }
    }

    pub fn guess(self, guess_word: String) -> Result<(FalloutHackingGame, bool), (&'static str, FalloutHackingGame)> {
        if guess_word.len() != self.word_length {
            println!("{}  {}", guess_word.len(), self.word_length);
            for a_char in guess_word.chars() {
                println!("{}", a_char);
            }
            return Err(("Your guess is the wrong number of characters", self));
        }
        else {
            //TODO
            Ok((FalloutHackingGame::make_game(self.difficulty), false))
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
    use self::regex::Regex;

    extern crate regex;

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
    fn test_make_game_print() {
        let game =
            FalloutHackingGame::make_game(GameDifficulty::VeryEasy);
        let game_print = game.make_game_print();

        let total_words = game.other_words.len() + 1;

        let mut regex_str = r"Difficulty: Very Easy\n".to_owned();
        let word_regex = "(\\w{".to_owned() +
            game.word_length.to_string().as_ref() + "}\n)".to_owned().as_ref();
        let all_words_regex = word_regex + "{" + total_words.to_string().as_str() + "}";
        regex_str += all_words_regex.as_ref();

        let game_print_regex = Regex::new(regex_str.as_str()).unwrap();
        println!("Regex\n{}\n\n", regex_str);

        println!("{}", game_print);
        assert!(game_print_regex.is_match(game_print.as_str()));

    }

    #[test]
    fn test_make_guess_print() {
        let game =
            FalloutHackingGame::make_game(GameDifficulty::VeryHard);
        let init_guess_print = game.make_guess_print();
        assert_eq!(init_guess_print, "Guess (4 left)?");
        let game = game.increment_guess();
        assert_eq!(game.make_guess_print(), "Guess (3 left)?");
        let game = game.increment_guess();
        assert_eq!(game.make_guess_print(), "Guess (2 left)?");
        let game = game.increment_guess();
        assert_eq!(game.make_guess_print(), "Guess (1 left)?");
        let game = game.increment_guess();
        assert_eq!(game.make_guess_print(), "Guess (4 left)?");
    }


}