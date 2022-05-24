//one
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

//two
// pub fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// // This is a really bad adding function, its purpose is to fail in this
// // example.
// #[allow(dead_code)]
// fn bad_add(a: i32, b: i32) -> i32 {
//     a - b
// }

// #[cfg(test)]
// mod tests {
//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use super::*;

//     #[test]
//     fn test_add() {
//         assert_eq!(add(1, 2), 3);
//     }

//     #[test]
//     fn test_bad_add() {
//         // This assert would fire and test will fail.
//         // Please note, that private functions can be tested too!
//         assert_eq!(bad_add(1, 2), 3);
//     }
// }



//three
//None of the previous unit test examples had a return type. But in Rust 2018, your unit tests can return Result<()>, which lets you use ? in them! This can make them much more concise.
// fn sqrt(number: f64) -> Result<f64, String> {
//     if number >= 0.0 {
//         Ok(number.powf(0.5))
//     } else {
//         Err("negative floats don't have square roots".to_owned())
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_sqrt() -> Result<(), String> {
//         let x = 4.0;
//         assert_eq!(sqrt(x)?.powf(2.0), x);
//         // return Err("Sample erro".to_owned())
//         return Ok(());
//     }
// }


//four 
// cargo test -- --ignored
// pub fn add_two(a: i32) -> i32 {
//     a + 2
// }

// #[test]
// fn it_works() {
//     assert_eq!(4, add_two(2));
// }

// #[test]
// #[ignore]
// fn expensive_test() {
//     // Code that takes an hour to run...
// }