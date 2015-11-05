/*
Project Euler 0003

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
*/

use num::integer;
use std::collections::HashSet;
use std::cmp;

pub fn run() -> u64 {
    largest_prime_factor( 600851475143 )
}

fn largest_prime_factor( num : u64 ) -> u64 {
    if num <= 3 { num }
    else {
        factorise( num, 2 )
            .iter()
            .fold( 1, |largest, &p| cmp::max( largest, p ))
    }
}

fn factorise( mut num : u64, mut try_factor : u64 ) -> HashSet<u64> {
    let mut factors = HashSet::new();
    while num > 1 {
        let reduced = reduce_by(num, try_factor);
        if num != reduced {
            num = reduced;
            factors.insert(try_factor);
        }
        if try_factor == 2 {
            try_factor = 3;
        }
        else {
            try_factor += 2;
        }
    }
    factors
}

fn reduce_by( mut num : u64, try_factor : u64 ) -> u64 {
    while num % try_factor == 0 {
        num = integer::div_floor(num, try_factor);
    }
    num
}
