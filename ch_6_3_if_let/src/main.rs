#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
enum UsState {
	Alabama,
	Alaska,
	// --snip--
}

enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState),
}

fn main() {
	let coin = Coin::Penny;
	let mut count = 0;
	if let Coin::Quarter(state) = coin {
		println!("State quarter from {:?}!", state);
	} else {
		count += 1;
	}
	println!("count is {}", count);
}
