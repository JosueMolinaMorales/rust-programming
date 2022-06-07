#[derive(Debug)] // Allows the struct to be printed using {:?}
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    /* OLD WAY
    let width1 = 30;
    let height1 = 30;

    println!(
        "The area of the rectangle is {} square pixels",
        area(width1, height1)
    );
    */
    let scale = 2;
    // New way with structs
    let rectangle = Rectangle {
        width: dbg!(30 * scale), /* Prints: [src\main.rs:20] 30 * scale = 60 */
        height: 30
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rectangle)
    );

    println!("The width is: {}", rectangle.width);

    // println!("The rectangle: {}", rectangle); ERROR: Struct does not implement std::fmt::Display
    println!("The rectangle: {:#?}", rectangle);

    dbg!(&rectangle);
}

// fn area(width: u32, hieght: u32) -> u32 {
//     width * hieght
// }

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
