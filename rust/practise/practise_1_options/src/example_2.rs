//zero
// // Re-implementation of integer division (/)
// fn division(dividend: i32, divisor: i32) -> i32 {
//     if divisor == 0 {
//         // Division by zero triggers a panic
//         panic!("division by zero");
//     } else {
//         dividend / divisor
//     }
// }

// // The `main` task
// fn main() {
//     // Heap allocated integer
//     let _x = Box::new(0i32);

//     // This operation will trigger a task failure
//     division(3, 0);

//     println!("This point won't be reached!");

//     // `_x` should get destroyed at this point
// }

//one
// // An integer division that doesn't `panic!`
// fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
//     if divisor == 0 {
//         // Failure is represented as the `None` variant
//         None
//     } else {
//         // Result is wrapped in a `Some` variant
//         Some(dividend / divisor)
//     }
// }

// // This function handles a division that may not succeed
// fn try_division(dividend: i32, divisor: i32) {
//     // `Option` values can be pattern matched, just like other enums
//     match checked_division(dividend, divisor) {
//         None => println!("{} / {} failed!", dividend, divisor),
//         Some(quotient) => {
//             println!("{} / {} = {}", dividend, divisor, quotient)
//         }
//     }
// }

// fn main() {
//     try_division(4, 2);
//     try_division(1, 0);

//     // Binding `None` to a variable needs to be type annotated
//     let none: Option<i32> = None;
//     let _equivalent_none = None::<i32>;

//     let optional_float = Some(0f32);

//     // Unwrapping a `Some` variant will extract the value wrapped.
//     println!(
//         "{:?} unwraps to {:?}",
//         optional_float,
//         optional_float.unwrap()
//     );

//     // Unwrapping a `None` variant will `panic!`
//     println!("{:?} unwraps to {:?}", none, none.unwrap());
// }

// two
// use std::fs::File;
// use std::io::prelude::*;
// use std::io;

// struct Info {
//     name: String,
//     age: i32,
//     rating: i32,
// }

// fn write_info(info: &Info) -> io::Result<()> {
//     // Early return on error
//     let mut file = match File::create("my_best_friends.txt") {
//            Err(e) => return Err(e),
//            Ok(f) => f,
//     };
//     if let Err(e) = file.write_all(format!("name: {}\n", info.name).as_bytes()) {
//         return Err(e)
//     }
//     if let Err(e) = file.write_all(format!("age: {}\n", info.age).as_bytes()) {
//         return Err(e)
//     }
//     if let Err(e) = file.write_all(format!("rating: {}\n", info.rating).as_bytes()) {
//         return Err(e)
//     }
//     Ok(())
// }

//two
// use std::fs::File;
// use std::io::prelude::*;
// use std::io;

// struct Info {
//     name: String,
//     age: i32,
//     rating: i32,
// }

// fn write_info(info: &Info) -> io::Result<()> {
//     let mut file = File::create("my_best_friends.txt")?;
//     // Early return on error
//     file.write_all(format!("name: {}\n", info.name).as_bytes())?;
//     file.write_all(format!("age: {}\n", info.age).as_bytes())?;
//     file.write_all(format!("rating: {}\n", info.rating).as_bytes())?;
//     Ok(())
// }
