use std::thread::{sleep, spawn};
use std::time::Duration;

fn sleepus() {
	for i in 1..=10 {
		println!("Sleepus {}",i);
		sleep(Duration::from_millis(500));
	}
}

fn interruptus() {
	for i in 1..=5 {
		println!("Interruptus {}",i);
		sleep(Duration::from_millis(500));
	}
}

pub fn test1() {
	sleepus();
	interruptus();
}

pub fn test2() {
	let sleepus = spawn(sleepus);
	let int = spawn(interruptus);

	sleepus.join().unwrap();
	int.join().unwrap();
}

pub fn test3() {
	let sleepus = spawn(sleepus);
	interruptus();

	sleepus.join().unwrap();
}

mod asyncnew;
use asyncnew::demo;

pub async fn test4() {
	demo().await;
}