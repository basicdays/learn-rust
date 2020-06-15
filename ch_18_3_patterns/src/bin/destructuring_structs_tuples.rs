#![allow(unused_variables)]

fn main() {
	struct Point {
		x: i32,
		y: i32,
	}

	// 4 vars: feet, inches, x, y
	let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}
