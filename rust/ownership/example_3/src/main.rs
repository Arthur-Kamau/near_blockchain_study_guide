/*
//one

#[derive(Debug)]
struct DropMe;

impl Drop for DropMe {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}
impl DropMe{
    fn sample(&self) {
        println!("Sample!");
    }
}


/**
 *
 *
 * Begin outer scope.
 * Begin inner scope.
 * x: DropMe
 * Dropping!
 * End outer scope.
 */
fn main() {
    println!("Begin outer scope.");

    let vecs = vec![1,2,3,4];
    {
        println!("Begin inner scope.");
 
        let y =  vecs;
        let x = DropMe;

        println!("x: {:?}  == y: {:?}", x, y);
    }

    println!("End outer scope. {:?}", vecs);
}
*/

// two

fn pass_number_by_reference(number: &i8) -> bool {
    number.is_negative()
}

fn pass_number_by_value(number: i8) -> bool {
    number.is_negative()
}

fn pass_vec_by_reference(vec: &Vec<i8>) -> bool {
    vec.is_empty()
}

fn main() {
    // numbers implement Copy, and so can be passed by value or reference
    let number = 42;

    // does not move number because of borrow
    let is_negative_by_ref = pass_number_by_reference(&number);

    // moves number, which can never be used again
    let is_negative_by_value = pass_number_by_value(number);

    // copy not implemented - must be passed by reference
    let vec = vec![];

    // does not move vec
    let is_empty = pass_vec_by_reference(&vec);

    println!("is_negative_by_value: {}", is_negative_by_value);
    println!("is_negative_by_ref: {}", is_negative_by_ref);
    println!("vec {:?} is_empty: {}", vec, is_empty);
}


