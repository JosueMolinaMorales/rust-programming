fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("{}, {}, World!", s1, s2);

    let s = String::from("Hello");
    //takes_ownership(s);
    println!("After method: {}", s);

    let a_str = String::from("Hello");
    let len = calculate_length(&a_str);

    println!("The size of {} is {}", a_str, len);
}

fn calculate_length(some_string: &String) -> usize {
    some_string.len()
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}