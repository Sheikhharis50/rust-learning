fn main() {
    let number: u32 = 5;

    println!("for loop from 1..5");
    for i in 1..number {
        println!("{}", i);
    }

    println!("while loop from 1..5");
    let mut i = 1;
    while i < number {
        println!("{}", i);
        i += 1;
    }
}
