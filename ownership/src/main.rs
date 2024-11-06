fn main() {
    println!("Scope: ");
    let mut s = String::from("hello");
    {
        s.push_str(", world");
        println!("s = {}\n", s);
    }

    println!("Ownership (Heap): ");
    let s1 = String::from("Haris");
    let s2 = s1.clone() + " Zahid";
    println!("s1 = {}", s1);
    println!("s2 = {}\n", s2);

    println!("Ownership (Stack): ");
    let x = 5;
    let y = x + 1;
    println!("x = {}, y = {}\n", x, y);

    println!("Take Ownership: ");
    s = takes_ownership(s);
    println!("{}\n", s);

    println!("Gives Ownership: ");
    let s3 = gives_ownership();
    println!("{}\n", s3);

    println!("Calculate Length: ");
    let s4 = String::from("hello");
    let (s4, len) = calculate_length(s4);
    println!("The length of '{}' is {}.\n", s4, len);
}

fn takes_ownership(some_string: String) -> String {
    // some_string comes into scope
    println!("{some_string}");
    return some_string;
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope
    some_string // some_string is returned and
                // moves out to the calling
                // function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
