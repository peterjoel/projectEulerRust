use std::ops::Add;

pub struct Fib<T>
where T : Add<Output = T> + Copy {
    value: T,
    next: T,
}

impl <T> Fib<T>
where T : Add<Output = T> + Copy {
    pub fn new( a : T, b : T ) -> Fib<T> {
        Fib { value : a, next : b }
    }
}


impl <T> Iterator for Fib<T>
where T : Add<Output = T> + Copy
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let value = self.value;
        let next = value + self.next;
        self.value = self.next;
        self.next = next;
        Some( value )
    }
}
