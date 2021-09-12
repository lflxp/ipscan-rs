mod ping;
use ping::{pings, pingsspawn};

// mod server;
// use server::run;

mod mthread;
use mthread::{test1, test2, test3, test4};

use async_std::task::{sleep, spawn};


#[async_std::main]
pub async fn main() {
    println!("Hello, world!");
    pings().await;
    // let p1 = spawn(pings());
    let p1 = spawn(pingsspawn());

    // run();

    test1();
    test2();
    test3();
    test4().await;
    p1.await;
}

