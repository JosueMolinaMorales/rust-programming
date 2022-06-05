fn main() {
    /* Variables are immutable
    let x = 6;
    println!("The value of x is: {}", x);
    x=6;
    println!("The value of x is: {}", x); ERROR
    */
    let mut x = 6;
    println!("The value of x is: {}", x);
    x = 5;
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("There are {} seconcds in 3 hours", THREE_HOURS_IN_SECONDS);

    let y = 7;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value of y is: {} ", y);
}
/*  NOTES
    * By Default variables are immutable
    * There are constants but constants are different that variables

    Constants:
    * You are not allowed to use 'mut' with constants
    * Constants are always immutable
    * Declare constants using 'const' instead of 'let'
    * Constant variables must be type annotated

    Shadowing
    You can delcare a new variables with the same name as a previous variable
    The first variable is shadowed by the second, which means that the second variables value is what the program sees when the variable is used
    We are effectively creating a new variable when we use the let keyword again,
    So we can change the type of the value

    Types:
        Scalar Types:
        * Scalar types represents a single value
        * Integers
        * Floating point
        * Numbers
        * Booleans
        * Characters
    Integer Types
    8 bit -- Signed: i8 -- Unsigned: u8
    
*/