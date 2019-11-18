
extern crate rand;
use self::rand::Rng;

use crate::project_238_fallout_hacking::fallout_hacking::GameDifficulty;

/*
Constants representing the minimum and maximum number of words the player
may select from for a given difficulty
*/
const MIN_VERY_EASY_NW :usize = 4;
const MAX_VERY_EASY_NW :usize = 7;

const MIN_EASY_NW :usize = 5;
const MAX_EASY_NW :usize = 8;

const MIN_AVERAGE_NW :usize = 6;
const MAX_AVERAGE_NW :usize = 10;

const MIN_HARD_NW :usize = 8;
const MAX_HARD_NW :usize = 12;

const MIN_VERY_HARD_NW:usize = 10;
const MAX_VERY_HARD_NW:usize = 14;

pub fn get_word_number_range(difficulty: &GameDifficulty) -> (usize, usize) {
    return match difficulty {
        GameDifficulty::VeryEasy => (MIN_VERY_EASY_NW, MAX_VERY_EASY_NW),
        GameDifficulty::Easy => (MIN_EASY_NW, MAX_EASY_NW),
        GameDifficulty::Average => (MIN_AVERAGE_NW, MAX_AVERAGE_NW),
        GameDifficulty::Hard => (MIN_HARD_NW, MAX_HARD_NW),
        GameDifficulty::VeryHard => (MIN_VERY_HARD_NW, MAX_VERY_HARD_NW)
    };
}

/*
Constants representing the minimum and maximum length of each word the player
may select from for a given difficulty
*/
const MIN_VERY_EASY_WL :usize = 4;//[4]
const MAX_VERY_EASY_WL :usize = 5;

const MIN_EASY_WL :usize = 5;//[5,6,7]
const MAX_EASY_WL :usize = 8;

const MIN_AVERAGE_WL :usize = 8;//[8,9]
const MAX_AVERAGE_WL :usize = 10;

const MIN_HARD_WL :usize = 10;//[10,11,12]
const MAX_HARD_WL :usize = 13;

const MIN_VERY_HARD_WL:usize = 13;//[13,14,15]
const MAX_VERY_HARD_WL:usize = 16;

fn get_word_length_range(difficulty: &GameDifficulty) -> (usize, usize) {

    return match difficulty {
        GameDifficulty::VeryEasy => (MIN_VERY_EASY_WL, MAX_VERY_EASY_WL),
        GameDifficulty::Easy => (MIN_EASY_WL, MAX_EASY_WL),
        GameDifficulty::Average => (MIN_AVERAGE_WL, MAX_AVERAGE_WL),
        GameDifficulty::Hard => (MIN_HARD_WL, MAX_HARD_WL),
        GameDifficulty::VeryHard => (MIN_VERY_HARD_WL, MAX_VERY_HARD_WL)
    };
}

fn get_num_words_range(difficulty: &GameDifficulty) -> (usize, usize) {

    return match difficulty {
        GameDifficulty::VeryEasy => (MIN_VERY_EASY_NW, MAX_VERY_EASY_NW),
        GameDifficulty::Easy => (MIN_EASY_NW, MAX_EASY_NW),
        GameDifficulty::Average => (MIN_AVERAGE_NW, MAX_AVERAGE_NW),
        GameDifficulty::Hard => (MIN_HARD_NW, MAX_HARD_NW),
        GameDifficulty::VeryHard => (MIN_VERY_HARD_NW, MAX_VERY_HARD_NW)
    };
}

pub fn make_word_length(difficulty: &GameDifficulty) -> usize {
    let mut rng = rand::thread_rng();
    let (min_wl, max_wl) = get_word_length_range(&difficulty);
    return rng.gen_range(min_wl, max_wl);
}

pub fn make_num_words(difficulty: &GameDifficulty) -> usize {
    let mut rng = rand::thread_rng();
    let (min_nw, max_nw) = get_num_words_range(difficulty);
    return rng.gen_range(min_nw, max_nw);
}

pub fn get_rand_num_x_to_y(x: usize, y: usize) -> usize {
    let mut rng = rand::thread_rng();
    return rng.gen_range(x, y);
}

pub const NUM_GUESSES_PER_GAME: usize = 4;


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_num_words() {
        let very_easy_nw = make_num_words(&GameDifficulty::VeryEasy);
        let easy_nw = make_num_words(&GameDifficulty::Easy);
        let avg_nw = make_num_words(&GameDifficulty::Average);
        let hard_nw = make_num_words(&GameDifficulty::Hard);
        let very_hard_nw = make_num_words(&GameDifficulty::VeryHard);

        let very_easy_range = MIN_VERY_EASY_NW..MAX_VERY_EASY_NW;
        let easy_range = MIN_EASY_NW..MAX_EASY_NW;
        let avg_range = MIN_AVERAGE_NW..MAX_AVERAGE_NW;
        let hard_range = MIN_HARD_NW..MAX_HARD_NW;
        let very_hard_range = MIN_VERY_HARD_NW..MAX_VERY_HARD_NW;

        assert!(very_easy_range.contains(&very_easy_nw));
        assert!(easy_range.contains(&easy_nw));
        assert!(avg_range.contains(&avg_nw));
        assert!(hard_range.contains(&hard_nw));
        assert!(very_hard_range.contains(&very_hard_nw));
    }

    #[test]
    fn test_get_word_length() {
        let very_easy_wl = make_word_length(&GameDifficulty::VeryEasy);
        let easy_wl = make_word_length(&GameDifficulty::Easy);
        let avg_wl = make_word_length(&GameDifficulty::Average);
        let hard_wl = make_word_length(&GameDifficulty::Hard);
        let very_hard_wl = make_word_length(&GameDifficulty::VeryHard);

        let very_easy_range = MIN_VERY_EASY_WL..MAX_VERY_EASY_WL;
        let easy_range = MIN_EASY_WL..MAX_EASY_WL;
        let avg_range = MIN_AVERAGE_WL..MAX_AVERAGE_WL;
        let hard_range = MIN_HARD_WL..MAX_HARD_WL;
        let very_hard_range = MIN_VERY_HARD_WL..MAX_VERY_HARD_WL;

        assert!(very_easy_range.contains(&very_easy_wl));
        assert!(easy_range.contains(&easy_wl));
        assert!(avg_range.contains(&avg_wl));
        assert!(hard_range.contains(&hard_wl));
        assert!(very_hard_range.contains(&very_hard_wl));

    }

}
