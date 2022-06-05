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
| 16 bit | `i16 | `u16` |
| 32 bit | `i32` | `u32` |
| 64 bit | `i64` | `u64` |
| 128 bit | `i128` | `u128` |
| arch | `isize` | `usize |

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

### Ownership Rules
* Each value in Rust has a variable that's called its *owner*
* There can only be on owner at a time
* When the owner goes out of scope, the value will be dropped

### Variable Scope
A scope is the rnage whitin a program for which an item is valid

### The String Type
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
## Chapter 5 - Using Structs to Structure Related Data

## Chapter 6 - Enums and Pattern Matching

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



