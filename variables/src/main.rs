fn variables() {
    // initiallize immutable variable
    let x = 5;
    println!("x = {}", x);

    // reinitiallize mutable variable
    let mut x = x + 1;
    println!("x = {}", x);

    // change mutable variable
    x = 6;
    println!("x = {}", x);

    // destructure values into variables
    let (bunnies, carrots) = (8, 12);
    println!("bunnies = {}, carrots = {}", bunnies, carrots);

    let spaces = "   ";
    println!("spaces length: {}", spaces.len());

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {}", guess);

    let y: f32 = 3.22; // f32
    println!("Foat: {}", y);

    let f: bool = false; // with explicit type annotation
    println!("Boolean: {}", f);

    let z: char = 'ℤ';
    println!("Character: {}", z);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple (500, 6.4, 1): {:?}", tup);

    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array ([1, 2, 3, 4, 5]): {:?}", arr);

    println!("Array 2nd index value (arr[2]): {}", arr[2]);

    arr = [3; 5];
    println!("Array ([3; 5]): {:?}", arr);
}

fn operations() {
    // addition
    let sum = 5 + 10;
    println!("sum: 5 + 10 = {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: 95.5 - 4.3 = {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("product: 4 * 30 = {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient: 56.7 / 32.2 = {}", quotient);
    let truncated = -5 / 3; // Results in -1
    println!("truncated: -5 / 3 = {}", truncated);

    // remainder
    let remainder = 43 % 5;
    println!("remainder: 43 % 5 = {}", remainder);
}

fn main() {
    println!("-> Variables Start");
    variables();
    println!("-> Variables End");

    println!("-> Operations Start");
    operations();
    println!("-> Operations End");
}
