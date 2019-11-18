extern crate dailyprogrammer_lib;

use dailyprogrammer_lib::project_238_fallout_hacking::fallout_hacking::{FalloutHackingGame,
                                                                        GameDifficulty};

use std::io;
use std::io::prelude::*;


fn main() -> io::Result<()> {

    let difficulty_res = collect_difficulty();

    match difficulty_res {
        Ok(difficulty) => {
            let game = FalloutHackingGame::make_game(difficulty);
            let game_print = game.print_game();

            return make_guesses(game);
        }
        Err(err_msg) => println!("{}", err_msg)
    }

    Ok(())

}

fn make_guesses(game: FalloutHackingGame) -> io::Result<()> {
    let (next_game, has_won) = make_guess(game);
    if has_won {
        println!("Congratulations! You've won!");
        return Ok(());
    }
    else {
        next_game.print_game();
        return make_guesses(next_game);
    }
}

fn make_guess(game: FalloutHackingGame) -> (FalloutHackingGame, bool) {
    let mut next_guess = String::new();
    io::stdin().read_line(&mut next_guess);
    next_guess = next_guess.to_ascii_lowercase().trim().parse().unwrap();
    let maybe_game = game.guess(next_guess);
    match maybe_game {
        Ok((new_game, has_won)) => (new_game, has_won),
        Err((msg, old_game)) => {
            println!("{}", msg);
            return (old_game, false)
        }
    }
}


fn collect_difficulty() -> Result<GameDifficulty, &'static str> {

    println!("Enter your desired difficulty. Options:");
    println!("Very Easy (1)");
    println!("Easy (2)");
    println!("Average (3)");
    println!("Hard (4)");
    println!("Very Hard (5)");
    println!("q to quit");

    let mut difficulty_selection = String::new();
    io::stdin().read_line(&mut difficulty_selection);
    let parsable_text = difficulty_selection.trim();

    return match parsable_text.to_ascii_lowercase().as_str() {
        "1" => Ok(GameDifficulty::VeryEasy),
        "very easy" => Ok(GameDifficulty::VeryEasy),
        "2" => Ok(GameDifficulty::Easy),
        "easy" => Ok(GameDifficulty::Easy),
        "3" => Ok(GameDifficulty::Average),
        "average" => Ok(GameDifficulty::Average),
        "4" => Ok(GameDifficulty::Hard),
        "hard" => Ok(GameDifficulty::Hard),
        "5" => Ok(GameDifficulty::VeryHard),
        "very hard" => Ok(GameDifficulty::VeryHard),
        "q" => Err("Quitting"),
        _ => collect_difficulty()
    };

}