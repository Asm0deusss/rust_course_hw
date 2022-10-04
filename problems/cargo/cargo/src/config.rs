#![forbid(unsafe_code)]

use crate::games::flip_coin::config::FlipCoinConfig;
use crate::games::roll_dice::config::RollDiceConfig;
use crate::games::{flip_coin::game::FlipCoinGame, roll_dice::game::RollDiceGame};
use crate::r#trait::{init_game, Round};

use serde::Deserialize;
#[derive(Deserialize)]
#[serde(tag = "type", content = "config")]
pub enum GameConfig {
    FlipCoin(FlipCoinConfig),
    RollDice(RollDiceConfig),
}

pub fn get_game(config: GameConfig) -> Box<dyn Round> {
    let object: Box<dyn Round> = match config {
        GameConfig::FlipCoin(flip_coin_config) => {
            Box::new(init_game::<FlipCoinGame, _>(flip_coin_config))
        }
        GameConfig::RollDice(roll_dice_config) => {
            Box::new(init_game::<RollDiceGame, _>(roll_dice_config))
        }
    };
    object
}
