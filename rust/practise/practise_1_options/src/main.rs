use std::fs::File;
use std::io::ErrorKind;

fn main() {

    //one
    //     let x: Result<i32, &str> = Ok(-3);
    // assert_eq!(x.is_ok(), true);

    // let x: Result<i32, &str> = Err("Some error message");
    // assert_eq!(x.is_ok(), false);

    // two
    //     let x: Result<i32, &str> = Ok(-3);
    // assert_eq!(x.is_err(), false);

    // let x: Result<i32, &str> = Err("Some error message");
    // assert_eq!(x.is_err(), true);

    // three
    //     let x: Result<u32, &str> = Ok(2);
    // assert_eq!(x.err(), None);

    // let x: Result<u32, &str> = Err("Nothing here");
    // assert_eq!(x.err(), Some("Nothing here"));

    // four
    // fn write_message() -> io::Result<()> {
    //     let mut file = File::create("valuable_data.txt")?;
    //     file.write_all(b"important message")?;
    //     Ok(())
    // }

    // five
    // let f = File::open("hello.txt");
    // let _f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };
}
