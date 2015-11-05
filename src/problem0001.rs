/*
Project Euler 0001

If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
The sum of these multiples is 23.
Find the sum of all the multiples of 3 or 5 below 1000.
*/

pub fn run() -> u32 {
    solution_immut(1000)
}

#[allow(dead_code)]
fn solution_mut(n : u32) -> u32 {
    let mut sum = 0;
    for x in 0..n {
        if x % 3 == 0 || x % 5 == 0 {
            sum += x;
        }
    }
    sum
}

#[allow(dead_code)]
fn solution_immut(n : u32) -> u32 {
    (0..n)
        .filter( |x| x % 3 == 0 || x % 5 == 0 )
        .fold( 0, |sum, x| x + sum )
}
