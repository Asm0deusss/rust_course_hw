#![forbid(unsafe_code)]

pub mod config;
pub mod games;
pub mod r#trait;

use config::{get_game, GameConfig};
use r#trait::{FairRound, Round, UnfairRound};

type Game = Box<dyn Round>;

pub fn play_game(x: &mut Game, fair_rounds: usize, unfair_rounds: usize) -> Option<u8> {
    let mut last_win: u8 = 0;

    if fair_rounds == 0 && unfair_rounds == 0 {
        return None;
    }

    for _ in 0..fair_rounds {
        last_win = <dyn Round as FairRound>::play(x.as_mut());
    }

    for _ in 0..unfair_rounds {
        last_win = <dyn Round as UnfairRound>::play(x.as_mut());
    }

    Some(last_win)
}

pub fn play_games(games: &[(String, usize, usize)]) -> Vec<Option<u8>> {
    let mut res: Vec<Option<u8>> = vec![];

    for cur_game in games.iter() {
        let x = serde_json::from_str::<GameConfig>(&cur_game.0).unwrap();
        res.push(play_game(&mut get_game(x), cur_game.1, cur_game.2));
    }

    res
}
