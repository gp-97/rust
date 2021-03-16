/*
    Borrowing is of 2 types:
        1. Shared borrowing:
            ->  piece of data being shared by single or multiple variables, but
                cannot be altered.
            ->  operand1 = & operand2
            ->  values are COPIED.

        2. Mutable borrowing:
            ->  A piece of data that is shared and altered by a single variable,
                but the data is inaccessible to other variables at that time.
            ->  operand1 = &mut operand2
            ->  values are MOVED.
*/

fn main() {
    let x = 10;
    let mut y = 13; //immutable reference to a variable
    let a = &x;

    println!("Value of a:{}", a);
    println!("Value of x:{}", x); // x value remains the same since it is immutably borrowed

    let b = &mut y; //mutable reference to a variable
    println!("Value of b:{}", b);

    *b = 11; // derefencing
    println!("Value of b:{}", b); // updated value of b
    println!("Value of y:{}", y); // y value can be changed as it is mutuably borrowed
}
