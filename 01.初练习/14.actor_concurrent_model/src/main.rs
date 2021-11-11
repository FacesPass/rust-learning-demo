// 与消息传递模型非常相似的并发模型是 actor 模型
use actix::prelude::*;
use futures::future;
use futures::future::Future;
use std::time::Duration;
use std::time::Instant;
use tokio::timer::Delay;

struct Add(u32, u32);
impl Message for Add {
    type Result = Result<u32, ()>;
}

struct Adder;
impl Actor for Adder {
    type Context = SyncContext<Self>;
}

impl Handler<Add> for Adder {
    type Result = Result<u32, ()>;

    fn handle(&mut self, msg: Add, _: &mut Self::Context) -> Self::Result {
        let sum = msg.0 + msg.1;
        println!("Computed: {} + {} = {}", msg.0, msg.1, sum);
        Ok(msg.0 + msg.1)
    }
}

fn main() {
    System::run(|| {
        let adder = SyncArbiter::start(3, || Adder);
        for n in 5..10 {
            adder.do_send(Add(n, n + 1));
        }

        tokio::spawn(future::lazy(|| {
            Delay::new(Instant::now() + Duration::from_secs(1)).then(|_| {
                System::current().stop();
                future::ok(())
            })
        }));
    });
}
