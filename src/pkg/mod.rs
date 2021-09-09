// mod parse;
// use parse::{scan,Parse};

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
					data.push(format!("{}.{}.{}.{}\n",x,y,z,g));
					// if y%18 == 0 {
					// 	print!("push {}.{}.{}.{}\n",x,y,z,g);
					// }
					print!("push {}.{}.{}.{}\n",x,y,z,g);
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
