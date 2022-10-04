#![forbid(unsafe_code)]

use crate::r#trait::{FairRound, InitGame, UnfairRound};

use super::config::FlipCoinConfig;
use serde::Deserialize;
#[derive(Deserialize)]
pub struct FlipCoinGame {
    cfg: FlipCoinConfig,
    birth_win: bool,
    last_winner: u8,
}

impl FairRound for FlipCoinGame {
    fn play(&mut self) -> u8 {
        let last_win;
        let mut birth_win = false;

        if self.cfg.players_proba.0 == self.cfg.players_proba.1 {
            last_win = self.cfg.birthday_player;
            birth_win = true;
        } else if self.cfg.players_proba.0 > self.cfg.players_proba.1 {
            last_win = self.cfg.players.0;
        } else {
            last_win = self.cfg.players.1;
        }

        self.last_winner = last_win;
        self.birth_win = birth_win;
        last_win
    }
}

impl UnfairRound for FlipCoinGame {
    fn play(&mut self) -> u8 {
        if self.birth_win && self.last_winner == self.cfg.players.0 {
            self.cfg.players_proba.0 -= 0.2;
            self.cfg.players_proba.1 += 0.2;
        }

        FairRound::play(self)
    }
}

impl InitGame<FlipCoinConfig> for FlipCoinGame {
    fn init(config: FlipCoinConfig) -> Self {
        Self {
            cfg: (config),
            birth_win: (false),
            last_winner: (0),
        }
    }
}
