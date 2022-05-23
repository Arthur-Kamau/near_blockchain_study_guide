
/**
 * In case of primitive types, contents from one variable is copied to another. So, there is no ownership move happening. This is because a primitive variable needs less resources than an object.

*/
fn main(){
   let u1 = 10;
   let u2 = u1;  // u1 value copied(not moved) to u2

   println!("u1 = {}",u1);
}

/*
*
fn main(){
    let v = vec![1,2,3];     // vector v owns the object in heap
    let v2 = v;              // moves ownership to v2
    display(v2);             // v2 is moved to display and v2 is invalidated
    println!("In main {:?}",v2);    //v2 is No longer usable here
 }
 fn display(v:Vec<i32>){
    println!("inside display {:?}",v);
 }

 */