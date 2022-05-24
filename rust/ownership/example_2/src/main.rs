//one
#[derive(Debug)] // no more copy
struct Person {
    age: u8
}

fn main() {
    let alice = Person { age: 8 };
    let bob = &alice; // bob borrows alice

    println!("alice: {:?}\nbob: {:?}", alice, bob);
}


/*
// two

fn sum(vector: &Vec<i32>) -> i32 { // borrow signature
    let mut sum = 0;

    for item in vector {
        sum = sum + item
    }

    sum
}

fn main() {
    let v = vec![1,2,3,4];
   // let v_ref = &v;  // v_ref borrows v
    let s = sum(&v);

    println!("sum of {:?}: {}", v, s); // no error
}
*/
 

//four
/*
fn main() {
    let a = vec![1, 2, 3];
    let b = a;

    println!("a: {:?}\nb: {:?}", a, b); // error: borrow of moved value: `a`
}
*/

// five
/*
fn sum(vector: Vec<i32>) -> i32 {
    let mut sum = 0; // mutability, more on this later

    for item in vector {
        sum = sum + item
    }

    sum
}

fn main() {
    let v = vec![1,2,3,4];
    let s = sum(v); // watch out, v was MOVED!

    println!("sum: {}", s);
}

*/