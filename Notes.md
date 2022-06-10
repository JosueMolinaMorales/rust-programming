# Rust Programming Language Notes

## Chapter 1

## Chapter 2

## Chapter 3 - Types and Variables
* By Default variables are immutable
```rust
let x = 6;
println!("The value of x is: {}", x);
x=6;
println!("The value of x is: {}", x); //ERROR
```
* There are constants but constants are different that variables

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

### Constants:
* You are not allowed to use 'mut' with constants
* Constants are always immutable
* Declare constants using 'const' instead of 'let'
* Constant variables must be type annotated

### Shadowing
You can delcare a new variables with the same name as a previous variable
The first variable is shadowed by the second, which means that the second variables value is what the program sees when the variable is used
We are effectively creating a new variable when we use the let keyword again,
So we can change the type of the value

```rust
let y = 7;
let y = y + 1;
{
    let y = y * 2;
    println!("The value of y in the inner scope is: {}", y);
}
```

### Scalar Types
* Scalar types represents a single value
* Integers
* Floating point
* Numbers
* Booleans
* Characters

#### Integer Types

| Length | Signed | Unsigned |
| ------ | ------ | -------- |
| 8 bit | `i8` | `u8` |
| 16 bit | `i16` | `u16` |
| 32 bit | `i32` | `u32` |
| 64 bit | `i64` | `u64` |
| 128 bit | `i128` | `u128` |
| arch | `isize` | `usize` |

* `isize` and `usize` depends on the architecutre of the computer your programming on. 64 bits if you are on a 64 bit machine, or 32 bit

#### Number literals
| Number Literals | Example |
| --------------- | ------- |
| Decimal | 98_222 or 98222 | 
| Hex | 0xff |
| Octal | 0o77 |
| Binary | ob1111_0000 |
| Byte (`u8` only) | b'A' |

#### Floating Point Types
Rust's floating-point types are f32 and f64. Default is f64

```rust
fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}
```

### Compound Types
Compound Types can group multiple values into one type. 

Rust has two primitive compound types: tubles and arrays

#### Tuble Type
A tuple is a general way of grouping together a numberof values with a variable of types into one compound type.

Tuples have a fixed length.

Each position in the tupe has a type, and the types of the different values in the tuple dont have to be the same. Annotations are optional
```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

Getting a value from a tuple:
```rust
fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; // Destructuring
    println!("The value of y is {}", y);
}
```

```rust
fn main() {
    let tup = (500, 6.4, 1);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
}
```

#### Array Type
Unlike tuples, every element of an array must have the same type.

Arrays in Rust have a **fixed length**

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

```rust
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
}
```

```rust
let a = [3; 5]; // a = [3, 3, 3, 3, 3]
```

##### Accessing array elements
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1]; 
}
```

### Functions
The `main` function is the entry point of many programs.

The keyword `fn` allows you to declare new functions

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

Variables can be assigned to expressions that return values. Such as:
* `let y = 6` 6 is an expression
* A function
* A macro
* A new Scope block

```rust
fn main() {
    let y = {
        let x = 7;
        x + 1
    };
}
```

The expression:
```rust
{
    let x = 7;
    x + 1
}
```

Evaluates to 8 and is stored in `y`

`x + 1` Does not have a semicolon. Expressions do not include ending semicolons. If you add a semicolon to the end of the express, you turn it into a statement and it will not return a value

#### Functions with Return Values
You don't name return values, but you declare their type after an arrow `->`

In Rust, The return value of the function is synonymous with the value of the final epxression in the block of the body of a function. 

You can return early from a function by using the `return` keyworked and specifying a value

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

### Comments

### Control Flow
#### If expressions
```rust
fn main() {
    let number = 3;
    if number < 5 {
        println!("True");
    } else {
        println!("False");
    }
}
```

Because if is an expression, we can use it on the right side of a let statment
```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}
```

The branches of the if statement must return the same value. Else an error can show:
```sh
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:4:44
  |
4 |     let number = if condition { 5 } else { "six" };
  |                                 -          ^^^^^ expected integer, found `&str`
  |                                 |
  |                                 expected because of this

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` due to previous error
```

### Loops
Rust has three kinds of loops: `loop`, `while` and `for`

#### Loop
The `loop` keyword tells Rust to execute a block of code over and over again until you expliciyt tell it to stop
```rust
fn main() {
    loop {
        println!("Again!");
    }
}
```
The keyword `break` within the lopp will tell the program when to stop

You can optionally add a *loop label* on a loop that we can use the break or continue to specify that those keywords apply to the labeled loops instead of the innermost loop

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("Count = {}", count);
        let mut remaining = 10;
        loop {
            println!("Remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", )
}
```

A use of `loop` is to retry an operation you know might fail. You might also need to pass the result of that operation out of the loop to the rest of your code

To do this you can add the value you want returned after the break expression you use to stop the loop.

```rust
fn main() {
    let mut count = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
}
```

#### For loops
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {}", element);
    }
}
```

Range *similar to python*
```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF");
}
```
`.rev()` reverses the range

## Chapter 4 - Understanding Ownership
### What is Ownership?
*Ownership* is a set of rules that governs how a Rust program manages memory

Rust approach: Memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won't compile.

#### Ownership Rules
* Each value in Rust has a variable that's called its *owner*
* There can only be on owner at a time
* When the owner goes out of scope, the value will be dropped

#### Variable Scope
A scope is the rnage whitin a program for which an item is valid

#### The String Type
You can create a string from a string literal using the `from` function

```rust
let s = String::from("Hello");
```
This kind of string can be mutated
```rust
let mut s = String::from("Hello");
s.push_str(", world!"); // push_str() appends a literal to a String
```
The memory is automatically returned once the variable that owns it goes out of scope

When doing the follow snippet:
```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;

    println!("{}, World!", s1);
}
```
Both s1 and s2 point to the same string object in memory. When s1 and s2 both go out of scope, the memory could be released twice. 

To avoid this, after the line `let s2 = s1` Rust considers `s1` as no longer valid, to ensure safe memory. Rust doesn't need to free anything when s1 goes out of scope.

Shallow and deep copies are what other langauges would refer to the above example. In Rust, this would be known as a move. `s1` was *moved* to `s2`

Rust will never create "deep" copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance

To create a deep copy, use the `clone()` method. 

#### Ownership and Functions
Passing a variables to a function will move or copy, just as assignment
```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

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

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```
When `s` is passed to `takes_ownership()` the function now has ownership of `s` and when the function finished executing, that memory is freed. `s` is no longer available to use

### References and Borrowing
A *reference* is like a pointer in thats its an address we can follow to access data stored at that saddress that is owned by some other variable.

Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

`&s1` sends a references, since it is a just a reference, `calculate_length()` does not take ownership of the variable, and can not release the memory

By default, references are immutable, so a compilation error will occur.

### Mutable References
```rust
fn main() {
    let mut s = String::from("Hello!");
    change(&mut s);
}
fn change(some_string: &mut String) {
    some_string.push_str(", World!");
}
```
Mutable references have one big restriction: You can have only one mutable reference to a particular piece of data at a time

A references scope starts from where it is introduced and continues through the last time that reference is used.

### Slice Type
A string slice is a reference to part of a String
```rust
let s = String::from("Hello world");

let hello = &s[0..5];
let world = &s[6..11];
```
`hello` is a reference to a portion of the String.

## Chapter 5 - Using Structs to Structure Related Data
A `struct` or `structure` is a custom data type that lets you package together and name multiple related values that make up a meaningful group.

A `struct` is like an objects data attributes.

### Defining and Instantiating Structs
In a struct, you will name each piece of data so its clear what the values mean

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
```
`active`, `username`, ... etc are known as fields

To use a struct, you create an instance of that struct by specifying concrete values for each field.
```rust
fn main() {
    let user = User {
        email: String::from("someone@wxample.com"),
        username: String::from("Someusername"),
        active: true,
        sign_in_count: 1
    };
}
```
To access a field: `user.username` if an instance of the struct is mutable, you can change the value of the field
```rust
fn main() {
    let user = User {
        email: String::from("someone@wxample.com"),
        username: String::from("Someusername"),
        active: true,
        sign_in_count: 1
    };
    user.email = String::from("NewEmail@hello.com");
}
```

To create a struct instance with information for another instance you do the following:
```rust
fn main() {
    let user1 = User {
        email: String::from("someone@wxample.com"),
        username: String::from("Someusername"),
        active: true,
        sign_in_count: 1
    };
    let user2 = User {
        email: String::from("user2email@mc.com");
        ..user1
    };
}
```
### An Example Program Using Structs
Files in ./5-Using-Structs/rectangles

### Method Syntax
Methods are similar to functions, but are defined within the context of a struct (or enum, or a trait object), and their first parameter is self, which represents the instance of the struct the method is being called on

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

To define the method within the context of `Rectangle`, use an `impl` (implementation) block

Everything within the `impl` block will be associated with the type

Note: that `&self` is not `self` because the method is only borrowing the reference

To change the instance of the struct in that method use `&mut self`

#### Associated Functions
All functions defined within the `impl` block are called *associated functions* b/c they're associated with the type named after the `impl`

Defining associated functions without the keyword `self` is possible, since a function may not need read access to the instance. An example of this is: `String::from`

Associated functions that aren't methods are often used for constructors that will return a new instance of the struct

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            heigth: size
        }
    }
}
```

To call this function, you use the `::` syntax with the struct name: `let sq = Rectangle::square(3)`

## Chapter 6 - Enums and Pattern Matching
### Defning an Enum
Enums are a way of defining custom data types in a differenet way than structs.

An example: Looking at the two major standards for IP addresses, since there are only two we can *enumerate* through the variants

An IP address can be version four or six but not both. An enum value can obly be one of its variants

```rust
enum IpAddrKind {
    V4,
    V6
}
```

#### Enum Values
Creating instances of each of the two variants
```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

At the momemnt we dont have a way to store the actual IP address *data*, we only know the kind.

We can use structs
```rust
enum IpAddrKind {
    V4,
    V6
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1")
};
let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1")
};
```

We can put data directly into each enum variant

```rust
enum IpAddr {
    V4(String),
    V6(String)
}
let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

The name of each enum variant that we define also becomes a function that constructs an instance of the enum.

`IpAddr::V4()` is a function call that takes a string argument and returns an instance of the IpAddr type.

Variants can have different types and amounts of associated data.

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}
let home = IpAddr::V4(127, 0, 0, 1);
```

You can put any kind of data inside an enum variant

Another example
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}
```

* `Quit` has not data associated with it all
* `Move` has named fields like a struct
* `Write` includes a single String
* `ChangeColor` incudes three i32 values

You are able to define methods on structs using impl
```rust
impl Message {
    fn call(&self) {
        // Method
    }
}
let m = Message::Write(String::from("Hello"));
m.call();
```

#### The Opetion Enum and Its Advantages Over Null Values
The `Option` type encodes the very common scenario in which a value could be something or it could be nothing

Rust does not have the value `null` but has an enum that can enccode the concept of a value being present or absent, `Option<T>`

```rust
let some_number = Some(5);
let some_string = Some("A string");

let absent_number: Option<i32> = None;
```

* `some_number` has type `Option<i32>`
* `some_string` has type `Option<&str>`
* The rust compiler requires annotations to a variable that holds the None value

Only when using `Option<T>` do you have to worry about not having a value for a variable

The following would throw an error:
```rust
let x = Some(5);
let y = 6;
x + y
```

### The match Control Flow Construct
`match` allows you to compare a value against a series of patterns and then execute code based on which pattern matches

Patterns can be made up of literal values, wildcards, and many other things

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

#### Matching with Option<T>
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}
let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

### Concise Control Flow with if let
The `if let` syntax lets you combine `if` and `let` to handle values that match one pattern while ignoring the rest

Without `if let`
```rust
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => ()
}
```

With `if let`
```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}
```

## Chapter 7 - 

## Chapter 8 - Common Collections

## Chapter 9 - Error Handling

## Chapter 10 - Generic Types, Traits, and Lifetimes

## Chapter 11 - Writing Automated Tests

## Chapter 13 - Functional Language Features: Iterators and Closures

## Chapter 15 - Smart Pointers

## Chapter 16 - Fearless Concurrency

## Chapter 17 - Object Oriented Programming Features of Rust

## Chapter 18 - Patterns and Matching

## Chapter 19 - Advanced Features



