mod math;

use math::add;
use rand::{Rng, random};

fn main() {
    // println!("Hello, world!");
    let b = random::<char>();
    println!("{}", b);

    if random() && random() {
        println!("You won twice in a row!");
    } else {
        println!("Try again...");
    }

    let result = add(1, 2);
    println!("1 + 2 = {}", result);
}
