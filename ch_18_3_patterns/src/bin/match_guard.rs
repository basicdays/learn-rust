fn part1() {
	let num = Some(4);

	match num {
		Some(x) if x < 5 => println!("less than five: {}", x),
		Some(x) => println!("{}", x),
		None => (),
	}
}

fn part2() {
	let x = Some(5);
	let y = 10;

	match x {
		Some(50) => println!("Got 50"),
		Some(n) if n == y => println!("Matched, n = {}", n),
		_ => println!("Default case, x = {:?}", x),
	}

	println!("at the end: x = {:?}, y = {}", x, y);
}

fn part3() {
	let x = 4;
	let y = false;

	match x {
		4 | 5 | 6 if y => println!("yes"),
		_ => println!("no"),
	}
}

fn main() {
	part1();
	part2();
	part3();
}
