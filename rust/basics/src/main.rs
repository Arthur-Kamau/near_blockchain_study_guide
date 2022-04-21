fn main() {

// parameters
    another_function(5);


//return 
    let x = five();
    println!("Terun The value of x is: {}", x);


    // controll flow 
    let number = 3;

    if number < 5 {
        println!("number less than 5, condition was true");
    } else {
        println!("greater than 5, condition was false");
    }


    // assignment on condition
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number asignment condition  is: {}", number);


    //infinite loop
    let mut count = 0;
    println!("count = {}", count);
    loop {

        if(count==10){
            break
        }
         count += 1;
    }
    // also check `https://doc.rust-lang.org/book/ch03-05-control-flow.html`
}

// parameters
fn another_function(x: i32) {
    println!("Parameter The value of x is: {}", x);
}

//return 
fn five() -> i32 {
   return 5
   // or just `5`
}

