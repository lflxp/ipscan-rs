// mod ping;
// use ping::pings;

mod server;

use server::run;

fn main() {
    println!("Hello, world!");
    // pings();

    run();
}
