// 解析ip，如:10-200.10.1-100.10 => [iplist]

pub trait Parse {
	fn parse(&self) -> Vec<&str>;
	fn string(&self) -> String;
}

#[derive(Debug)]
pub struct Ips {
	pub input: String,
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

// 扫描ip
pub fn scan(item: impl Parse) -> impl Parse {
	println!("Scan ips {}",item.string());
	item
}