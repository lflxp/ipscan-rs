use async_std::task::{sleep, spawn};
use std::time::Duration;

async fn sleep1() {
	for i in 1..=10 {
		println!("Sleepus {}", i);
		sleep(Duration::from_millis(500)).await;
	}
}

async fn intus() {
	for i in 1..=5 {
		println!("Intus {}", i);
		sleep(Duration::from_millis(1000)).await;
	}
}

pub async fn demo() {
	let s1 = spawn(sleep1());
	intus().await;

	s1.await;
}