use baby_stark_math_lib;

use crate::field::FieldElement;

mod field;
// mod polynomial;

fn main() {
    println!("Hello, world!");

    let x = baby_stark_math_lib::gcd::gcd(28, 32);
    println!("x = {}", x);

}


