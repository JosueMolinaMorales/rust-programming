use std::collections::HashMap;

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
    let five = &vec[1];
    println!("{}", five);

    // Iterating over string
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // Hashmap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores)
}
