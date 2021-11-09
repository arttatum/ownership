fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
                                    // println!("{}", s); would cause compiler error.
                                    
    let t = String::from("hello");  // t comes into scope

    let s3 = takes_ownership_and_gives_back(t); // t leaves scope, s3 comes into scope                                    

    let y = calculate_length(&s3);  // passes a REFERENCE to s3 into the function, s3 remains in scope

    let mut q = String::from("hello"); // mutable variable q comes into scope.

    change(&mut q); // the change method 'borrows' the value of q, since we're passing in a (mutable) reference

    // q now has the value "Hello, world!" since the change method appended to the string.
    
    let x = 5;                      // x comes into scope


    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn takes_ownership_and_gives_back(some_string: String) -> String { // some_string comes into scope
    println!("{}", some_string);
    some_string
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.
  
  fn calculate_length(s: &String) -> usize { // s is a reference to a String. s 'borrows' the value from the argument, but gives it back.
  s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, nothing happens.
// By default, references are immutable, just like variables.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.