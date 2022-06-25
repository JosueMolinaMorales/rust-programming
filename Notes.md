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
    - [Indexing into String](#indexing-into-string)
      - [How Rust stores Strings](#how-rust-stores-strings)
    - [Methods for Iterating Over String](#methods-for-iterating-over-string)
    - [Stroring Keys with Associated Values in Hash Maps](#stroring-keys-with-associated-values-in-hash-maps)
      - [Creating a Hash Map](#creating-a-hash-map)
      - [Hash Maps and Ownership](#hash-maps-and-ownership)
      - [Accessing Values in a Hash Map](#accessing-values-in-a-hash-map)
      - [Insert a value if the key has no value](#insert-a-value-if-the-key-has-no-value)
      - [Updating a Value based on the Old Value](#updating-a-value-based-on-the-old-value)
      - [Hashing functions](#hashing-functions)
  - [Chapter 9 - Error Handling](#chapter-9---error-handling)
    - [Unrecoverable Errors with `panic!`](#unrecoverable-errors-with-panic)
      - [Using a `panic!` Backtrace](#using-a-panic-backtrace)
    - [Recovering Errors with `Rusult`](#recovering-errors-with-rusult)
      - [Matching on Different Errors](#matching-on-different-errors)
      - [Shortcuts for Panic on Error: unwrap and expect](#shortcuts-for-panic-on-error-unwrap-and-expect)
      - [Propagating Errors](#propagating-errors)
      - [A Shortcut for Propagating Errors: The ? Operator](#a-shortcut-for-propagating-errors-the--operator)
  - [Chapter 10 - Generic Types, Traits, and Lifetimes](#chapter-10---generic-types-traits-and-lifetimes)
    - [Generic Data Types](#generic-data-types)
      - [In Function Definitions](#in-function-definitions)
      - [In Struct Definitions](#in-struct-definitions)
      - [In Enum Definitions](#in-enum-definitions)
      - [In Method Definitions](#in-method-definitions)
    - [Traits: Defining Shared Behavior](#traits-defining-shared-behavior)
      - [Defining a Trait](#defining-a-trait)
      - [Implementing a Trait on a Type](#implementing-a-trait-on-a-type)
      - [Default Implementations](#default-implementations)
      - [Traits as Parameters](#traits-as-parameters)
      - [Trait Bound Syntax](#trait-bound-syntax)
      - [Specifying Multiple Trait Bounds with the + Syntax](#specifying-multiple-trait-bounds-with-the--syntax)
      - [Returning Types that Implements Traits](#returning-types-that-implements-traits)
    - [Validating References with Lifetimes](#validating-references-with-lifetimes)
      - [Preventing Dangling References with Lifetimes](#preventing-dangling-references-with-lifetimes)
      - [The Borrow Checker](#the-borrow-checker)
      - [Generic Lifetimes in Functions](#generic-lifetimes-in-functions)
      - [Lifetime Annotation Syntax](#lifetime-annotation-syntax)
      - [Lifetime Annotations in Function Signatures](#lifetime-annotations-in-function-signatures)
      - [Thinking in Terms of Lifetimes](#thinking-in-terms-of-lifetimes)
      - [Lifetime Annotations in Method Definitions](#lifetime-annotations-in-method-definitions)
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
A `String` can grow in size and its contents can change. you can use the `+` or the `format!` macro to concatenate `String` values

We can grow a `String` using the `push_str` method
```rust
let mut s = String::from("foo");
s.push_str("bar");
```
`push` method takes a single chracter as a parameter and adds it to the `String`
```rust
let mut s = String::from("lo");
s.push('l');
```

Using the `+` operator
```rust
let s1 = String::from("Hello, ");
let s2 = String::from("World!");
let s3 = s1 + &s2; // Note s1 has been moved here and can no longer be used
```
The reason `s1` can no longer be used and `&s2` was used is because of signature of the method that called when the `+` is used.
```rust
fn add(self, s: &str) -> String {}
```
So since `let s3 = s1 + &s2` looks like it will copy both strings and create a new one, this statement actually takes ownership of `s1`, appends a copy of the contents of `s2` and then returns ownership of the result

For more complicated string combining, we can use `format!` macro:
```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{}-{}-{}", s1, s2, s3);
```
`format` uses references to the string so it does not take ownership

### Indexing into String
If you try to access parts of a string using indexing syntax in Rust, you'll get an error
```rust
let s1 = String::from("Hello");
let h = s1[0];
```

#### How Rust stores Strings
A string is a wrapper over a `Vec<u8>`
```rust
let hello = String::from("Hola");
```
`Hola` uses 4 to be stored Using UTF-8
```rust
let hello = String::from("Здравствуйте");
```
You would think `Здравствуйте` uses 12 bytes but Rust uses 24 bytes using UTF-8 to store this string. Indexing this thinking its length is 12 would not be good

If you really need to use indices to create string slices, you use a string slice
```rust
let hello = "Здравствуйте";
let s = &hello[0..4];
```
`s` will store the first 4 bytes of `hello`

### Methods for Iterating Over String
The best way to operate on pieces of string is to be explicit about whether you want characters or bytes

For individual Unicode scalar values, use the `chars` method.
`Code in ./8-Common-Collections/collection-practice`

### Stroring Keys with Associated Values in Hash Maps
The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V` using a *hashing function*, which determines how tit places these keys and values into memory

#### Creating a Hash Map
One way is to create an empty hash map and adding elements with `insert`
```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```
Like vectors, hash maps must have all of the keys be the same type and all of the values be the same type

Another way of constructing a hash map is by using interators and the `collect` method on a vector of tuples, where each tuple consists of a key and its value
```rust
use std::collections:HashMap;
let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];
let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
```
The `collect` method gathers data into a number of collection types, including `HashMap`. `zip` method combines two serpate vectors and creates an interator of tuples

The type annotation `HashMap<_, _>` is needed here because it's possible to `collect` into many different data structures and Rust doesn't know which you want unless you specify

#### Hash Maps and Ownership
For types that implement the `copy` trait like `i32` the values are copied into the hash map. For owned values like `String`, the values will be moved and the has map will be the owner of those values

If we insert references to values into the hash map, the values won't be moved into the hash map.
The values that the references point to must be valid for at least as long as the hash map is valid

#### Accessing Values in a Hash Map
We can get a value out of a hash map by providing its key to the `get` method
```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
let team_name = String::from("Blue");
let score = scores.get(&team_name);
```
`get` returns an `Option<&v>` if there is no value for that key in the hash amp then `get` will return `None`

#### Insert a value if the key has no value 
It's common to check whether a particular key has a value and, if it doesn't, insert a value for it.

`entry` takes the key you want to check as a paramater. The return value of the `entry` method is enum called `Entry` that represents a value that might or might not exist
```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);
```
The `or_insert` method on the `Entry` is defined to return a mutable reference to the value for the corresponding `Entry` key if that key exists, and if not, inserts the parameter4 as the new value for this key and returns a mutable reference to the new value

#### Updating a Value based on the Old Value
Another common use case for hash maps is to look up a key's value and then update it based on the old Value
```rust
use std::collections::HashMap;
let text = "hello world wonderful world!";
let mut map = HashMap::new();
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```

#### Hashing functions
By default, Hash Map uses a hashing function called *SlipHash* that can provide resistance to Denial of Service attacked involving hash tables

## Chapter 9 - Error Handling
Rust groups errors into two major categories: *recoverable* and *unrecoverable* errors
* Recoverable errors are things like *file not found* error, we most likely just want to report the problem to the user
* Unrecoverable errors are always symtoms of bugs. 
Rust doesnt have exceptions. Instead, it has the type `Result<T, E>` for recoverable errors and the `panic!` macro that stops  execution when the program encounters an unrecoverable error.

### Unrecoverable Errors with `panic!`
When the `panic!` macro executes, your program will print a failure message, unwind and clean up the stack, and then quit.

By default, when a panic occurs, the program starts *unwinding*, which means Rust walks back up the stack and cleans up the data from each function it encounters. This takes a lot of work. Rust allows you to choose the alternative of immediately *aborting*, which ends the program without cleaning up. Memory that the program was using will then need to be cleaned up by the operating system. 

#### Using a `panic!` Backtrace
```rust
fn main() {
    let v = vec![1, 2, 3];
    v[99];
}
```
This code attempts to acces an index in vector beyond the range of valid indexes. Rust will panic. 

We can set the `RUST_BACKTRACE` environment variable to get a backtrace of exectly what happened to cause the error. 

### Recovering Errors with `Rusult`
Most errors aren't serious enought to require the program to stop entirely.

The `Result` enum is defined as having two variants, `Ok`and `Err`:
```rust
enum Result<T, E> {
    Ok(T),
    Err(E)
}
```
The `T` and `E` are generic type parameters. `T` represents the type of the value that will be returned in a sucess case within the `Ok` variant, and `E` represents the type of the error that will be returned in a failure case

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
}
```

#### Matching on Different Errors
If we want to take different actions for different failure reasons

The type of the value that `File::open` returns inside the `Err` variant is `io::Error`, which is a struct. This struct has a method `kind` that we can call to get an `io::ErrorKind` value. 

Here's another way to write the same logic but using closues and the `unwrap_or_else` method:
```rust 
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

#### Shortcuts for Panic on Error: unwrap and expect
The `unwrap` method is a shortcut method implemented just like the `match` expression. If the `Result` value is the `Ok` variant, `unwrap` will return the value inside the `Ok`. If the `Result` is the `Err` variant, `unwrap` will call the `panic!` macro for us

```rust
use std::fs::File;
fn main() {
    let f = File::open("hello.txt").unwrap();
}
```
Similarly, the `expect` method lets us also choose the `panic!` error message. Using `expect` instead of `unwrap` and providing good error messages can convey your intent 
```rust
use std::fs::File;
fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

#### Propagating Errors
When a function's implemetaion calls something that might fail, the function can handle the error or it can return the error to the calling code.
```rust
use std::fs::File;
use std::io::{ self, Read };
fn read_username_from_file() -> Result<string, io::Error> {
    let f = File::open("Hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}
```

#### A Shortcut for Propagating Errors: The ? Operator
The pattern of propagating errors is so common in Rust that Rust provides the question mark `?`
```rust
use std::fs::File;
use std::io::{ self, Read };
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```
The `?` placed after a `Result` value is defined to work in almost the same wasy as the `match` expressions.

Error values that have the `?` operator called on them go through the `from` function, defined in the `From` trait in the standard library

We can chain methods called immediately after the `?`
```rust
use std::fs::File;
use std::io::{ self, Read };
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```
## Chapter 10 - Generic Types, Traits, and Lifetimes
*generics*: abstract stand-ins for concrete types or other properties

*traits*: Define behavior in a generic way

*lifetimes*: a variety of generics that give the compiler information about how references relate to each other

### Generic Data Types
#### In Function Definitions
To parameterize the types in a new single function, we need to name the type parameter, just as we do for the value parameters. You can use any identifier as a type parameter name. 
```rust
fn largest<T>(list: &[T]) -> T {}
```
Example:
```rust
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```
The above code will not compile. The follow error message is displayed:
```sh
Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0369]: binary operation `>` cannot be applied to type `T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- T
  |            |
  |            T
  |
help: consider restricting type parameter `T`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
  |             ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
error: could not compile `chapter10` due to previous error
```
The error mentions `std::cmp::PartialOrd` which is a *trait*. This error states that this function will not work with all types of `T`. 

To enable comparison, the standard library has the `std::cmp::PartialOrd` trait that you can implement on types

#### In Struct Definitions
```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

#### In Enum Definitions
```rust
enum Option<T> {
    Some(T),
    None
}
```

#### In Method Definitions
```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}
```
We can also specify constraints on generic types when defining methods on the type. Example:
```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```
This code means the type `Point<f32>` will have a `distance_from_origin` method, other instances of `Point<T>` where `T` is not of type `f32` will not have thid method defined

### Traits: Defining Shared Behavior
A *trait* defines functionality a particular type has and can share with other types. 

Traits are similar to *interfaces* in other languages

#### Defining a Trait
A types behavior consists of the methods we can call on that type. Different types share the same behavior if we can call the same methods on all of those types
```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```
Here we define a trait using the `trait` keyword and then the trait's name. Inside the curly bracket, we declare the method signatures that describe the behaviors of the types that implement this trait

Each type implementing this trait must provide its own custom behavior for the body of the method

#### Implementing a Trait on a Type
```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> {
        format!("{}, by {}, ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

#### Default Implementations
Instead of requiring implemetations for all methods on every type, you can hav default default behaviors

#### Traits as Parameters
```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```
This methods takes any type that implements the `Summary` trait

#### Trait Bound Syntax
The `impl Trait` syntax works but its syntax sugar for a longer form known as a trait bound:
```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());   
}
```

#### Specifying Multiple Trait Bounds with the + Syntax
If you need a type to implement multiple traits you can do the following:
```rust
pub fn notify(item: &(impl Summary + Display)) {}
```

Using too many traits can be a downside, you can use the `where` clause:
```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
```
to
```rust
fn some_function<T, U>(t: &T, u: &U) -> i32 
    where T: Display + Clone,
          U: Clone + Debug
{}
```

#### Returning Types that Implements Traits
```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            ".."
        )
    }
}
```

### Validating References with Lifetimes
Lifetimes are another kind of generic that we've already been using. Rather than unsuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be

Every reference in Rust has a *lifetime*, which is the scope for which that reference is valid. Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred. 

#### Preventing Dangling References with Lifetimes
The main aim of lifetimes is to prevent *dangling references* which cause a program to refence data other than the data its intended to reference. 

```rust
{
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r);
}
```
The outer scope declares a variable named `r` with no inital value, and the inner scope declares a variable named `x` with the inital value of 5. This code causes the following error:

```sh
Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0597]: `x` does not live long enough
  --> src/main.rs:7:17
   |
7  |             r = &x;
   |                 ^^ borrowed value does not live long enough
8  |         }
   |         - `x` dropped here while still borrowed
9  | 
10 |         println!("r: {}", r);
   |                           - borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `chapter10` due to previous error
```
If Rust allowed this code to work, `r` would be referencing memory that was deallocated when `x` went out of scope, and anything we tried to do with `r` wouldn't work correctly.

#### The Borrow Checker
The Rust compiler has a *borrow checker* that compares scopes to determine whether all borrows are valid

#### Generic Lifetimes in Functions
```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```
The above code does not compile. The return type needs a generic lifetime parameter on it because Rust can't tell whether the reference being returned refers to x or y. 

When defining the function, we don't know the concrete values that will be passed into this function, so we don't know whether the `if` case or the `else` case will execute.

To fix this error, we'll add generic lifetime parameters that define the relationship between the references so the borrow checker can perform its analysis

#### Lifetime Annotation Syntax
Lifetime annotations don't change how long any of the references live. Rather, they describe the relationships of the lifetimes of multiple referneces to each other without affecting the lifetimes. Functions can accept references with any lifetime by specifying a generic lifetime parameter

Lifetime annotations have a slightly unusual syntax: the names of the lifetime parameters must start with an apostrophe (`'`) and are usually all lowercase and very short We place lifetime parameter annotations after the `&` of a reference, using a space to separate the annotation from the reference's type

```rust
&i32            // a reference
&'a i32         // a reference with an explicit lifetime
&'a mut i32     // a mutable reference with an explicit lifetime
```
One lifetime annotation by itself doesn't have much meaning, because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other. 

Example: If we have a function with the parameter `first` that is a reference to an `i32` with lifetime `'a` The function also has another paremeter named `second` reference to an `i32` that also has the lifetime `'a`. The lifetime annotations indicate the references `first` and `second` must both live as long as that generic lifetime

#### Lifetime Annotations in Function Signatures
Going back to the `longest` function. 
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
We need to declare generic lifetime parameters inside angle vrackets between the function name and the parameter list. We want the signature to express the following constraint: the returned reference will be valid as long as both the parameters are valid. 

The function signature now tells Rust that for some lifetime `'a`, the function takes two parameters, both of which are string slices that live at least as long as lifetime `'a`. In practice, it means that the lifetime of the reference returned by the `longest` function is the same as the smaller of the lifetimes of the references passed in 

When we specify the lifetime parameters in this function signature, we're not changing the lifetimes of any values passed in or returned. Rather, we're specifying that the borrow checker should reject any values that don't adhere to these contraints. 

When we pass concrete references to `longest`, the concrete lifetime that is substituted for `'a` is the part of the scope of `x` that overlaps with the scope of `y`. In other words the generic lifetime of `'a` will get the concreate lifetime that is equal to the smaller of the lifetimes of `x` and `y`. The returned references will also be valid for the length of the smaller of the lifetimes of `x` and `y`

```rust
fn main() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```
string1 is valid until the end of the outer scope, string2 is valid until the end of the inner scope, and result references something that is valid until the end of the inner scope. 

The following will not compile
```rust
fn main() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```
`result`'s lifetime is the same of `string2`

#### Thinking in Terms of Lifetimes
The way in which you need specify lifetime parameters depends on what your function is doing.

When returning a reference from a function, *the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters*. If the reference returned does not refer to one of the parameters, it must refer to a value created within this function. However this would be a dangling reference because the value will go out of scope at the end of the function.  
```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```
The following function won't compile. Even though we've specified a lifetime parameter `'a` for the return type, this implementation will fail to compile because the return value lifetime is not related to the lifetime of the parameters at all. 

The best fix would be to return an owned data type rather than a reference so the calling function is then responsible for cleaning up the value

#### Lifetime Annotations in Method Definitions

## Chapter 11 - Writing Automated Tests

## Chapter 12 
Files can be found in `/12-Building-CLI-Program`

#### Guidelines for splitting up main()

* Split your program into a main.rs and a lib.rs and move your program's logic to lib.rs
* main.rs should handle setting up configurations, calling a run function in lib.rs, and handling errors

#### Returning Errors
`Result<(), Box<dyn Error>>`, `Box<dyn Error>` is a trait object. It means the function will return a type that implements the Error trait

The `dyn` keyword is short for dynamic

## Chapter 13 - Functional Language Features: Iterators and Closures

### Closures: Anonymous Functions that can capture their environment
Rust's closures are anonymous functions you can save in a variable or pass as arguments to other functions. You can create the closue in one place then call the closure to evaluate it in a different context.

### Processing a Series of Items with Iterators
An iterator is responsible for the logic of iterating over each item and determining when the sequences has finished.

In Rust, iterators are *lazy* meaning they have no effect until you call methods that consume the iterator to use it up.

```rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();

for val in v1_iter {
    println!("{}", val);
}
```

#### The Iterator trait and the next method
All iterators implement a trait named `Iterator` that is defined in the standard library.

#### Iterator Adapators
`Map`
```rust
let v1: Vec<i32> = vec![1,2,3];
let v2: Vec<_> = v1.iter().map(|x| x+1).collect();
```

`Filter` takes a closure that takes each item from the iterator and returns a boolean. If the closure returns true, the value will be included in the iterator produced by filter.
```rust
let v1: Vec<i32> = vec![4, 1, 5, 12];
let greater_than_four = v1.iter().filter(|x| x > 4).collect();
```

## Chapter 15 - Smart Pointers
A pointer is a general concept for a variable that contains an address in memory. This address refers to, or points at, some other data. The most common kind of pointer in Rust is a reference.

Smart Pointers, are data structures that not only act like a pointer but also have additional metadata and capabilities. Smart pointers own the data they point to

Smart pointers are implemented using structs and implement the `Deref` and `Drop` traits.

`Deref` traits allows an instance of the smart pointer struct to behave like a references so you can write code that works with either references or smart pointers

`Drop` trait allows you to customize the code that is run when an instance of the smart pointer goes out of scope.

Common Smart pointers:
* `Box<T>` for allocating values on the heap
* `Rc<T>` a reference counting type that enables multiple ownership
* `Ref<T>` and `RefMut<T>` accessed through `RefCell<T>` a type that enforces the borrowing rules at runtime instead of compile time



## Chapter 16 - Fearless Concurrency

## Chapter 17 - Object Oriented Programming Features of Rust

## Chapter 18 - Patterns and Matching

## Chapter 19 - Advanced Features



