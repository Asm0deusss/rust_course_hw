#![forbid(unsafe_code)]

use crate::r#trait::{FairRound, InitGame, UnfairRound};

use super::config::RollDiceConfig;

use serde::Deserialize;
#[derive(Deserialize)]
pub struct RollDiceGame {
    cfg: RollDiceConfig,
    last_win: u8,
}

impl FairRound for RollDiceGame {
    fn play(&mut self) -> u8 {
        let mut first_score: f64 = 0.0;
        let mut second_score: f64 = 0.0;

        for i in 0..6 {
            if i % 2 == 0 {
                first_score += i as f64 * self.cfg.probas[i];
            } else {
                second_score += i as f64 * self.cfg.probas[i];
            }
        }

        let last_win = if first_score >= second_score {
            self.cfg.players.0
        } else {
            self.cfg.players.1
        };

        self.last_win = last_win;
        last_win
    }
}

impl UnfairRound for RollDiceGame {
    fn play(&mut self) -> u8 {
        let last_win = if self.last_win == self.cfg.players.0 {
            0
        } else {
            1
        };

        let mut mx_value: f64 = self.cfg.probas[0];
        let mut mx_index: usize = 0;
        let mut mn_value: f64 = self.cfg.probas[0];
        let mut mn_index: usize = 0;

        for i in 0..6 {
            if last_win == 0 {
                if i % 2 == 0 {
                    if mx_value < self.cfg.probas[i] {
                        mx_value = self.cfg.probas[i];
                        mx_index = i;
                    }
                } else if mn_value > self.cfg.probas[i] {
                    mn_value = self.cfg.probas[i];
                    mn_index = i;
                }
            } else if i % 2 == 0 {
                if mn_value > self.cfg.probas[i] {
                    mn_value = self.cfg.probas[i];
                    mn_index = i;
                }
            } else if mx_value < self.cfg.probas[i] {
                mx_value = self.cfg.probas[i];
                mx_index = i;
            }
        }

        if mx_value > mn_value {
            self.cfg.probas[mn_index] = mx_value;
            self.cfg.probas[mx_index] = mn_value;
        }

        FairRound::play(self)
    }
}

impl InitGame<RollDiceConfig> for RollDiceGame {
    fn init(config: RollDiceConfig) -> Self {
        Self {
            cfg: (config),
            last_win: (0),
        }
    }
}
