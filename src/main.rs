


fn main(){
    println!("Hello, world!");
    for number in fibonacci_numbers() {
        println!("{}", number);
}
}
#[derive(Debug)]
struct Fibonacci {
    a: u32,
    b: u32,
}


impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new_a = self.a + self.b;

		self.b= self.a;
		self.a = new_a;

		Some(self.a)
    }
}

fn fibonacci_numbers() -> Fibonacci {
    Fibonacci { a: 1, b: 0 }
}


// Bài 2: Lifetime
// Yêu cầu: Sửa lỗi Lifetime 

// use std::fmt;

// #[derive(Debug)]
// struct  StrDisplayable (Vec<&'static str>);

// impl fmt::Display for StrDisplayable{
//     fn fmt<'a>(&'a self, f: &'a mut fmt::Formatter) ->fmt::Result {
//         for v in &self.0 {
//             write!(f, "\n{}",v)?;
//         }
//         Ok(())
//     }
// }

// fn main() {
//         let vec: Vec<&str> = vec!["a","bc","def"];
//         let vec_Foo = StrDisplayable(vec);
//         println!("{}",vec_Foo);
// }