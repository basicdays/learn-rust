#![allow(dead_code, unused_variables)]

fn part1() {
	fn largest<T: PartialOrd>(list: &[T]) -> &T {
		let mut largest = &list[0];

		for item in list {
			if item > largest {
				largest = item;
			}
		}

		largest
	}

	let number_list = vec![34, 50, 25, 100, 65];

	let result = largest(&number_list);
	println!("The largest number is {}", result);
	assert_eq!(result, &100);

	let char_list = vec!['y', 'm', 'a', 'q'];

	let result = largest(&char_list);
	println!("The largest char is {}", result);
	assert_eq!(result, &'y');
}

fn part2() {
	struct Point<T> {
		x: T,
		y: T,
	}

	let integer = Point { x: 5, y: 10 };
	let float = Point { x: 1.0, y: 4.0 };
}

fn part3() {
	struct Point<T> {
		x: T,
		y: T,
	}

	impl<T> Point<T> {
		fn x(&self) -> &T {
			&self.x
		}
	}

	impl Point<f32> {
		fn distance_from_origin(&self) -> f32 {
			(self.x.powi(2) + self.y.powi(2)).sqrt()
		}
	}

	let p = Point { x: 5, y: 10 };

	println!("p.x = {}", p.x());
}

fn part4() {
	struct Point<T, U> {
		x: T,
		y: U,
	}

	impl<T, U> Point<T, U> {
		fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
			Point {
				x: self.x,
				y: other.y,
			}
		}
	}

	let p1 = Point { x: 5, y: 10.4 };
	let p2 = Point { x: "Hello", y: 'c' };

	let p3 = p1.mixup(p2);

	println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn main() {
	part1();
	part2();
	part3();
	part4();
}
