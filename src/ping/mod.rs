mod network_operator;
mod validator;

use std::env;
use crate::{ping::scan::{Parse,Ips}};
use futures::executor::block_on;
use async_std::task::{sleep, spawn};
use std::time::Duration;

// 串行执行
pub async fn pings() {
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
      // network_operator::ping2(rs)
  }
}

use stopwatch::{Stopwatch};

// 一个一个执行，并行执行
pub async fn pingsspawn() {
  let start = Stopwatch::start_new();
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

      // match network_operator::ping2(rs) {
      //     Ok(_) => (), 
      //     Err(e) => {
      //         println!("{}", e);
      //         ()
      //     }
      // }

      for r in rs {
        network_operator::ping6run(&r).await;
      }
  }
  network_operator::print_result(start).await
}

pub mod scan {
	use futures::{future::*, join};
	use std::thread;

	pub trait Parse {
		fn parse(&self) -> Vec<&str>;
		fn string(&self) -> String;
	}

	#[derive(Debug)]
	pub struct Ips {
		pub input: String,
		pub isok: bool,
	}

	impl Parse for Ips {
		// 实现接口定义方法
		fn parse(&self) -> Vec<&str> {
			let output: Vec<&str> = self.input.split(".").collect();

			output
		}

		fn string(&self) -> String {
			format!("input is {}",self.input)
		}
	}

	pub async fn run(data: Vec<&str>) -> Vec<String> {
		let mut a = Vec::new();
		let mut b = Vec::new();
		let mut c = Vec::new();
		let mut d = Vec::new();
		for (index,n) in data.iter().enumerate() {
			if index == 0 {
				a = strtoi32(n);
			} else if index == 1 {
				b = strtoi32(n)
			} else if index == 2 {
				c = strtoi32(n)
			} else if index == 3 {
				d = strtoi32(n)
			}
		}

		let info = getdata(a, b, c, d).await;
		info
		// getdata(a, b, c, d).await;
	}

	async fn getdata(a: Vec<i32>,b: Vec<i32>,c: Vec<i32>,d: Vec<i32>) -> Vec<String> {
		let data = &mut Vec::new();
		// for x in a.clone() {
		// 	for y in b.clone() {
		// 		for z in c.clone() {
		// 			for g in d.clone() {
		// 				data.push(format!("{}.{}.{}.{}\n",x,y,z,g));
		// 				print!("push {}.{}.{}.{}\n",x,y,z,g);
		// 			}
		// 		}
		// 	}
		// }

		// let mut funcList = Vec::new();
		for x in a.clone() {
			// funcList.push(getsubtask(x,b, c, d, &mut data));
			// funcList.push(getsubtask(x, b.clone(), c.clone(), d.clone()));

			// getsubtask(x, b.clone(), c.clone(), d.clone()).await;
			getsubtask(x, b.clone(), c.clone(), d.clone(),data).await;

			// thread::spawn(move||{
			// 	// getsubtask(x, b.clone(), c.clone(), d.clone());
			// 	test().await;
			// });
		}
		// join_all(funcList).await;
		data.to_vec()
	}

	async fn test() {
		for i in 0..=100 {
			print!("{}", i);
		}
	}

	async fn getsubtask(x:i32,b: Vec<i32>,c: Vec<i32>,d: Vec<i32>,data: &mut Vec<String>) {
		for y in b.clone() {
			for z in c.clone() {
				for g in d.clone() {
					data.push(format!("{}.{}.{}.{}",x,y,z,g));
					// if y%18 == 0 {
					// 	print!("push {}.{}.{}.{}\n",x,y,z,g);
					// }
					// print!("push {}.{}.{}.{}\n",x,y,z,g);
				}
			}
		}
	}

	fn strtoi32(input: &str) -> Vec<i32> {
		let mut v = Vec::new();
		if input.contains("-") {
			// Vec<str> -> Vec<i32> -> for x in a..b
			let tmp:Vec<i32> = input.split("-").map(|x| x.parse::<i32>().unwrap()).collect();
			for x in tmp[0]..tmp[1] {
				v.push(x);
			}
		} else {
			v.push(input.parse::<i32>().unwrap());
		}
		v
	}

	// pub fn new(args: Vec<String>) -> Result<Ips, &'static str> {
	// 	if args.len() < 2 {
	// 		println!("参数不足");
	// 		return Err("参数不足");
	// 	}

	// 	Ok(Ips {
	// 		input: args[1].clone().to_string(),
	// 		isok: true,
	// 	})
	// }

	pub fn new2(args: Vec<String>) -> Ips {
		if args.len() < 2 {
			println!("参数不足");
			return Ips {
				isok: false,
				input: "e.r.r.o.r".to_string(),
			}
		}

		Ips {
			input: args[1].clone().to_string(),
			isok: true,
		}
	}
}
