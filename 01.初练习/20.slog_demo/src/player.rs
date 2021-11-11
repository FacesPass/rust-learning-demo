use crate::weapon::PlasmaCannon;
use crate::PlayingCharacter;
use slog::{info, o, Logger};

pub struct Player {
  name: String,
  logger: Logger,
  weapon: PlasmaCannon,
}

impl PlayingCharacter for Player {
  fn shoot(&self) {
    info!(self.logger, "{} shooting with {}", self.name, self.weapon);
    self.weapon.fire();
  }
}

impl Player {
  pub fn new(logger: &Logger, name: &str) -> Self {
    // Player 上的 new 方法获取了根记录器
    let player_log = logger.new(o!("Player"=>format!("{}",name)));
    let weapon_log = player_log.new(o!("PlasmaCannon" => "M435"));
    Self {
      name: name.to_string(),
      logger: player_log,
      weapon: PlasmaCannon(weapon_log),
    }
  }
}
