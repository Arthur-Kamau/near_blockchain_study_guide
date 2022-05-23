mod example;
use example::{example_function_1, example_function_2};

mod samples;

fn main() {
    println!("Hello, world!");

    //CALLING FUNCTION FROM EXAMPLE MOD
    example_function_1();
    example_function_2();

    // CALLING FUNCTION FROM SAMPLE MOD
    samples::sample::sample_function();
}
