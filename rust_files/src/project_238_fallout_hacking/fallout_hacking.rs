extern crate rand;

use self::rand::Rng;


//Constants
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



struct FalloutHackingGame {

    word_length: usize,
    other_words: Vec<String>,
    correct_word: String,
    difficulty: GameDifficulty
}

enum GameDifficulty {
    VeryEasy,
    Easy,
    Average,
    Hard,
    VeryHard
}

impl FalloutHackingGame {

    fn make_game(difficulty: GameDifficulty) -> FalloutHackingGame {
        FalloutHackingGame {
            word_length:0,
            other_words: vec![],
            correct_word: "".to_owned(),
            difficulty
        }
    }

}

fn make_word_length(difficulty: GameDifficulty) -> usize {
    let mut rng = rand::thread_rng();
    return match difficulty {
        GameDifficulty::VeryEasy => rng.gen_range(MIN_VERY_EASY_WL, MAX_VERY_EASY_WL),
        GameDifficulty::Easy => rng.gen_range(MIN_EASY_WL, MAX_EASY_WL),
        GameDifficulty::Average => rng.gen_range(MIN_AVERAGE_WL, MAX_AVERAGE_WL),
        GameDifficulty::Hard => rng.gen_range(MIN_HARD_WL, MAX_HARD_WL),
        GameDifficulty::VeryHard => rng.gen_range(MIN_VERY_HARD_WL, MAX_VERY_HARD_WL)
    };
}




#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_make_game_model() {


        assert!(false);
    }

    #[test]
    fn test_get_word_length() {
        let very_easy_wl = make_word_length(GameDifficulty::VeryEasy);
        let easy_wl = make_word_length(GameDifficulty::Easy);
        let avg_wl = make_word_length(GameDifficulty::Average);
        let hard_wl = make_word_length(GameDifficulty::Hard);
        let very_hard_wl = make_word_length(GameDifficulty::VeryHard);

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