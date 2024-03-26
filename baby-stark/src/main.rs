use baby_stark_math_lib;

use crate::field::FieldElement;

pub mod field;
pub mod polynomial;
pub mod mpolynomial;

fn main() {
    println!("Hello, world!");

    let x = baby_stark_math_lib::gcd::gcd(28, 32);
    println!("x = {}", x);

}


