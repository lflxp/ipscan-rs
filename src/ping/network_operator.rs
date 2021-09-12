//! This module is used for handling operations related to networking

use std::error::Error;
use std::thread;
use std::time;
use oping::{Ping, PingResult};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum InputType {
    IPv4,
    IPv6,
    HOSTNAME,
    UNKNOWN
}

static mut IS_LOOPING: bool = true;
static mut SUCCESS: u32 = 0;
static mut FAILURE: u32 = 0;
static mut TIME: f64 = 0.0;

use stopwatch::{Stopwatch};

/**
 * This function registers an signal action handler to SIGINT.
 * At signal interrrupt, the function should let the ping loop
 * stop and create a report. 
 */
pub fn register_sig_action() {
    
    // set an action handler
    match ctrlc::set_handler(move || {
        // set the value of is_pinging to false to stop
        // since it is not threadsafe, use unsafe
        // however, this program only uses main thread so it's fine
        unsafe {
            IS_LOOPING = false;
        }

    }) {
        Ok(_) => {}
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

/**
 * This function sends a ping and waits for response.
 * Based on the result, it updates the success_ratio.
 * 
 * `address` - Value of the address
 */
pub fn ping(address: &str) {
    let start = Stopwatch::start_new();
    // keep pinging 
    unsafe {
        while IS_LOOPING {

            // ping every 1 second
            thread::sleep(time::Duration::from_millis(1000));

            // create ICMP packet using external library oping
            // while searching for external rust libraries, I realized
            // many of them are not being updated anymore or was abandoned.
            let mut ping = Ping::new();
            // max wait time is 5 seconds
            match ping.set_timeout(5.0) {
                Ok(_) => {},
                Err(err) => {
                    println!("{:?}", err);
                } 
            }
            // set host based on address type
            match ping.add_host(address) {
                Ok(_) => {},
                Err(err) => {
                    println!("{:?}", err);
                }
            }

            // send ICMP packet
            let responses = match ping.send() {
                Ok(iter) => iter,
                Err(err) => {
                    // timeout or sudo
                    println!("Please run with \"sudo\"");
                    println!("{}", err);
                    // update result as failure
                    FAILURE += 1;
                    continue;
                }
            };

            // check response and update result
            for response in responses {
                if response.dropped > 0 {
                    println!("No response from {} (loss)", response.address);
                } else {
                    // display success result
                    println!("Response from {} with {} ms", response.address, response.latency_ms);
                    // update result
                    SUCCESS += 1;
                    TIME += response.latency_ms;
                }
            }
        }
    }

    // print result
    print_result(start);
}

/**
 * This function prints the result of pings to the terminal
 */
pub async fn print_result(tt: Stopwatch) {
    println!("--- Ping result ---");
    unsafe {
        println!("TOTAL  : {} packets", SUCCESS + FAILURE);
        println!("SUCCESS: {}", SUCCESS);
        println!("FAILURE: {}", FAILURE);
        // safe casting using keyword as
        println!("TIME   : {:.3} ms", TIME / (SUCCESS + FAILURE) as f64);
        println!("Times: {:.3} ms", tt.elapsed_ms() as f64)
    }
}

pub fn ping2(address: Vec<String>) -> PingResult<()> {
    let start = Stopwatch::start_new();
    unsafe {
        for addr in address.iter() {
            // println!("ip is |{}|", addr);
    
            // create ICMP packet using external library oping
            // while searching for external rust libraries, I realized
            // many of them are not being updated anymore or was abandoned.
            let mut ping = Ping::new();
            
            // max wait time is 5 seconds
            ping.set_timeout(0.1)?;
            ping.add_host(addr)?;
    
            // send ICMP packet
            let responses = ping.send()?;
    
            // check response and update result
            for response in responses {
                if response.dropped > 0 {
                    println!("No response from host {} (loss)", response.address);
                    FAILURE += 1;
                } else {
                    // display success result
                    println!("Response from host {} (address {}): latency {} ms", response.hostname, response.address, response.latency_ms);
                    SUCCESS += 1;
                    TIME += response.latency_ms;
                }
            }
        }
    }

    // print result
    print_result(start);
    Ok(())
}

// async
pub fn ping3(address: Vec<String>) -> PingResult<()>{
    let start = Stopwatch::start_new();
    // create ICMP packet using external library oping
    // while searching for external rust libraries, I realized
    // many of them are not being updated anymore or was abandoned.
    let mut ping = Ping::new();
    
    // max wait time is 5 seconds
    ping.set_timeout(0.1)?;

    for addr in address.iter() {
        // println!("ip is |{}|", addr);
        ping.add_host(addr)?;        
    } 

    // send ICMP packet
    let responses = ping.send()?;

    // check response and update result
    for response in responses {
        // if response.dropped > 0 {
        //     println!("No response from host {} (loss)", response.address);
        // } else {
        //     // display success result
        //     println!("Response from host {} (address {}): latency {} ms", response.hostname, response.address, response.latency_ms);
        // }

        if response.dropped <= 0 {
            println!("Response from host {} (address {}): latency {} ms", response.hostname, response.address, response.latency_ms);
        }
    }

    // print result
    print_result(start);

    Ok(())
}

// async spawn process
async fn ping6(address: &str) -> PingResult<()>{
    // create ICMP packet using external library oping
    // while searching for external rust libraries, I realized
    // many of them are not being updated anymore or was abandoned.
    let mut ping = Ping::new();
    
    // max wait time is 5 seconds
    ping.set_timeout(0.1)?;

        // println!("ip is |{}|", addr);
    ping.add_host(address)?;        

    // send ICMP packet
    let responses = ping.send()?;

    unsafe {
        // check response and update result
        for response in responses {
            if response.dropped > 0 {
                println!("No response from host {} (loss)", response.address);
                FAILURE += 1;
            } else {
                // display success result
                println!("Response from host {} (address {}): latency {} ms", response.hostname, response.address, response.latency_ms);
                SUCCESS += 1;
                TIME += response.latency_ms;
            }

            // if response.dropped <= 0 {
            //     println!("Response from host {} (address {}): latency {} ms", response.hostname, response.address, response.latency_ms);
            // }
        }
    }

    Ok(())
}

pub async fn ping6run(addr: &str) {
    // 不处理错误
    ping6(addr).await.unwrap();
}