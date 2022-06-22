


// fn main(){
//     println!("Hello, world!");
//     fibonacci_numbers();
//     for number in fibonacci_numbers() {
//         println!("{}", number);
// }

// #[derive(Debug)]
// struct Fibonacci {
//     a: u32,
//     b: u32,
// }

// impl Iterator for Fibonacci {
//     type Item = u32;

//     fn next(&mut self) -> Option<u32> {
//         Some(self.a+self.b)
//     }
// }

// fn fibonacci_numbers() -> Fibonacci {
//     Fibonacci { a: 1, b: 0 }
// }


// Bài 2: Lifetime
// Yêu cầu: Sửa lỗi Lifetime 

use std::fmt;

#[derive(Debug)]
struct  StrDisplayable (Vec<&'static str>);

impl fmt::Display for StrDisplayable{
    fn fmt<'a>(&'a self, f: &'a mut fmt::Formatter) ->fmt::Result {
        for v in &self.0 {
            write!(f, "\n{}",v)?;
        }
        Ok(())
    }
}

fn main() {
        let vec: Vec<&str> = vec!["a","bc","def"];
        let vec_Foo = StrDisplayable(vec);
        println!("{}",vec_Foo);
}