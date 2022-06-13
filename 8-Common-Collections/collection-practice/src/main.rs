
fn main() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(3);
    vec.push(5);
    println!("{:?}", vec);

    // getting a number from a vector
    let three = match vec.get(3) {
        Some(val) => val,
        None => &1
    };
    println!("{}", three);

    // Geting a number by indexing
    let five = &vec[3];
    println!("{}", five);
}
