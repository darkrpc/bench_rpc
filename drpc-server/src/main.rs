#![feature(test)]
extern crate serde;
extern crate test;

use std::process::exit;
use std::time::Duration;
use drpc::client::Client;
use drpc::codec::BinCodec;
use drpc::server::Server;


//cargo run --release --package drpc-server --bin drpc-server
#[tokio::main]
async fn main() {
    tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("start bench");
        run().await;
    });
    let mut s = Server::<BinCodec>::new();
    s.register_fn("handle", handle);
    s.serve("0.0.0.0:10000").await;
}

pub async fn handle(req: i32) -> drpc::Result<i32> {
    Ok(req + 1)
}

async fn run() {
    let c = Client::<BinCodec>::dial("127.0.0.1:10000").await.unwrap();
    let now = std::time::Instant::now();
    let total = 100000;
    for _ in 0..total {
        let resp: i32 = c.call("handle", 1).await.unwrap();
    }
    now.qps(total);
    now.time(total);
    _ = exit(0);
}


pub trait QPS {
    fn qps(&self, total: u64);
    fn time(&self, total: u64);
    fn cost(&self);
}

impl QPS for std::time::Instant {
    fn qps(&self, total: u64) {
        let time = self.elapsed();
        println!(
            "Total {},QPS: {} QPS/s",
            total,
            (total as u128 * 1000000000 as u128 / time.as_nanos() as u128)
        );
    }

    fn time(&self, total: u64) {
        let time = self.elapsed();
        println!(
            "Total {},Time: {:?} ,each:{} ns/op",
            total,
            &time,
            time.as_nanos() / (total as u128)
        );
    }

    fn cost(&self) {
        let time = self.elapsed();
        println!("cost:{:?}", time);
    }
}