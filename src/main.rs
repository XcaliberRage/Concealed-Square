#![feature(destructuring_assignment)]
use regex::Regex;
use cast::{u64, f64};

const START_SQUARE: u64 = 1020304050607080900;

fn main() {

    let pat = Regex::new(r"^1\d2\d3\d4\d5\d6\d7\d8\d9\d0$").unwrap();
    let mut pow = START_SQUARE;

    let mut sqrt = (pow as f64).sqrt().ceil();

    let mut sqrt_pow = (sqrt as u64).pow(2);

    while !pat.is_match(&*sqrt_pow.to_string()) {

        println!(" The square root of {} is {}", sqrt_pow, sqrt);

        loop {
            sqrt += 1.0;
            sqrt_pow = (sqrt as u64).pow(2);

            if sqrt_pow % 10 == 0 { break; }
        }

    }

    println!("The square root of {} is {}", sqrt_pow, sqrt);
}


fn decs_inc(mut decs: Vec<u64>, i: usize) -> Vec<u64> {


    if decs[i] == 9 {
        if i == 8{
            panic!("Can't increment higher!");
        }
        decs[i] = 0;
        return decs_inc(decs, i+1);
    } else {
        decs[i] += 1;
        return decs;
    }

}