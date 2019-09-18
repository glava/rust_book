
//    Each value in Rust has a variable that’s called its owner.
//    There can only be one owner at a time.
//    When the owner goes out of scope, the value will be dropped.
// 
//   
// At any given time, you can have either one mutable reference or any number of immutable references.
//    References must always be valid.


fn main() {
    let s = String::from("hello");  // s comes into scope.
    let s2 = s; // this is not shallow copy nor deep. this is move as s is no longer valid
    
    // s2's value is now valid, you can't use s...
    // you can't move s because compiler 
    // a) think s2 is owner and s is not important
    // b) double free error trying to drop from both places
    // ... and so is no longer valid here.

    // the only way to use it if you use clone and do a deep copy

    let s3 = s2.clone();

    print!("{}, {}", s3, s2);

    // this can't be applied for data types stored on heap
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    takes_ownership(s2);             


    

    let x = 5;                      // x comes into scope.

    makes_copy(x);                  // x would move into the function,
    // but i32 is Copy, so it’s okay to still
    // use x afterward.

    //if you want with ownership to continue using the passed
    // variable you need to return it (maybe in a tuple with function
    // value of computation)
    let some_string = String::from("hello");  // s comes into scope.
    let (some_string, len) = i_return_back_ownershipt_by_passing_through(some_string);

    // or use the reference way of passing value
    let len2 = operation_on_borrowed_variable(&some_string);

    // you can chage only if you marke it as mut on funciton arg
    
    let mut s = String::from("from mutable variable");
    let len3 = mutable_operation_on_borrowed_variable(&mut s);



} // Here, x goes out of scope, then s. But since s's value was moved, nothing
// special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope.
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn i_return_back_ownershipt_by_passing_through(some_string: String) -> (String, usize) {
    let len= some_string.len();
    (some_string, len)
}

fn operation_on_borrowed_variable(some_string: &String) -> usize {
    // but because you are borrowing you can't change it!
    some_string.len()
}

fn mutable_operation_on_borrowed_variable(some_string: &mut String) {
    // but because you are borrowing you can't change it!
    some_string.push_str("hello again")
}