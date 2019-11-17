
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



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_make_game_model() {


        assert!(false);
    }

}