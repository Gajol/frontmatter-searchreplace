use serde::{Serialize, Deserialize};
use std::env;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);

    // Prints each argument on a separate line
    for argument in env::args() {
        println!("argument: {}", argument);
    }

    let args: Vec<String> = env::args().collect();
    //    let filename: &args[2];  fails - learn???

    println!("first file: {}", &args[2]);

    let contents = fs::read_to_string(&args[2])
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

}
