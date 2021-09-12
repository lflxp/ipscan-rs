mod pkg;
use pkg::{scan};

use std::env;
use crate::pkg::scan::{Parse,Ips};
use futures::executor::block_on;
mod network_operator;
mod validator;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    // validate argument - check argument length
    // due to ownership, we give a reference of the vector to the function
    if !validator::is_valid_length(&args) {
        println!("Usage: ping [<hostname> | <ip>]");
        return;
    }

    // get argument type (ip, hostname)
    let arg_type: network_operator::InputType = validator::get_type_of_arg(&args[1]);
    if arg_type == network_operator::InputType::UNKNOWN {
        println!("Error: could not recongize the input type (hostname or ip)");
        return;
    }

    // println! is a macro
    println!("__________[ Ping ]__________");
    println!("Received a type: {:?}", &arg_type);
    println!("Use ctrl-C to get report");

    // 添加ctrl+c 退出
    network_operator::register_sig_action();

    // let info: Ips = scan::new(args).unwrap();
    let info: Ips = scan::new2(args);
    
    if info.isok {
        let detail = info.parse();
        // block_on(scan::run(detail));
        let rs = block_on(scan::run(detail));

        match network_operator::ping2(rs) {
            Ok(_) => (), 
            Err(e) => {
                println!("{}", e);
                ()
            }
        }
    }
}
