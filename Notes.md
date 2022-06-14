# Rust Programming Language Notes
## Table of Contents
- [Rust Programming Language Notes](#rust-programming-language-notes)
  - [Table of Contents](#table-of-contents)
  - [Chapter 1](#chapter-1)
  - [Chapter 2](#chapter-2)
  - [Chapter 3 - Types and Variables](#chapter-3---types-and-variables)
    - [Constants](#constants)
    - [Shadowing](#shadowing)
    - [Scalar Types](#scalar-types)
      - [Integer Types](#integer-types)
      - [Number literals](#number-literals)
      - [Floating Point Types](#floating-point-types)
    - [Compound Types](#compound-types)
      - [Tuble Type](#tuble-type)
      - [Array Type](#array-type)
        - [Accessing array elements](#accessing-array-elements)
    - [Functions](#functions)
      - [Functions with Return Values](#functions-with-return-values)
    - [Comments](#comments)
    - [Control Flow](#control-flow)
      - [If expressions](#if-expressions)
    - [Loops](#loops)
      - [Loop](#loop)
      - [For loops](#for-loops)
  - [Chapter 4 - Understanding Ownership](#chapter-4---understanding-ownership)
    - [What is Ownership?](#what-is-ownership)
      - [Ownership Rules](#ownership-rules)
      - [Variable Scope](#variable-scope)
      - [The String Type](#the-string-type)
      - [Ownership and Functions](#ownership-and-functions)
    - [References and Borrowing](#references-and-borrowing)
    - [Mutable References](#mutable-references)
    - [Slice Type](#slice-type)
  - [Chapter 5 - Using Structs to Structure Related Data](#chapter-5---using-structs-to-structure-related-data)
    - [Defining and Instantiating Structs](#defining-and-instantiating-structs)
    - [An Example Program Using Structs](#an-example-program-using-structs)
    - [Method Syntax](#method-syntax)
      - [Associated Functions](#associated-functions)
  - [Chapter 6 - Enums and Pattern Matching](#chapter-6---enums-and-pattern-matching)
    - [Defning an Enum](#defning-an-enum)
      - [Enum Values](#enum-values)
      - [The Opetion Enum and Its Advantages Over Null Values](#the-opetion-enum-and-its-advantages-over-null-values)
    - [The match Control Flow Construct](#the-match-control-flow-construct)
      - [Matching with Option](#matching-with-option)
    - [Concise Control Flow with if let](#concise-control-flow-with-if-let)
  - [Chapter 7 - Managing Growing Projects with Packages, Crates, and Modules](#chapter-7---managing-growing-projects-with-packages-crates-and-modules)
    - [Packages and Crates](#packages-and-crates)
    - [Defining Modules to Control Scope and Privacy](#defining-modules-to-control-scope-and-privacy)
      - [Modules Quick Reference](#modules-quick-reference)
      - [Defining a module](#defining-a-module)
    - [Paths for Referring to an item in the Module Tree](#paths-for-referring-to-an-item-in-the-module-tree)
    - [Exposing Paths with the pub Keyword](#exposing-paths-with-the-pub-keyword)
    - [Starting Relative Paths with super](#starting-relative-paths-with-super)
    - [Making Structs and Enums Public](#making-structs-and-enums-public)
    - [Bringing Paths into Scope with the use Keyword](#bringing-paths-into-scope-with-the-use-keyword)
      - [Providing New Names with the as Keyword](#providing-new-names-with-the-as-keyword)
      - [Using External Packages](#using-external-packages)
      - [The Glob Operator](#the-glob-operator)
    - [Separating Modules into Different Files](#separating-modules-into-different-files)
  - [Chapter 8 - Common Collections](#chapter-8---common-collections)
    - [Sotring Lists of Values with Vectors](#sotring-lists-of-values-with-vectors)
      - [Reading Elements of Vectors](#reading-elements-of-vectors)
      - [Iterating over the Values in a Vector](#iterating-over-the-values-in-a-vector)
      - [Using an Enum to Store Multiple Types](#using-an-enum-to-store-multiple-types)
    - [Storing UTF-8 Encoded Text with Strings](#storing-utf-8-encoded-text-with-strings)
      - [What is a String?](#what-is-a-string)
      - [Creating a New String](#creating-a-new-string)
      - [Updating a string](#updating-a-string)
  - [Chapter 9 - Error Handling](#chapter-9---error-handling)
  - [Chapter 10 - Generic Types, Traits, and Lifetimes](#chapter-10---generic-types-traits-and-lifetimes)
  - [Chapter 11 - Writing Automated Tests](#chapter-11---writing-automated-tests)
  - [Chapter 13 - Functional Language Features: Iterators and Closures](#chapter-13---functional-language-features-iterators-and-closures)
  - [Chapter 15 - Smart Pointers](#chapter-15---smart-pointers)
  - [Chapter 16 - Fearless Concurrency](#chapter-16---fearless-concurrency)
  - [Chapter 17 - Object Oriented Programming Features of Rust](#chapter-17---object-oriented-programming-features-of-rust)
  - [Chapter 18 - Patterns and Matching](#chapter-18---patterns-and-matching)
  - [Chapter 19 - Advanced Features](#chapter-19---advanced-features)

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

### Constants
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

#### Matching with Option
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
`if let` takes a pattern and an expression separated by an equal sign. It works the same way as a match, where the expression is given to the match and the pattern is its first arm

The code in the `if let` block isnt run if the value doesn't match the pattern

You can include an `else` with an `if let`. The block of code that goes with the `lese` is the same as the block of code that would go with the `_` case in the `match` expression

```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1
}
```
using `if let else`
```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}", state);
} else {
    count += 1;
}
```

## Chapter 7 - Managing Growing Projects with Packages, Crates, and Modules
* Packages: A cargo feature that lets you build, test and share creates
* Creates: A tree of modules that produces a library or executable
* Modules and use: Let you control the organization, scope and privacy of paths
* Paths: A way of naming an item, such as a struct, function, or module

### Packages and Crates
A *package* is one or more creates that provide  a set of functionality. A package contains a Cargo.toml files that describes how to build those crates

A *crate* can be a binary crate or a library crate. *Binary Crates* are programs you can compile to an executable that you can run, such as a command-line program or a server They must have a function called main that defines what happns when the executable runs

*Library Crates* don't have a `main` function, they don't compile to an executable. They define functionality intended to be shared with multiple projects.

The *crate root* is a source file that the Rust compiler starts from and makes up the root module of your crate

A package can contain at most one library create. It can contain as many binary crates as youd like, but it must contain at least one crate

### Defining Modules to Control Scope and Privacy
#### Modules Quick Reference
How modules, paths, the `use` keyword and the `pub` keyword work in the compiler:
* **Start from the crate root**: When compiling a crate, the compiler first looks in the crate root file
* **Declaring Modules**: In the crate root file, you can declare a new module named, say, "garden" with `mod garden;` The compiler will look for the code inside the module in these places:
    * Inline
    * in the file *src/garden.rs*
    * In the file *src/garden/mod.rs*
* **Declaring Submodules**: In any file other than the crate root thats being compiled as part of the crate, you can declare submodules (Ex: `mod vegetables` inside `src/garden`) The compiler will look for the code inside submodules in these places within a directory named for the parent module:
    * Inline, directly following the declaration
    * in the files *src/garden/vegtables.rs*
    * In the file *src/garden/vegetables/mod.rs*
* **Paths to code in modules**: Once a module is being compiled as part of your crate, you can refer to code in that module (Ex: an `Asparagus` type in the garden vegetables module) from anywhere else in theis crate by using the path `crate::garden::vegetables::Asparagus` as long as the privacy rules allow
* **Private vs Public**: Code within a module si private from its parent modules by default. To make a module public, declare it with `pub mod` instead of `mod`. To make items within a public module public as well, use `pub` before their declarations
* **The `use` keyword**: Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths

Here's a binary crate named `backyard` that illustrates these rules
```
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```
The crate root file, *src/main.rs* contains:

```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
```
The `pub mod garden` means the compiler includes the code it finds in *src/garden.rs* 

#### Defining a module
Start with the `mod` keyword and then specify the name of the module, place curly brackets around the body of the module.

Inside modules you can have other modules (i.e. submodules) or structs, enums, constans, traits, or functions

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
```
By using modules, you group related definitions together and name why thy're related. 

*src/main.rs* and *src.lib.rs* are called crate roots. Either of these files form a module named crate at the root of the crate's module structure
```
crate
└── front_of_house
    ├── hosting
    │   ├── add_to_waitlist
    │   └── seat_at_table
    └── serving
        ├── take_order
        ├── serve_order
        └── take_payment
```
If Module A is contained inside module B, A is the *child* of B and B is the *parent* of A

### Paths for Referring to an item in the Module Tree
When you want to call a function you need to know its path. A path can take two froms:
* An *absolute path* starts from a crate root by using a crate name or a literal crate
* A *relative path* starts from the current module ans uses `self`, `super` or an identifier in the current module

Both absolute and relative paths are followed by one or more identifiers separated by double colons (::)

Examples
```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}
pub fn eat_at_resturant() {
    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path
    front_of_house::hosting::add_to_waitlist();
}
```

Modules define Rust's *privacy boundary*: the line that encapsulates the implementation details external code ins't allowed to know about, call, or rely on.  So if you want to make an item private, you put it in a module

There is an error in the code above, hosting is private and cannot be accessed in `eat_at_resturant()`

You can expose inner parts of child modules code to outer ancestor modules by using the `pub` keyword

### Exposing Paths with the pub Keyword
We want the `eat_at_resturant` function in the parent module to have access to the `add_to_waitlist` function in the child module

```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}
```
This would still result in an error. We can access front_of_house and hosting but the contains of hosting are still private. **Making the module public doesn't make its contents public** THe pub keyword on a module only lets code in its ancestor refer to it

Making the change `fn add_to_waitlist() {}` -> `pub fn add_to_waitlist() {}` and now the code will compile.

The `front_of_house` module isn't public, but because the `eat_at_resturant` function is defined in the same module as `front_of_house`(i.e. they are siblings) one can refere to `front_of_house` from `eat_at_resturant`

### Starting Relative Paths with super
You can construct relative paths that begin in the parent module by using super at the start of the path. **This is like starting a filesystem path with the `..` syntax**

```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order(); 
        super::serve_order();
    }
    fn cook_order() {}
}
```
`fix_incorrect_order` is in `back_of_house`, using `super` we can go to the parent module of `back_of_house` which is the crate in this case. 

### Making Structs and Enums Public
If we use pub before a struct definition, we make the struct public, but the struct's fields will still be private. We can make each field public or not on a case-by-case basis

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile
    // meal.seasonal_fruit = String::from("blueberries");
}
```

Because `back_of_house::Breakfast` has a private field, the struct needs to provide a public associated funtion that constructs an instance of Breakfast. If Breakfast didn't have such a function, we couldnt create an instance of `Breakfast` in `eat_at_resturant` because we couldn't set the value of the private `seasonal_fruit` field in `eat_at_restaurant`

If we make an enum public, all of its variants are then public. We only need the `pub` before the `enum` keyword

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

### Bringing Paths into Scope with the use Keyword
There is a way to simplify paths. We can bring a path into scope once and then call the items in that path as if they're local items with the `use` keyword

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```
Adding `use` and a path in a scope is similar to creating a symbolic link in the filesystem. By adding `use crate::front_of_house::hosting` in the crate root, `hosting` is now a valid name in that scope

Another way:
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
```
The first way to do it is the idiomatic way to bring a function into scope with `use`. Bringing the function's parent module into scope with `use` means we have to specify the parent module when calling the function

Specifying the parent module when calling the function makes it clear that the function isn't locally defined while still minimizing repetition of the full path

WHen bringing in structs, enums, and other items with `use`, it's idiomatic to specify the full path
```rust
use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new(); 
    map.insert(1, 2);
}
```

The exception to this idiom is if we're  bringing two items with the same name into scope with `use` statements
```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // Hello
}

fn function2() -> io::Result {
    // Hello
}
```

#### Providing New Names with the as Keyword
Another solution is using the keyword `as` and specifying a new local name or alias for the type
```rust
use std::fmt::Result;
use std::io::Result as IoResult;
fn function1() -> Result {
    // Snip
}
fn function2() -> IoResult<()> {
    // Snip
}
```

#### Using External Packages
```rust
use std::cmp::Ordering;
use std::io;
```
Can become
```rust
use std::{ cmp::Ordering, io };
```
Another Example
```rust
use std::io;
use std::io::Write;
```
Can become
```rust
use std::{ self, Write };
```

#### The Glob Operator
IF you want to bring *all* public items defined in a path into scope, we can specify that path followed by *, the glob operator
```rust
use std::collections::*;
```

### Separating Modules into Different Files
The following code has been written in the root file i.e. *src/main.rs*
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```
If we want to remove `front_of_house` to its own file, remove the code inside the curly brackets for the module leaving only `mod front_of_house;` declaration

```rust
mod front_of_house;

pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

You would add the code of `front_of_house` to *src/front_of_house.rs*

Now we want to move `hosting` that is a child of `front_of_house`. To do this, `hosting` will be in a directory named for its place in the module tree

Change *src/front_of_house.rs* to cotain only the declaration of the `hosting` module
```rust
pub mod hosting;
```
Then we create a *src/front_of_house* directory and a file *src/front_of_house/hosting.rs* to contain definitions made in the `hosting` module

The rules the compiler follows to know what files to look in for modules' code:
* For a module named `front_of_house` declare in the crate root, the compiler will look for the module's code in:
    * *src/front_of_house.rs* 
    * *src/front_of_house/mod.rs*
* For a module named `hosting` that is a submodule of `front_of_house`, the compiler will look for the module's code in:
    * *src/front_of_house/hosting.rs*
    * *src/front_of_house/hosting/mod.rs*

## Chapter 8 - Common Collections
* A *vector* allows you to store a variable number of values next to each other
* A *string* is a collection of characters
* A *hash map* allows you to associate a value with a particular key

### Sotring Lists of Values with Vectors
Vectors can only store values of the same type. 

Creating a New Vector
```rust
let v: Vec<i32> = Vec::new();
```
Rust provides the `vec!` macro which can create a new vector that holds the values you give it

```rust
let v = vec![1, 2, 3];
```

Adding to a vector:
```rust
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
```

Like any other struct, a vector is freed when it goes out of scope
```rust
{
    let v = vec![1, 2, 3, 4];
    // do stuff with v
} // <- v goes out of scope and is freed
```

#### Reading Elements of Vectors
Two ways to reference a value stored in a vector:
* indexing 
* using the `get` method 

```rust
let v = vec![1, 2, 3, 4];
let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element.")
}
```
Indexing, or using & and [] can lead to the programming crashing when there is an out of bounds error

Using `get` would would return `Option<&T>` where you can use
```rust
match vec.get(i) {
    Some(T) => // Do something
    None => // DO SOmething
}
```

When the program has a valid reference, the borrow checker enforces the ownership and borrowing rules to ensure this reference and any other references to the contents of the vector remain valid

```rust
let mut v = vec![1, 2, 3, 4, 5];
let first = &vec[0];
v.push(6);
println!("{}", first);
```
The above code would cause a compilation error

#### Iterating over the Values in a Vector
Immutable references
```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```
Mutable References
```rust
let v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```
To change the value that the mutable references refers to we have to use the `*` dereference operator to get to the value in `i` before we can use the `+=` operator

#### Using an Enum to Store Multiple Types
You can use enums to store multiple types in vectors since an enum is one type and vectors can only hold one type
```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}
let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Float(3.4);
    SpreadsheetCell::Text(String::from("blue"))
];
```

### Storing UTF-8 Encoded Text with Strings
Three reasons why beninners get stuck on string:
* Rust's propensity for exposing possible errors
* String being a more complicated datas tructure
* UTF-8

#### What is a String?
Rust has one string type in the core language, which is the string slice `str` that is usually seen in its borrowed form `&str`. String literals are stored in the program's binary and are therefore string slices

#### Creating a New String
`new` function creates a string
```rust
let mut s = String::new();
```
This creates a new empty string called `s`. Often we have initial data. for that we can use the `to_string` method

```rust
let data = "initial contents";
let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();
```
We can also use the function `String::from` to create a `String` from a string literal

#### Updating a string
A `String` can grow in size and its contents can change. you can use the `+` 

## Chapter 9 - Error Handling

## Chapter 10 - Generic Types, Traits, and Lifetimes

## Chapter 11 - Writing Automated Tests

## Chapter 13 - Functional Language Features: Iterators and Closures

## Chapter 15 - Smart Pointers

## Chapter 16 - Fearless Concurrency

## Chapter 17 - Object Oriented Programming Features of Rust

## Chapter 18 - Patterns and Matching

## Chapter 19 - Advanced Features



