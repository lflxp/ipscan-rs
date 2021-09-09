mod pkg;
use pkg::{scan};

use std::env;
use crate::pkg::scan::{Parse,Ips};
use futures::executor::block_on;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    // let info: Ips = scan::new(args).unwrap();
    let info: Ips = scan::new2(args);
    
    if info.isok {
        let detail = info.parse();
        // block_on(scan::run(detail));
        let rs = block_on(scan::run(detail));
        for x in rs {
            print!("------- {}", x);
        }
    }
}
