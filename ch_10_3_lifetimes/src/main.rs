#![allow(dead_code, unused_variables)]

fn part1() {
	fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
		if x.len() > y.len() {
			x
		} else {
			y
		}
	}

	// fn wrong_lifetimes() {
	//     let a = "a";
	//     let c;
	//     {
	//         let b = String::from("b");
	//         c = longest(a, &b);
	//     }
	//     println!("{}", c);
	// }

	let string1 = String::from("abcd");
	let string2: &'static str = "xyz";

	let result = longest(string1.as_str(), string2);
	println!("The longest string is {}", result);
}

fn part2() {
	struct ImportantExcerpt<'a> {
		part: &'a str,
	}
	impl<'a> ImportantExcerpt<'a> {
		fn level(&self) -> i32 {
			3
		}
	}

	fn main() {
		let novel = String::from("Call me Ishmael. Some years ago...");
		let first_sentence = novel.split('.').next().expect("Could not find a '.'");
		let i = ImportantExcerpt {
			part: first_sentence,
		};
	}
}

fn part3() {
	use std::fmt::Display;

	fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
	where
		T: Display,
	{
		println!("Announcement! {}", ann);
		if x.len() > y.len() {
			x
		} else {
			y
		}
	}
}

fn main() {
	part1();
}
