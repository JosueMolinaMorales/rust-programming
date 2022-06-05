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

### Comments

### Control Flow

## Chapter 4 - Understanding Ownership

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



