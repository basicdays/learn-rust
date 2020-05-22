#![allow(unused_variables)]

fn part1() {
    // type is implied as i32 due to usage
    let mut v1 = Vec::new();

    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);
}

fn part2() {
    let v2 = vec![1, 2, 3, 4, 5];

    // would panic if index 2 didn't exist
    let third = &v2[2];
    println!("The third element is {}", third);

    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

fn part3() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}

fn part4() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}
fn part5() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn main() {
    part1();
    part2();
    part3();
    part4();
    part5();
}
