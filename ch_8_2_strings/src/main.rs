#![allow(unused_variables)]

fn part1() {
	let mut s1 = String::from("foo");
	let s2 = "bar";
	s1.push_str(s2);
	println!("s2 is {}", s2);
}

fn part2() {
	let s1 = String::from("Hello, ");
	let s2 = String::from("world!");
	let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
}

fn part3() {
	let s1 = String::from("tic");
	let s2 = String::from("tac");
	let s3 = String::from("toe");

	let s = format!("{}-{}-{}", s1, s2, s3);
}

fn part4() {
	for c in "नमस्ते".chars() {
		println!("{}", c);
	}
}

fn part5() {
	for b in "नमस्ते".bytes() {
		println!("{}", b);
	}
}

fn main() {
	part1();
	part2();
	part3();
	part4();
	part5();
}
