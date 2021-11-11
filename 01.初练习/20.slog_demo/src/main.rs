mod enemy;
mod player;
mod weapon;

use enemy::Enemy;
use player::Player;
use rand::Rng;
use slog::{info, o, Drain, Logger};
use slog_async::Async;
use std::io::stdout;
use std::thread;
use std::time::Duration;

pub trait PlayingCharacter {
    fn shoot(&self);
}

struct Game {
    logger: Logger,
    player: Player,
    enemy: Enemy,
}

impl Game {
    fn simulate(&mut self) {
        info!(self.logger, "Launching game!");
        let enemy_or_player: Vec<&dyn PlayingCharacter> = vec![&self.enemy, &self.player];
        loop {
            let mut rng = rand::thread_rng();
            let a = rng.gen_range(500, 1000);
            thread::sleep(Duration::from_millis(a));
            let player = enemy_or_player[{
                if a % 2 == 0 {
                    1
                } else {
                    0
                }
            }];
            player.shoot();
        }
    }
}

fn main() {
    // 使用 slog_json::Json 创建了变量 drain，它可以将消息记录为JSON 对象
    let drain = slog_json::Json::new(stdout())
        .add_default_keys()
        .build()
        .fuse();
    // 将其传递给另一种管道 Async，它会将所有日志调用转移到单独的线程。
    let async_drain = Async::new(drain).build().fuse();
    let game_info = format!("v{}", env!("CARGO_PKG_VERSION"));
    // o!宏传递日志消息的初始上下文来创建 root_logger
    //使用环境变量 CARGO_PKG_VERSION 输出游戏的名称和版本
    let root_log_context = o!("CARGO_PKG_VERSION"=>game_info);
    let root_logger = Logger::root(async_drain, root_log_context);
    let mut game = Game {
        logger: root_logger.clone(),
        player: Player::new(&root_logger, "Bob"),
        enemy: Enemy::new(&root_logger, "Malice"),
    };
    game.simulate()
}
