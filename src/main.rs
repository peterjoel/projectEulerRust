#![allow(dead_code)]

extern crate num;
extern crate primal;
extern crate time;

mod fib;
mod problem0001;
mod problem0002;
mod problem0003;

// cheap switch
use problem0003::{run};


fn main() {
    let t0 = time::precise_time_ns();
    let result = run();
    let t1 = time::precise_time_ns();
    println!("result = {:?}", result);
    println!("in {}s", ((t1-t0) as f64)/1e9);
}
