fn conditions(number: u32, comparison: u32) {
    if number == comparison {
        println!("wohoo! found the number");
    } else if number < comparison {
        println!("too less");
    } else {
        println!("too big");
    }
}

fn main() {
    println!("Condition not matched");
    conditions(5, 1);
    println!("Condition matched");
    conditions(5, 5);
}
