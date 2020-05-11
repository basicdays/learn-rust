fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    // todo: I'm sure there's a better way to do this
    println!(
        "five is {}",
        five.map_or(String::from("none"), |val| val.to_string())
    );

    let six = plus_one(five);
    match six {
        Some(val) => println!("six is {}", val),
        None => println!("six is none"),
    }

    let none = plus_one(None);
    if let Some(val) = none {
        println!("none is {}", val);
    } else {
        println!("none is none");
    }
}
