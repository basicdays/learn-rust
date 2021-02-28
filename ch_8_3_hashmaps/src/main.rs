#![allow(unused_mut, unused_variables)]
use std::collections::HashMap;

fn part1() {
	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);
}

fn part2() {
	let teams = vec![String::from("Blue"), String::from("Yellow")];
	let initial_scores = vec![10, 50];

	let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
}

fn part3() {
	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);

	let team_name = String::from("Blue");
	let score = scores.get(&team_name);
}

fn part4() {
	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);

	for (key, value) in &scores {
		println!("{}: {}", key, value);
	}
}

fn part5() {
	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Blue"), 25);

	println!("{:?}", scores);
}

fn part6() {
	let mut scores = HashMap::new();
	scores.insert(String::from("Blue"), 10);

	scores.entry(String::from("Yellow")).or_insert(50);
	scores.entry(String::from("Blue")).or_insert(50);

	println!("{:?}", scores);
}

fn part7() {
	let text = "hello world wonderful world";

	let mut map = HashMap::new();

	for word in text.split_whitespace() {
		let count = map.entry(word).or_insert(0);
		*count += 1;
	}

	println!("{:?}", map);
}

fn main() {
	part1();
	part2();
	part3();
	part4();
	part5();
	part6();
	part7();
}
