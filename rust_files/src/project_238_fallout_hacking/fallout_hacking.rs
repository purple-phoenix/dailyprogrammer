
struct FalloutHackingGame {

    size: usize

}

impl FalloutHackingGame {

    fn make_game() -> FalloutHackingGame {
        FalloutHackingGame {size:0}
    }

}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_make_game_model() {


        assert!(false);
        assert_eq!(FalloutHackingGame{size:0}, FalloutHackingGame::make_game())
    }

}